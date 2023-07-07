use std::fmt::Display;

use axum::body::Bytes;
use axum::http::{Request, Response, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use hyper::body::HttpBody;
use hyper::Body;
use tracing::info;

const IGNORED_EXTENSIONS: [&str; 5] = [".js", ".html", ".css", ".png", ".jpeg"];
const IGNORED_PATHS: [&str; 1] = ["/example/path"];

pub async fn log_request_response(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut do_log = true;

    let path = &req.uri().path().to_string();

    for ext in IGNORED_EXTENSIONS {
        if path.ends_with(ext) {
            do_log = false;
            break;
        }
    }

    for skip_path in IGNORED_PATHS {
        if path.ends_with(skip_path) {
            do_log = false;
            break;
        }
    }

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
