use serde::{Deserialize, Serialize};

use super::dcs_runtime::{GetMissionListResponse, Settings};

pub type AddMissionsResponse = bool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteMissionsResponse {
    pub result: bool,
    pub deleted_missions: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StartMissionResponse {
    pub mission_list: GetMissionListResponse,
    pub res: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartServerResponse {
    pub mission_list: GetMissionListResponse,
    pub res: i32,
}

pub type GetPauseServerResponse = ();

pub type GetResumeServerResponse = ();

pub type SetServerSettingsResponse = bool;

pub type SetServerSettingsRequest = Settings;

pub type KickPlayerResponse = bool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KickPlayerRequest {
    pub id: i32,
    pub reason: String,
}

pub type BanPlayerResponse = bool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BanPlayerRequest {
    pub id: i32,
    pub reason: String,
    pub ucid: String,
    pub period: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendChatRequest {
    pub all: bool,
    pub msg: String,
}

pub type SendChatResponse = ();
