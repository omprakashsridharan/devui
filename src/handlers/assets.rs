use axum::{
    http::StatusCode,
    response::Response,
};
use http::HeaderMap;
use std::fs;
use std::path::Path as StdPath;

/// Handler for serving static assets
/// Serves files from the frontend/dist/assets directory
pub async fn serve_asset(
    asset_path: String,
) -> Result<Response<String>, StatusCode> {
    let full_path = StdPath::new("frontend/dist/assets").join(&asset_path);

    match fs::read_to_string(&full_path) {
        Ok(content) => {
            let content_type = if asset_path.ends_with(".js") {
                "application/javascript"
            } else if asset_path.ends_with(".css") {
                "text/css"
            } else {
                "application/octet-stream"
            };

            let mut headers = HeaderMap::new();
            headers.insert("content-type", content_type.parse().unwrap());
            headers.insert("cache-control", "public, max-age=31536000".parse().unwrap());

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", content_type)
                .header("cache-control", "public, max-age=31536000")
                .body(content)
                .unwrap();

            Ok(response)
        }
        Err(_) => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Asset not found".to_string())
                .unwrap();
            Ok(response)
        }
    }
}
