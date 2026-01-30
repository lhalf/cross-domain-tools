use crate::system::SENDER_ADDRESS;
use crate::system::System;
use axum::http::Method;
use parameterized::parameterized;
use reqwest::StatusCode;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(1);

#[parameterized(method = {
    Method::GET,
    Method::POST,
    Method::PUT,
    Method::DELETE,
    Method::OPTIONS,
    Method::PATCH,
    Method::TRACE,
})]
fn proxy_method(method: Method) {
    let system = System::start();

    let response = client()
        .request(method.clone(), format!("http://{SENDER_ADDRESS}/"))
        .send()
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());

    assert_eq!(1, system.server.received_requests().len());
    assert_eq!(method, system.server.received_requests()[0].method());
}

fn client() -> reqwest::blocking::Client {
    reqwest::blocking::ClientBuilder::new()
        .timeout(TIMEOUT)
        .build()
        .unwrap()
}
