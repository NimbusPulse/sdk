# NimbusPulse SDK

NimbusPulse provides SDKs for interacting with the NimbusPulse coordinator API for DCS server hosting.

This repository currently contains:

- [`javascript/`](javascript/README.md): JavaScript and TypeScript client published as `@nimbuspulse/client`
- [`rust/`](rust/README.md): Rust client published as `nimbuspulse-client`
- [`c#/`](c%23/README.md): C# client for .NET 10

## Current API Coverage

Across the SDKs, the repository currently covers:

- Health checks
- Server lifecycle management, including monthly subscription reactivation
- Server lookup and runtime retrieval
- Terrain changes
- Mod config retrieval, installation changes, and uninstallation changes
- Resource metrics
- Chat retrieval
- File management and file transfer
- Mission upload and mission rotation endpoints
- DCS pause/resume and settings updates
- Player kick / ban
- SRS client inspection and moderation for servers with the SRS mod installed
- Webconsole execution for servers with the webconsole mod installed
- Trigger create, list, and delete

Trigger management is not stable yet and will change in the future. See each
SDK's README for its exact current coverage.

## License

MIT. See [`LICENSE`](LICENSE).
