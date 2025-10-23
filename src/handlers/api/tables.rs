use crate::sql::{DatabaseConfig, postgres::PostgresIntrospector, DatabaseIntrospector};
use axum::{
    http::StatusCode,
    response::Response,
};
use std::sync::Arc;

/// Handler for GET /api/tables endpoint
/// Returns a list of database tables
pub async fn get_tables(
    db_config: Option<Arc<DatabaseConfig>>,
) -> Result<Response<String>, StatusCode> {
    if let Some(config) = db_config {
        if let Some(postgres_config) = config.postgres() {
            match PostgresIntrospector::new(postgres_config).await {
                Ok(introspector) => {
                    match introspector.get_tables().await {
                        Ok(tables) => {
                            let json = serde_json::to_string(&tables)
                                .unwrap_or_else(|_| "[]".to_string());
                            let response = Response::builder()
                                .status(StatusCode::OK)
                                .header("content-type", "application/json")
                                .body(json)
                                .unwrap();
                            Ok(response)
                        }
                        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
                    }
                }
                Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        } else {
            Err(StatusCode::BAD_REQUEST)
        }
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}
