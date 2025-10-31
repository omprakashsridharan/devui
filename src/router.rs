use crate::handlers::dev_ui_services;
use crate::services::kafka::router::router as kafka_router;
use crate::services::kafka::service::{Service as KafkaService, ServiceError as KafkaServiceError};
use crate::services::sql::router::router as sql_router;
use crate::services::sql::service::{Service as SqlService, SqlServiceError};
use crate::DevUIConfig;
use axum::{routing::get, Router};
use thiserror::Error;
use tower_http::services::{ServeDir, ServeFile};

#[derive(Error, Debug)]
pub enum DevUIError {
    #[error("connection manager error")]
    SqlServiceError(#[from] SqlServiceError),
    #[error("kafka service error")]
    KafkaServiceError(#[from] KafkaServiceError),
}

pub async fn dev_ui_router(dev_ui_config: DevUIConfig) -> Result<Router, DevUIError> {
    let sql_service = SqlService::new(dev_ui_config.sql_config.clone())
        .await
        .map_err(DevUIError::SqlServiceError)?;
    let kafka_service = KafkaService::new(dev_ui_config.kafka_config.clone())
        .map_err(DevUIError::KafkaServiceError)?;
    kafka_service.metadata("local".to_string()).unwrap();
    let api_router = Router::new()
        .route("/services", get(dev_ui_services))
        .nest("/services/sql", sql_router(sql_service))
        .nest("/services/kafka", kafka_router(kafka_service));
    Ok(Router::new().nest("/api", api_router).fallback_service(
        ServeDir::new("frontend/dist")
            .not_found_service(ServeFile::new("frontend/dist/index.html")),
    ))
}
