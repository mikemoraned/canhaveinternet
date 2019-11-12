use reqwest;

fn main() -> Result<(), reqwest::Error> {
    let status = reqwest::get("https://www.ibm.com/uk-en")?.status();

    println!("status = {:?}", status);

    Ok(())
}
