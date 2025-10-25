// src/services/sql/connection_pool.rs
use crate::handlers::api::sql::DatabaseType;
use crate::services::sql::config::DatabaseConfig;
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Column, Pool, Postgres, Row};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionPoolError {
    #[error("Could not connect to database")]
    InitError,
    #[error("sqlx error")]
    SqlxError(#[from] sqlx::Error),
}

// Make the trait object-safe by removing async from the trait method
// and using a different approach for connection creation
#[async_trait::async_trait]
pub trait ConnectionPool: Send + Sync {
    fn database_type(&self) -> DatabaseType;

    /// Execute a query and return results
    async fn execute_query(&self, query: &str) -> Result<Vec<Value>, sqlx::Error>;

    /// Get connection info
    fn get_connection_info(&self) -> String;
}

// Factory trait for creating connection pools
#[async_trait::async_trait]
pub trait ConnectionPoolFactory: Send + Sync {
    async fn create_pool(
        config: DatabaseConfig,
    ) -> Result<Box<dyn ConnectionPool>, ConnectionPoolError>;
}

#[derive(Clone)]
pub struct PostgresConnectionPool {
    pool: Pool<Postgres>,
    connection_info: String,
}

impl PostgresConnectionPool {
    pub fn new(pool: Pool<Postgres>, connection_info: String) -> Self {
        Self {
            pool,
            connection_info,
        }
    }
}

#[async_trait::async_trait]
impl ConnectionPool for PostgresConnectionPool {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::POSTGRES
    }

    async fn execute_query(&self, query: &str) -> Result<Vec<Value>, sqlx::Error> {
        let rows = sqlx::query(query).fetch_all(&self.pool).await?;
        let mut results = Vec::new();

        for row in rows {
            let mut map = serde_json::Map::new();
            for (i, column) in row.columns().iter().enumerate() {
                let value: Value = row.try_get(i).unwrap_or(Value::Null);
                map.insert(column.name().to_string(), value);
            }
            results.push(Value::Object(map));
        }

        Ok(results)
    }

    fn get_connection_info(&self) -> String {
        self.connection_info.clone()
    }
}

// Factory implementation for Postgres
pub struct PostgresConnectionPoolFactory;

#[async_trait::async_trait]
impl ConnectionPoolFactory for PostgresConnectionPoolFactory {
    async fn create_pool(
        config: DatabaseConfig,
    ) -> Result<Box<dyn ConnectionPool>, ConnectionPoolError> {
        match config {
            DatabaseConfig::Postgres(postgres_config) => {
                let connection_string = format!(
                    "postgres://{}:{}@{}:{}/{}",
                    postgres_config.username,
                    postgres_config.password,
                    postgres_config.host,
                    postgres_config.port,
                    postgres_config.database
                );

                let pool = PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&connection_string)
                    .await
                    .map_err(ConnectionPoolError::SqlxError)?;

                let connection_info = format!(
                    "postgres://{}@{}:{}/{}",
                    postgres_config.username,
                    postgres_config.host,
                    postgres_config.port,
                    postgres_config.database
                );

                Ok(Box::new(PostgresConnectionPool::new(pool, connection_info)))
            }
            _ => Err(ConnectionPoolError::InitError),
        }
    }
}

// For backward compatibility, keep the enum wrapper
#[derive(Clone)]
pub enum ConnectionPoolType {
    Postgres(PostgresConnectionPool),
}

impl ConnectionPoolType {
    pub fn postgres(pool: Pool<Postgres>, connection_info: String) -> Self {
        Self::Postgres(PostgresConnectionPool::new(pool, connection_info))
    }
}

#[async_trait::async_trait]
impl ConnectionPool for ConnectionPoolType {
    fn database_type(&self) -> DatabaseType {
        match self {
            Self::Postgres(pool) => pool.database_type(),
        }
    }

    async fn execute_query(&self, query: &str) -> Result<Vec<Value>, sqlx::Error> {
        match self {
            Self::Postgres(pool) => pool.execute_query(query).await,
        }
    }

    fn get_connection_info(&self) -> String {
        match self {
            Self::Postgres(pool) => pool.get_connection_info(),
        }
    }
}

impl Into<DatabaseType> for ConnectionPoolType {
    fn into(self) -> DatabaseType {
        self.database_type()
    }
}
