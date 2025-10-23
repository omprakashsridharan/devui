use axum::{
    http::StatusCode,
    response::Response,
};

/// Handler for serving the React SPA
/// Returns the main HTML file for all non-API routes
pub async fn serve_spa() -> Response<String> {
    let html = include_str!("../../frontend/dist/index.html");

    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html; charset=utf-8")
        .body(html.to_string())
        .unwrap()
}
