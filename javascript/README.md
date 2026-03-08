# @nimbuspulse/client

TypeScript SDK for the NimbusPulse coordinator API.

## Install

### npm

- Node.js `^24 || ^25`

```bash
npm install @nimbuspulse/client
```

### JSR

```bash
deno add jsr:@nimbuspulse/client
```

## Usage

### npm / Node.js

```ts
import Client from "@nimbuspulse/client";

const client = new Client(process.env.NIMBUSPULSE_API_KEY!);
const servers = await client.getServers();
```

### JSR / Deno

```ts
import Client from "jsr:@nimbuspulse/client";

const client = new Client(Deno.env.get("NIMBUSPULSE_API_KEY")!);
const servers = await client.getServers();
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

## Package Exports

The Node.js client includes extra filesystem helpers such as `uploadFileFrom`, `downloadFileTo`, and `uploadMission`.

## Cross-Runtime Notes

JSR publishes the portable client. The Node.js-only filesystem helpers are not available there.

## Development

Build the package locally with:

```bash
npm run build
```

Available scripts:

- `npm run dev`
- `npm run build`
- `npm run clean`

## Repository

The monorepo root README contains the cross-language overview:

- [`../README.md`](../README.md)
