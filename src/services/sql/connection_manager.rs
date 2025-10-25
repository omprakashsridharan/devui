use crate::handlers::api::sql::DatabaseType;
use crate::services::sql::config::{Config, DatabaseConfig, PostgresConfig};
use crate::services::sql::connection_pool::ConnectionPool;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Clone)]
pub struct ConnectionManager {
    pools: HashMap<String, ConnectionPool>,
}

#[derive(Error, Debug)]
pub enum ConnectionManagerError {
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("connection not found")]
    ConnectionNotFound,
}

impl ConnectionManager {
    pub async fn new(sql_config: Config) -> Result<Self, ConnectionManagerError> {
        let mut pools: HashMap<String, ConnectionPool> = HashMap::new();
        for (connection_name, database_config) in sql_config.database_configs {
            match database_config {
                DatabaseConfig::Postgres(postgres_config) => {
                    let pool = Self::create_postgres_connections(postgres_config).await?;
                    pools.insert(connection_name.to_string(), ConnectionPool::Postgres(pool));
                    tracing::info!("created postgres connection pool for {}", connection_name);
                }
            }
        }
        Ok(Self { pools })
    }

    async fn create_postgres_connections(
        config: PostgresConfig,
    ) -> Result<Pool<Postgres>, ConnectionManagerError> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.database
        );
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .map_err(ConnectionManagerError::SqlxError)?;

        Ok(pool)
    }

    pub fn get_connections(&self) -> Vec<(String, ConnectionPool)> {
        self.pools
            .clone()
            .into_iter()
            .map(|(cn, cp)| (cn, cp))
            .collect()
    }

    pub fn get_connection(
        &self,
        connection_name: String,
    ) -> Result<ConnectionPool, ConnectionManagerError> {
        let database_pool = self
            .pools
            .get(&connection_name)
            .ok_or(ConnectionManagerError::ConnectionNotFound)?;
        match database_pool {
            ConnectionPool::Postgres(pool) => {
                let pool = pool.clone();
                Ok(ConnectionPool::Postgres(pool))
            }
        }
    }
}
