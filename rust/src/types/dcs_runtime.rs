use std::collections::HashMap;

use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct DcsRuntime {
    pub paused: bool,
    pub mission_info: Option<MissionInfo>,
    pub mission_list: Option<MissionList>,
    pub players: Option<PlayersResponse>,
    pub settings: Option<ServerSettings>,
}

#[derive(Deserialize, Debug, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct MissionInfo {
    result_red: Option<i32>,
    result_blue: Option<i32>,
    mission_filename: String,
    mission_time: f32,
    mission_name: String,
    mission_description: String,
}

#[derive(Deserialize, Debug, Default, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]

pub struct MissionList {
    pub mission_list: Vec<String>,
    pub mission_theatres: Vec<String>,
    pub list_start_index: i32,
    pub list_shuffle: bool,
    pub list_loop: bool,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct PlayersResponse {
    pub players: Players,
    pub server_id: i32,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct Players {
    pub banned: HashMap<String, Player>,
    pub all: HashMap<String, Player>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct Player {
    pub ping: i32,
    pub side: i32,
    pub slot: String,
    pub id: i32,
    pub name: String,
    pub score: i32,
    pub ucid: String,
    pub started: bool,
    pub lang: String,
    pub ipaddr: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct ServerSettings {
    pub mission_list: MissionList,
    pub settings: Settings,
    pub ip: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct Settings {
    pub description: String,
    pub require_pure_textures: bool,
    pub list_start_index: i32,
    pub advanced: AdvancedSettings,
    pub port: i32,
    pub mode: i32,
    pub bind_address: String,
    pub is_public: bool,
    pub list_shuffle: bool,
    pub password: String,
    pub list_loop: bool,
    pub name: String,
    pub require_pure_scripts: bool,
    pub mission_list: Vec<String>,
    pub require_pure_clients: bool,
    pub require_pure_models: bool,
    pub max_players: i32,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct AdvancedSettings {
    pub allow_change_tailno: bool,
    pub disable_events: bool,
    pub allow_ownship_export: bool,
    pub allow_object_export: bool,
    pub pause_on_load: bool,
    pub allow_sensor_export: bool,
    pub event_takeoff: bool,
    pub pause_without_clients: bool,
    pub client_outbound_limit: i32,
    pub client_inbound_limit: i32,
    pub server_can_screenshot: bool,
    pub allow_players_pool: bool,
    pub voice_chat_server: bool,
    pub allow_change_skin: bool,
    pub event_connect: bool,
    pub event_ejecting: bool,
    pub event_kill: bool,
    pub event_crash: bool,
    pub event_role: bool,
    pub resume_mode: i32,
    pub max_ping: i32,
    pub allow_trial_only_clients: bool,
    pub allow_dynamic_radio: bool,
}
