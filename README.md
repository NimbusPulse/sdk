# NimbusPulse SDK

NimbusPulse provides SDKs for interacting with the NimbusPulse coordinator API for DCS server hosting.

This repository currently contains:

- `javascript/`: Node.js + TypeScript client published as `@nimbuspulse/client`
- `rust/`: Rust client crate published as `nimbuspulse-client`

## Repository Layout

| Path          | Language             | Version |
| ------------- | -------------------- | ------- |
| `javascript/` | Node.js / TypeScript | `0.3.1` |
| `rust/`       | Rust                 | `0.3.1` |

## Current API Coverage

Both implementations cover:

- Health checks
- Server lifecycle management, including monthly subscription reactivation
- Server lookup and runtime retrieval
- Terrain changes
- Mod config retrieval, installation changes, and uninstallation changes
- Server log retrieval
- Resource metrics
- Chat retrieval
- File management and file transfer
- Mission upload and mission rotation endpoints
- DCS pause/resume and settings updates
- Player kick / ban
- SRS client inspection and moderation for servers with the SRS mod installed
- Webconsole execution for servers with the webconsole mod installed
- Trigger create, list, and delete

Trigger management is not stable yet and will change in the future.

## Package-Specific Docs

- JavaScript package docs: [`javascript/README.md`](javascript/README.md)
- Rust package docs: [`rust/README.md`](rust/README.md)

## License

MIT. See [`LICENSE`](LICENSE).
