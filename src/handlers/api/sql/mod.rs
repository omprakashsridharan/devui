pub mod postgres;

use axum::extract::State;
use axum::response::Json;
use serde::Serialize;
use crate::state::SqlState;

#[derive(Serialize)]
pub enum DatabaseType {
    POSTGRES,
}

#[derive(Serialize)]
pub struct ConnectionResponseItem {
    name: String,
    database_type: DatabaseType,
}

pub async fn connections(State(sql_state): State<SqlState>) -> Json<Vec<ConnectionResponseItem>> {
    let mut connections: Vec<ConnectionResponseItem> = Vec::new();
    for (connection_name, _) in sql_state.config.postgres.clone() {
        connections.push(ConnectionResponseItem {
            name: connection_name,
            database_type: DatabaseType::POSTGRES,
        })
    }
    Json(connections)
}