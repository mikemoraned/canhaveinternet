extern crate dotenv;
use influx_db_client::{
    Client, Point, Points, Precision, points
};
use url::Url;
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
        
        let nanos_per_second = 1000000000i64;

        let points = points!(
            Point::new("speedtest")
                .add_timestamp(test.timestamp.timestamp() * nanos_per_second)
                .add_tag("agent_name", agent_name)
                .add_tag("test_type", test.test_type)
                .add_field("ping_jitter", test.ping.jitter)
                .add_field("ping_latency", test.ping.latency)
                .add_field("download_bandwidth", test.download.bandwidth as i64)
                .add_field("upload_bandwidth", test.upload.bandwidth as i64)
        );
                                                             
        client.write_points(points, Some(Precision::Nanoseconds), None).await?;

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
    let password = dotenv::var("INFLUXDB_PASSWORD").unwrap();
    let username = "snitch-agent";
    let database = "snitch-agent";
    let client = Client::new(Url::parse(&host)?, database).set_authentication(username, &password);
     
    task::spawn(async move {
        periodically_run_speedtest(&cli.speedtest_binary_path, &cli.agent_name, &client).await?;

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    }).await?
}