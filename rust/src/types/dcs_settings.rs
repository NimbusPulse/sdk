use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct DcsSettings {
    pub server_name: String,
    pub server_password: String,
    pub max_players: u32,
    pub use_own_credentials: bool,
    pub credentials_username: String,
    pub credentials_password: String,
    pub use_voice_chat: bool,
}
