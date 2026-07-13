using System.Net;
using System.Net.Http.Headers;
using System.Text.Json;
using ApiClient = NimbusPulse.Client.Client;

namespace NimbusPulse.Client.Tests;

public sealed class ClientRequestTests
{
    private static readonly Guid ServerId = Guid.Parse("11111111-1111-1111-1111-111111111111");
    private static readonly Guid ProductId = Guid.Parse("44444444-4444-4444-4444-444444444444");

    private const string InstanceJson = """
        {
          "id": "11111111-1111-1111-1111-111111111111",
          "node_id": "22222222-2222-2222-2222-222222222222",
          "user_id": "33333333-3333-3333-3333-333333333333",
          "product_id": "44444444-4444-4444-4444-444444444444",
          "game_type": "dcs",
          "billing_type": "hourly",
          "port": 10308,
          "webgui_port": 8088,
          "ftp_port": 21,
          "ftp_username": "pilot",
          "ftp_password": "secret",
          "pid": null,
          "status": "ServerStarted",
          "want_delete": false,
          "wanted_terrains": ["Caucasus"],
          "rented_at": 1700000000,
          "rented_until": null,
          "active_mods": ["srs"],
          "subscription_cancel_at_period_end": false,
          "created_at": "2026-01-02T03:04:05Z",
          "dcs_settings": null
        }
        """;

    [Fact]
    public async Task HealthAsync_SendsPerRequestBearerTokenWithoutMutatingInjectedDefaultHeaders()
    {
        var handler = RecordingHttpMessageHandler.Returning(statusCode: HttpStatusCode.NoContent);
        using var httpClient = new HttpClient(handler);
        httpClient.DefaultRequestHeaders.Authorization =
            new AuthenticationHeaderValue("Basic", "injected-credential");
        using var client = new ApiClient("test-api-key", httpClient);

        await client.HealthAsync();

        var request = Assert.Single(handler.Requests);
        Assert.Equal(HttpMethod.Get, request.Method);
        Assert.Equal(new Uri("https://coordinator.nimbuspulse.com/health"), request.Uri);
        Assert.Equal("Bearer", request.Authorization?.Scheme);
        Assert.Equal("test-api-key", request.Authorization?.Parameter);
        Assert.Equal("Basic", httpClient.DefaultRequestHeaders.Authorization?.Scheme);
        Assert.Equal("injected-credential", httpClient.DefaultRequestHeaders.Authorization?.Parameter);
    }

    [Fact]
    public async Task HealthAsync_IgnoresInjectedBaseAddressAndUsesFixedCoordinatorUrl()
    {
        var handler = RecordingHttpMessageHandler.Returning(statusCode: HttpStatusCode.NoContent);
        using var httpClient = new HttpClient(handler)
        {
            BaseAddress = new Uri("https://ignored.example/"),
        };
        using var client = new ApiClient("key", httpClient);

        await client.HealthAsync();

        Assert.Equal(
            new Uri("https://coordinator.nimbuspulse.com/health"),
            Assert.Single(handler.Requests).Uri);
    }

    [Fact]
    public void Dispose_DoesNotDisposeInjectedHttpClientOrHandler()
    {
        var handler = RecordingHttpMessageHandler.Returning(statusCode: HttpStatusCode.NoContent);
        using var httpClient = new HttpClient(handler);
        var client = new ApiClient("key", httpClient);

        client.Dispose();

        Assert.False(handler.IsDisposed);
    }

    [Fact]
    public async Task HealthAsync_PropagatesCancellationToHttpHandler()
    {
        var enteredHandler = new TaskCompletionSource(TaskCreationOptions.RunContinuationsAsynchronously);
        var handler = new RecordingHttpMessageHandler(async (_, cancellationToken) =>
        {
            enteredHandler.SetResult();
            await Task.Delay(Timeout.InfiniteTimeSpan, cancellationToken);
            return RecordingHttpMessageHandler.Response(statusCode: HttpStatusCode.NoContent);
        });
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);
        using var cancellation = new CancellationTokenSource();

