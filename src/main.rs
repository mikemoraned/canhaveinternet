use reqwest;
use std::{thread, time};

fn main() -> Result<(), reqwest::Error> {
    let delay = time::Duration::from_secs(30);

    loop {
        let status = reqwest::get("https://www.ibm.com/uk-en")?.status();
        println!("status = {:?}", status);
        thread::sleep(delay);
    }
}
