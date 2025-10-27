use axum::Json;
use http::StatusCode;

pub async fn clusters() -> Result<Json<Vec<String>>, StatusCode> {
    Ok(Json(Vec::from([])))
}
