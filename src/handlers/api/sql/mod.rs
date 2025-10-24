pub mod postgres;

use crate::extractors::sql::SqlService;
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

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TableInfo {
    pub name: String,
    pub schema: String,
    pub columns: Vec<ColumnInfo>,
}

/// Database column information
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub default_value: Option<String>,
}

pub async fn connections(State(sql_service_extractor): State<SqlService>) -> Result<Json<Vec<ConnectionResponseItem>>, StatusCode> {
    let mut connections: Vec<ConnectionResponseItem> = Vec::new();
    if let Some(sql_state) = sql_service_extractor.sql_state {
        for (connection_name, _) in sql_state.config.postgres.clone() {
            connections.push(ConnectionResponseItem {
                name: connection_name,
                database_type: DatabaseType::POSTGRES,
            })
        }
        Ok(Json(connections))
    } else {
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
