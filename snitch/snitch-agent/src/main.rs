extern crate dotenv;
use futures::prelude::*;
use influxdb2::Client;
use influxdb2::models::DataPoint;
use std::time::Duration;
use tokio::{time, task};

mod speedtest;

async fn periodically_run_speedtest(speedtest_binary: &str, agent_name: &str, client: &Client) 
    -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        println!("Running test");
        let test: speedtest::Speedtest = speedtest::run_speedtest(&speedtest_binary).unwrap();
        println!("Test type {}, for timestamp: {}, {:?}", test.test_type, test.timestamp, test.ping);
        let bucket = "snitch-agent";

        let nanos_per_second = 1000000000i64;
        let points = vec![
            DataPoint::builder("speedtest")
                .timestamp(test.timestamp.timestamp() * nanos_per_second)
                .tag("agent_name", agent_name)
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
    let agent_name = "agent1";
    task::spawn(async move {
        periodically_run_speedtest(speedtest_binary, agent_name, &client).await?;

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    }).await?
}