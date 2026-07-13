using System.Net;
using System.Text.Json;
using System.Text.Json.Nodes;
using ApiClient = NimbusPulse.Client.Client;

namespace NimbusPulse.Client.Tests;

public sealed class ClientResponseTests
{
    private static readonly Guid ServerId = Guid.Parse("11111111-1111-1111-1111-111111111111");

    private const string ResourceJson = """
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
          "pid": 321,
          "status": "ServerStarted",
          "want_delete": false,
          "wanted_terrains": ["Caucasus", "Syria"],
          "rented_at": 1700000000,
          "rented_until": 1800000000,
          "active_mods": ["srs"],
          "subscription_cancel_at_period_end": true,
          "created_at": "2026-01-02T03:04:05Z",
          "dcs_settings": {
            "initial_server_name": "Training server",
            "initial_server_password": "password",
            "initial_max_players": 24,
            "enable_io": true,
            "enable_os": false,
            "enable_lfs": true,
            "initial_use_voice_chat": true
          },
          "region": "de",
          "ip": "203.0.113.7",
          "domain": "server.example",
          "runtime": {
            "current_action": "StartingMission",
            "last_full_update": 1720000000,
            "paused": false,
            "mission_info": {
              "result_red": 1,
              "result_blue": null,
              "mission_filename": "training.miz",
              "mission_time": 321.5,
              "mission_name": "Training",
              "mission_description": "A training mission"
            },
            "missionList": {},
            "missionTheatres": ["Syria"],
            "listStartIndex": 2,
            "listShuffle": true,
            "listLoop": false,
            "players": {
              "banned": {},
              "all": {
                "7": {
                  "ping": 42,
                  "side": 2,
                  "slot": "F-16C",
                  "id": 7,
                  "name": "Viper",
                  "score": 99,
                  "ucid": "ucid-7",
                  "started": true,
                  "lang": "en",
                  "ipaddr": "198.51.100.4"
                }
              }
            },
            "server_id": 17,
            "mission_list": {
              "missionList": ["training.miz"],
              "missionTheatres": {},
              "listStartIndex": 1,
              "listShuffle": false,
              "listLoop": true
            },
            "settings": {
              "description": "Test settings",
              "require_pure_textures": true,
              "listStartIndex": 3,
              "advanced": {
                "event_Takeoff": true,
                "maxPing": 250
              },
              "port": 10308,
              "mode": 1,
              "bind_address": "0.0.0.0",
              "isPublic": true,
              "listShuffle": false,
              "password": "password",
              "listLoop": true,
              "name": "Training server",
              "require_pure_scripts": false,
              "missionList": {},
              "require_pure_clients": true,
              "require_pure_models": false,
              "maxPlayers": 24
            },
            "ip": "203.0.113.7"
          },
          "permissions": {
            "owner_user": {
              "id": "33333333-3333-3333-3333-333333333333",
              "name": "Owner"
            },
            "permissions": ["instance:view", "instance:actions"]
          }
        }
        """;

