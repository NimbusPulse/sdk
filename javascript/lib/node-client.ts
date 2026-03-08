import { readFile, writeFile } from "node:fs/promises";
import * as path from "node:path";
import Client from "./client.ts";

export type { BinaryInput, Fetch } from "./client.ts";

export default class NodeClient extends Client {
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

  public async downloadFileTo(
    id: string,
    remotePath: string,
    destinationPath: string,
  ): Promise<void> {
    const content = await this.downloadFile(id, remotePath);
    await writeFile(destinationPath, content);
  }

  public async uploadMission(id: string, missionPath: string): Promise<void> {
    const missionName = path.parse(missionPath).base;
    await this.uploadFileFrom(id, `Missions/${missionName}`, missionPath);
    await this.addMissions(id, [missionName]);
  }
}
