use crate::services::kafka::router::KafkaServiceState;
use axum::extract::{Path, State};
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ProduceRequest {
    pub key: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct ProduceResponse {}

pub async fn produce(
    State(KafkaServiceState(service)): State<KafkaServiceState>,
    Path((cluster_name, topic_name)): Path<(String, String)>,
    Json(produce_request): Json<ProduceRequest>,
) -> Result<Json<ProduceResponse>, StatusCode> {
    match service
        .produce(
            cluster_name,
            topic_name,
            produce_request.key,
            produce_request.value,
        )
        .await
    {
        Ok(_) => Ok(Json(ProduceResponse {})),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
