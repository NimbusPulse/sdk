import { readFile, writeFile } from "fs/promises";
import * as path from "path";
import type {
  AddMissionsResponse,
  BanPlayerRequest,
  BanPlayerResponse,
  BillingType,
  CreateInstanceRequest,
  DcsChatSafe,
  DcsRuntimeSafe,
  DeleteMissionsResponse,
  EditInstanceRequest,
  FileListResponse,
  GetPauseServerResponse,
  GetResumeServerResponse,
  InstanceResource,
  InstanceSafe,
  InstancesResponse,
  KickPlayerRequest,
  KickPlayerResponse,
  MoveFileRequest,
  Region,
  SendChatRequest,
  SendChatResponse,
  ServerResourcesResponse,
  SetServerSettingsRequest,
  SetServerSettingsResponse,
  SrsModRequest,
  SrsServerInfo,
  StartMissionResponse,
  StartServerResponse,
  Terrain,
  WebConsoleExecuteRequest,
} from "./types.js";

export type Fetch = {
  (input: URL | RequestInfo, init?: RequestInit | undefined): Promise<Response>;
  (
    input: string | URL | Request,
    init?: RequestInit | undefined,
  ): Promise<Response>;
};

export type BinaryInput = Uint8Array | ArrayBuffer;

export default class Client {
  private apiKey: string;
  private fetch: Fetch;
  private static readonly BASE_URL = "https://coordinator.nimbuspulse.com";

  constructor(apiKey: string) {
    this.apiKey = apiKey;
    this.fetch = fetch;
  }

  public setApiKey(apiKey: string): void {
    this.apiKey = apiKey;
  }

  public setFetch(fetch: Fetch): void {
    this.fetch = fetch;
  }

  private buildUrl(
    pathname: string,
    query?: Record<string, string | number | boolean | null | undefined>,
  ): string {
    const url = new URL(`${Client.BASE_URL}${pathname}`);

    if (query) {
      for (const [key, value] of Object.entries(query)) {
        if (value === undefined || value === null) {
          continue;
        }

        url.searchParams.set(key, String(value));
      }
    }

    return url.toString();
  }

  private async request(
    url: string,
    init: RequestInit = {},
  ): Promise<Response> {
    const headers = new Headers(init.headers);
    headers.set("Authorization", `Bearer ${this.apiKey}`);

    if (
      init.body &&
      !headers.has("Content-Type") &&
      !(init.body instanceof FormData)
    ) {
      headers.set("Content-Type", "application/json");
    }

    const response = await this.fetch(url, {
      ...init,
      headers,
    });

    if (!response.ok) {
      const body = await response.text().catch(() => "");
      const suffix = body ? `: ${body}` : "";
      throw new Error(
        `Failed request (${response.status} ${response.statusText})${suffix}`,
      );
    }

    return response;
  }

  private async requestJson<T>(
    url: string,
    init: RequestInit = {},
  ): Promise<T> {
    const response = await this.request(url, init);
    return (await response.json()) as T;
  }

  private async requestVoid(
    url: string,
    init: RequestInit = {},
  ): Promise<void> {
    await this.request(url, init);
  }

  private async requestBytes(
    url: string,
    init: RequestInit = {},
  ): Promise<Uint8Array> {
    const response = await this.request(url, init);
    return new Uint8Array(await response.arrayBuffer());
  }

  private createJsonBody(value: unknown): string {
    return JSON.stringify(value);
  }

  private createFileForm(file: BinaryInput, filename: string): FormData {
    const content = file instanceof Uint8Array ? file : new Uint8Array(file);
    const bytes = new Uint8Array(content.byteLength);
    bytes.set(content);

    const form = new FormData();
    form.append(
      "file",
      new Blob([bytes], { type: "application/octet-stream" }),
      filename,
    );
    return form;
  }

  public async health(): Promise<void> {
    await this.requestVoid(this.buildUrl("/health"), {
      method: "GET",
    });
  }

  public async createServer(
    name: string,
    billingType: BillingType,
    region: Region,
    password: string | null,
    maxPlayers: number,
    plan: string,
    activeMods: string[],
    terrains: Terrain[],
    useVoiceChat: boolean,
    enableIo: boolean,
    enableOs: boolean,
    enableLfs: boolean,
  ): Promise<InstanceSafe> {
    const payload: CreateInstanceRequest = {
      product_id: plan,
      billing_type: billingType,
      region,
      active_mods: activeMods,
      wanted_terrains: terrains,
      settings: {
        initial_server_name: name,
        initial_server_password: password ?? "",
        initial_max_players: maxPlayers,
        initial_use_voice_chat: useVoiceChat,
        enable_io: enableIo,
        enable_os: enableOs,
        enable_lfs: enableLfs,
      },
    };

    return await this.requestJson<InstanceSafe>(this.buildUrl("/game_servers"), {
      method: "POST",
      body: this.createJsonBody(payload),
    });
  }

