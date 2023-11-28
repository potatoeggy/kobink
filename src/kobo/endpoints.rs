use std::str::FromStr;

use super::models::{self, AuthDeviceRequest, AuthDeviceResponse};

use axum::{
    http::{request, HeaderMap, StatusCode},
    response::Response,
    routing::{get, post},
    Extension, Json, Router,
};
use reqwest::header::HOST;

pub fn create_kobo_router() -> Router {
    Router::new()
        .route("/auth/device", post(handle_auth_device))
        .route("/user/profile", get(stub))
        .route("/user/loyalty/benefits", get(stub))
        .route("/deals", get(stub))
        .route("/analytics/gettests", get(stub))
        .route("/affiliate", get(stub))
        .route("/initialization", get(handle_initialization))
}

async fn handle_auth_device(
    Json(_): Json<AuthDeviceRequest>,
) -> (StatusCode, Json<AuthDeviceResponse>) {
    // basically a stub
    (
        StatusCode::OK,
        Json(AuthDeviceResponse::from_user_key("SAMPLE_USER_KEY")),
    )
}

async fn handle_initialization<'a>(headers: HeaderMap) -> (StatusCode, String) {
    // proxy this request to the actual Kobo store, it's a bunch of constants
    let client = reqwest::Client::new();
    let mut new_headers = reqwest::header::HeaderMap::new();
    for (key, value) in headers.iter() {
        let Ok(name) = reqwest::header::HeaderName::from_str(&*key.to_string()) else {
            continue;
        };
        new_headers.insert(name, value.to_str().unwrap_or("").parse().unwrap());
    }
    new_headers.remove(HOST);
    println!("{:?}", headers);

    // TODO: test if we need any other header other than auth
    let res = client
        .get("https://storeapi.kobo.com/v1/initialization")
        .headers(new_headers)
        .send()
        .await
        .expect("failed to send request to kobo");
    let body = res.text().await;
    println!("{:?}", body);

    (StatusCode::OK, body.unwrap_or("ERROR".to_string()))
}

async fn stub() -> (StatusCode, &'static str) {
    (StatusCode::OK, "{}")
}
