use crate::services::sql::models::TableData;
use crate::services::sql::router::SqlServiceState;
use axum::extract::{Path, Query, State};
use axum::Json;
use http::StatusCode;
use std::collections::HashMap;

pub async fn table_data(
    State(SqlServiceState(sql_service)): State<SqlServiceState>,
    Path((connection_name, table_name)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<TableData>, StatusCode> {
    // Convert query parameters to filters HashMap
    let filters = if params.is_empty() {
        None
    } else {
        Some(params)
    };

    match sql_service
        .table_data(connection_name, table_name, filters)
        .await
    {
        Ok(table_data) => Ok(Json(table_data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
