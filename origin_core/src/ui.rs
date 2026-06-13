use rust_embed::RustEmbed;
use axum::{
    response::IntoResponse,
    http::{StatusCode, header, Uri},
};

#[derive(RustEmbed)]
#[folder = "../origin_ui/dist"]
pub struct WebAssets;

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/');
    if path.is_empty() {
        path = "index.html";
    }
    
    match WebAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            (
                [(header::CONTENT_TYPE, mime.as_ref())],
                content.data,
            ).into_response()
        }
        None => {
            if let Some(index) = WebAssets::get("index.html") {
                (
                    [(header::CONTENT_TYPE, "text/html")],
                    index.data,
                ).into_response()
            } else {
                (StatusCode::NOT_FOUND, "404 Not Found").into_response()
            }
        }
    }
}
