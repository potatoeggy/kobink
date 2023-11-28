use std::sync::{Arc, Mutex};

use crate::library::models::LibraryState;

use super::{
    library::create_library_router,
    models::{AuthDeviceRequest, AuthDeviceResponse},
};

use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

const SYNCTOKEN_HEADER: &'static str = "x-kobo-synctoken";
const INITIALIZATION_DATA: &'static str = include_str!("./data/initialization.json");

pub fn create_kobo_router() -> Router<Arc<Mutex<LibraryState>>> {
    Router::new()
        .route("/auth/device", post(handle_auth_device))
        .route("/user/profile", get(stub))
        .route("/user/loyalty/benefits", get(stub))
        .route("/deals", get(stub))
        .route("/products/*_", get(stub))
        .route("/analytics/gettests", post(stub))
        .route("/affiliate", get(stub))
        .route("/initialization", get(handle_initialization))
        .route("/user/wishlist", get(stub))
        .nest("/library", create_library_router())
}

async fn handle_auth_device(Json(_): Json<AuthDeviceRequest>) -> impl IntoResponse {
    // basically a stub
    let mut headers = HeaderMap::new();
    headers.insert(
        SYNCTOKEN_HEADER,
        HeaderValue::from_static("SAMPLE_SYNCTOKEN"),
    );
    (
        StatusCode::OK,
        headers,
        Json(AuthDeviceResponse::from_user_key("SAMPLE_USER_KEY")),
    )
}

async fn handle_initialization(headers: HeaderMap) -> (StatusCode, &'static str) {
    // proxy this request to the actual Kobo store, it's a bunch of constants
    // let client = reqwest::Client::new();
    // let mut new_headers = reqwest::header::HeaderMap::new();
    // for (key, value) in headers.iter() {
    //     let Ok(name) = reqwest::header::HeaderName::from_str(&*key.to_string()) else {
    //         continue;
    //     };
    //     new_headers.insert(name, value.to_str().unwrap_or("").parse().unwrap());
    // }
    // new_headers.remove(HOST);
    // new_headers.insert(
    //     SYNCTOKEN_HEADER,
    //     SyncTokenModel::default().to_string().parse().unwrap(),
    // );

    // new_headers.insert("x-kobo-apitoken", "e30=".parse().unwrap());
    // println!("{:?}", new_headers);

    // // TODO: test if we need any other header other than auth
    // // TODO: make this work - there are auth / bearer issues
    // let res = client
    //     .get("https://storeapi.kobo.com/v1/initialization")
    //     .headers(new_headers)
    //     .send()
    //     .await
    //     .expect("failed to send request to kobo");
    // let body = res.text().await;
    // println!("{:?}", body);

    (StatusCode::OK, INITIALIZATION_DATA)
}

async fn stub() -> (StatusCode, &'static str) {
    (StatusCode::OK, "{}")
}
