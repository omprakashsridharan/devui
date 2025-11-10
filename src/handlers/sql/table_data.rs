use crate::services::sql::models::TableData;
use crate::services::sql::router::SqlServiceState;
use axum::extract::{Path, Query, State};
use axum::Json;
use http::StatusCode;
use std::collections::HashMap;

pub async fn table_data(
    State(SqlServiceState(sql_service)): State<SqlServiceState>,
    Path((connection_name, schema_name, table_name)): Path<(String, String, String)>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<TableData>, StatusCode> {
    // Separate pagination parameters from filters
    let page = params.get("page").and_then(|p| p.parse::<u64>().ok());
    let page_size = params.get("page_size").and_then(|p| p.parse::<u64>().ok());

    // Filter out pagination parameters to get actual filters
    let mut filters = params.clone();
    filters.remove("page");
    filters.remove("page_size");

    let filters = if filters.is_empty() {
        None
    } else {
        Some(filters)
    };

    match sql_service
        .table_data(connection_name, schema_name, table_name, filters, page, page_size)
        .await
    {
        Ok(table_data) => Ok(Json(table_data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
