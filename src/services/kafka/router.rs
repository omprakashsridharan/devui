use crate::handlers::kafka::clusters::clusters;
use crate::services::kafka::service::Service;
use axum::routing::get;
use axum::Router;

#[derive(Clone)]
pub struct KafkaServiceState(pub(crate) Service);

pub fn router(service: Service) -> Router {
    Router::new()
        .route("/clusters", get(clusters))
        .with_state(KafkaServiceState(service))
}
