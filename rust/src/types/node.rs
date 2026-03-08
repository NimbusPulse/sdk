use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::region::Region;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
    pub region: Region,
    pub ip: String,
    pub domain: String,
}
