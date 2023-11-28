use http_body_util::StreamBody;
use tokio;
use tokio_util::io::ReaderStream;

use axum::{
    extract::Path,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

use crate::server::AppError;

use super::{LIBRARY, LIBRARY_PATH};

pub fn create_download_router() -> Router {
    Router::new().route("/download/:book_id/:book_format", get(handle_file_download))
}

async fn handle_file_download(
    Path(book_id): Path<String>,
    Path(book_format): Path<String>,
) -> impl IntoResponse {
    let library = *LIBRARY.lock().unwrap();

    // unsafe, figure out how to do this properly
    let book = library.find_book_by_id(&book_id).unwrap();
    let book = book.clone();
    // unlock mutex
    drop(library);
    let file = tokio::fs::File::open(book.path).await?;
    let stream = ReaderStream::new(file);
    let body = StreamBody::new(stream);

    let headers = [
        ("Content-Type", "application/epub+zip"),
        (
            "Content-Disposition",
            &format!("attachment; filename=\"{}.epub\"", book.title),
        ),
    ];

    Ok((headers, body).into_response())
}
