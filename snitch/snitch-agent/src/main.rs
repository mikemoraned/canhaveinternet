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
        println!("Test type {}, for timestamp: {}, {:?}, download: {:?}, upload: {:?}", 
            test.test_type, test.timestamp, test.ping, test.download, test.upload);
        let bucket = "snitch-agent";

        let nanos_per_second = 1000000000i64;
        let points = vec![
            DataPoint::builder("speedtest")
                .timestamp(test.timestamp.timestamp() * nanos_per_second)
                .tag("agent_name", agent_name)
                .tag("test_type", test.test_type)
                .field("ping_jitter", test.ping.jitter)
                .field("ping_latency", test.ping.latency)
                .field("download_bandwidth", test.download.bandwidth as i64)
                .field("upload_bandwidth", test.upload.bandwidth as i64)
                .build()?
            ];
                                                             
        client.write(bucket, stream::iter(points)).await?;

        println!("Waiting ...");
        time::sleep(Duration::from_secs(60)).await;
    }
}

use clap::{Parser};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    speedtest_binary_path: String,
    agent_name: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();

    let host= dotenv::var("INFLUXDB_HOST").unwrap();
    let org = dotenv::var("INFLUXDB_ORG").unwrap();
    let token = dotenv::var("INFLUXDB_TOKEN").unwrap();
    let client = Client::new(host, org, token);
     
    task::spawn(async move {
        periodically_run_speedtest(&cli.speedtest_binary_path, &cli.agent_name, &client).await?;

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    }).await?
}