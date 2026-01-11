use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::dcs_settings::DcsSettings;

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct Instance {
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
    pub status: InstanceStatus,
    pub want_delete: bool,
    pub wanted_terrains: Vec<Terrain>,
    pub rented_at: u64,
    pub rented_until: Option<u32>,
    pub active_mods: Vec<String>,
    pub created_at: String,
    pub dcs_settings: Option<DcsSettings>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub enum Terrain {
    Afghanistan,
    Caucasus,
    Falklands,
    Iraq,
    Kola,
    MarianaIslands,
    Nevada,
    Normandy,
    PersianGulf,
    Sinai,
    Syria,
    TheChannel,
    GermanyCW,
}

#[derive(Debug, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub enum InstanceStatus {
    AwaitingContainer,
    InstallingBaseGame {
        progress: Option<u8>,
    },
    InstallingTerrains {
        installed: Vec<Terrain>,
        processing: Option<Terrain>,
        processing_progress: Option<u8>,
        is_post_creation: bool,
    },
    InstallingMods,
    InstallingPost,
    UninstallingTerrains {
        want_uninstall: Vec<Terrain>,
        after_install: Vec<Terrain>,
    },
    ServerStarted,
    ServerStopped {
        was_error: bool,
        reason: InstanceStoppedReason,
    },
    ServerExpired,
    ServerDeleted,
    WantServerStarted {
        current_try: u32,
    },
    WantServerStopped {
        error_passthrough: Option<(bool, InstanceStoppedReason)>,
    },
    WantUpdateServer {
        was_stopped: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub enum InstanceStoppedReason {
    StoppedNormally,
    StoppedUnexpectedly,
    MaxTriesReached,
    ServerUpdating,
    RebootRequestedThroughFile,
    DcsSessionExpired,
    StoppedForRestart { scheduled: bool },
}
