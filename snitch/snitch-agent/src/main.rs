use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use prometheus_client::encoding::text::Encode;
use prometheus_client::encoding::text::encode;
// use prometheus_client::metrics::counter::{Atomic, Counter};
// use prometheus_client::metrics::family::Family;
// use prometheus_client::registry;
use prometheus_client::registry::{Registry, self};
use std::sync::Mutex;
// use std::io::Write;

mod speedtest;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

struct AppState {
    registry: Mutex<Registry>,
}

// #[get("/metrics")]
// async fn run_speedtest() -> impl Responder {
//     let test: speedtest::Speedtest = speedtest::run_speedtest().unwrap();
//     HttpResponse::Ok().body(format!("Test type {}, for timestamp: {}, {:?}", test.test_type, test.timestamp, test.ping))
// }

#[get("/metrics")]
async fn metrics(data: web::Data<AppState>) -> impl Responder {
    let registry = &*(data.registry.lock().unwrap()); 
    
    let mut buffer = vec![];
    encode(&mut buffer, registry).unwrap();

    let body = String::from_utf8(buffer).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut registry = <Registry>::default();

    let data = web::Data::new(AppState {
        registry: Mutex::new(registry)
    });

    HttpServer::new(move|| {
        App::new()
            .app_data(data.clone())
            .service(hello)
            .service(metrics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    // let test: speedtest::Speedtest = speedtest::run_speedtest().unwrap();
    // println!("Test type {}, for timestamp: {}, {:?}", test.test_type, test.timestamp, test.ping);
}
