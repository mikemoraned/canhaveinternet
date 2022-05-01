use serde::{Deserialize, Serialize};
use chrono::DateTime;
use chrono::Utc;
use serde_json::Result;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ping {
    pub jitter: f64,
    pub latency: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Speedtest {
    #[serde(rename = "type")]
    pub test_type: String,
    pub timestamp: DateTime<Utc>,
    pub ping: Ping
}


pub fn run_speedtest(speedtest_binary : &str) -> Result<Speedtest> {
    let output = Command::new(speedtest_binary)
        .arg("--format")
        .arg("json-pretty")
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", &stdout);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

    Ok(serde_json::from_str(&stdout)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Result;

    #[test]
    fn can_parse_example() -> Result<()> {
        let data = r#"
        {
            "type": "result",
            "timestamp": "2022-05-01T14:07:28Z",
            "ping": {
                "jitter": 4.2119999999999997,
                "latency": 68.644000000000005
            },
            "download": {
                "bandwidth": 1624264,
                "bytes": 19185120,
                "elapsed": 12412
            },
            "upload": {
                "bandwidth": 407593,
                "bytes": 5687760,
                "elapsed": 14941
            },
            "packetLoss": 0,
            "isp": "Plusnet",
            "interface": {
                "internalIp": "192.168.2.30",
                "name": "en0",
                "macAddr": "18:3E:EF:E3:77:BC",
                "isVpn": false,
                "externalIp": "212.159.124.188"
            },
            "server": {
                "id": 22971,
                "host": "speedtest.telf.uk.as61049.net",
                "port": 8080,
                "name": "Exascale",
                "location": "Telford",
                "country": "United Kingdom",
                "ip": "185.231.218.10"
            },
            "result": {
                "id": "665584f8-9895-4a9b-878b-73db433573b9",
                "url": "https://www.speedtest.net/result/c/665584f8-9895-4a9b-878b-73db433573b9",
                "persisted": true
            }
        }"#;

        let _t: Speedtest = serde_json::from_str(data)?;

        Ok(())
    }
}