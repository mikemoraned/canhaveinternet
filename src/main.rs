use async_std::io;
use async_std::task;
use std::time::{Duration, SystemTime};
use surf;

async fn check() -> Result<(), surf::Exception> {
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

    Ok(())
}

fn main() -> io::Result<()> {
    let delay = Duration::from_secs(30);

    task::block_on(async {
        loop {
            task::spawn(check());
            task::sleep(delay).await;
        }
    })
}
