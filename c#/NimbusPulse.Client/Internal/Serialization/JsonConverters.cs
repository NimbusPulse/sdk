using System.Text.Json;
using System.Text.Json.Serialization;

namespace NimbusPulse.Client;

internal sealed class EmptyObjectOrStringListJsonConverter
    : JsonConverter<IReadOnlyList<string>>
{
    public override IReadOnlyList<string> Read(
        ref Utf8JsonReader reader,
        Type typeToConvert,
        JsonSerializerOptions options)
    {
        using var document = JsonDocument.ParseValue(ref reader);
        var root = document.RootElement;

        if (root.ValueKind == JsonValueKind.Object && !root.EnumerateObject().Any())
        {
            return Array.Empty<string>();
        }

        if (root.ValueKind != JsonValueKind.Array)
        {
            throw new JsonException("Expected an array or an empty object for a mission list.");
        }

        var values = new List<string>(root.GetArrayLength());
        foreach (var element in root.EnumerateArray())
        {
            if (element.ValueKind != JsonValueKind.String)
            {
                throw new JsonException("Mission lists may contain only strings.");
            }

            values.Add(element.GetString()!);
        }

        return values;
    }

    public override void Write(
        Utf8JsonWriter writer,
        IReadOnlyList<string> value,
        JsonSerializerOptions options)
    {
        writer.WriteStartArray();
        foreach (var item in value)
        {
            writer.WriteStringValue(item);
        }

        writer.WriteEndArray();
    }
}

internal sealed class EmptyObjectOrBannedPlayerListJsonConverter
    : JsonConverter<IReadOnlyList<BannedPlayer>>
{
    public override IReadOnlyList<BannedPlayer> Read(
        ref Utf8JsonReader reader,
        Type typeToConvert,
        JsonSerializerOptions options)
    {
        using var document = JsonDocument.ParseValue(ref reader);
        var root = document.RootElement;

        if (root.ValueKind == JsonValueKind.Object && !root.EnumerateObject().Any())
        {
            return Array.Empty<BannedPlayer>();
        }

        if (root.ValueKind != JsonValueKind.Array)
        {
            throw new JsonException("Expected an array or an empty object for banned players.");
        }

        return root.Deserialize<List<BannedPlayer>>(options)
            ?? throw new JsonException("Could not deserialize the banned-player list.");
    }

    public override void Write(
        Utf8JsonWriter writer,
        IReadOnlyList<BannedPlayer> value,
        JsonSerializerOptions options)
    {
        writer.WriteStartArray();
        foreach (var item in value)
        {
            JsonSerializer.Serialize(writer, item, options);
        }

        writer.WriteEndArray();
    }
}
