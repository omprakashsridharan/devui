use crate::handlers::api::sql::DatabaseType;
use crate::services::sql::models::TableInfo;
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
}
