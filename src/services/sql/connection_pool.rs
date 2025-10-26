use crate::handlers::sql::DatabaseType;
use crate::services::sql::models::{TableInfo, TableRow};
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
    ) -> Result<Vec<TableRow>, ConnectionPoolError>;
}
