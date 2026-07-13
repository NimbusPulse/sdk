using System.Text.Json;
using System.Text.Json.Serialization;

namespace NimbusPulse.Client;

public record Instance
{
    [JsonPropertyName("id")]
    public Guid Id { get; init; }

    [JsonPropertyName("node_id")]
    public Guid NodeId { get; init; }

    [JsonPropertyName("user_id")]
    public Guid UserId { get; init; }

    [JsonPropertyName("product_id")]
    public Guid ProductId { get; init; }

    [JsonPropertyName("game_type")]
    public GameType GameType { get; init; }

    [JsonPropertyName("billing_type")]
    public BillingType BillingType { get; init; }

    [JsonPropertyName("port")]
    public int Port { get; init; }

    [JsonPropertyName("webgui_port")]
    public int WebGuiPort { get; init; }

    [JsonPropertyName("ftp_port")]
    public int FtpPort { get; init; }

    [JsonPropertyName("ftp_username")]
    public string FtpUsername { get; init; } = string.Empty;

    [JsonPropertyName("ftp_password")]
    public string FtpPassword { get; init; } = string.Empty;

    [JsonPropertyName("pid")]
    public int? Pid { get; init; }

    [JsonPropertyName("status")]
    public JsonElement Status { get; init; }

    [JsonPropertyName("want_delete")]
    public bool WantDelete { get; init; }

    [JsonPropertyName("wanted_terrains")]
    public IReadOnlyList<Terrain> WantedTerrains { get; init; } = Array.Empty<Terrain>();

    [JsonPropertyName("rented_at")]
    public long RentedAt { get; init; }

    [JsonPropertyName("rented_until")]
    public long? RentedUntil { get; init; }

    [JsonPropertyName("active_mods")]
    public IReadOnlyList<string> ActiveMods { get; init; } = Array.Empty<string>();

    [JsonPropertyName("subscription_cancel_at_period_end")]
    public bool SubscriptionCancelAtPeriodEnd { get; init; }

    [JsonPropertyName("created_at")]
    public string CreatedAt { get; init; } = string.Empty;

    [JsonPropertyName("dcs_settings")]
    public DcsInstanceSettings? DcsSettings { get; init; }
}

public sealed record InstanceResource : Instance
{
    [JsonPropertyName("region")]
    public Region Region { get; init; } = Region.Invalid;

    [JsonPropertyName("ip")]
    public string Ip { get; init; } = string.Empty;

    [JsonPropertyName("domain")]
    public string Domain { get; init; } = string.Empty;

    [JsonPropertyName("runtime")]
    public DcsRuntime? Runtime { get; init; }

    [JsonPropertyName("permissions")]
    public InstancePermissions? Permissions { get; init; }
}

public sealed record DcsInstanceSettings
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

public sealed record InstancePermissions
{
    [JsonPropertyName("owner_user")]
    public UserResource OwnerUser { get; init; } = new();

    [JsonPropertyName("permissions")]
    public IReadOnlyList<Permission> Permissions { get; init; } = Array.Empty<Permission>();
}

public sealed record UserResource
{
    [JsonPropertyName("id")]
    public Guid Id { get; init; }

    [JsonPropertyName("name")]
    public string Name { get; init; } = string.Empty;
}
