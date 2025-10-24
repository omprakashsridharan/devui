use crate::handlers::{sql_connections, postgres_tables, spa::serve_spa};
use crate::services::sql::connection_manager::{ConnectionManager, ConnectionManagerError};
use crate::services::sql::Config;
use axum::extract::{FromRef, FromRequestParts, Path};
use axum::{routing::get, Router};
use std::sync::Arc;
use http::request::Parts;
use http::StatusCode;
use sqlx::{Pool, Postgres};
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
                .route("/api/sql/connections", get(sql_connections))
                .route("/api/sql/postgres/{connection_name}", get(postgres_tables))
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

pub struct PostgresPool {
    pub connection_name: String,
    pub pool: Pool<Postgres>,
}

impl FromRequestParts<Arc<DevUIState>> for PostgresPool {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &Arc<DevUIState>,
    ) -> Result<Self, Self::Rejection> {
        // Extract the connection_name from the path
        let path = Path::<String>::from_request_parts(parts, state)
            .await
            .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid path".to_string()))?;

        let connection_name = path.to_string();

        // Get the pool from the connection manager
        let pool = state
            .sql_state
            .connection_manager
            .get_postgres_connections(connection_name.clone())
            .await
            .map_err(|_| {
                (
                    StatusCode::NOT_FOUND,
                    format!("Connection '{}' not found", connection_name),
                )
            })?;

        Ok(PostgresPool {
            connection_name,
            pool,
        })
    }
}