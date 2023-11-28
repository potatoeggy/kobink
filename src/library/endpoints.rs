use std::sync::{Arc, Mutex};

use axum::{
    debug_handler,
    extract::{Path, Request, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::services::ServeFile;

use super::{models::LibraryState, LIBRARY_PATH};

pub fn create_download_router() -> Router<Arc<Mutex<LibraryState>>> {
    Router::new().route(
        "/:book_id/:book_format/:filename",
        get(handle_file_download),
    )
}

#[debug_handler]
async fn handle_file_download(
    Path((book_id, book_format)): Path<(String, String)>,
    State(library): State<Arc<Mutex<LibraryState>>>,
    request: Request,
) -> Response {
    let book = {
        let library = library.lock().unwrap();
        let book = library.find_book_by_id(&book_id).unwrap();
        book.clone()
    };

    ServeFile::new(&book.path)
        .oneshot(request)
        .await
        .into_response()
}
