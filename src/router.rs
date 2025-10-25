use crate::handlers::{dev_ui_services, spa::serve_spa, sql_connections, tables};
use crate::services::sql::config::Config;
use crate::services::sql::service::{Service as SqlService, SqlServiceError};
use crate::state::DevUIState;
use axum::{routing::get, Router};
use std::sync::Arc;
use thiserror::Error;
use tower_http::services::ServeDir;
use crate::DevUIConfig;

#[derive(Error, Debug)]
pub enum DevUIError {
    #[error("connection manager error")]
    SqlServiceError(#[from] SqlServiceError),
}

pub async fn dev_ui_router(dev_ui_config: DevUIConfig) -> Result<Router, DevUIError> {
    let sql_config = dev_ui_config.sql_config;
    let sql_service = SqlService::new(sql_config.clone())
        .await
        .map_err(DevUIError::SqlServiceError)?;
    Ok(Router::new()
        .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
        .route("/api/services", get(dev_ui_services))
        .route("/api/services/sql/connections", get(sql_connections))
        .route(
            "/api/services/sql/connections/{connection_name}/tables",
            get(tables),
        )
        .with_state(Arc::new(DevUIState { sql_service }))
        .route("/{*path}", get(serve_spa)))
}
