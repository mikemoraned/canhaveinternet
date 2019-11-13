use async_std::io;
use async_std::prelude::*;
use async_std::task;
use std::time;
use surf;

async fn check() -> Result<(), surf::Exception> {
    let url = "https://www.ibm.com/uk-en";
    let response = surf::get(url).await?;
    println!("status = {:?}", response.status());

    Ok(())
}

fn main() -> io::Result<()> {
    let delay = time::Duration::from_secs(30);

    task::block_on(async {
        loop {
            task::block_on(check());
            task::sleep(delay).await;
        }
    })
}
