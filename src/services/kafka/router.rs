use crate::handlers::kafka::clusters::clusters;
use crate::handlers::kafka::metadata::metadata;
use crate::services::kafka::service::Service;
use axum::routing::get;
use axum::Router;

#[derive(Clone)]
pub struct KafkaServiceState(pub(crate) Service);

pub fn router(service: Service) -> Router {
    Router::new()
        .route("/clusters", get(clusters))
        .route("/clusters/{cluster_name}/metadata", get(metadata))
        .with_state(KafkaServiceState(service))
}
