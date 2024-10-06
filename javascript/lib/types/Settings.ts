// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AdvancedSettings } from "./AdvancedSettings";

export type Settings = {
  description: string;
  require_pure_textures: boolean;
  list_start_index: number;
  advanced: AdvancedSettings;
  port: number;
  mode: number;
  bind_address: string;
  is_public: boolean;
  list_shuffle: boolean;
  password: string;
  list_loop: boolean;
  name: string;
  require_pure_scripts: boolean;
  mission_list: Array<string>;
  require_pure_clients: boolean;
  require_pure_models: boolean;
  max_players: number;
};