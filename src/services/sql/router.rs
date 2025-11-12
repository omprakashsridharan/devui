use crate::handlers::sql::connections;
use crate::handlers::sql::table_data::table_data;
use crate::handlers::sql::tables::tables;
use crate::handlers::sql::update_table::update_table;
use crate::services::sql::service::Service;
use axum::routing::{get, put};
use axum::Router;

#[derive(Clone)]
pub struct SqlServiceState(pub(crate) Service);

pub fn router(sql_service: Service) -> Router {
    Router::new()
        .route("/connections", get(connections))
        .route("/connections/{connection_name}/tables", get(tables))
        .route(
            "/connections/{connection_name}/schemas/{schema_name}/tables/{table_name}",
            get(table_data),
        )
        .route(
            "/connections/{connection_name}/schemas/{schema_name}/tables/{table_name}",
            put(update_table),
        )
        .with_state(SqlServiceState(sql_service))
}
