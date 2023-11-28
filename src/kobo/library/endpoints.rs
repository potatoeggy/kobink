use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post, put},
    Json, Router,
};

use crate::{kobo::models::SyncTokenModel, library::models::LibraryState, server::AppError};

use super::models::{Entitlement, SyncEvent, SyncResponse};

pub fn create_library_router() -> Router<Arc<Mutex<LibraryState>>> {
    Router::new()
        .route("/:book_id/state", put(stub))
        .route("/tags/:book_or_tag_id_idk/items", post(stub))
        .route("/tags/:book_or_tag_id_idk/items/delete", post(stub))
        .route("/sync", get(handle_sync))
}

async fn stub() -> StatusCode {
    StatusCode::OK
}

async fn handle_sync(
    State(library): State<Arc<Mutex<LibraryState>>>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<SyncResponse>), AppError> {
    // let sync_token = SyncTokenModel::from_headers(&headers)?;

    let res = library
        .lock()
        .unwrap()
        .books
        .iter()
        .map(SyncEvent::new)
        .collect();
    Ok((StatusCode::OK, Json(res)))
}
