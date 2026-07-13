using System.Text.Json;

namespace NimbusPulse.Client;

internal static class NimbusPulseJson
{
    public static JsonSerializerOptions Options { get; } = new(JsonSerializerDefaults.Web)
    {
        PropertyNameCaseInsensitive = true,
    };
}
