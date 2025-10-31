pub mod kafka;
pub mod sql;

use axum::Json;
use serde::Serialize;
// pub use spa::*;

#[derive(Serialize)]
pub struct DevUIService {
    name: String,
    available: bool,
}

pub async fn dev_ui_services() -> Json<Vec<DevUIService>> {
    Json(vec![
        DevUIService {
            name: "SQL".to_string(),
            available: true,
        },
        DevUIService {
            name: "Kafka".to_string(),
            available: true,
        },
    ])
}
