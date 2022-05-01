mod speedtest;

fn main() {
    let test: speedtest::Speedtest = speedtest::run_speedtest().unwrap();
    println!("Test type {}, for timestamp: {}, {:?}", test.test_type, test.timestamp, test.ping);
}
