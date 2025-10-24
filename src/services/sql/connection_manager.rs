use std::collections::HashMap;
use sqlx::{Error, PgPool, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use thiserror::Error;
use crate::services::sql::{Config, PostgresConfig};

pub struct ConnectionManager {
    postgres_pools: HashMap<String, Pool<Postgres>>,
}

#[derive(Error, Debug)]
pub enum ConnectionManagerError{
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
    #[error("connection not found")]
    ConnectionNotFound,
}

impl ConnectionManager {
    pub async fn new(config: Config) -> Result<Self, ConnectionManagerError> {
        let mut postgres_pools: HashMap<String, Pool<Postgres>> = HashMap::new();
        for (connection_name, postgres_config) in config.postgres {
            let pool = Self::create_postgres_connections(postgres_config).await?;
            postgres_pools.insert(connection_name.to_string(), pool);
            tracing::info!("created postgres connection pool for {}", connection_name);
        }
        Ok(Self { postgres_pools })
    }

    async fn create_postgres_connections(config: PostgresConfig) -> Result<Pool<Postgres>, ConnectionManagerError> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username,
            config.password,
            config.host,
            config.port,
            config.database
        );
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string).await.map_err(ConnectionManagerError::SqlxError)?;

        Ok(pool)
    }

    pub async fn get_postgres_connections(&self, connection_name: String) -> Result<Pool<Postgres>, ConnectionManagerError> {
        if let Some(pool) = self.postgres_pools.get(&connection_name) {
            Ok(pool.clone())
        } else {
            Err(ConnectionManagerError::ConnectionNotFound)
        }
    }
}