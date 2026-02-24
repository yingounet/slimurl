use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsRecord {
    pub id: i64,
    pub link_id: i64,
    pub accessed_at: String,
    pub ip_hash: Option<String>,
    pub country: Option<String>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub referer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub pv: i64,
    pub uv: i64,
    pub countries: HashMap<String, i64>,
    pub devices: HashMap<String, i64>,
    pub browsers: HashMap<String, i64>,
    pub referer: HashMap<String, i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStatsRequest {
    pub link_id: i64,
    pub ip_hash: Option<String>,
    pub country: Option<String>,
    pub device_type: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub referer: Option<String>,
}
