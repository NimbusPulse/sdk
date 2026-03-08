# nimbuspulse-client

Rust SDK for the NimbusPulse coordinator API.

## Install

Add the crate to your project:

```bash
cargo add nimbuspulse-client
```

## Usage

```rust
use nimbuspulse_client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new(std::env::var("NIMBUSPULSE_API_KEY")?);
    let servers = client.get_servers().await?;

    println!("servers: {}", servers.len());

    Ok(())
}
```

## Implemented Capabilities

- Health checks
- Create, fetch, update, start, stop, restart, full restart, update, and delete servers
- Runtime lookup, chat retrieval, and resource metrics
- Terrain changes
- File listing, directory creation, upload, download, move, and delete
- Mission upload, add, delete, select, and start
- DCS pause / resume, settings save, kick, ban, and chat send
- SRS client listing, kick, and ban for servers with the SRS mod installed
- Webconsole execution for servers with the webconsole mod installed
- Trigger create, list, and delete

## Trigger Support

Trigger management is currently Rust-only in this repository.

This feature is not stable yet and will change in the future.

## Development

```bash
cargo check
```

## Repository

The monorepo root README contains the cross-language overview:

- [`../README.md`](../README.md)
