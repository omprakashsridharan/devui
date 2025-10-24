use std::sync::Arc;
use axum::extract::State;
use axum::response::Json;
use serde::Serialize;
use crate::services::sql::Config;

#[derive(Serialize)]
pub enum DatabaseType {
    POSTGRES
}

#[derive(Serialize)]
pub struct ConnectionResponseItem {
    name: String,
    database_type: DatabaseType,
}

pub async fn connections(
    State(sql_config): State<Arc<Config>>
) -> Json<Vec<ConnectionResponseItem>> {
    let mut connections: Vec<ConnectionResponseItem> = Vec::new();
    for (connection_name,_) in Arc::clone(&sql_config).postgres.clone() {
        connections.push(ConnectionResponseItem {
            name: connection_name,
            database_type: DatabaseType::POSTGRES
        })
    }
    Json(connections)
}
