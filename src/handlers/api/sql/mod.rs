pub mod postgres;

use axum::extract::State;
use axum::response::Json;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TableInfo {
    pub name: String,
    pub schema: String,
    pub columns: Vec<ColumnInfo>,
}

/// Database column information
#[derive(Debug, Clone, Serialize,  PartialEq)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub default_value: Option<String>,
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