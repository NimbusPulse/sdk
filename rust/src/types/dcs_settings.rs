use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct DcsSettings {
    pub initial_server_name: String,
    pub initial_server_password: String,
    pub initial_max_players: u32,
    pub use_own_credentials: bool,
    pub credentials_username: String,
    pub credentials_password: String,
    pub initial_use_voice_chat: bool,
}
