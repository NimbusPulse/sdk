using System.Net;
using System.Net.Http.Headers;
using System.Text;

namespace NimbusPulse.Client.Tests;

internal sealed record RecordedRequest(
    HttpMethod Method,
    Uri Uri,
    string? Body,
    AuthenticationHeaderValue? Authorization,
    MediaTypeHeaderValue? ContentType);

internal sealed class RecordingHttpMessageHandler(
    Func<HttpRequestMessage, CancellationToken, Task<HttpResponseMessage>> responder)
    : HttpMessageHandler
{
    private readonly Func<HttpRequestMessage, CancellationToken, Task<HttpResponseMessage>> _responder = responder;

    public List<RecordedRequest> Requests { get; } = [];

    public bool IsDisposed { get; private set; }

    protected override async Task<HttpResponseMessage> SendAsync(
        HttpRequestMessage request,
        CancellationToken cancellationToken)
    {
        Requests.Add(new RecordedRequest(
            request.Method,
            request.RequestUri!,
            request.Content is null ? null : await request.Content.ReadAsStringAsync(cancellationToken),
            request.Headers.Authorization,
            request.Content?.Headers.ContentType));

        return await _responder(request, cancellationToken);
    }

    protected override void Dispose(bool disposing)
    {
        IsDisposed = true;
        base.Dispose(disposing);
    }

    public static RecordingHttpMessageHandler Returning(
        string? json = null,
        HttpStatusCode statusCode = HttpStatusCode.OK)
    {
        return new RecordingHttpMessageHandler((_, _) => Task.FromResult(Response(json, statusCode)));
    }

    public static HttpResponseMessage Response(
        string? json = null,
        HttpStatusCode statusCode = HttpStatusCode.OK)
    {
        return new HttpResponseMessage(statusCode)
        {
            Content = json is null
                ? null
                : new StringContent(json, Encoding.UTF8, "application/json"),
        };
    }
}
