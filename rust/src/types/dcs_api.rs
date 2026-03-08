use serde::{Deserialize, Serialize};

use super::dcs_runtime::{GetMissionListResponse, Settings};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddMissionsResponse(pub bool);

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetPauseServerResponse;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GetResumeServerResponse;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SetServerSettingsResponse(pub bool);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetServerSettingsRequest(pub Settings);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KickPlayerResponse(pub bool);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KickPlayerRequest {
    pub id: i32,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BanPlayerResponse(pub bool);

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SendChatResponse;
