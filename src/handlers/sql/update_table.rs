use crate::services::sql::models::UpdateData;
use crate::services::sql::router::SqlServiceState;
use axum::extract::{Path, State};
use axum::Json;
use http::StatusCode;

pub async fn update_table(
    State(SqlServiceState(sql_service)): State<SqlServiceState>,
    Path((connection_name, schema_name, table_name)): Path<(String, String, String)>,
    Json(update_data): Json<UpdateData>,
) -> Result<Json<()>, StatusCode> {
    // Construct expected schema-qualified table name
    let expected_table_name = if schema_name == "public" {
        table_name.clone()
    } else {
        format!("{}.{}", schema_name, table_name)
    };

    // Validate that the table_name in the path matches the table_name in the body
    if update_data.table_name != expected_table_name {
        return Err(StatusCode::BAD_REQUEST);
    }

    match sql_service
        .clone()
        .update_table(connection_name, update_data)
        .await
    {
        Ok(_) => Ok(Json(())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
