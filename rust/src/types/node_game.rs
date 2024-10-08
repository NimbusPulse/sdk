use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use super::dcs_settings::DcsSettings;

#[derive(Debug, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub struct NodeGame {
    pub id: Uuid,
    pub node_id: Uuid,
    pub user_id: String,
    pub product_id: Uuid,
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

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
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

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
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
        reason: ServerStoppedReason,
    },
    ServerExpired,
    ServerDeleted,
    WantServerStarted {
        current_try: u32,
    },
    WantServerStopped,
    WantUpdateServer,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, export_to = "../../javascript/lib/types/")]
pub enum ServerStoppedReason {
    StoppedNormally,
    StoppedUnexpectedly,
    MaxTriesReached,
}
