use crate::handlers::sql::DatabaseType;
use crate::services::sql::models::{TableData, TableInfo};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionPoolError {
    #[error("Could not connect to database")]
    InitError,
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
}

#[async_trait::async_trait]
pub trait ConnectionPool: Send + Sync {
    fn database_type(&self) -> DatabaseType;

    async fn tables(&self) -> Result<Vec<TableInfo>, ConnectionPoolError>;

    async fn table_data(
        &self,
        table_name: String,
        filters: Option<HashMap<String, String>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<TableData, ConnectionPoolError>;

    async fn table_count(&self, table_name: String) -> Result<u64, ConnectionPoolError>;
}
