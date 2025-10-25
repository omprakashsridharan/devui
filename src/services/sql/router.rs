use crate::handlers::{sql_connections, tables};
use crate::services::sql::service::Service;
use axum::routing::get;
use axum::Router;

#[derive(Clone)]
pub struct SqlServiceState(pub(crate) Service);

pub fn router(sql_service: Service) -> Router {
    Router::new()
        .route("/connections", get(sql_connections))
        .route("/connections/{connection_name}/tables", get(tables))
        .with_state(SqlServiceState(sql_service))
}
