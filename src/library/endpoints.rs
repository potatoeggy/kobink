use http_body_util::{combinators::BoxBody, StreamBody};
use tokio;
use tokio_util::io::ReaderStream;

use axum::{
    body::Body,
    extract::{Path, Request},
    http::{request, HeaderMap, StatusCode, Uri},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use tower::ServiceExt;
use tower_http::services::{ServeDir, ServeFile};

use crate::server::AppError;

use super::{LIBRARY, LIBRARY_PATH};

pub fn create_download_router() -> Router {
    Router::new().route("/:book_id/:book_format", get(handle_file_download))
}

async fn handle_file_download(
    Path(book_id): Path<String>,
    Path(book_format): Path<String>,
    request: Request,
) -> Result<Response, AppError> {
    let library = LIBRARY.lock().unwrap();

    // unsafe, figure out how to do this properly
    let book = library.find_book_by_id(&book_id).unwrap();
    let book = book.clone();
    // unlock mutex
    drop(library);

    Ok(ServeFile::new(&book.path)
        .oneshot(request)
        .await
        .into_response())
}
