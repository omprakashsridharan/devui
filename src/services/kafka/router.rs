use crate::services::kafka::service::Service;
use axum::Router;

#[derive(Clone)]
pub struct KafkaServiceState(pub(crate) Service);

pub fn router(service: Service) -> Router {
    Router::new().with_state(KafkaServiceState(service))
}
