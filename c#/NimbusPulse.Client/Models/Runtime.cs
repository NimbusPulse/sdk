using System.Text.Json.Serialization;

namespace NimbusPulse.Client;

public sealed record DcsRuntime
{
    [JsonPropertyName("current_action")]
    public CurrentRuntimeAction? CurrentAction { get; init; }

    [JsonPropertyName("last_full_update")]
    public long LastFullUpdate { get; init; }

    [JsonPropertyName("paused")]
    public bool Paused { get; init; }

    [JsonPropertyName("mission_info")]
    public GetMissionInfoResponse MissionInfo { get; init; } = new();

    [JsonPropertyName("missionList")]
    [JsonConverter(typeof(EmptyObjectOrStringListJsonConverter))]
    public IReadOnlyList<string> MissionList { get; init; } = Array.Empty<string>();

    [JsonPropertyName("missionTheatres")]
    [JsonConverter(typeof(EmptyObjectOrStringListJsonConverter))]
    public IReadOnlyList<string> MissionTheatres { get; init; } = Array.Empty<string>();

    [JsonPropertyName("listStartIndex")]
    public int ListStartIndex { get; init; }

    [JsonPropertyName("listShuffle")]
    public bool ListShuffle { get; init; }

    [JsonPropertyName("listLoop")]
    public bool ListLoop { get; init; }

    [JsonPropertyName("players")]
    public Players Players { get; init; } = new();

    [JsonPropertyName("server_id")]
    public int ServerId { get; init; }

    [JsonPropertyName("mission_list")]
    public GetMissionListResponse ServerMissionList { get; init; } = new();

    [JsonPropertyName("settings")]
    public Settings Settings { get; init; } = new();

    [JsonPropertyName("ip")]
    public string Ip { get; init; } = string.Empty;
}

public sealed record GetMissionInfoResponse
{
    [JsonPropertyName("result_red")]
    public int? ResultRed { get; init; }

    [JsonPropertyName("result_blue")]
    public int? ResultBlue { get; init; }

    [JsonPropertyName("mission_filename")]
    public string MissionFilename { get; init; } = string.Empty;

    [JsonPropertyName("mission_time")]
    public float MissionTime { get; init; }

    [JsonPropertyName("mission_name")]
    public string MissionName { get; init; } = string.Empty;

    [JsonPropertyName("mission_description")]
    public string MissionDescription { get; init; } = string.Empty;
}

public sealed record GetMissionListResponse
{
    [JsonPropertyName("missionList")]
    [JsonConverter(typeof(EmptyObjectOrStringListJsonConverter))]
    public IReadOnlyList<string> MissionList { get; init; } = Array.Empty<string>();

    [JsonPropertyName("missionTheatres")]
    [JsonConverter(typeof(EmptyObjectOrStringListJsonConverter))]
    public IReadOnlyList<string> MissionTheatres { get; init; } = Array.Empty<string>();

    [JsonPropertyName("listStartIndex")]
    public int ListStartIndex { get; init; }

    [JsonPropertyName("listShuffle")]
    public bool ListShuffle { get; init; }

    [JsonPropertyName("listLoop")]
    public bool ListLoop { get; init; }
}
