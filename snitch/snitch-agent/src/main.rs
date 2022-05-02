extern crate dotenv;
use futures::prelude::*;
use influxdb2::Client;
use chrono::{DateTime, Utc};
use influxdb2::models::DataPoint;
use std::time::Duration;
use tokio::{time, task};

mod speedtest;

async fn periodically_run_speedtest(speedtest_binary: &str, client: &Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        println!("Running test");
        let test: speedtest::Speedtest = speedtest::run_speedtest(&speedtest_binary).unwrap();
        println!("Test type {}, for timestamp: {}, {:?}", test.test_type, test.timestamp, test.ping);
        let bucket = "snitch-agent";

        let points = vec![
            DataPoint::builder("speedtest")
                .tag("test_type", test.test_type)
                .field("ping_jitter", test.ping.jitter)
                .field("ping_latency", test.ping.latency)
                .build()?
            ];
                                                             
        client.write(bucket, stream::iter(points)).await?;

        println!("Waiting ...");
        time::sleep(Duration::from_secs(60)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let host= dotenv::var("INFLUXDB_HOST").unwrap();
    let org = dotenv::var("INFLUXDB_ORG").unwrap();
    let token = dotenv::var("INFLUXDB_TOKEN").unwrap();
    let client = Client::new(host, org, token);
     
    let speedtest_binary = "/opt/homebrew/bin/speedtest";
    task::spawn(async move {
        periodically_run_speedtest(speedtest_binary, &client).await?;

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    }).await?
}