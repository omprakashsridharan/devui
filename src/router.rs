use std::sync::Arc;
use crate::handlers::{connections, spa::serve_spa};
use crate::services::sql::connection_manager::{ConnectionManager, ConnectionManagerError};
use crate::services::sql::Config;
use axum::{routing::get, Router};
use thiserror::Error;
use tower_http::services::ServeDir;

#[derive(Error, Debug)]
pub enum DevUIError {
    #[error("connection manager error")]
    SqlConnectionManagerError(#[from] ConnectionManagerError),
}

pub async fn dev_ui_router(sql_config_option: Option<Config>) -> Result<Router, DevUIError> {
    match sql_config_option {
        Some(sql_config) => {
            let _connection_manager = ConnectionManager::new(sql_config.clone())
                .await
                .map_err(DevUIError::SqlConnectionManagerError)?;
            Ok(Router::new()
                .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
                .route("/{*path}/", get(serve_spa))
                .route("/api/sql/connections", get(connections))
                .with_state(Arc::new(sql_config)))
                // .with_state(Arc::new(connection_manager)))
        }
        None => Ok(Router::new()
            .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
            .route("/*path", get(serve_spa))),
    }
}
