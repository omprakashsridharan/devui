use crate::handlers::{connections, spa::serve_spa};
use crate::services::sql::connection_manager::{ConnectionManager, ConnectionManagerError};
use crate::services::sql::Config;
use axum::extract::FromRef;
use axum::{routing::get, Router};
use std::sync::Arc;
use thiserror::Error;
use tower_http::services::ServeDir;

#[derive(Error, Debug)]
pub enum DevUIError {
    #[error("connection manager error")]
    SqlConnectionManagerError(#[from] ConnectionManagerError),
}

#[derive(Clone)]
pub struct SqlState {
    pub config: Config,
    pub connection_manager: ConnectionManager,
}

#[derive(Clone)]
pub struct DevUIState {
    pub sql_state: SqlState,
}

pub async fn dev_ui_router(sql_config_option: Option<Config>) -> Result<Router, DevUIError> {
    match sql_config_option {
        Some(sql_config) => {
            let connection_manager = ConnectionManager::new(sql_config.clone())
                .await
                .map_err(DevUIError::SqlConnectionManagerError)?;
            Ok(Router::new()
                .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
                .route("/api/sql/connections", get(connections))
                .with_state(Arc::new(DevUIState {
                    sql_state: SqlState {
                        config: sql_config,
                        connection_manager,
                    },
                }))
                .route("/{*path}", get(serve_spa)))
        }
        None => Ok(Router::new()
            .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
            .route("/*path", get(serve_spa))),
    }
}

// support converting an `AppState` in an `ApiState`
impl FromRef<Arc<DevUIState>> for SqlState {
    fn from_ref(dev_ui_state: &Arc<DevUIState>) -> SqlState {
        dev_ui_state.sql_state.clone()
    }
}
