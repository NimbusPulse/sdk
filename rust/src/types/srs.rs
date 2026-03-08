use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SrsClient {
    #[serde(rename = "ClientGuid")]
    pub client_guid: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Coalition")]
    pub coalition: i32,
    #[serde(rename = "AllowRecord")]
    pub allow_record: bool,
    #[serde(rename = "Seat")]
    pub seat: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SrsServerInfo {
    #[serde(rename = "Clients")]
    pub clients: Vec<SrsClient>,
    #[serde(rename = "ServerVersion")]
    pub server_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SrsModRequest {
    pub guid: String,
}
