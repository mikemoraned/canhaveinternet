use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

mod speedtest;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

    // let test: speedtest::Speedtest = speedtest::run_speedtest().unwrap();
    // println!("Test type {}, for timestamp: {}, {:?}", test.test_type, test.timestamp, test.ping);
}
