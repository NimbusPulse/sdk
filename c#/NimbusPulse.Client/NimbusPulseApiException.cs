using System.Net;

namespace NimbusPulse.Client;

/// <summary>
/// Represents a non-success response from the NimbusPulse coordinator.
/// </summary>
public sealed class NimbusPulseApiException : HttpRequestException
{
    public NimbusPulseApiException(
        HttpStatusCode statusCode,
        string? reasonPhrase,
        string responseBody,
        Uri? requestUri = null)
        : base(
            CreateMessage(statusCode, reasonPhrase, responseBody),
            inner: null,
            statusCode)
    {
        ResponseBody = responseBody;
        RequestUri = requestUri;
    }

    /// <summary>
    /// Gets the response body returned by the coordinator.
    /// </summary>
    public string ResponseBody { get; }

    /// <summary>
    /// Gets the request URI associated with the response, when available.
    /// </summary>
    public Uri? RequestUri { get; }

    private static string CreateMessage(
        HttpStatusCode statusCode,
        string? reasonPhrase,
        string responseBody)
    {
        var message = $"NimbusPulse request failed ({(int)statusCode} {reasonPhrase ?? statusCode.ToString()})";
        return string.IsNullOrEmpty(responseBody) ? message : $"{message}: {responseBody}";
    }
}
