use serde::{Deserialize, Serialize};
use chrono::DateTime;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    jitter: f32,
    latency: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Speedtest {
    #[serde(rename = "type")]
    pub test_type: String,
    pub timestamp: DateTime<Utc>,
    pub ping: Ping
}
