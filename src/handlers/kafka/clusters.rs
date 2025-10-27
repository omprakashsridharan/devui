use crate::services::kafka::router::KafkaServiceState;
use axum::extract::State;
use axum::Json;
use http::StatusCode;

pub async fn clusters(
    State(KafkaServiceState(service)): State<KafkaServiceState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    Ok(Json(service.get_clusters()))
}
