use crate::sql::{DatabaseConfig, postgres::PostgresIntrospector, DatabaseIntrospector};
use axum::{
    extract::State,
    http::StatusCode,
    response::Response,
};
use std::sync::Arc;

/// Handler for GET /api/tables endpoint
/// Returns a list of database tables
pub async fn get_tables(
    State(db_config): State<Arc<DatabaseConfig>>,
) -> Result<Response<String>, StatusCode> {
    if let Some(postgres_config) = db_config.postgres() {
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
}
