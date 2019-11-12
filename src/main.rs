use reqwest;

fn main() -> Result<(), reqwest::Error> {
    let status = reqwest::get("https://www.rust-lang.org")?.status();

    println!("status = {:?}", status);

    Ok(())
}
