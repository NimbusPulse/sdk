use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::dcs_settings::DcsSettings;

#[derive(Debug, Deserialize, Clone)]
pub struct NodeGame {
    pub id: Uuid,
    pub node_id: Uuid,
    pub user_id: String,
    pub ip: String,
    pub port: u32,
    pub webgui_port: u32,
    pub ftp_port: u32,
    pub ftp_username: String,
    pub ftp_password: String,
    pub pid: Option<u32>,
    pub status: NodeGameStatus,
    pub want_delete: bool,
    pub wanted_terrains: Vec<Terrain>,
    pub rented_at: u64,
    pub active_mods: Vec<String>,
    pub created_at: String,
    pub dcs_settings: Option<DcsSettings>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Terrain {
    Caucasus,
    Falklands,
    Kola,
    MarianaIslands,
    Nevada,
    Normandy,
    PersianGulf,
    Sinai,
    Syria,
    TheChannel,
}

#[derive(Debug, Clone, Deserialize)]
pub enum NodeGameStatus {
    InstallingBaseGame {
        progress: Option<u8>,
    },
    InstallingTerrains {
        installed: Vec<Terrain>,
        processing: Option<Terrain>,
        processing_progress: Option<u8>,
    },
    InstallingMods,
    InstallingPost,
    ServerStarted,
    ServerStopped {
        was_error: bool,
        reason: String,
    },
    ServerExpired,
    ServerDeleted,
    WantServerStarted,
    WantServerStopped,
    WantUpdateServer,
}
