extern crate dotenv;
use futures::prelude::*;
use influxdb2::Client;
use chrono::{DateTime, Utc};
use influxdb2::models::DataPoint;


mod speedtest;

// use std::time::Duration;
// use async_std::task;
// async fn periodically_run_speedtest(speedtest_binary: Box<String>, ping_jitter: Box<Histogram>) {
//     loop {
//         println!("Running test");
//         let test: speedtest::Speedtest = speedtest::run_speedtest(&speedtest_binary).unwrap();
//         println!("Test type {}, for timestamp: {}, {:?}", test.test_type, test.timestamp, test.ping);
//         ping_jitter.observe(test.ping.jitter);
//         task::sleep(Duration::from_secs(60)).await;
//     }
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host= dotenv::var("INFLUXDB_HOST").unwrap();
    let org = dotenv::var("INFLUXDB_ORG").unwrap();
    let token = dotenv::var("INFLUXDB_TOKEN").unwrap();
    let bucket = "snitch-agent";
    let client = Client::new(host, org, token);
     
    let points = vec![
        DataPoint::builder("cpu")
            .tag("host", "server01")
            .field("usage", 0.5)
            .build()?,
        DataPoint::builder("cpu")
            .tag("host", "server01")
            .tag("region", "us-west")
            .field("usage", 0.87)
            .build()?,
    ];
                                                             
    client.write(bucket, stream::iter(points)).await?;

    Ok(())
}