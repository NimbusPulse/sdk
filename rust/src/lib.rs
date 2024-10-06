use anyhow::{bail, Ok, Result};
use serde::Serialize;
use ts_rs::TS;
use types::node_game::{NodeGame, Terrain};
use uuid::Uuid;

mod types;

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct CreateServerRequest {
    pub product_id: Uuid,
    pub settings: DcsSettingsPayload,
    pub active_mods: Vec<String>,
    pub wanted_terrains: Vec<Terrain>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct DcsSettingsPayload {
    pub server_name: String,
    pub server_password: String,
    pub max_players: u32,
    pub use_own_credentials: bool,
    pub credentials: Option<DcsCredentials>,
    pub use_voice_chat: bool,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct DcsCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
    reqwest_client: reqwest::Client,
}

impl Client {
    const BASE_URL: &'static str = "https://coordinator.nimbuspulse.com";

    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            reqwest_client: reqwest::Client::new(),
            api_key: api_key.into(),
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
            product_id: plan,
            settings: DcsSettingsPayload {
                server_name: name.into(),
                server_password: password.map(|p| p.into()).unwrap_or_default(),
                max_players,
                use_own_credentials: credentials.is_some(),
                credentials,
                use_voice_chat,
            },
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
