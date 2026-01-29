use crate::system::SENDER_ADDRESS;
use crate::system::System;
use reqwest::StatusCode;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(1);

#[test]
fn get_method() {
    let _system = System::start();

    let response = client()
        .get(format!("http://{SENDER_ADDRESS}/"))
        .send()
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
}

fn client() -> reqwest::blocking::Client {
    reqwest::blocking::ClientBuilder::new()
        .timeout(TIMEOUT)
        .build()
        .unwrap()
}