        var request = client.HealthAsync(cancellation.Token);
        await enteredHandler.Task;
        cancellation.Cancel();

        await Assert.ThrowsAnyAsync<OperationCanceledException>(() => request);
    }

    [Fact]
    public async Task CreateServerAsync_SendsExpectedWirePayload()
    {
        var handler = RecordingHttpMessageHandler.Returning(InstanceJson, HttpStatusCode.Created);
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);
        var request = new CreateServerRequest
        {
            Name = "Training server",
            BillingType = BillingType.Monthly,
            Region = Region.Germany,
            Password = null,
            MaxPlayers = 24,
            ProductId = ProductId,
            ActiveMods = ["srs", "tacview"],
            Terrains = [Terrain.Caucasus, Terrain.PersianGulf],
            UseVoiceChat = true,
            EnableIo = true,
            EnableOs = false,
            EnableLfs = true,
        };

        var instance = await client.CreateServerAsync(request);

        Assert.Equal(ServerId, instance.Id);
        var sent = Assert.Single(handler.Requests);
        Assert.Equal(HttpMethod.Post, sent.Method);
        Assert.Equal(new Uri("https://coordinator.nimbuspulse.com/game_servers"), sent.Uri);
        Assert.Equal("application/json", sent.ContentType?.MediaType);
        using var document = JsonDocument.Parse(sent.Body!);
        var root = document.RootElement;
        Assert.Equal(ProductId.ToString(), root.GetProperty("product_id").GetString());
        Assert.Equal("monthly", root.GetProperty("billing_type").GetString());
        Assert.Equal("de", root.GetProperty("region").GetString());
        Assert.Equal(["srs", "tacview"], Strings(root.GetProperty("active_mods")));
        Assert.Equal(["Caucasus", "PersianGulf"], Strings(root.GetProperty("wanted_terrains")));

        var settings = root.GetProperty("settings");
        Assert.Equal("Training server", settings.GetProperty("initial_server_name").GetString());
        Assert.Equal(string.Empty, settings.GetProperty("initial_server_password").GetString());
        Assert.Equal(24, settings.GetProperty("initial_max_players").GetInt32());
        Assert.True(settings.GetProperty("initial_use_voice_chat").GetBoolean());
        Assert.True(settings.GetProperty("enable_io").GetBoolean());
        Assert.False(settings.GetProperty("enable_os").GetBoolean());
        Assert.True(settings.GetProperty("enable_lfs").GetBoolean());
    }

    [Fact]
    public async Task UpdateServerAsync_SendsTaggedDcsSettingsPayload()
    {
        var handler = RecordingHttpMessageHandler.Returning(ResourceJson());
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);
        var request = new UpdateServerRequest
        {
            EnableIo = true,
            EnableOs = false,
            EnableLfs = true,
        };

        var server = await client.UpdateServerAsync(ServerId, request);

        Assert.Equal(ServerId, server.Id);
        var sent = Assert.Single(handler.Requests);
        Assert.Equal(HttpMethod.Put, sent.Method);
        Assert.Equal(
            new Uri($"https://coordinator.nimbuspulse.com/game_servers/{ServerId}"),
            sent.Uri);
        using var document = JsonDocument.Parse(sent.Body!);
        var root = document.RootElement;
        Assert.Equal("dcs", root.GetProperty("game_type").GetString());
        var settings = root.GetProperty("settings");
        Assert.True(settings.GetProperty("enable_io").GetBoolean());
        Assert.False(settings.GetProperty("enable_os").GetBoolean());
        Assert.True(settings.GetProperty("enable_lfs").GetBoolean());
    }

    [Fact]
    public async Task ChangeServerTerrainsAsync_SendsTerrainArrayAndAcceptsEmpty204()
    {
        var handler = RecordingHttpMessageHandler.Returning(statusCode: HttpStatusCode.NoContent);
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        await client.ChangeServerTerrainsAsync(
            ServerId,
            [Terrain.Syria, Terrain.MarianaIslandsWWII]);

        var sent = Assert.Single(handler.Requests);
        Assert.Equal(HttpMethod.Put, sent.Method);
        Assert.Equal(
            new Uri($"https://coordinator.nimbuspulse.com/game_servers/{ServerId}/terrains"),
            sent.Uri);
        using var document = JsonDocument.Parse(sent.Body!);
        Assert.Equal(["Syria", "MarianaIslandsWWII"], Strings(document.RootElement));
    }

    [Theory]
    [InlineData("start")]
    [InlineData("stop")]
    [InlineData("restart")]
    [InlineData("full_restart")]
    [InlineData("update")]
    public async Task LifecycleActionAsync_PostsToExpectedRoute(string action)
    {
        var handler = RecordingHttpMessageHandler.Returning(InstanceJson);
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        var instance = action switch
        {
            "start" => await client.StartServerAsync(ServerId),
            "stop" => await client.StopServerAsync(ServerId),
            "restart" => await client.RestartServerAsync(ServerId),
            "full_restart" => await client.FullRestartServerAsync(ServerId),
            "update" => await client.UpdateGameServerAsync(ServerId),
            _ => throw new ArgumentOutOfRangeException(nameof(action)),
        };

        Assert.Equal(ServerId, instance.Id);
        var sent = Assert.Single(handler.Requests);
        Assert.Equal(HttpMethod.Post, sent.Method);
        Assert.Equal(
            new Uri($"https://coordinator.nimbuspulse.com/game_servers/{ServerId}/{action}"),
            sent.Uri);
        Assert.Null(sent.Body);
    }

    [Fact]
    public async Task GetMethods_UseCollectionAndItemRoutes()
    {
        var responses = new Queue<string>(["[]", ResourceJson()]);
        var handler = new RecordingHttpMessageHandler((_, _) =>
            Task.FromResult(RecordingHttpMessageHandler.Response(responses.Dequeue())));
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        var servers = await client.GetServersAsync();
        var server = await client.GetServerAsync(ServerId);

        Assert.Empty(servers);
        Assert.Equal(ServerId, server.Id);
        Assert.Collection(
            handler.Requests,
            request =>
            {
                Assert.Equal(HttpMethod.Get, request.Method);
                Assert.Equal(
                    new Uri("https://coordinator.nimbuspulse.com/game_servers"),
                    request.Uri);
            },
            request =>
            {
                Assert.Equal(HttpMethod.Get, request.Method);
                Assert.Equal(
                    new Uri($"https://coordinator.nimbuspulse.com/game_servers/{ServerId}"),
                    request.Uri);
            });
    }

    [Fact]
    public async Task DeleteAndReactivateAsync_UseExpectedRoutesAndAcceptEmpty204()
    {
        var handler = RecordingHttpMessageHandler.Returning(statusCode: HttpStatusCode.NoContent);
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        await client.DeleteServerAsync(ServerId);
        await client.ReactivateServerAsync(ServerId);

        Assert.Collection(
            handler.Requests,
            request =>
            {
                Assert.Equal(HttpMethod.Delete, request.Method);
                Assert.Equal(
                    new Uri($"https://coordinator.nimbuspulse.com/game_servers/{ServerId}"),
                    request.Uri);
            },
            request =>
            {
                Assert.Equal(HttpMethod.Post, request.Method);
                Assert.Equal(
                    new Uri($"https://coordinator.nimbuspulse.com/game_servers/{ServerId}/reactivate"),
                    request.Uri);
            });
    }

    private static string[] Strings(JsonElement array) =>
        array.EnumerateArray().Select(element => element.GetString()!).ToArray();

    private static string ResourceJson() =>
        InstanceJson[..^1] +
        ",\n  \"region\": \"de\",\n  \"ip\": \"203.0.113.7\",\n  \"domain\": \"server.example\",\n  \"runtime\": null,\n  \"permissions\": null\n}";
}
