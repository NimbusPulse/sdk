using System.Text.Json.Serialization;

namespace NimbusPulse.Client;

internal sealed record CreateInstanceWireRequest
{
    [JsonPropertyName("product_id")]
    public Guid ProductId { get; init; }

    [JsonPropertyName("region")]
    public Region Region { get; init; }

    [JsonPropertyName("billing_type")]
    public BillingType BillingType { get; init; }

    [JsonPropertyName("settings")]
    public DcsSettingsCreateWireRequest Settings { get; init; } = new();

    [JsonPropertyName("active_mods")]
    public IReadOnlyCollection<string> ActiveMods { get; init; } = Array.Empty<string>();

    [JsonPropertyName("wanted_terrains")]
    public IReadOnlyCollection<Terrain> WantedTerrains { get; init; } = Array.Empty<Terrain>();

    public static CreateInstanceWireRequest From(CreateServerRequest request)
    {
        ArgumentNullException.ThrowIfNull(request);

        return new CreateInstanceWireRequest
        {
            ProductId = request.ProductId,
            Region = request.Region,
            BillingType = request.BillingType,
            ActiveMods = request.ActiveMods,
            WantedTerrains = request.Terrains,
            Settings = new DcsSettingsCreateWireRequest
            {
                InitialServerName = request.Name,
                InitialServerPassword = request.Password ?? string.Empty,
                InitialMaxPlayers = request.MaxPlayers,
                InitialUseVoiceChat = request.UseVoiceChat,
                EnableIo = request.EnableIo,
                EnableOs = request.EnableOs,
                EnableLfs = request.EnableLfs,
            },
        };
    }
}

internal sealed record DcsSettingsCreateWireRequest
{
    [JsonPropertyName("initial_server_name")]
    public string InitialServerName { get; init; } = string.Empty;

    [JsonPropertyName("initial_server_password")]
    public string InitialServerPassword { get; init; } = string.Empty;

    [JsonPropertyName("initial_max_players")]
    public int InitialMaxPlayers { get; init; }

    [JsonPropertyName("enable_io")]
    public bool EnableIo { get; init; }

    [JsonPropertyName("enable_os")]
    public bool EnableOs { get; init; }

    [JsonPropertyName("enable_lfs")]
    public bool EnableLfs { get; init; }

    [JsonPropertyName("initial_use_voice_chat")]
    public bool InitialUseVoiceChat { get; init; }
}

internal sealed record EditInstanceWireRequest
{
    [JsonPropertyName("game_type")]
    public GameType GameType { get; init; } = GameType.Dcs;

    [JsonPropertyName("settings")]
    public DcsSettingsUpdateWireRequest Settings { get; init; } = new();

    public static EditInstanceWireRequest From(UpdateServerRequest request)
    {
        ArgumentNullException.ThrowIfNull(request);

        return new EditInstanceWireRequest
        {
            Settings = new DcsSettingsUpdateWireRequest
            {
                EnableIo = request.EnableIo,
                EnableOs = request.EnableOs,
                EnableLfs = request.EnableLfs,
            },
        };
    }
}

internal sealed record DcsSettingsUpdateWireRequest
{
    [JsonPropertyName("enable_io")]
    public bool EnableIo { get; init; }

    [JsonPropertyName("enable_os")]
    public bool EnableOs { get; init; }

    [JsonPropertyName("enable_lfs")]
    public bool EnableLfs { get; init; }
}
