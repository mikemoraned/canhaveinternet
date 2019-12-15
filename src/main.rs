use async_std::future;
use async_std::task;
use prometheus::{histogram_opts, register_histogram, Encoder, Histogram, Registry, TextEncoder};
use std::time::{Duration, SystemTime};
use surf;
use tide;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

fn handle_success(
    response: surf::Response,
    start: SystemTime,
    elapsed: Duration,
    successful_request_histogram: Histogram,
) -> Result<(), GenericError> {
    println!(
        "status = {:?}, start = {:?}, elapsed = {:?}",
        response.status(),
        start.duration_since(SystemTime::UNIX_EPOCH)?,
        elapsed
    );
    successful_request_histogram.observe(elapsed.as_millis() as f64);

    Ok(())
}

fn handle_error(
    error: GenericError,
    start: SystemTime,
    elapsed: Duration,
    failed_request_histogram: Histogram,
) -> Result<(), GenericError> {
    println!(
        "error = {:?}, start = {:?}, elapsed = {:?}",
        error,
        start.duration_since(SystemTime::UNIX_EPOCH)?,
        elapsed
    );
    failed_request_histogram.observe(elapsed.as_millis() as f64);

    Ok(())
}

async fn check(
    successful_request_histogram: Histogram,
    failed_request_histogram: Histogram,
) -> Result<(), GenericError> {
    let url = "https://canhaveinternet.houseofmoran.io/";
    let timeout = Duration::from_millis(5000);

    let start = SystemTime::now();
    let timed_result = future::timeout(timeout, surf::get(url)).await;
    let elapsed = start.elapsed().unwrap();
    match timed_result {
        Result::Ok(surf_result) => match surf_result {
            Result::Ok(response) => {
                handle_success(response, start, elapsed, successful_request_histogram)?;
            }
            Result::Err(e) => {
                handle_error(e, start, elapsed, failed_request_histogram)?;
            }
        },
        Result::Err(e) => {
            handle_error(Box::new(e), start, elapsed, failed_request_histogram)?;
        }
    }

    Ok(())
}

struct AppState {
    registry: Registry,
}

async fn dump(req: tide::Request<AppState>) -> String {
    println!("dump called");
    let registry = &req.state().registry;

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    String::from_utf8(buffer).unwrap()
}

fn spawn_check(registry: &Registry) {
    let buckets: Vec<f64> = (0..10).map(|i| (i * 100) as f64).collect();
    let successful_request_histogram = register_histogram!(histogram_opts!(
        "canhaveinternet_request_succeeded_histogram",
        "histogram of successful requests sent",
        buckets.clone()
    ))
    .unwrap();
    let failed_request_histogram = register_histogram!(histogram_opts!(
        "canhaveinternet_request_failed_histogram",
        "histogram of failed requests sent",
        buckets.clone()
    ))
    .unwrap();
    registry
        .register(Box::new(successful_request_histogram.clone()))
        .unwrap();
    registry
        .register(Box::new(failed_request_histogram.clone()))
        .unwrap();
    task::spawn(async move {
        let delay = Duration::from_secs(30);
        loop {
            task::spawn(check(
                successful_request_histogram.clone(),
                failed_request_histogram.clone(),
            ));
            task::sleep(delay).await;
        }
    });
}

async fn healthcheck(_: tide::Request<AppState>) -> String {
    "hunky dory".into()
}

fn main() -> Result<(), std::io::Error> {
    let registry = Registry::new();

    spawn_check(&registry);

    task::block_on(async {
        let mut app = tide::Server::with_state(AppState { registry });
        app.at("/metrics").get(dump);
        app.at("/healthcheck/alive").get(healthcheck);
        app.at("/healthcheck/ready").get(healthcheck);
        app.listen("0.0.0.0:8000").await.unwrap()
    });

    Ok(())
}
