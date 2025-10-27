use crate::services::kafka::models::ClusterMetadata;
use crate::services::kafka::router::KafkaServiceState;
use axum::extract::{Path, State};
use axum::Json;
use http::StatusCode;

pub async fn metadata(
    State(KafkaServiceState(service)): State<KafkaServiceState>,
    Path(cluster_name): Path<String>,
) -> Result<Json<ClusterMetadata>, StatusCode> {
    match service.metadata(cluster_name) {
        Ok(metadata) => Ok(Json(metadata)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
