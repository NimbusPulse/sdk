use anyhow::{bail, Ok, Result};
use serde::Serialize;
use types::node_game::{NodeGame, Terrain};
use uuid::Uuid;

mod types;

#[derive(Debug, Serialize)]
pub struct CreateServerRequest {
    // Stupid? Yes. Will be removed? Yes. Now? No. After beta and before release? Yes.
    pub user_id: String,
    pub settings: DcsSettingsPayload,
    // Not used at all and will never be, don't even think this is a security risk
    pub minio_password: String,
    pub active_mods: Vec<String>,
    pub wanted_terrains: Vec<Terrain>,
}

#[derive(Debug, Serialize)]
pub struct DcsSettingsPayload {
    pub server_name: String,
    pub server_password: String,
    pub max_players: u32,
    pub use_own_credentials: bool,
    pub credentials: Option<DcsCredentials>,
    pub use_voice_chat: bool,
}

#[derive(Debug, Serialize)]
pub struct DcsCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    user_id: Uuid,
    reqwest_client: reqwest::Client,
}

impl Client {
    const BASE_URL: &'static str = "https://coordinator.nimbuspulse.com";

    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
            api_key: api_key.into(),
            // TODO(timm): Yes, this is my user ID. No, it's not a security risk. Yes, it will be removed.
            user_id: Uuid::parse_str("0191fb3a-ba96-7341-9e20-f770b8f36b7c").unwrap(),
        }
    }

    pub fn set_api_key(&mut self, api_key: impl Into<String>) {
        self.api_key = api_key.into();
    }

    pub async fn create_server(
        &self,
        name: impl Into<String>,
        password: Option<impl Into<String>>,
        max_players: u32,
        plan: Uuid,
        active_mods: Vec<impl Into<String>>,
        terrains: Vec<Terrain>,
        credentials: Option<DcsCredentials>,
        use_voice_chat: bool,
    ) -> Result<NodeGame> {
        let payload = CreateServerRequest {
            user_id: self.user_id.to_string(),
            settings: DcsSettingsPayload {
                server_name: name.into(),
                server_password: password.map(|p| p.into()).unwrap_or_default(),
                max_players,
                use_own_credentials: credentials.is_some(),
                credentials,
                use_voice_chat,
            },
            minio_password: String::new(),
            active_mods: active_mods.into_iter().map(|m| m.into()).collect(),
            wanted_terrains: terrains,
        };

        let response = self
            .reqwest_client
            .post(format!("{}/game_servers", Self::BASE_URL))
            .bearer_auth(self.api_key.clone())
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to create server: {:?}", response));
        }

        Ok(response.json::<NodeGame>().await?)
    }

    pub async fn get_servers(&self) -> Result<Vec<NodeGame>> {
        let response = self
            .reqwest_client
            .get(format!("{}/game_servers", Self::BASE_URL))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to create server: {:?}", response));
        }

        Ok(response.json::<Vec<NodeGame>>().await?)
    }

    pub async fn start_server(&self, id: &Uuid) -> Result<()> {
        let response = self
            .reqwest_client
            .post(format!("{}/game_servers/{}/start", Self::BASE_URL, id))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to start server: {:?}", response));
        }

        Ok(())
    }

    pub async fn stop_server(&self, id: &Uuid) -> Result<()> {
        let response = self
            .reqwest_client
            .post(format!("{}/game_servers/{}/stop", Self::BASE_URL, id))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to stop server: {:?}", response));
        }

        Ok(())
    }

    pub async fn delete_server(&self, id: &Uuid) -> Result<()> {
        let response = self
            .reqwest_client
            .delete(format!("{}/game_servers/{}", Self::BASE_URL, id))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to delete server: {:?}", response));
        }

        Ok(())
    }
}
