use anyhow::{bail, Ok, Result};
use serde::Serialize;
pub use types::instance::{Instance, InstanceStatus, Terrain};
use types::{dcs_runtime::DcsRuntime, system_resources::ServerResources};
use uuid::Uuid;

mod types;

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct CreateInstanceRequest {
    pub product_id: Uuid,
    pub settings: DcsSettingsPayload,
    pub active_mods: Vec<String>,
    pub wanted_terrains: Vec<Terrain>,
}

#[derive(Debug, Serialize)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct DcsSettingsPayload {
    pub initial_server_name: String,
    pub initial_server_password: String,
    pub initial_max_players: u32,
    pub enable_io: bool,
    pub enable_os: bool,
    pub enable_lfs: bool,
    pub initial_use_voice_chat: bool,
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
        use_voice_chat: bool,
        enable_io: bool,
        enable_os: bool,
        enable_lfs: bool,
    ) -> Result<Instance> {
        let payload = CreateInstanceRequest {
            product_id: plan,
            settings: DcsSettingsPayload {
                initial_server_name: name.into(),
                initial_server_password: password.map(|p| p.into()).unwrap_or_default(),
                initial_max_players: max_players,
                initial_use_voice_chat: use_voice_chat,
                enable_io,
                enable_os,
                enable_lfs,
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
            bail!(format!(
                "Failed to create server: {:?}",
                response.text().await?
            ));
        }

        Ok(response.json::<Instance>().await?)
    }

    pub async fn get_runtime(&self, id: &Uuid) -> Result<DcsRuntime> {
        let response = self
            .reqwest_client
            .get(format!("{}/game_servers/{}/runtime", Self::BASE_URL, id))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to get runtime: {:?}", response));
        }

        Ok(response.json::<DcsRuntime>().await?)
    }

    pub async fn get_server_resources(&self, id: &Uuid) -> Result<ServerResources> {
        let response = self
            .reqwest_client
            .get(format!("{}/game_servers/{}/resources", Self::BASE_URL, id))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to get server resources: {:?}", response));
        }

        Ok(response.json::<ServerResources>().await?)
    }

    pub async fn add_missions(&self, id: &Uuid, missions: Vec<String>) -> Result<()> {
        let response = self
            .reqwest_client
            .post(format!(
                "{}/game_servers/{}/dcs-api/missions",
                Self::BASE_URL,
                id
            ))
            .bearer_auth(self.api_key.clone())
            .json(&missions)
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Failed to add missions: {:?}", response.text().await?);
        }

        Ok(())
    }

    pub async fn start_mission(&self, id: &Uuid, mission_idx: u32) -> Result<()> {
        let response = self
            .reqwest_client
            .post(format!(
                "{}/game_servers/{}/dcs-api/missions/{}/start",
                Self::BASE_URL,
                id,
                mission_idx
            ))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Failed to start mission: {:?}", response);
        }

        Ok(())
    }

    pub async fn get_servers(&self) -> Result<Vec<Instance>> {
        let response = self
            .reqwest_client
            .get(format!("{}/game_servers", Self::BASE_URL))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to create server: {:?}", response));
        }

        Ok(response.json::<Vec<Instance>>().await?)
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

    pub async fn resume_server(&self, id: &Uuid) -> Result<()> {
        let response = self
            .reqwest_client
            .post(format!(
                "{}/game_servers/{}/dcs-api/resume",
                Self::BASE_URL,
                id
            ))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Failed to resume server: {:?}", response);
        }

        Ok(())
    }

    pub async fn pause_server(&self, id: &Uuid) -> Result<()> {
        let response = self
            .reqwest_client
            .post(format!(
                "{}/game_servers/{}/dcs-api/pause",
                Self::BASE_URL,
                id
            ))
            .bearer_auth(self.api_key.clone())
            .send()
            .await?;

        if !response.status().is_success() {
            bail!("Failed to pause server: {:?}", response);
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
