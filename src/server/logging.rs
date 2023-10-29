use std::fmt::Display;

use axum::{
    body::Bytes,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use hyper::{body::HttpBody, Body};
use tracing::info;

const IGNORED_EXTENSIONS: [&str; 5] = [".js", ".html", ".css", ".png", ".jpeg"];
const IGNORED_PATHS: [&str; 1] = ["/example/path"];

pub async fn log_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let path = &req.uri().path().to_string();

    // log iff path does not end with ignored extensions
    let do_log = IGNORED_EXTENSIONS
        .iter()
        .find(|&ext| path.ends_with(ext))
        .is_none()
        && IGNORED_PATHS
            .iter()
            .find(|&ext| path.ends_with(ext))
            .is_none();

    let (req_parts, req_body) = req.into_parts();

    // Print request
    let bytes = buffer_and_print(req_parts.method.as_str(), path, req_body, do_log).await?;
    let req = Request::from_parts(req_parts, hyper::Body::from(bytes));

    let res = next.run(req).await;

    let (mut res_parts, res_body) = res.into_parts();

    // Print response
    let bytes = buffer_and_print("response", path, res_body, do_log).await?;

    // When your encoding is chunked there can be problems without removing the header
    res_parts.headers.remove("transfer-encoding");
    let res = Response::from_parts(res_parts, Body::from(bytes));
    Ok(res)
}

// Consumes body and prints
async fn buffer_and_print<B>(
    direction: &str,
    path: &str,
    body: B,
    log: bool,
) -> Result<Bytes, (StatusCode, String)>
where
    B: HttpBody<Data = Bytes>,
    B::Error: Display,
{
    let bytes = match hyper::body::to_bytes(body).await {
        Ok(bytes) => bytes,
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {} body: {}", direction, err),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        if log {
            if body.len() > 2000 {
                info!("{} {}: {}...", direction, path, &body[0..2000]);
            } else if body.is_empty() {
                info!("{} {}: <empty>", direction, path);
            } else {
                info!("{} {}: {}", direction, path, body);
            }
        }
    }

    Ok(bytes)
}
