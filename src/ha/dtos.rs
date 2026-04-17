use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize)]
pub struct HaMessageResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct HaConfigResponse {
    #[tabled(display = "display_string_vec")]
    pub components: Vec<String>,
    pub config_dir: String,
    pub elevation: i64,
    pub latitude: f64,
    pub location_name: String,
    pub longitude: f64,
    pub time_zone: String,
    #[tabled(skip)]
    pub unit_system: HaUnitSystem,
    pub version: String,
    #[tabled(display = "display_string_vec")]
    pub whitelist_external_dirs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Tabled)]
pub struct HaUnitSystem {
    pub length: String,
    pub mass: String,
    pub temperature: String,
    pub volume: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaComponentsResponse(pub Vec<String>);

#[derive(Debug, Serialize, Deserialize)]
pub struct HaEventsResponse(pub Vec<HaEvent>);

#[derive(Debug, Serialize, Deserialize)]
pub struct HaEvent {
    pub event: String,
    pub listener_count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaServicesResponse(pub Vec<HaService>);

#[derive(Debug, Serialize, Deserialize)]
pub struct HaService {
    pub domain: String,
    pub services: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaHistoryResponse(pub Vec<Vec<HaStateChange>>);

#[derive(Debug, Serialize, Deserialize)]
pub struct HaStateChange {
    pub attributes: Option<HaStateChangeAttribute>,
    pub entity_id: Option<String>,
    pub last_changed: DateTime<Utc>,
    pub last_updated: Option<DateTime<Utc>>,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaStateChangeAttribute {
    pub friendly_name: Option<String>,
    pub unit_of_measurement: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaHistoryOptions {
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
pub struct HaHistoryQuery {
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

impl HaHistoryQuery {
    pub fn from_default(filter_entity_ids: Vec<String>) -> Self {
        Self {
            filter_entity_id: filter_entity_ids.join(","),
            end_time: None,
            minimal_response: false,
            no_attributes: false,
            significant_changes_only: false,
        }
    }

    pub fn from_query_options(value: &HaHistoryOptions, filter_entity_ids: Vec<String>) -> Self {
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
pub struct HaLogbookResponse(pub Vec<HaLogbookEntry>);

#[derive(Debug, Serialize, Deserialize)]
pub struct HaLogbookEntry {
    pub context_user_id: Option<String>,
    pub domain: String,
    pub entity_id: String,
    pub message: String,
    pub name: String,
    pub when: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaLogbookOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaStatesResponse(pub Vec<HaState>);

#[derive(Debug, Serialize, Deserialize)]
pub struct HaState {
    pub attributes: HashMap<String, Value>,
    pub entity_id: String,
    pub last_changed: DateTime<Utc>,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaStateUpdateRequest {
    pub state: String,
    pub attributes: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaStateUpdateResponse {
    pub attributes: HashMap<String, Value>,
    pub entity_id: String,
    pub last_changed: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub state: String,
}

pub type HaEventData = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
pub struct HaDomainServiceResponse(pub Vec<HaState>);

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceData {
    pub entity_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HaDomainServiceReturnResponse {
    pub changed_states: Vec<HaState>,
    pub service_response: HashMap<String, Value>,
}

/* DISPLAY HELPERS REMOVE FROM THIS FILE EVENTUALLY */

fn display_string_vec(values: &[String]) -> String {
    values.join(", ")
}
