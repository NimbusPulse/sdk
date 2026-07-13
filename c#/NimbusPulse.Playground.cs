#:project NimbusPulse.Client/NimbusPulse.Client.csproj
#:property PublishAot=false

using NimbusPulse.Client;

var apiKey = Environment.GetEnvironmentVariable("NIMBUSPULSE_API_KEY");
if (string.IsNullOrWhiteSpace(apiKey))
{
    Console.Error.WriteLine("Set NIMBUSPULSE_API_KEY before running this playground.");
    return 1;
}

using var client = new Client(apiKey);

Console.WriteLine("Checking NimbusPulse API health...");
await client.HealthAsync();
Console.WriteLine("Health: OK");

Console.WriteLine("Fetching servers...");
var servers = await client.GetServersAsync();
Console.WriteLine($"Servers: {servers.Count}");

foreach (var server in servers)
{
    var endpoint = string.IsNullOrWhiteSpace(server.Domain)
        ? server.Ip
        : server.Domain;

    Console.WriteLine(
        $"- {server.Id} | {server.DcsSettings} | {server.Status} | {endpoint}");
}

return 0;
