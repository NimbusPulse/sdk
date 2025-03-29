use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct DcsRuntime {
    pub paused: bool,
    pub mission_info: Option<MissionInfo>,
    pub mission_list: Option<MissionList>,
    pub players: Option<PlayersResponse>,
    pub settings: Option<ServerSettings>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct MissionInfo {
    pub result_red: Option<i32>,
    pub result_blue: Option<i32>,
    pub mission_filename: String,
    pub mission_time: f32,
    pub mission_name: String,
    pub mission_description: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct MissionList {
    #[serde(
        rename = "missionList",
        deserialize_with = "super::deserialize_mission_field"
    )]
    pub mission_list: Vec<String>,
    #[serde(
        rename = "missionTheatres",
        deserialize_with = "super::deserialize_mission_field"
    )]
    pub mission_theatres: Vec<String>,
    #[serde(rename = "listStartIndex")]
    pub list_start_index: i32,
    #[serde(rename = "listShuffle")]
    pub list_shuffle: bool,
    #[serde(rename = "listLoop")]
    pub list_loop: bool,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct PlayersResponse {
    pub players: Players,
    pub server_id: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct Players {
    #[serde(deserialize_with = "super::deserialize_array_object")]
    pub banned: Vec<BannedPlayer>,
    pub all: HashMap<String, Player>,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct BannedPlayer {
    pub banned_from: i64,
    pub banned_until: i64,
    pub ipaddr: String,
    pub name: String,
    pub reason: String,
    pub ucid: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
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

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct ServerSettings {
    pub mission_list: MissionList,
    pub settings: Settings,
    pub ip: String,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct Settings {
    pub description: String,
    pub require_pure_textures: bool,
    #[serde(rename = "listStartIndex")]
    pub list_start_index: i32,
    pub advanced: AdvancedSettings,
    pub port: i32,
    pub mode: i32,
    pub bind_address: String,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "listShuffle")]
    pub list_shuffle: bool,
    pub password: String,
    #[serde(rename = "listLoop")]
    pub list_loop: bool,
    pub name: String,
    pub require_pure_scripts: bool,
    #[serde(
        rename = "missionList",
        deserialize_with = "super::deserialize_mission_field"
    )]
    pub mission_list: Vec<String>,
    pub require_pure_clients: bool,
    pub require_pure_models: bool,
    #[serde(rename = "maxPlayers")]
    pub max_players: i32,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct AdvancedSettings {
    pub allow_change_tailno: bool,
    pub disable_events: bool,
    pub allow_ownship_export: bool,
    pub allow_object_export: bool,
    pub pause_on_load: bool,
    pub allow_sensor_export: bool,
    #[serde(rename = "event_Takeoff")]
    pub event_takeoff: bool,
    pub pause_without_clients: bool,
    pub client_outbound_limit: i32,
    pub client_inbound_limit: i32,
    pub server_can_screenshot: bool,
    pub allow_players_pool: bool,
    pub voice_chat_server: bool,
    pub allow_change_skin: bool,
    #[serde(rename = "event_Connect")]
    pub event_connect: bool,
    #[serde(rename = "event_Ejecting")]
    pub event_ejecting: bool,
    #[serde(rename = "event_Kill")]
    pub event_kill: bool,
    #[serde(rename = "event_Crash")]
    pub event_crash: bool,
    #[serde(rename = "event_Role")]
    pub event_role: bool,
    pub resume_mode: i32,
    #[serde(rename = "maxPing")]
    pub max_ping: i32,
    pub allow_trial_only_clients: bool,
    pub allow_dynamic_radio: bool,
    #[serde(rename = "redPasswordHash")]
    pub red_password_hash: Option<String>,
    #[serde(rename = "bluePasswordHash")]
    pub blue_password_hash: Option<String>,
    #[serde(rename = "redPassword")]
    pub red_password: Option<String>,
    #[serde(rename = "bluePassword")]
    pub blue_password: Option<String>,
}
