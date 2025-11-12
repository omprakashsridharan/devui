use crate::handlers::sql::DatabaseType;
pub(crate) use crate::services::sql::connection_manager::{
    ConnectionManager, ConnectionManagerError,
};
use crate::services::sql::connection_pool::ConnectionPoolError;
use crate::services::sql::models::{TableData, TableInfo, TablesBySchema, UpdateData};
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

    pub async fn tables(self, connection_name: String) -> Result<TablesBySchema, SqlServiceError> {
        let pool = self
            .connection_manager
            .get_connection(&connection_name)
            .map_err(SqlServiceError::ConnectionManagerError)?;
        let table_info = pool
            .tables()
            .await
            .map_err(SqlServiceError::ConnectionPoolError)?;

        // Group tables by schema
        use std::collections::BTreeMap;
        let mut schema_map: BTreeMap<String, Vec<TableInfo>> = BTreeMap::new();

        for table in table_info {
            schema_map
                .entry(table.schema.clone())
                .or_default()
                .push(table);
        }

        let schemas: Vec<_> = schema_map
            .into_iter()
            .map(|(name, tables)| crate::services::sql::models::SchemaInfo { name, tables })
            .collect();

        Ok(TablesBySchema { schemas })
    }

    pub async fn table_data(
        self,
        connection_name: String,
        schema_name: String,
        table_name: String,
        filters: Option<std::collections::HashMap<String, String>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<TableData, SqlServiceError> {
        let pool = self
            .connection_manager
            .get_connection(&connection_name)
            .map_err(SqlServiceError::ConnectionManagerError)?;

        // Always construct schema-qualified table name to avoid ambiguity when same table exists in multiple schemas
        let schema_qualified_name = format!("{}.{}", schema_name, table_name);

        let table_data = pool
            .table_data(schema_qualified_name, filters, page, page_size)
            .await
            .map_err(SqlServiceError::ConnectionPoolError)?;
        Ok(table_data)
    }

    pub async fn update_table(
        self,
        connection_name: String,
        update_data: UpdateData,
    ) -> Result<(), SqlServiceError> {
        let pool = self
            .connection_manager
            .get_connection(&connection_name)
            .map_err(SqlServiceError::ConnectionManagerError)?;
        pool.update_table(update_data)
            .await
            .map_err(SqlServiceError::ConnectionPoolError)?;
        Ok(())
    }
}
