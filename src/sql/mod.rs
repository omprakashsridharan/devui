pub mod postgres;

use serde::{Deserialize, Serialize};

/// Database configuration for DevUI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub postgres: Option<PostgresConfig>,
}

/// PostgreSQL connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: Option<String>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            postgres: None,
        }
    }
}

impl DatabaseConfig {
    /// Create a new database configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Add PostgreSQL configuration
    pub fn with_postgres(mut self, config: PostgresConfig) -> Self {
        self.postgres = Some(config);
        self
    }

    /// Get PostgreSQL configuration
    pub fn postgres(&self) -> Option<&PostgresConfig> {
        self.postgres.as_ref()
    }
}

/// Database table information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableInfo {
    pub name: String,
    pub schema: String,
    pub columns: Vec<ColumnInfo>,
}

/// Database column information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ColumnInfo {
    pub name: String,
    pub data_type: String,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub default_value: Option<String>,
}

/// Database introspection trait
#[async_trait::async_trait]
pub trait DatabaseIntrospector {
    /// Get all tables in the database
    async fn get_tables(&self) -> Result<Vec<TableInfo>, Box<dyn std::error::Error + Send + Sync>>;

    /// Get table schema information
    async fn get_table_schema(&self, table_name: &str) -> Result<TableInfo, Box<dyn std::error::Error + Send + Sync>>;

    /// Execute a query and return results
    async fn execute_query(&self, query: &str) -> Result<QueryResult, Box<dyn std::error::Error + Send + Sync>>;
}

/// Query execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<serde_json::Value>>,
    pub row_count: usize,
}

impl QueryResult {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            row_count: 0,
        }
    }
}
