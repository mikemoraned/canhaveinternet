use async_std::io;
use async_std::task;
use prometheus::{histogram_opts, register_histogram, Encoder, Histogram, Registry, TextEncoder};
use std::time::{Duration, SystemTime};
use surf;
use tide;

async fn check(request_counter: Histogram) -> Result<(), surf::Exception> {
    let url = "https://www.ibm.com/uk-en";
    let start = SystemTime::now();
    let response = surf::get(url).await?;
    let elapsed = start.elapsed()?;
    println!(
        "status = {:?}, start = {:?}, elapsed = {:?}",
        response.status(),
        start.duration_since(SystemTime::UNIX_EPOCH)?,
        elapsed
    );

    request_counter.observe(elapsed.as_millis() as f64);

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
    let opts = histogram_opts!(
        "canhaveinternet_request_histogram",
        "histogram of requests sent",
        buckets
    );
    let request_histogram = register_histogram!(opts).unwrap();
    registry
        .register(Box::new(request_histogram.clone()))
        .unwrap();
    task::spawn(async move {
        let delay = Duration::from_secs(30);
        loop {
            task::spawn(check(request_histogram.clone()));
            task::sleep(delay).await;
        }
    });
}

fn main() -> io::Result<()> {
    let registry = Registry::new();

    spawn_check(&registry);

    let mut app = tide::App::with_state(AppState { registry });
    app.at("/metrics").get(dump);
    app.serve("127.0.0.1:8000")
}
