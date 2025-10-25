pub mod tables;

use crate::state::SqlServiceState;
use axum::extract::State;
use axum::response::Json;
use http::StatusCode;
use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum DatabaseType {
    POSTGRES,
}

#[derive(Clone, Serialize)]
pub struct ConnectionResponseItem {
    name: String,
    database_type: DatabaseType,
}

pub async fn connections(
    State(SqlServiceState(sql_service)): State<SqlServiceState>,
) -> Result<Json<Vec<ConnectionResponseItem>>, StatusCode> {
    let mut connections: Vec<ConnectionResponseItem> = Vec::new();
    for (connection_name, connection_pool) in sql_service.get_connections() {
        connections.push(ConnectionResponseItem {
            name: connection_name,
            database_type: connection_pool.into(),
        })
    }
    Ok(Json(connections))
}
