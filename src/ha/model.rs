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
