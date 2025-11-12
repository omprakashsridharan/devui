use crate::assets::FrontendAssets;
use crate::handlers::dev_ui_services;
use crate::services::kafka::router::router as kafka_router;
use crate::services::kafka::service::{Service as KafkaService, ServiceError as KafkaServiceError};
use crate::services::sql::router::router as sql_router;
use crate::services::sql::service::{Service as SqlService, SqlServiceError};
use crate::DevUIConfig;
use axum::{
    body::Body,
    http::{header, StatusCode, Uri},
    response::Response,
    routing::get,
    Router,
};
use bytes::Bytes;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DevUIError {
    #[error("connection manager error")]
    SqlServiceError(#[from] SqlServiceError),
    #[error("kafka service error")]
    KafkaServiceError(#[from] KafkaServiceError),
}

pub async fn dev_ui_router(dev_ui_config: DevUIConfig) -> Result<Router, DevUIError> {
    let mut api_router = Router::new()
        .route("/services", get(dev_ui_services))
        .with_state(dev_ui_config.clone());

    if let Some(sql_config) = dev_ui_config.sql_config {
        let sql_service = SqlService::new(sql_config.clone())
            .await
            .map_err(DevUIError::SqlServiceError)?;
        api_router = api_router.nest("/services/sql", sql_router(sql_service));
    }

    if let Some(kafka_config) = dev_ui_config.kafka_config {
        let kafka_service =
            KafkaService::new(kafka_config.clone()).map_err(DevUIError::KafkaServiceError)?;
        api_router = api_router.nest("/services/kafka", kafka_router(kafka_service));
    }

    Ok(Router::new()
        .nest("/api", api_router)
        .fallback(serve_embedded_assets))
}

async fn serve_embedded_assets(uri: Uri) -> Response<Body> {
    let path = uri.path().trim_start_matches('/');

    // If it's an empty path or ends with /, try index.html
    let path = if path.is_empty() || path.ends_with('/') {
        "index.html"
    } else {
        path
    };

    // Try to get the embedded file
    match FrontendAssets::get(path) {
        Some(content) => {
            let body = Bytes::copy_from_slice(content.data.as_ref());
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, get_content_type(path))
                .body(Body::from(body))
                .unwrap_or_else(|_| {
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Internal Server Error"))
                        .unwrap()
                })
        }
        None => {
            // File not found, try index.html for SPA routing
            match FrontendAssets::get("index.html") {
                Some(content) => {
                    let body = Bytes::copy_from_slice(content.data.as_ref());
                    Response::builder()
                        .status(StatusCode::OK)
                        .header(header::CONTENT_TYPE, "text/html")
                        .body(Body::from(body))
                        .unwrap_or_else(|_| {
                            Response::builder()
                                .status(StatusCode::INTERNAL_SERVER_ERROR)
                                .body(Body::from("Internal Server Error"))
                                .unwrap()
                        })
                }
                None => Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::from("Not Found"))
                    .unwrap(),
            }
        }
    }
}

fn get_content_type(path: &str) -> &'static str {
    match path {
        p if p.ends_with(".html") => "text/html",
        p if p.ends_with(".js") => "application/javascript",
        p if p.ends_with(".css") => "text/css",
        p if p.ends_with(".json") => "application/json",
        p if p.ends_with(".png") => "image/png",
        p if p.ends_with(".jpg") | p.ends_with(".jpeg") => "image/jpeg",
        p if p.ends_with(".svg") => "image/svg+xml",
        p if p.ends_with(".ico") => "image/x-icon",
        p if p.ends_with(".woff") => "font/woff",
        p if p.ends_with(".woff2") => "font/woff2",
        p if p.ends_with(".ttf") => "font/ttf",
        _ => "application/octet-stream",
    }
}
