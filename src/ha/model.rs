use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct HaStatusMessage {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaConfig {
    pub components: Vec<String>,
    pub config_dir: String,
    pub elevation: i64,
    pub latitude: f64,
    pub location_name: String,
    pub longitude: f64,
    pub time_zone: String,
    pub unit_system: UnitSystem,
    pub version: String,
    pub whitelist_external_dirs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitSystem {
    pub length: String,
    pub mass: String,
    pub temperature: String,
    pub volume: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Components(pub Vec<String>);

#[derive(Debug, Serialize, Deserialize)]
pub struct Events(pub Vec<EventObject>);

#[derive(Debug, Serialize, Deserialize)]
pub struct EventObject {
    pub event: String,
    pub listener_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Services(pub Vec<ServiceObject>);

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceObject {
    pub domain: String,
    pub services: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryResponse(pub Vec<Vec<StateChange>>);

#[derive(Debug, Serialize, Deserialize)]
pub struct StateChange {
    pub attributes: Option<StateChangeAttribute>,
    pub entity_id: Option<String>,
    pub last_changed: DateTime<Utc>,
    pub last_updated: Option<DateTime<Utc>>,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StateChangeAttribute {
    pub friendly_name: Option<String>,
    pub unit_of_measurement: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub minimal_response: bool,

    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub no_attributes: bool,

    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub significant_changes_only: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryQuery {
    pub filter_entity_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,

    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub minimal_response: bool,

    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub no_attributes: bool,

    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub significant_changes_only: bool,
}

impl HistoryQuery {
    pub fn from_default(filter_entity_ids: Vec<String>) -> Self {
        Self {
            filter_entity_id: filter_entity_ids.join(","),
            end_time: None,
            minimal_response: false,
            no_attributes: false,
            significant_changes_only: false,
        }
    }

    pub fn from_query_options(value: &HistoryOptions, filter_entity_ids: Vec<String>) -> Self {
        Self {
            filter_entity_id: filter_entity_ids.join(","),
            end_time: value.end_time,
            minimal_response: value.minimal_response,
            no_attributes: value.no_attributes,
            significant_changes_only: value.significant_changes_only,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogbookResponse(pub Vec<LogbookEntry>);

#[derive(Debug, Serialize, Deserialize)]
pub struct LogbookEntry {
    pub context_user_id: Option<String>,
    pub domain: String,
    pub entity_id: String,
    pub message: String,
    pub name: String,
    pub when: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogbookOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatesResponse(pub Vec<StateObject>);

#[derive(Debug, Serialize, Deserialize)]
pub struct StateObject {
    pub attributes: HashMap<String, String>,
    pub entity_id: String,
    pub last_changed: DateTime<Utc>,
    pub state: String,
}
