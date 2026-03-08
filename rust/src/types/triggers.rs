use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "config")]
pub enum TriggerCondition {
    PlayerCount {
        operator: ComparisonOperator,
        threshold: u32,
    },
    OnEvent {
        event_type: String,
    },
    Schedule {
        cron_expression: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "params")]
pub enum TriggerAction {
    RestartInstance,
    StopInstance,
    ExecuteLuaScript { script: String },
    SendChatMessage { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Trigger {
    pub id: Option<Uuid>,
    pub instance_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub condition: TriggerCondition,
    pub action: TriggerAction,
    pub last_executed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreateTriggerRequest {
    pub name: String,
    pub description: Option<String>,
    pub condition: TriggerCondition,
    pub action: TriggerAction,
}
