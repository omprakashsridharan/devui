use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
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


impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn with_postgres(mut self, connection_name: String, config: PostgresConfig) -> Self {
        if self.database_configs.contains_key(&connection_name) {
            panic!("Connection with name {} already exists", connection_name);
        }
        self.database_configs
            .insert(connection_name, DatabaseConfig::Postgres(config));
        self
    }
}
