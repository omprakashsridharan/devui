pub mod api;
pub mod spa;

use axum::extract::State;
pub use api::sql::connections as sql_connections;
pub use api::sql::postgres::tables as postgres_tables;
use axum::Json;
use serde::Serialize;
use crate::extractors::sql::SqlService;
// pub use spa::*;

#[derive(Serialize)]
pub struct DevUIService {
    name: String,
    available: bool,
}

pub async fn dev_ui_services(State(sql_service): State<SqlService>) -> Json<Vec<DevUIService>> {
    Json(vec![DevUIService {
        name: "SQL".to_string(),
        available: sql_service.sql_state.is_some(),
    }])
}
