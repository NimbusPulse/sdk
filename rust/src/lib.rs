use std::path::PathBuf;

use anyhow::{Ok, Result, bail};
use serde::{Deserialize, Serialize};
pub use types::billing::BillingType;
pub use types::dcs_api::{
    AddMissionsResponse, BanPlayerRequest, BanPlayerResponse, DeleteMissionsResponse,
    GetPauseServerResponse, GetResumeServerResponse, KickPlayerRequest, KickPlayerResponse,
    SendChatRequest, SendChatResponse, SetServerSettingsRequest, SetServerSettingsResponse,
    StartMissionResponse, StartServerResponse,
};
pub use types::dcs_chat::DcsChat;
pub use types::dcs_runtime::{
    AdvancedSettings, BannedPlayer, CurrentRuntimeAction, DcsRuntime, GetMissionInfoResponse,
    GetMissionListResponse, GetPlayersResponse, GetServerSettingsResponse, Player, Players,
    Settings,
};
pub use types::dcs_settings::{DcsSettings, DcsSettingsPayload, DcsSettingsUpdatePayload};
pub use types::files::{
    FileDownloadResponse, FileInfo, FileListResponse, FileUploadRequest, MoveFileRequest,
};
pub use types::instance::{
    ApiError, GameRuntime, GameType, Instance, InstanceNodeResource, InstanceResource,
    InstanceStatus, InstanceStoppedReason, InstancesResponse, Terrain,
};
pub use types::region::Region;
pub use types::srs::{SrsClient, SrsModRequest, SrsServerInfo};
pub use types::system_resources::{PrometheusSeries, ServerResourcesResponse};
pub use types::system_resources_periode::SystemResourcesPeriod;
pub use types::triggers::{
    ComparisonOperator, CreateTriggerRequest, Trigger, TriggerAction, TriggerCondition,
};
pub use types::webconsole::WebConsoleExecuteRequest;

use reqwest::multipart::{Form, Part};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub use uuid::Uuid;

mod types;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateInstanceRequest {
    pub product_id: Uuid,
    pub region: Region,
    pub billing_type: BillingType,
    pub settings: DcsSettingsPayload,
    pub active_mods: Vec<String>,
    pub wanted_terrains: Vec<Terrain>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "game_type", content = "settings", rename_all = "snake_case")]
pub enum EditInstanceRequest {
    Dcs(DcsSettingsUpdatePayload),
}

impl EditInstanceRequest {
    pub fn dcs(settings: DcsSettingsUpdatePayload) -> Self {
        Self::Dcs(settings)
    }
}

impl From<DcsSettingsUpdatePayload> for EditInstanceRequest {
    fn from(settings: DcsSettingsUpdatePayload) -> Self {
        Self::dcs(settings)
    }
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

    async fn send(&self, request: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        Ok(request.bearer_auth(self.api_key.clone()).send().await?)
    }

    async fn send_json<T>(&self, request: reqwest::RequestBuilder) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let response = self.send(request).await?;
        if !response.status().is_success() {
            bail!(
                response
                    .text()
                    .await
                    .unwrap_or_else(|_| "request failed".to_string())
            );
        }

