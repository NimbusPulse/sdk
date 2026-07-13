using System.Text.Json.Serialization;

namespace NimbusPulse.Client;

public sealed record Players
{
    [JsonPropertyName("banned")]
    [JsonConverter(typeof(EmptyObjectOrBannedPlayerListJsonConverter))]
    public IReadOnlyList<BannedPlayer> Banned { get; init; } = Array.Empty<BannedPlayer>();

    [JsonPropertyName("all")]
    public IReadOnlyDictionary<string, Player> All { get; init; } =
        new Dictionary<string, Player>();
}

public sealed record Player
{
    [JsonPropertyName("ping")]
    public int Ping { get; init; }

    [JsonPropertyName("side")]
    public int Side { get; init; }

    [JsonPropertyName("slot")]
    public string Slot { get; init; } = string.Empty;

    [JsonPropertyName("id")]
    public int Id { get; init; }

    [JsonPropertyName("name")]
    public string Name { get; init; } = string.Empty;

    [JsonPropertyName("score")]
    public int Score { get; init; }

    [JsonPropertyName("ucid")]
    public string Ucid { get; init; } = string.Empty;

    [JsonPropertyName("started")]
    public bool Started { get; init; }

    [JsonPropertyName("lang")]
    public string Lang { get; init; } = string.Empty;

    [JsonPropertyName("ipaddr")]
    public string IpAddress { get; init; } = string.Empty;
}

public sealed record BannedPlayer
{
    [JsonPropertyName("banned_from")]
    public long BannedFrom { get; init; }

    [JsonPropertyName("banned_until")]
    public long BannedUntil { get; init; }

    [JsonPropertyName("ipaddr")]
    public string IpAddress { get; init; } = string.Empty;

    [JsonPropertyName("name")]
    public string Name { get; init; } = string.Empty;

    [JsonPropertyName("reason")]
    public string Reason { get; init; } = string.Empty;

    [JsonPropertyName("ucid")]
    public string Ucid { get; init; } = string.Empty;
}
