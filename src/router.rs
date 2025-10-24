use crate::handlers::{dev_ui_services, postgres_tables, spa::serve_spa, sql_connections};
use crate::services::sql::connection_manager::{ConnectionManager, ConnectionManagerError};
use crate::services::sql::Config;
use crate::state::{DevUIState, SqlState};
use axum::{routing::get, Router};
use std::sync::Arc;
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
            let connection_manager = ConnectionManager::new(sql_config.clone())
                .await
                .map_err(DevUIError::SqlConnectionManagerError)?;
            Ok(Router::new()
                .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
                .route("/api/services", get(dev_ui_services))
                .route("/api/services/sql/connections", get(sql_connections))
                .route(
                    "/api/services/sql/postgres/{connection_name}/tables",
                    get(postgres_tables),
                )
                .with_state(Arc::new(DevUIState {
                    sql_state: Some(SqlState {
                        config: sql_config,
                        connection_manager,
                    }),
                }))
                .route("/{*path}", get(serve_spa)))
        }
        None => Ok(Router::new()
            .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
            .route("/*path", get(serve_spa))),
    }
}
