pub mod api;
pub mod spa;

pub use api::sql::connections as sql_connections;
pub use api::sql::table_data::table_data;
pub use api::sql::tables::tables;
use axum::Json;
use serde::Serialize;
// pub use spa::*;

#[derive(Serialize)]
pub struct DevUIService {
    name: String,
    available: bool,
}

pub async fn dev_ui_services() -> Json<Vec<DevUIService>> {
    Json(vec![DevUIService {
        name: "SQL".to_string(),
        available: true,
    }])
}
