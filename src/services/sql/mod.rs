pub mod connection_manager;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Database configuration for DevUI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub postgres: HashMap<String, PostgresConfig>,
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

impl Default for Config {
    fn default() -> Self {
        Self {
            postgres: HashMap::new(),
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
        if self.postgres.contains_key(&connection_name) {
            panic!(
                "Postgres connection with name {} already exists",
                connection_name
            );
        }
        self.postgres.insert(connection_name, config);
        self
    }
}