use axum::{
    http::StatusCode,
    middleware,
    routing::{get, post},
    Json, Router,
};

mod errors;
pub use errors::AppError;
mod logging;
use crate::{
    kobo::{self, create_kobo_router},
    library::create_download_router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::info;

pub async fn start_server() {
    let mut app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        .nest("/kobo/:key/v1", create_kobo_router())
        .nest("/download", create_download_router())
        .layer(middleware::from_fn(logging::log_request_response));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr.to_string())
        .await
        .unwrap();
    info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
