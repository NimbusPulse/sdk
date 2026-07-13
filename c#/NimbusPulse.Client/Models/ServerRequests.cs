namespace NimbusPulse.Client;

public sealed record CreateServerRequest
{
    public string Name { get; init; } = string.Empty;

    public BillingType BillingType { get; init; }

    public Region Region { get; init; }

    public string? Password { get; init; }

    public int MaxPlayers { get; init; }

    public Guid ProductId { get; init; }

    public IReadOnlyCollection<string> ActiveMods { get; init; } = Array.Empty<string>();

    public IReadOnlyCollection<Terrain> Terrains { get; init; } = Array.Empty<Terrain>();

    public bool UseVoiceChat { get; init; }

    public bool EnableIo { get; init; }

    public bool EnableOs { get; init; }

    public bool EnableLfs { get; init; }
}

public sealed record UpdateServerRequest
{
    public bool EnableIo { get; init; }

    public bool EnableOs { get; init; }

    public bool EnableLfs { get; init; }
}
