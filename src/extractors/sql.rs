use crate::state::{DevUIState, SqlServiceState};
use axum::extract::{FromRef, FromRequestParts, Path};
use std::sync::Arc;
use http::request::Parts;
use http::StatusCode;
use crate::services::sql::connection_manager::ConnectionPool;

pub struct SqlPool {
    pub connection_name: String,
    pub pool: ConnectionPool,
}

impl FromRequestParts<Arc<DevUIState>> for SqlPool{
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

        if let Some(sql_state) = state.sql_state.as_ref() {
            // Get the pool from the connection manager
            let pool = sql_state
                .connection_manager
                .get_connections(connection_name.clone())
                .await
                .map_err(|_| {
                    (
                        StatusCode::NOT_FOUND,
                        format!("Connection '{}' not found", connection_name),
                    )
                })?;

            Ok(SqlPool {
                connection_name,
                pool,
            })
        } else {
            Err((StatusCode::NOT_FOUND, "Sql config not found".to_string()))
        }
    }
}
