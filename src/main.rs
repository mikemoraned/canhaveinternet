use async_std::future;
use async_std::io;
use async_std::task;
use prometheus::{histogram_opts, register_histogram, Encoder, Histogram, Registry, TextEncoder};
use std::time::{Duration, SystemTime};
use surf;
use tide;

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn check(
    successful_request_histogram: Histogram,
    failed_request_histogram: Histogram,
) -> Result<(), GenericError> {
    let url = "https://www.ibm.com/uk-en";
    let timeout = Duration::from_millis(1000);

    let start = SystemTime::now();
    let response = future::timeout(timeout, surf::get(url)).await;
    let elapsed = start.elapsed()?;
    match response {
        Result::Ok(surf_response) => match surf_response {
            Result::Ok(response) => {
                println!(
                    "status = {:?}, start = {:?}, elapsed = {:?}",
                    response.status(),
                    start.duration_since(SystemTime::UNIX_EPOCH)?,
                    elapsed
                );
                successful_request_histogram.observe(elapsed.as_millis() as f64);
            }
            Result::Err(error) => {
                println!(
                    "error = {:?}, start = {:?}, elapsed = {:?}",
                    error,
                    start.duration_since(SystemTime::UNIX_EPOCH)?,
                    elapsed
                );
                failed_request_histogram.observe(elapsed.as_millis() as f64);
            }
        },
        Result::Err(error) => {
            println!(
                "error = {:?}, start = {:?}, elapsed = {:?}",
                error,
                start.duration_since(SystemTime::UNIX_EPOCH)?,
                elapsed
            );
            failed_request_histogram.observe(elapsed.as_millis() as f64);
        }
    }

    Ok(())
}

struct AppState {
    registry: Registry,
}

async fn dump(cx: tide::Context<AppState>) -> tide::EndpointResult<String> {
    println!("dump called");
    let registry = &cx.state().registry;

    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = registry.gather();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    Ok(String::from_utf8(buffer).unwrap())
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

fn main() -> io::Result<()> {
    let registry = Registry::new();

    spawn_check(&registry);

    let mut app = tide::App::with_state(AppState { registry });
    app.at("/metrics").get(dump);
    app.serve("0.0.0.0:8000")
}
