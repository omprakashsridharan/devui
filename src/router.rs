use crate::handlers::{dev_ui_services, spa::serve_spa};
use crate::services::kafka::router::router as kafka_router;
use crate::services::kafka::service::{Service as KafkaService, ServiceError as KafkaServiceError};
use crate::services::sql::router::router as sql_router;
use crate::services::sql::service::{Service as SqlService, SqlServiceError};
use crate::DevUIConfig;
use axum::{routing::get, Router};
use thiserror::Error;
use tower_http::services::ServeDir;

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
    Ok(Router::new()
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .route("/api/services", get(dev_ui_services))
        .nest("/api/services/sql", sql_router(sql_service))
        .nest("/api/services/kafka", kafka_router(kafka_service))
        .route("/{*path}", get(serve_spa)))
}
