use serde::Deserialize;
use ts_rs::TS;

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct DcsSettings {
    pub initial_server_name: String,
    pub initial_server_password: String,
    pub initial_max_players: u32,
    pub use_own_credentials: bool,
    pub credentials_username: String,
    pub credentials_password: String,
    pub initial_use_voice_chat: bool,
}