  public async getServers(): Promise<InstancesResponse> {
    return await this.requestJson<InstancesResponse>(
      this.buildUrl("/game_servers"),
      {
        method: "GET",
      },
    );
  }

  public async getServer(id: string): Promise<InstanceResource> {
    return await this.requestJson<InstanceResource>(
      this.buildUrl(`/game_servers/${id}`),
      {
        method: "GET",
      },
    );
  }

  public async getRuntime(id: string): Promise<DcsRuntimeSafe> {
    const server = await this.getServer(id);
    if (!server.runtime) {
      throw new Error("Server runtime is not available");
    }

    return server.runtime;
  }

  public async updateServer(
    id: string,
    payload: EditInstanceRequest,
  ): Promise<InstanceResource> {
    return await this.requestJson<InstanceResource>(
      this.buildUrl(`/game_servers/${id}`),
      {
        method: "PUT",
        body: this.createJsonBody(payload),
      },
    );
  }

  public async changeServerTerrains(
    id: string,
    terrains: Terrain[],
  ): Promise<void> {
    await this.requestVoid(this.buildUrl(`/game_servers/${id}/terrains`), {
      method: "PUT",
      body: this.createJsonBody(terrains),
    });
  }

  public async getChat(id: string): Promise<DcsChatSafe[]> {
    return await this.requestJson<DcsChatSafe[]>(
      this.buildUrl(`/game_servers/${id}/chat`),
      {
      method: "GET",
      },
    );
  }

  public async startServer(id: string): Promise<InstanceSafe> {
    return await this.requestJson<InstanceSafe>(
      this.buildUrl(`/game_servers/${id}/start`),
      {
        method: "POST",
      },
    );
  }

  public async stopServer(id: string): Promise<InstanceSafe> {
    return await this.requestJson<InstanceSafe>(
      this.buildUrl(`/game_servers/${id}/stop`),
      {
        method: "POST",
      },
    );
  }

  public async fullRestartServer(id: string): Promise<InstanceSafe> {
    return await this.requestJson<InstanceSafe>(
      this.buildUrl(`/game_servers/${id}/full_restart`),
      {
        method: "POST",
      },
    );
  }

  public async restartServer(id: string): Promise<InstanceSafe> {
    return await this.requestJson<InstanceSafe>(
      this.buildUrl(`/game_servers/${id}/restart`),
      {
        method: "POST",
      },
    );
  }

  public async updateGameServer(id: string): Promise<InstanceSafe> {
    return await this.requestJson<InstanceSafe>(
      this.buildUrl(`/game_servers/${id}/update`),
      {
        method: "POST",
      },
    );
  }

  public async deleteServer(id: string): Promise<void> {
    await this.requestVoid(this.buildUrl(`/game_servers/${id}`), {
      method: "DELETE",
    });
  }

  public async getServerResources(
    id: string,
    period: "now" | "hour" | "day" | "week",
  ): Promise<ServerResourcesResponse> {
    return await this.requestJson<ServerResourcesResponse>(
      this.buildUrl(`/game_servers/${id}/resources`, { periode: period }),
      {
        method: "GET",
      },
    );
  }

  public async listFiles(
    id: string,
    filePath: string,
  ): Promise<FileListResponse> {
    return await this.requestJson<FileListResponse>(
      this.buildUrl(`/game_servers/${id}/files`, { path: filePath }),
      {
        method: "GET",
      },
    );
  }

  public async createDirectory(
    id: string,
    directoryPath: string,
  ): Promise<void> {
    await this.requestVoid(
      this.buildUrl(`/game_servers/${id}/files/directory`, {
        path: directoryPath,
      }),
      {
        method: "POST",
      },
    );
  }

  public async uploadFile(
    id: string,
    filePath: string,
    file: BinaryInput,
    filename = "upload.bin",
  ): Promise<void> {
    await this.requestVoid(
      this.buildUrl(`/game_servers/${id}/files/upload`, { path: filePath }),
      {
        method: "POST",
        body: this.createFileForm(file, filename),
      },
    );
  }

  public async uploadFileFrom(
    id: string,
    destinationPath: string,
    localFilePath: string,
  ): Promise<void> {
    const file = await readFile(localFilePath);
    await this.uploadFile(
      id,
      destinationPath,
      file,
      path.basename(localFilePath),
    );
  }

  public async downloadFile(id: string, filePath: string): Promise<Uint8Array> {
    return await this.requestBytes(
      this.buildUrl(`/game_servers/${id}/files/download`, { path: filePath }),
      {
        method: "GET",
      },
    );
  }

