use crate::system::SENDER_ADDRESS;
use crate::system::System;
use axum::http::Method;
use parameterized::parameterized;
use reqwest::StatusCode;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(2);

#[test]
fn proxy_response_status_code() {
    let _system = System::start();

    let response = client()
        .get(format!("http://{SENDER_ADDRESS}/teapot"))
        .send()
        .unwrap();

    assert_eq!(StatusCode::IM_A_TEAPOT, response.status());
}

#[test]
fn proxy_response_headers() {
    let _system = System::start();

    let response = client()
        .get(format!("http://{SENDER_ADDRESS}/headers"))
        .send()
        .unwrap();

    assert_eq!("headers", response.headers().get("x-target").unwrap());
}

#[parameterized(method = {
    Method::GET,
    Method::POST,
    Method::PUT,
    Method::DELETE,
    Method::OPTIONS,
    Method::PATCH,
    Method::TRACE,
})]
fn proxy_request_method(method: Method) {
    let system = System::start();

    let response = client()
        .request(method.clone(), format!("http://{SENDER_ADDRESS}/path"))
        .send()
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());

    assert_eq!(1, system.server.received_requests().len());
    assert_eq!(method, system.server.received_requests()[0].method());
}

#[test]
fn target_unavailable_returns_504() {
    let mut system = System::start();

    system.server.stop();

    let response = client()
        .get(format!("http://{SENDER_ADDRESS}/path"))
        .send()
        .unwrap();

    assert_eq!(StatusCode::GATEWAY_TIMEOUT, response.status());

    assert!(system.server.received_requests().is_empty());
}

#[test]
fn proxy_path() {
    let system = System::start();

    let response = client()
        .get(format!("http://{SENDER_ADDRESS}/another/path"))
        .send()
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());

    assert_eq!(1, system.server.received_requests().len());
    assert_eq!(
        "/another/path",
        system.server.received_requests()[0].uri().path()
    );
}

#[test]
fn proxy_headers() {
    let system = System::start();

    let response = client()
        .get(format!("http://{SENDER_ADDRESS}/path"))
        .header("x-foo", "bar")
        .send()
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());

    assert_eq!(1, system.server.received_requests().len());
    assert_eq!(
        "bar",
        system.server.received_requests()[0].headers()["x-foo"]
    );
}

#[test]
fn proxy_query() {
    let system = System::start();

    let response = client()
        .get(format!("http://{SENDER_ADDRESS}/path?foo=bar"))
        .send()
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());

    assert_eq!(1, system.server.received_requests().len());
    assert_eq!(
        Some("foo=bar"),
        system.server.received_requests()[0].uri().query()
    );
}

#[test]
fn proxy_json_body() {
    let system = System::start();

    let response = client()
        .get(format!("http://{SENDER_ADDRESS}/path"))
        .body(r#"{"foo":"bar"}"#) // test hangs when this is here?
        .send()
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());

    assert_eq!(1, system.server.received_requests().len());
    assert_eq!(
        br#"{"foo":"bar"}"#.as_slice(),
        system.server.received_requests()[0].body()
    );
}

pub fn client() -> reqwest::blocking::Client {
    reqwest::blocking::ClientBuilder::new()
        .timeout(TIMEOUT)
        .build()
        .unwrap()
}