        Ok(response.json::<T>().await?)
    }

    async fn send_unit(&self, request: reqwest::RequestBuilder, context: &str) -> Result<()> {
        let response = self.send(request).await?;
        if !response.status().is_success() {
            bail!("{context}: {}", response.text().await.unwrap_or_default());
        }

        Ok(())
    }

    pub async fn create_server(
        &self,
        region: Region,
        billing_type: BillingType,
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
            region,
            billing_type,
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

        self.send_json(
            self.reqwest_client
                .post(format!("{}/game_servers", Self::BASE_URL))
                .json(&payload),
        )
        .await
    }

    pub async fn get_runtime(&self, id: &Uuid) -> Result<DcsRuntime> {
        let server = self.get_server(id).await?;

        match server.runtime {
            Some(GameRuntime::Dcs(runtime)) => Ok(runtime),
            None => bail!("server runtime is not available"),
        }
    }

    pub async fn get_server_resources(
        &self,
        id: &Uuid,
        period: SystemResourcesPeriod,
    ) -> Result<ServerResourcesResponse> {
        self.send_json(self.reqwest_client.get(format!(
            "{}/game_servers/{}/resources?periode={}",
            Self::BASE_URL,
            id,
            period
        )))
        .await
    }

    pub async fn health(&self) -> Result<()> {
        self.send_unit(
            self.reqwest_client
                .get(format!("{}/health", Self::BASE_URL)),
            "health check failed",
        )
        .await
    }

    pub async fn get_servers(&self) -> Result<InstancesResponse> {
        self.send_json(
            self.reqwest_client
                .get(format!("{}/game_servers", Self::BASE_URL)),
        )
        .await
    }

    pub async fn get_server(&self, id: &Uuid) -> Result<InstanceResource> {
        self.send_json(
            self.reqwest_client
                .get(format!("{}/game_servers/{}", Self::BASE_URL, id)),
        )
        .await
    }

    pub async fn update_server(
        &self,
        id: &Uuid,
        payload: &EditInstanceRequest,
    ) -> Result<InstanceResource> {
        self.send_json(
            self.reqwest_client
                .put(format!("{}/game_servers/{}", Self::BASE_URL, id))
                .json(payload),
        )
        .await
    }

    pub async fn change_server_terrains(&self, id: &Uuid, terrains: &[Terrain]) -> Result<()> {
        self.send_unit(
            self.reqwest_client
                .put(format!("{}/game_servers/{}/terrains", Self::BASE_URL, id))
                .json(terrains),
            "failed to change terrains",
        )
        .await
    }

    pub async fn get_chat(&self, id: &Uuid) -> Result<Vec<DcsChat>> {
        self.send_json(self.reqwest_client.get(format!(
            "{}/game_servers/{}/chat",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn start_server(&self, id: &Uuid) -> Result<Instance> {
        self.send_json(self.reqwest_client.post(format!(
            "{}/game_servers/{}/start",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn stop_server(&self, id: &Uuid) -> Result<Instance> {
        self.send_json(self.reqwest_client.post(format!(
            "{}/game_servers/{}/stop",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn full_restart_server(&self, id: &Uuid) -> Result<Instance> {
        self.send_json(self.reqwest_client.post(format!(
            "{}/game_servers/{}/full_restart",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn restart_server(&self, id: &Uuid) -> Result<Instance> {
        self.send_json(self.reqwest_client.post(format!(
            "{}/game_servers/{}/restart",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn update_game_server(&self, id: &Uuid) -> Result<Instance> {
        self.send_json(self.reqwest_client.post(format!(
            "{}/game_servers/{}/update",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn delete_server(&self, id: &Uuid) -> Result<()> {
        self.send_unit(
            self.reqwest_client
                .delete(format!("{}/game_servers/{}", Self::BASE_URL, id)),
            "failed to delete server",
        )
        .await
    }

    pub async fn list_files(&self, id: &Uuid, path: impl Into<String>) -> Result<FileListResponse> {
        self.send_json(self.reqwest_client.get(format!(
            "{}/game_servers/{}/files?path={}",
            Self::BASE_URL,
            id,
            path.into()
        )))
        .await
    }

    pub async fn create_directory(&self, id: &Uuid, path: impl Into<String>) -> Result<()> {
        self.send_unit(
            self.reqwest_client.post(format!(
                "{}/game_servers/{}/files/directory?path={}",
                Self::BASE_URL,
                id,
                path.into()
            )),
            "failed to create directory",
        )
        .await
    }

    pub async fn upload_file(
        &self,
        id: &Uuid,
        path: impl Into<String>,
        file: Vec<u8>,
    ) -> Result<()> {
        let part = Part::bytes(file)
            .file_name("upload.bin")
            .mime_str("application/octet-stream")?;
        let form = Form::new().part("file", part);

        self.send_unit(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/files/upload?path={}",
                    Self::BASE_URL,
                    id,
                    path.into()
                ))
                .multipart(form),
            "failed to upload file",
        )
        .await
    }

    pub async fn upload_file_from(
        &self,
        id: &Uuid,
        path: impl Into<String>,
        file: impl Into<PathBuf>,
    ) -> Result<()> {
        let form = Form::new().file("file", file.into()).await?;

        self.send_unit(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/files/upload?path={}",
                    Self::BASE_URL,
                    id,
                    path.into()
                ))
                .multipart(form),
            "failed to upload file",
        )
        .await
    }

    pub async fn download_file(&self, id: &Uuid, path: impl Into<String>) -> Result<Vec<u8>> {
        let response = self
            .send(self.reqwest_client.get(format!(
                "{}/game_servers/{}/files/download?path={}",
                Self::BASE_URL,
                id,
                path.into()
            )))
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to download file: {:?}", response));
        }

        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }

    pub async fn download_file_to(
        &self,
        id: &Uuid,
        path: impl Into<String>,
        destination: impl Into<PathBuf>,
    ) -> Result<()> {
        let response = self
            .send(self.reqwest_client.get(format!(
                "{}/game_servers/{}/files/download?path={}",
                Self::BASE_URL,
                id,
                path.into()
            )))
            .await?;

        if !response.status().is_success() {
            bail!(format!("Failed to download file: {:?}", response));
        }

        let bytes = response.bytes().await?;
        let mut file = File::create(destination.into()).await?;
        file.write_all(&bytes).await?;

        Ok(())
    }

    pub async fn delete_file(&self, id: &Uuid, path: impl Into<String>) -> Result<()> {
        self.send_unit(
            self.reqwest_client.delete(format!(
                "{}/game_servers/{}/files?path={}",
                Self::BASE_URL,
                id,
                path.into()
            )),
            "failed to delete file",
        )
        .await
    }

    pub async fn move_file(&self, id: &Uuid, request: &MoveFileRequest) -> Result<()> {
        self.send_unit(
            self.reqwest_client
                .put(format!("{}/game_servers/{}/files/move", Self::BASE_URL, id))
                .json(request),
            "failed to move file",
        )
        .await
    }

    pub async fn add_missions(
        &self,
        id: &Uuid,
        missions: &[String],
    ) -> Result<AddMissionsResponse> {
        self.send_json(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/dcs-api/missions",
                    Self::BASE_URL,
                    id
                ))
                .json(missions),
        )
        .await
    }

    pub async fn delete_missions(
        &self,
        id: &Uuid,
        missions: &[i32],
    ) -> Result<DeleteMissionsResponse> {
        self.send_json(
            self.reqwest_client
                .delete(format!(
                    "{}/game_servers/{}/dcs-api/missions",
                    Self::BASE_URL,
                    id
                ))
                .json(missions),
        )
        .await
    }

    pub async fn select_mission(
        &self,
        id: &Uuid,
        mission_idx: i32,
    ) -> Result<StartMissionResponse> {
        self.send_json(self.reqwest_client.post(format!(
            "{}/game_servers/{}/dcs-api/missions/{}/select",
            Self::BASE_URL,
            id,
            mission_idx
        )))
        .await
    }

    pub async fn start_mission(&self, id: &Uuid, mission_idx: i32) -> Result<StartServerResponse> {
        self.send_json(self.reqwest_client.post(format!(
            "{}/game_servers/{}/dcs-api/missions/{}/start",
            Self::BASE_URL,
            id,
            mission_idx
        )))
        .await
    }

    pub async fn pause_server(&self, id: &Uuid) -> Result<GetPauseServerResponse> {
        self.send_json(self.reqwest_client.post(format!(
            "{}/game_servers/{}/dcs-api/pause",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn resume_server(&self, id: &Uuid) -> Result<GetResumeServerResponse> {
        self.send_json(self.reqwest_client.post(format!(
            "{}/game_servers/{}/dcs-api/resume",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn save_settings(
        &self,
        id: &Uuid,
        request: &SetServerSettingsRequest,
    ) -> Result<SetServerSettingsResponse> {
        self.send_json(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/dcs-api/settings",
                    Self::BASE_URL,
                    id
                ))
                .json(request),
        )
        .await
    }

    pub async fn kick_player(
        &self,
        id: &Uuid,
        request: &KickPlayerRequest,
    ) -> Result<KickPlayerResponse> {
        self.send_json(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/dcs-api/kick",
                    Self::BASE_URL,
                    id
                ))
                .json(request),
        )
        .await
    }

    pub async fn ban_player(
        &self,
        id: &Uuid,
        request: &BanPlayerRequest,
    ) -> Result<BanPlayerResponse> {
        self.send_json(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/dcs-api/ban",
                    Self::BASE_URL,
                    id
                ))
                .json(request),
        )
        .await
    }

    pub async fn send_chat(
        &self,
        id: &Uuid,
        request: &SendChatRequest,
    ) -> Result<SendChatResponse> {
        self.send_json(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/dcs-api/sendChat",
                    Self::BASE_URL,
                    id
                ))
                .json(request),
        )
        .await
    }

    pub async fn get_srs_clients(&self, id: &Uuid) -> Result<SrsServerInfo> {
        self.send_json(self.reqwest_client.get(format!(
            "{}/game_servers/{}/mods/srs/clients",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn kick_srs_client(&self, id: &Uuid, request: &SrsModRequest) -> Result<()> {
        self.send_unit(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/mods/srs/kick",
                    Self::BASE_URL,
                    id
                ))
                .json(request),
            "failed to kick srs client",
        )
        .await
    }

    pub async fn ban_srs_client(&self, id: &Uuid, request: &SrsModRequest) -> Result<()> {
        self.send_unit(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/mods/srs/ban",
                    Self::BASE_URL,
                    id
                ))
                .json(request),
            "failed to ban srs client",
        )
        .await
    }

    pub async fn execute_webconsole(
        &self,
        id: &Uuid,
        request: &WebConsoleExecuteRequest,
    ) -> Result<String> {
        self.send_json(
            self.reqwest_client
                .post(format!(
                    "{}/game_servers/{}/mods/webconsole/execute",
                    Self::BASE_URL,
                    id
                ))
                .json(request),
        )
        .await
    }

    pub async fn create_trigger(
        &self,
        id: &Uuid,
        request: &CreateTriggerRequest,
    ) -> Result<Trigger> {
        self.send_json(
            self.reqwest_client
                .post(format!("{}/game_servers/{}/triggers", Self::BASE_URL, id))
                .json(request),
        )
        .await
    }

    pub async fn list_triggers(&self, id: &Uuid) -> Result<Vec<Trigger>> {
        self.send_json(self.reqwest_client.get(format!(
            "{}/game_servers/{}/triggers",
            Self::BASE_URL,
            id
        )))
        .await
    }

    pub async fn delete_trigger(&self, id: &Uuid, trigger_id: &Uuid) -> Result<()> {
        self.send_unit(
            self.reqwest_client.delete(format!(
                "{}/game_servers/{}/triggers/{}",
                Self::BASE_URL,
                id,
                trigger_id
            )),
            "failed to delete trigger",
        )
        .await
    }
}
