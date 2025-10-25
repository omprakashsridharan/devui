use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database_configs: HashMap<String, DatabaseConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseConfig {
    Postgres(PostgresConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub ssl_mode: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            database_configs: HashMap::new(),
        }
    }
}

impl Config {
    /// Create a new database configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Add PostgreSQL configuration
    pub fn with_postgres(mut self, connection_name: String, config: PostgresConfig) -> Self {
        if self.database_configs.contains_key(&connection_name) {
            panic!(
                "Connection with name {} already exists",
                connection_name
            );
        }
        self.database_configs.insert(connection_name, DatabaseConfig::Postgres(config));
        self
    }
}