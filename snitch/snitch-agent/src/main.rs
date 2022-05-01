use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use prometheus_client::encoding::text::Encode;
use prometheus_client::encoding::text::encode;
use prometheus_client::metrics::counter::{Atomic, Counter};
use prometheus_client::metrics::histogram::{Histogram, linear_buckets};
use prometheus_client::metrics::family::Family;
use prometheus_client::registry;
use prometheus_client::registry::Registry;
use std::sync::Mutex;
use std::io::Write;

mod speedtest;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

struct AppState {
    registry: Mutex<Registry>,
}

#[get("/metrics")]
async fn metrics(data: web::Data<AppState>) -> impl Responder {
    let registry = &*(data.registry.lock().unwrap()); 
    
    let mut buffer = vec![];
    encode(&mut buffer, registry).unwrap();

    let body = String::from_utf8(buffer).unwrap();

    HttpResponse::Ok().body(body)
}

use std::time::Duration;
use async_std::task;
async fn periodically_run_speedtest(ping_jitter: Box<Histogram>) {
    loop {
        println!("Running test");
        let test: speedtest::Speedtest = speedtest::run_speedtest().unwrap();
        println!("Test type {}, for timestamp: {}, {:?}", test.test_type, test.timestamp, test.ping);
        ping_jitter.observe(test.ping.jitter);
        task::sleep(Duration::from_secs(60)).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut registry = <Registry>::default();
    let ping_jitter = Histogram::new(linear_buckets(0.0, 1.0, 100));
    registry.register("ping_jitter", "Ping Jitter", Box::new(ping_jitter.clone()));


    let data = web::Data::new(AppState {
        registry: Mutex::new(registry)
    });

    actix_rt::spawn(periodically_run_speedtest(Box::new(ping_jitter.clone())));

    HttpServer::new(move|| {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(metrics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
