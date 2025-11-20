use crate::handlers::sql::DatabaseType;
use crate::services::sql::config::{Config, DatabaseConfig};
use crate::services::sql::connection_pool::ConnectionPool;
use crate::services::sql::mysql::MysqlConnectionPool;
use crate::services::sql::postgres::PostgresConnectionPool;
use std::collections::HashMap;
use thiserror::Error;

pub struct ConnectionManager {
    pools: HashMap<String, Box<dyn ConnectionPool>>,
}

#[derive(Error, Debug)]
pub enum ConnectionManagerError {
    #[error("connection creation error: {0}")]
    ConnectionCreationError(String),
    #[error("connection not found")]
    ConnectionNotFound,
}

impl ConnectionManager {
    pub async fn new(sql_config: Config) -> Result<Self, ConnectionManagerError> {
        let mut pools: HashMap<String, Box<dyn ConnectionPool>> = HashMap::new();

        for (connection_name, database_config) in sql_config.database_configs {
            let connection_pool = match database_config {
                DatabaseConfig::Postgres(_) => PostgresConnectionPool::create_pool(database_config)
                    .await
                    .map_err(|e| ConnectionManagerError::ConnectionCreationError(e.to_string()))?,
                DatabaseConfig::Mysql(_) => MysqlConnectionPool::create_pool(database_config)
                    .await
                    .map_err(|e| ConnectionManagerError::ConnectionCreationError(e.to_string()))?,
            };

            pools.insert(connection_name.to_string(), connection_pool);
            tracing::info!("created connection pool for {}", connection_name);
        }

        Ok(Self { pools })
    }

    pub fn get_connections(&self) -> Vec<(String, DatabaseType)> {
        self.pools
            .iter()
            .map(|(name, pool)| (name.clone(), pool.database_type()))
            .collect()
    }

    pub fn get_connection(
        &self,
        connection_name: &str,
    ) -> Result<&dyn ConnectionPool, ConnectionManagerError> {
        self.pools
            .get(connection_name)
            .map(|pool| pool.as_ref())
            .ok_or(ConnectionManagerError::ConnectionNotFound)
    }
}
