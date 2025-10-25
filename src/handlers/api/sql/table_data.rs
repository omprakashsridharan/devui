use crate::services::sql::models::TableRow;
use crate::services::sql::router::SqlServiceState;
use axum::extract::{Path, State};
use axum::Json;
use http::StatusCode;

pub async fn table_data(
    State(SqlServiceState(sql_service)): State<SqlServiceState>,
    Path((connection_name, table_name)): Path<(String, String)>,
) -> Result<Json<Vec<TableRow>>, StatusCode> {
    match sql_service.table_data(connection_name, table_name).await {
        Ok(table_data) => Ok(Json(table_data)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
