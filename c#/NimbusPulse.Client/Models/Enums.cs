using System.Text.Json.Serialization;

namespace NimbusPulse.Client;

[JsonConverter(typeof(JsonStringEnumConverter<BillingType>))]
public enum BillingType
{
    [JsonStringEnumMemberName("hourly")]
    Hourly,

    [JsonStringEnumMemberName("monthly")]
    Monthly,
}

[JsonConverter(typeof(JsonStringEnumConverter<Region>))]
public enum Region
{
    [JsonStringEnumMemberName("de")]
    Germany,

    [JsonStringEnumMemberName("us")]
    USA,

    [JsonStringEnumMemberName("invalid")]
    Invalid,
}

[JsonConverter(typeof(JsonStringEnumConverter<GameType>))]
public enum GameType
{
    [JsonStringEnumMemberName("dcs")]
    Dcs,
}

[JsonConverter(typeof(JsonStringEnumConverter<Terrain>))]
public enum Terrain
{
    [JsonStringEnumMemberName("Afghanistan")]
    Afghanistan,

    [JsonStringEnumMemberName("Caucasus")]
    Caucasus,

    [JsonStringEnumMemberName("Falklands")]
    Falklands,

    [JsonStringEnumMemberName("Iraq")]
    Iraq,

    [JsonStringEnumMemberName("Kola")]
    Kola,

    [JsonStringEnumMemberName("MarianaIslands")]
    MarianaIslands,

    [JsonStringEnumMemberName("MarianaIslandsWWII")]
    MarianaIslandsWWII,

    [JsonStringEnumMemberName("Nevada")]
    Nevada,

    [JsonStringEnumMemberName("Normandy")]
    Normandy,

    [JsonStringEnumMemberName("PersianGulf")]
    PersianGulf,

    [JsonStringEnumMemberName("Sinai")]
    Sinai,

    [JsonStringEnumMemberName("Syria")]
    Syria,

    [JsonStringEnumMemberName("TheChannel")]
    TheChannel,

    [JsonStringEnumMemberName("GermanyCW")]
    GermanyCW,
}

[JsonConverter(typeof(JsonStringEnumConverter<Permission>))]
public enum Permission
{
    [JsonStringEnumMemberName("instance:view")]
    InstanceView,

    [JsonStringEnumMemberName("instance:actions")]
    InstanceActions,

    [JsonStringEnumMemberName("instance:chat")]
    InstanceChat,

    [JsonStringEnumMemberName("instance:players:manage")]
    InstancePlayersManage,

    [JsonStringEnumMemberName("instance:missions")]
    InstanceMissions,

    [JsonStringEnumMemberName("instance:settings")]
    InstanceSettings,

    [JsonStringEnumMemberName("instance:infrastructure")]
    InstanceInfrastructure,

    [JsonStringEnumMemberName("instance:mods")]
    InstanceMods,

    [JsonStringEnumMemberName("instance:scheduled_tasks")]
    InstanceScheduledTasks,

    [JsonStringEnumMemberName("instance:file:read")]
    InstanceFileRead,

    [JsonStringEnumMemberName("instance:file:write")]
    InstanceFileWrite,
}

[JsonConverter(typeof(JsonStringEnumConverter<CurrentRuntimeAction>))]
public enum CurrentRuntimeAction
{
    [JsonStringEnumMemberName("StartingMission")]
    StartingMission,

    [JsonStringEnumMemberName("StartingServer")]
    StartingServer,
}
