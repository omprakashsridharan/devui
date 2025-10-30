use crate::handlers::kafka::clusters::clusters;
use crate::handlers::kafka::consume::consume;
use crate::handlers::kafka::metadata::metadata;
use crate::handlers::kafka::produce::produce;
use crate::services::kafka::service::Service;
use axum::routing::{get, post};
use axum::Router;

#[derive(Clone)]
pub struct KafkaServiceState(pub(crate) Service);

pub fn router(service: Service) -> Router {
    Router::new()
        .route("/clusters", get(clusters))
        .route("/clusters/{cluster_name}/metadata", get(metadata))
        .route(
            "/clusters/{cluster_name}/topics/{topic_name}",
            post(produce),
        )
        .route(
            "/clusters/{cluster_name}/topics/{topic_name}/consume",
            get(consume),
        )
        .with_state(KafkaServiceState(service))
}
