pub mod spa;
pub mod sql;

use axum::Json;
use serde::Serialize;
pub use sql::connections as sql_connections;
pub use sql::table_data::table_data;
pub use sql::tables::tables;
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
