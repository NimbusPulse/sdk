using System.Text.Json.Serialization;

namespace NimbusPulse.Client;

public sealed record Settings
{
    [JsonPropertyName("description")]
    public string Description { get; init; } = string.Empty;

    [JsonPropertyName("require_pure_textures")]
    public bool RequirePureTextures { get; init; }

    [JsonPropertyName("listStartIndex")]
    public int ListStartIndex { get; init; }

    [JsonPropertyName("advanced")]
    public AdvancedSettings Advanced { get; init; } = new();

    [JsonPropertyName("port")]
    public int Port { get; init; }

    [JsonPropertyName("mode")]
    public int Mode { get; init; }

    [JsonPropertyName("bind_address")]
    public string BindAddress { get; init; } = string.Empty;

    [JsonPropertyName("isPublic")]
    public bool IsPublic { get; init; }

    [JsonPropertyName("listShuffle")]
    public bool ListShuffle { get; init; }

    [JsonPropertyName("password")]
    public string Password { get; init; } = string.Empty;

    [JsonPropertyName("listLoop")]
    public bool ListLoop { get; init; }

    [JsonPropertyName("name")]
    public string Name { get; init; } = string.Empty;

    [JsonPropertyName("require_pure_scripts")]
    public bool RequirePureScripts { get; init; }

    [JsonPropertyName("missionList")]
    [JsonConverter(typeof(EmptyObjectOrStringListJsonConverter))]
    public IReadOnlyList<string> MissionList { get; init; } = Array.Empty<string>();

    [JsonPropertyName("require_pure_clients")]
    public bool RequirePureClients { get; init; }

    [JsonPropertyName("require_pure_models")]
    public bool RequirePureModels { get; init; }

    [JsonPropertyName("maxPlayers")]
    public int MaxPlayers { get; init; }
}

public sealed record AdvancedSettings
{
    [JsonPropertyName("allow_change_tailno")]
    public bool AllowChangeTailNumber { get; init; }

    [JsonPropertyName("disable_events")]
    public bool DisableEvents { get; init; }

    [JsonPropertyName("allow_ownship_export")]
    public bool AllowOwnshipExport { get; init; }

    [JsonPropertyName("allow_object_export")]
    public bool AllowObjectExport { get; init; }

    [JsonPropertyName("pause_on_load")]
    public bool PauseOnLoad { get; init; }

    [JsonPropertyName("allow_sensor_export")]
    public bool AllowSensorExport { get; init; }

    [JsonPropertyName("event_Takeoff")]
    public bool EventTakeoff { get; init; }

    [JsonPropertyName("pause_without_clients")]
    public bool PauseWithoutClients { get; init; }

    [JsonPropertyName("client_outbound_limit")]
    public int ClientOutboundLimit { get; init; }

    [JsonPropertyName("client_inbound_limit")]
    public int ClientInboundLimit { get; init; }

    [JsonPropertyName("server_can_screenshot")]
    public bool ServerCanScreenshot { get; init; }

    [JsonPropertyName("allow_players_pool")]
    public bool AllowPlayersPool { get; init; }

    [JsonPropertyName("voice_chat_server")]
    public bool VoiceChatServer { get; init; }

    [JsonPropertyName("allow_change_skin")]
    public bool AllowChangeSkin { get; init; }

    [JsonPropertyName("event_Connect")]
    public bool EventConnect { get; init; }

    [JsonPropertyName("event_Ejecting")]
    public bool EventEjecting { get; init; }

    [JsonPropertyName("event_Kill")]
    public bool EventKill { get; init; }

    [JsonPropertyName("event_Crash")]
    public bool EventCrash { get; init; }

    [JsonPropertyName("event_Role")]
    public bool EventRole { get; init; }

    [JsonPropertyName("resume_mode")]
    public int ResumeMode { get; init; }

    [JsonPropertyName("maxPing")]
    public int MaxPing { get; init; }

    [JsonPropertyName("allow_trial_only_clients")]
    public bool AllowTrialOnlyClients { get; init; }

    [JsonPropertyName("allow_dynamic_radio")]
    public bool AllowDynamicRadio { get; init; }

    [JsonPropertyName("redPasswordHash")]
    public string? RedPasswordHash { get; init; }

    [JsonPropertyName("bluePasswordHash")]
    public string? BluePasswordHash { get; init; }

    [JsonPropertyName("redPassword")]
    public string? RedPassword { get; init; }

    [JsonPropertyName("bluePassword")]
    public string? BluePassword { get; init; }
}
