using System.Net.Http.Headers;
using System.Text;
using System.Text.Json;

namespace NimbusPulse.Client;

/// <summary>
/// Client for the NimbusPulse coordinator API.
/// </summary>
public sealed class Client : IDisposable
{
    private static readonly Uri _coordinatorBaseUri =
        new("https://coordinator.nimbuspulse.com/");

    private readonly string _apiKey;
    private readonly HttpClient _httpClient;
    private readonly bool _ownsHttpClient;
    private bool _disposed;

    /// <summary>
    /// Creates a NimbusPulse client.
    /// </summary>
    /// <param name="apiKey">NimbusPulse API key.</param>
    /// <param name="httpClient">Optional application-managed HTTP client.</param>
    public Client(string apiKey, HttpClient? httpClient = null)
    {
        ArgumentException.ThrowIfNullOrWhiteSpace(apiKey);

        _apiKey = apiKey;
        _ownsHttpClient = httpClient is null;
        _httpClient = httpClient ?? new HttpClient();
    }

    public Task HealthAsync(CancellationToken cancellationToken = default) =>
        SendAsync(HttpMethod.Get, "health", body: null, cancellationToken);

    public Task<Instance> CreateServerAsync(
        CreateServerRequest request,
        CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(request);

        var body = CreateInstanceWireRequest.From(request);

        return SendJsonAsync<Instance>(HttpMethod.Post, "game_servers", body, cancellationToken);
    }

    public async Task<IReadOnlyList<InstanceResource>> GetServersAsync(
        CancellationToken cancellationToken = default) =>
        await SendJsonAsync<List<InstanceResource>>(
            HttpMethod.Get,
            "game_servers",
            body: null,
            cancellationToken).ConfigureAwait(false);

    public Task<InstanceResource> GetServerAsync(
        Guid id,
        CancellationToken cancellationToken = default) =>
        SendJsonAsync<InstanceResource>(
            HttpMethod.Get,
            ServerPath(id),
            body: null,
            cancellationToken);

    public async Task<DcsRuntime> GetRuntimeAsync(
        Guid id,
        CancellationToken cancellationToken = default)
    {
        var server = await GetServerAsync(id, cancellationToken).ConfigureAwait(false);
        return server.Runtime
            ?? throw new InvalidOperationException("Server runtime is not available.");
    }

    public Task<InstanceResource> UpdateServerAsync(
        Guid id,
        UpdateServerRequest request,
        CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(request);

        var body = EditInstanceWireRequest.From(request);

        return SendJsonAsync<InstanceResource>(
            HttpMethod.Put,
            ServerPath(id),
            body,
            cancellationToken);
    }

    public Task ChangeServerTerrainsAsync(
        Guid id,
        IReadOnlyCollection<Terrain> terrains,
        CancellationToken cancellationToken = default)
    {
        ArgumentNullException.ThrowIfNull(terrains);

        return SendAsync(
            HttpMethod.Put,
            $"{ServerPath(id)}/terrains",
            terrains,
            cancellationToken);
    }

    public Task<Instance> StartServerAsync(
        Guid id,
        CancellationToken cancellationToken = default) =>
        SendJsonAsync<Instance>(
            HttpMethod.Post,
            $"{ServerPath(id)}/start",
            body: null,
            cancellationToken);

    public Task<Instance> StopServerAsync(
        Guid id,
        CancellationToken cancellationToken = default) =>
        SendJsonAsync<Instance>(
            HttpMethod.Post,
            $"{ServerPath(id)}/stop",
            body: null,
            cancellationToken);

    public Task<Instance> RestartServerAsync(
        Guid id,
        CancellationToken cancellationToken = default) =>
        SendJsonAsync<Instance>(
            HttpMethod.Post,
            $"{ServerPath(id)}/restart",
            body: null,
            cancellationToken);

    public Task<Instance> FullRestartServerAsync(
        Guid id,
        CancellationToken cancellationToken = default) =>
        SendJsonAsync<Instance>(
            HttpMethod.Post,
            $"{ServerPath(id)}/full_restart",
            body: null,
            cancellationToken);

    public Task<Instance> UpdateGameServerAsync(
        Guid id,
        CancellationToken cancellationToken = default) =>
        SendJsonAsync<Instance>(
            HttpMethod.Post,
            $"{ServerPath(id)}/update",
            body: null,
            cancellationToken);

    public Task DeleteServerAsync(
        Guid id,
        CancellationToken cancellationToken = default) =>
        SendAsync(HttpMethod.Delete, ServerPath(id), body: null, cancellationToken);

    public Task ReactivateServerAsync(
        Guid id,
        CancellationToken cancellationToken = default) =>
        SendAsync(
            HttpMethod.Post,
            $"{ServerPath(id)}/reactivate",
            body: null,
            cancellationToken);

    public void Dispose()
    {
        if (_disposed)
        {
            return;
        }

        _disposed = true;
        if (_ownsHttpClient)
        {
            _httpClient.Dispose();
        }
    }

    private async Task SendAsync(
        HttpMethod method,
        string path,
        object? body,
        CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();
        using var request = CreateRequest(method, path, body);
        using var response = await _httpClient.SendAsync(
            request,
            HttpCompletionOption.ResponseContentRead,
            cancellationToken).ConfigureAwait(false);

        await EnsureSuccessAsync(response, cancellationToken).ConfigureAwait(false);
    }

    private async Task<T> SendJsonAsync<T>(
        HttpMethod method,
        string path,
        object? body,
        CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();
        using var request = CreateRequest(method, path, body);
        using var response = await _httpClient.SendAsync(
            request,
            HttpCompletionOption.ResponseHeadersRead,
            cancellationToken).ConfigureAwait(false);

        await EnsureSuccessAsync(response, cancellationToken).ConfigureAwait(false);

        await using var stream = await response.Content
            .ReadAsStreamAsync(cancellationToken)
            .ConfigureAwait(false);
        var value = await JsonSerializer.DeserializeAsync<T>(
            stream,
            NimbusPulseJson.Options,
            cancellationToken).ConfigureAwait(false);

        return value ?? throw new JsonException(
            $"NimbusPulse returned a null {typeof(T).Name} response.");
    }

    private HttpRequestMessage CreateRequest(HttpMethod method, string path, object? body)
    {
        ThrowIfDisposed();

        var request = new HttpRequestMessage(method, new Uri(_coordinatorBaseUri, path));
        request.Headers.Authorization = new AuthenticationHeaderValue("Bearer", _apiKey);

        if (body is not null)
        {
            var json = JsonSerializer.Serialize(body, body.GetType(), NimbusPulseJson.Options);
            request.Content = new StringContent(json, Encoding.UTF8, "application/json");
        }

        return request;
    }

    private static async Task EnsureSuccessAsync(
        HttpResponseMessage response,
        CancellationToken cancellationToken)
    {
        if (response.IsSuccessStatusCode)
        {
            return;
        }

        var responseBody = await response.Content
            .ReadAsStringAsync(cancellationToken)
            .ConfigureAwait(false);
        throw new NimbusPulseApiException(
            response.StatusCode,
            response.ReasonPhrase,
            responseBody,
            response.RequestMessage?.RequestUri);
    }

    private static string ServerPath(Guid id) => $"game_servers/{id:D}";

    private void ThrowIfDisposed()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
    }
}
