use std::sync::Arc;
pub(crate) use crate::services::sql::connection_manager::{ConnectionManager, ConnectionManagerError};
use crate::services::sql::connection_pool::ConnectionPoolType;
use thiserror::Error;
use crate::handlers::api::sql::DatabaseType;

#[derive(Clone)]
pub struct Service {
    pub connection_manager: Arc<ConnectionManager>,
}

#[derive(Error, Debug)]
pub enum SqlServiceError {
    #[error("connection manager error")]
    ConnectionManagerError(#[from] ConnectionManagerError),
}

impl Service {
    pub fn new(connection_manager: ConnectionManager) -> Self {
        Self { connection_manager: Arc::new(connection_manager) }
    }

    pub fn get_connections(self) -> Vec<(String, DatabaseType)> {
        self.connection_manager.get_connections()
    }

    pub async fn tables(self, connection_name: String) -> Result<(), SqlServiceError> {
        let pool = self
            .connection_manager
            .get_connection(&connection_name)
            .map_err(SqlServiceError::ConnectionManagerError)?;
        Ok(())
    }
}
