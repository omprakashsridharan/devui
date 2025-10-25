use crate::handlers::{sql_connections, table_data, tables};
use crate::services::sql::service::Service;
use axum::routing::get;
use axum::Router;

#[derive(Clone)]
pub struct SqlServiceState(pub(crate) Service);

pub fn router(sql_service: Service) -> Router {
    Router::new()
        .route("/connections", get(sql_connections))
        .route("/connections/{connection_name}/tables", get(tables))
        .route(
            "/connections/{connection_name}/tables/{table_name}",
            get(table_data),
        )
        .with_state(SqlServiceState(sql_service))
}
