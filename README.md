# NimbusPulse SDK

NimbusPulse provides SDKs for interacting with the NimbusPulse coordinator API for DCS server hosting.

This repository currently contains:

- `javascript/`: Node.js + TypeScript client published as `@nimbuspulse/client`
- `rust/`: Rust client crate published as `nimbuspulse-client`

## Repository Layout

| Path          | Language             | Version |
| ------------- | -------------------- | ------- |
| `javascript/` | Node.js / TypeScript | `0.2.0` |
| `rust/`       | Rust                 | `0.2.0` |

## Current API Coverage

Both implementations cover:

- Health checks
- Server lifecycle management
- Server lookup and runtime retrieval
- Terrain changes
- Resource metrics
- Chat retrieval
- File management and file transfer
- Mission upload and mission rotation endpoints
- DCS pause/resume and settings updates
- Player kick / ban
- SRS client inspection and moderation for servers with the SRS mod installed
- Webconsole execution for servers with the webconsole mod installed

The Rust client also includes trigger management:

- Create trigger
- List triggers
- Delete trigger

This feature is not stable yet and will change in the future.

## Package-Specific Docs

- JavaScript package docs: [`javascript/README.md`](javascript/README.md)
- Rust package docs: [`rust/README.md`](rust/README.md)

## License

MIT. See [`LICENSE`](LICENSE).
