use crate::handlers::sql::DatabaseType;
pub(crate) use crate::services::sql::connection_manager::{
    ConnectionManager, ConnectionManagerError,
};
use crate::services::sql::connection_pool::ConnectionPoolError;
use crate::services::sql::models::{TableData, TableInfo};
use crate::SqlConfig;
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone)]
pub struct Service {
    pub connection_manager: Arc<ConnectionManager>,
}

#[derive(Error, Debug)]
pub enum SqlServiceError {
    #[error("connection manager error")]
    ConnectionManagerError(#[from] ConnectionManagerError),
    #[error("connection pool error")]
    ConnectionPoolError(#[from] ConnectionPoolError),
}

impl Service {
    pub async fn new(sql_config: SqlConfig) -> Result<Self, SqlServiceError> {
        let connection_manager = ConnectionManager::new(sql_config.clone())
            .await
            .map_err(SqlServiceError::ConnectionManagerError)?;
        Ok(Self {
            connection_manager: Arc::new(connection_manager),
        })
    }

    pub fn get_connections(self) -> Vec<(String, DatabaseType)> {
        self.connection_manager.get_connections()
    }

    pub async fn tables(self, connection_name: String) -> Result<Vec<TableInfo>, SqlServiceError> {
        let pool = self
            .connection_manager
            .get_connection(&connection_name)
            .map_err(SqlServiceError::ConnectionManagerError)?;
        let table_info = pool
            .tables()
            .await
            .map_err(SqlServiceError::ConnectionPoolError)?;
        Ok(table_info)
    }

    pub async fn table_data(
        self,
        connection_name: String,
        table_name: String,
        filters: Option<std::collections::HashMap<String, String>>,
    ) -> Result<TableData, SqlServiceError> {
        let pool = self
            .connection_manager
            .get_connection(&connection_name)
            .map_err(SqlServiceError::ConnectionManagerError)?;
        let table_data = pool
            .table_data(table_name, filters)
            .await
            .map_err(SqlServiceError::ConnectionPoolError)?;
        Ok(table_data)
    }
}
