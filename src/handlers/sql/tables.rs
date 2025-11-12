use crate::services::sql::models::TablesBySchema;
use crate::services::sql::router::SqlServiceState;
use axum::extract::{Path, State};
use axum::Json;
use http::StatusCode;

pub async fn tables(
    State(SqlServiceState(sql_service)): State<SqlServiceState>,
    Path(connection_name): Path<String>,
) -> Result<Json<TablesBySchema>, StatusCode> {
    match sql_service.tables(connection_name).await {
        Ok(tables) => Ok(Json(tables)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
