pub mod kafka;
pub mod sql;

use axum::{Json, extract::State};
use serde::Serialize;

use crate::DevUIConfig;

#[derive(Serialize)]
pub struct DevUIService {
    name: String,
    available: bool,
}

pub async fn dev_ui_services(State(dev_ui_config): State<DevUIConfig>) -> Json<Vec<DevUIService>> {
    let mut services = Vec::new();
    if let Some(sql_config) = dev_ui_config.sql_config {
        services.push(DevUIService {
            name: "SQL".to_string(),
            available: true,
        });
    }
    if let Some(kafka_config) = dev_ui_config.kafka_config {
        services.push(DevUIService {
            name: "Kafka".to_string(),
            available: true,
        });
    }
    Json(services)
}
