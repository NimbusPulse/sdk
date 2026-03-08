use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::billing::BillingType;
use super::dcs_runtime::DcsRuntime;
use super::dcs_settings::DcsSettings;
use super::region::Region;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GameType {
    Dcs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Instance {
    pub id: Uuid,
    pub node_id: Uuid,
    pub user_id: Uuid,
    pub product_id: Uuid,
    pub game_type: GameType,
    pub billing_type: BillingType,
    pub port: i32,
    pub webgui_port: i32,
    pub ftp_port: i32,
    pub ftp_username: String,
    pub ftp_password: String,
    pub pid: Option<i32>,
    pub status: InstanceStatus,
    pub want_delete: bool,
    pub wanted_terrains: Vec<Terrain>,
    pub rented_at: i64,
    pub rented_until: Option<i64>,
    pub active_mods: Vec<String>,
    pub created_at: String,
    pub dcs_settings: Option<DcsSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceNodeResource {
    pub region: Region,
    pub ip: String,
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InstanceResource {
    #[serde(flatten)]
    pub instance: Instance,
    #[serde(flatten)]
    pub node: InstanceNodeResource,
    pub runtime: Option<GameData>,
}

pub type InstancesResponse = Vec<InstanceResource>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GameData {
    Dcs(DcsRuntime),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Terrain {
    Afghanistan,
    Caucasus,
    Falklands,
    Iraq,
    Kola,
    MarianaIslands,
    MarianaIslandsWWII,
    Nevada,
    Normandy,
    PersianGulf,
    Sinai,
    Syria,
    TheChannel,
    GermanyCW,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
pub enum InstanceStoppedReason {
    StoppedNormally,
    StoppedUnexpectedly,
    MaxTriesReached,
    ServerUpdating,
    RebootRequestedThroughFile,
    DcsSessionExpired,
    StoppedForRestart { scheduled: bool },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiError {
    pub message: String,
    pub code: String,
}
