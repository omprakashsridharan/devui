use axum::Json;
use serde::Serialize;
use crate::extractors::postgres_pool::PostgresPool;

#[derive(Serialize)]
pub struct TablesResponse {
    connection_name: String,
    pool_status: String,
    message: String,
}

pub async fn tables(PostgresPool { connection_name, pool }: PostgresPool) -> Json<TablesResponse> {
    let pool_status = if pool.is_closed() {
        "closed".to_string()
    } else {
        "open".to_string()
    };

    Json(TablesResponse {
        connection_name,
        pool_status,
        message: "PostgresPool extractor successfully provided the connection pool".to_string(),
    })
}
