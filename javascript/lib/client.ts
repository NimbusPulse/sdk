import { CreateServerRequest } from "./types/CreateServerRequest";
import { DcsCredentials } from "./types/DcsCredentials";
import { NodeGame } from "./types/NodeGame";
import { Terrain } from "./types/Terrain";
import { readFile } from "fs/promises";
import { Client as FtpClient } from 'basic-ftp'
import { Readable } from "stream";
import path from "path";

type Fetch = { (input: URL | RequestInfo, init?: RequestInit | undefined): Promise<Response>; (input: string | URL | Request, init?: RequestInit | undefined): Promise<Response>; };

export default class Client {
    private apiKey: string;
    private fetch: Fetch;
    private static readonly BASE_URL = 'https://coordinator.nimbuspulse.com';

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

    private async ftp(server: NodeGame): Promise<FtpClient> {
        const client = new FtpClient();
        await client.access({
            host: server.ip,
            user: server.ftp_username,
            password: server.ftp_password,
            port: server.ftp_port,
            secure: false
        });

        return client;
    }

    private async fetchWithAuth(
        url: string,
        options: RequestInit
    ): Promise<Response> {
        const headers = {
            'Authorization': `Bearer ${this.apiKey}`,
            'Content-Type': 'application/json',
            ...options.headers
        };

        const response = await this.fetch(url, { ...options, headers });
        if (!response.ok) {
            throw new Error(`Failed request: ${response.statusText}`);
        }
        return response;
    }

    public async createServer(
        name: string,
        password: string | null,
        maxPlayers: number,
        plan: string,
        activeMods: string[],
        terrains: Terrain[],
        credentials: DcsCredentials | null,
        useVoiceChat: boolean
    ): Promise<NodeGame> {
        const payload: CreateServerRequest = {
            product_id: plan,
            settings: {
                server_name: name,
                server_password: password ?? '',
                max_players: maxPlayers,
                use_own_credentials: credentials !== null,
                credentials: credentials || null,
                use_voice_chat: useVoiceChat
            },
            active_mods: activeMods,
            wanted_terrains: terrains
        };

        const response = await this.fetchWithAuth(
            `${Client.BASE_URL}/game_servers`,
            {
                method: 'POST',
                body: JSON.stringify(payload)
            }
        );

        return await response.json();
    }

    public async getServers(): Promise<NodeGame[]> {
        const response = await this.fetchWithAuth(`${Client.BASE_URL}/game_servers`, {
            method: 'GET'
        });

        return await response.json();
    }

    public async startServer(id: string): Promise<void> {
        await this.fetchWithAuth(`${Client.BASE_URL}/game_servers/${id}/start`, {
            method: 'POST'
        });
    }

    public async stopServer(id: string): Promise<void> {
        await this.fetchWithAuth(`${Client.BASE_URL}/game_servers/${id}/stop`, {
            method: 'POST'
        });
    }

    public async deleteServer(id: string): Promise<void> {
        await this.fetchWithAuth(`${Client.BASE_URL}/game_servers/${id}`, {
            method: 'DELETE'
        });
    }

    public async uploadMission(id: string, missionPath: string): Promise<void> {
        const server = (await this.getServers()).find(server => server.id === id);
        if (!server) {
            throw new Error(`Server with id ${id} not found`);
        }

        const parsedMissionPath = path.parse(missionPath);
        const missionFile = await readFile(missionPath);

        const ftp = await this.ftp(server);

        await ftp.ensureDir('Missions')
        await ftp.uploadFrom(Readable.from(missionFile), parsedMissionPath.base);
        ftp.close()

        await this.addMission(server.id, [parsedMissionPath.base]);
    }

    private async addMission(id: string, missions: string[]): Promise<void> {
        await this.fetchWithAuth(`${Client.BASE_URL}/game_servers/${id}/dcs-api/missions`, {
            method: 'POST',
            body: JSON.stringify(missions)
        });
    }
}
