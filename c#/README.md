# NimbusPulse.Client

Minimal .NET client for the NimbusPulse coordinator lifecycle API. It targets .NET 10 and has no production package dependencies.

## Add a project reference

Reference the client from another project in this repository:

```bash
dotnet add reference ../c#/NimbusPulse.Client/NimbusPulse.Client.csproj
```

## Usage

```csharp
using NimbusPulse.Client;

var apiKey = Environment.GetEnvironmentVariable("NIMBUSPULSE_API_KEY")
    ?? throw new InvalidOperationException("NIMBUSPULSE_API_KEY is required.");

using var client = new Client(apiKey);
var servers = await client.GetServersAsync();

Console.WriteLine($"Servers: {servers.Count}");
```

Pass an `HttpClient` to the constructor when the application manages HTTP client lifetime or needs a custom HTTP transport, such as a mocked handler in tests. Requests always target the NimbusPulse coordinator endpoint.

## Implemented capabilities

- Health checks
- Create, list, fetch, and update servers
- Runtime lookup and terrain changes
- Start, stop, restart, full restart, and game-server update actions
- Delete and reactivate servers

All asynchronous operations accept an optional `CancellationToken`.

## Deferred coverage

This is intentionally not full parity with the JavaScript and Rust clients. Chat, metrics, mods, files, mission and DCS administration, SRS, webconsole, triggers, product-price constants, and NuGet publishing metadata remain out of scope.

## Development

```bash
dotnet build NimbusPulse.Client.slnx
dotnet test NimbusPulse.Client.slnx
```

### Live playground

Use the .NET 10 file-based playground for a read-only smoke test against the
coordinator. It checks health and lists servers; it does not create, update, or
delete anything.

```bash
export NIMBUSPULSE_API_KEY="your-api-key"
dotnet run --file NimbusPulse.Playground.cs
```

Keep the API key in the environment rather than adding it to the source file.