  public async downloadFileTo(
    id: string,
    remotePath: string,
    destinationPath: string,
  ): Promise<void> {
    const content = await this.downloadFile(id, remotePath);
    await writeFile(destinationPath, content);
  }

  public async deleteFile(id: string, filePath: string): Promise<void> {
    await this.requestVoid(
      this.buildUrl(`/game_servers/${id}/files`, { path: filePath }),
      {
        method: "DELETE",
      },
    );
  }

  public async moveFile(id: string, request: MoveFileRequest): Promise<void> {
    await this.requestVoid(this.buildUrl(`/game_servers/${id}/files/move`), {
      method: "PUT",
      body: this.createJsonBody(request),
    });
  }

  public async uploadMission(id: string, missionPath: string): Promise<void> {
    const parsedMissionPath = path.parse(missionPath);
    const missionName = parsedMissionPath.base;
    await this.uploadFileFrom(id, `Missions/${missionName}`, missionPath);
    await this.addMissions(id, [missionName]);
  }

  public async addMissions(
    id: string,
    missions: string[],
  ): Promise<AddMissionsResponse> {
    return await this.requestJson<AddMissionsResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/missions`),
      {
        method: "POST",
        body: this.createJsonBody(missions),
      },
    );
  }

  public async deleteMissions(
    id: string,
    missions: number[],
  ): Promise<DeleteMissionsResponse> {
    return await this.requestJson<DeleteMissionsResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/missions`),
      {
        method: "DELETE",
        body: this.createJsonBody(missions),
      },
    );
  }

  public async selectMission(
    id: string,
    missionId: number,
  ): Promise<StartMissionResponse> {
    return await this.requestJson<StartMissionResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/missions/${missionId}/select`),
      {
        method: "POST",
      },
    );
  }

  public async startMission(
    id: string,
    missionId: number,
  ): Promise<StartServerResponse> {
    return await this.requestJson<StartServerResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/missions/${missionId}/start`),
      {
        method: "POST",
      },
    );
  }

  public async pauseDcsServer(id: string): Promise<GetPauseServerResponse> {
    return await this.requestJson<GetPauseServerResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/pause`),
      {
        method: "POST",
      },
    );
  }

  public async resumeDcsServer(id: string): Promise<GetResumeServerResponse> {
    return await this.requestJson<GetResumeServerResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/resume`),
      {
        method: "POST",
      },
    );
  }

  public async saveSettings(
    id: string,
    settings: SetServerSettingsRequest,
  ): Promise<SetServerSettingsResponse> {
    return await this.requestJson<SetServerSettingsResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/settings`),
      {
        method: "POST",
        body: this.createJsonBody(settings),
      },
    );
  }

  public async kickPlayer(
    id: string,
    request: KickPlayerRequest,
  ): Promise<KickPlayerResponse> {
    return await this.requestJson<KickPlayerResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/kick`),
      {
        method: "POST",
        body: this.createJsonBody(request),
      },
    );
  }

  public async banPlayer(
    id: string,
    request: BanPlayerRequest,
  ): Promise<BanPlayerResponse> {
    return await this.requestJson<BanPlayerResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/ban`),
      {
        method: "POST",
        body: this.createJsonBody(request),
      },
    );
  }

  public async sendChat(
    id: string,
    request: SendChatRequest,
  ): Promise<SendChatResponse> {
    return await this.requestJson<SendChatResponse>(
      this.buildUrl(`/game_servers/${id}/dcs-api/sendChat`),
      {
        method: "POST",
        body: this.createJsonBody(request),
      },
    );
  }

  public async getSrsClients(id: string): Promise<SrsServerInfo> {
    return await this.requestJson<SrsServerInfo>(
      this.buildUrl(`/game_servers/${id}/mods/srs/clients`),
      {
        method: "GET",
      },
    );
  }

  public async kickSrsClient(
    id: string,
    request: SrsModRequest,
  ): Promise<void> {
    await this.requestVoid(this.buildUrl(`/game_servers/${id}/mods/srs/kick`), {
      method: "POST",
      body: this.createJsonBody(request),
    });
  }

  public async banSrsClient(id: string, request: SrsModRequest): Promise<void> {
    await this.requestVoid(this.buildUrl(`/game_servers/${id}/mods/srs/ban`), {
      method: "POST",
      body: this.createJsonBody(request),
    });
  }

  public async executeWebconsole(
    id: string,
    request: WebConsoleExecuteRequest,
  ): Promise<string> {
    return await this.requestJson<string>(
      this.buildUrl(`/game_servers/${id}/mods/webconsole/execute`),
      {
        method: "POST",
        body: this.createJsonBody(request),
      },
    );
  }

}