    [Fact]
    public async Task GetServerAsync_DeserializesFlattenedResourceAndMixedCaseRuntime()
    {
        var handler = RecordingHttpMessageHandler.Returning(ResourceJson);
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        var server = await client.GetServerAsync(ServerId);

        Assert.Equal(ServerId, server.Id);
        Assert.Equal(Guid.Parse("22222222-2222-2222-2222-222222222222"), server.NodeId);
        Assert.Equal(GameType.Dcs, server.GameType);
        Assert.Equal(BillingType.Hourly, server.BillingType);
        Assert.Equal(Region.Germany, server.Region);
        Assert.Equal("203.0.113.7", server.Ip);
        Assert.Equal("server.example", server.Domain);
        Assert.Equal([Terrain.Caucasus, Terrain.Syria], server.WantedTerrains);
        Assert.Equal(JsonValueKind.String, server.Status.ValueKind);
        Assert.Equal("ServerStarted", server.Status.GetString());
        Assert.Equal("Training server", server.DcsSettings?.InitialServerName);
        Assert.Equal(24, server.DcsSettings?.InitialMaxPlayers);

        var permissions = Assert.IsType<InstancePermissions>(server.Permissions);
        Assert.Equal("Owner", permissions.OwnerUser.Name);
        Assert.Equal(
            [Permission.InstanceView, Permission.InstanceActions],
            permissions.Permissions);

        var runtime = Assert.IsType<DcsRuntime>(server.Runtime);
        Assert.Equal(CurrentRuntimeAction.StartingMission, runtime.CurrentAction);
        Assert.Equal(1720000000, runtime.LastFullUpdate);
        Assert.False(runtime.Paused);
        Assert.Equal("training.miz", runtime.MissionInfo.MissionFilename);
        Assert.Equal(321.5f, runtime.MissionInfo.MissionTime);
        Assert.Empty(runtime.MissionList);
        Assert.Equal(["Syria"], runtime.MissionTheatres);
        Assert.Equal(2, runtime.ListStartIndex);
        Assert.True(runtime.ListShuffle);
        Assert.False(runtime.ListLoop);
        Assert.Empty(runtime.Players.Banned);
        var player = Assert.Single(runtime.Players.All).Value;
        Assert.Equal("Viper", player.Name);
        Assert.Equal(17, runtime.ServerId);
        Assert.Equal(["training.miz"], runtime.ServerMissionList.MissionList);
        Assert.Empty(runtime.ServerMissionList.MissionTheatres);
        Assert.Equal("Training server", runtime.Settings.Name);
        Assert.Empty(runtime.Settings.MissionList);
        Assert.Equal(24, runtime.Settings.MaxPlayers);
        Assert.True(runtime.Settings.Advanced.EventTakeoff);
        Assert.Equal(250, runtime.Settings.Advanced.MaxPing);
        Assert.Equal("203.0.113.7", runtime.Ip);
    }

    [Fact]
    public async Task GetServerAsync_PreservesObjectStatusAsJsonElement()
    {
        var resource = JsonNode.Parse(ResourceJson)!;
        resource["status"] = JsonNode.Parse("""{"InstallingBaseGame":{"progress":42}}""");
        var handler = RecordingHttpMessageHandler.Returning(resource.ToJsonString());
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        var server = await client.GetServerAsync(ServerId);

        Assert.Equal(JsonValueKind.Object, server.Status.ValueKind);
        Assert.Equal(
            42,
            server.Status.GetProperty("InstallingBaseGame").GetProperty("progress").GetInt32());
    }

    [Fact]
    public async Task GetRuntimeAsync_ReturnsRuntimeFromServerResponse()
    {
        var handler = RecordingHttpMessageHandler.Returning(ResourceJson);
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        var runtime = await client.GetRuntimeAsync(ServerId);

        Assert.Equal(17, runtime.ServerId);
        Assert.Equal("Training", runtime.MissionInfo.MissionName);
        Assert.Equal(
            new Uri($"https://coordinator.nimbuspulse.com/game_servers/{ServerId}"),
            Assert.Single(handler.Requests).Uri);
    }

    [Fact]
    public async Task GetRuntimeAsync_ThrowsWhenRuntimeIsAbsent()
    {
        var resource = JsonNode.Parse(ResourceJson)!;
        resource["runtime"] = null;
        var handler = RecordingHttpMessageHandler.Returning(resource.ToJsonString());
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        await Assert.ThrowsAsync<InvalidOperationException>(() => client.GetRuntimeAsync(ServerId));
    }

    [Fact]
    public async Task Any2xxResponse_IsAccepted()
    {
        var handler = RecordingHttpMessageHandler.Returning(
            statusCode: (HttpStatusCode)299);
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        await client.HealthAsync();
    }

    [Fact]
    public async Task NonSuccessResponse_ThrowsApiExceptionWithStatusAndRawBody()
    {
        const string responseBody = "{\"message\":\"server is unavailable\",\"code\":\"offline\"}";
        var handler = RecordingHttpMessageHandler.Returning(
            responseBody,
            HttpStatusCode.ServiceUnavailable);
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        var exception = await Assert.ThrowsAsync<NimbusPulseApiException>(
            () => client.HealthAsync());

        Assert.Equal(HttpStatusCode.ServiceUnavailable, exception.StatusCode);
        Assert.Equal(responseBody, exception.ResponseBody);
    }

    [Fact]
    public async Task MalformedSuccessfulJson_PropagatesJsonException()
    {
        var handler = RecordingHttpMessageHandler.Returning("not json");
        using var httpClient = new HttpClient(handler);
        using var client = new ApiClient("key", httpClient);

        await Assert.ThrowsAsync<JsonException>(() => client.GetServersAsync());
    }
}
