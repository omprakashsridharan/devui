use crate::handlers::sql::DatabaseType;
use crate::services::sql::config::DatabaseConfig;
use crate::services::sql::connection_pool::{ConnectionPool, ConnectionPoolError};
use crate::services::sql::models::{
    ColumnInfo, ForeignKeyInfo, TableData, TableInfo, TableRow, UpdateData,
};
use crate::services::sql::query_builder::common::{table_count, table_data, update_table};
use crate::services::sql::query_builder::postgres::{enum_values, table_columns, tables};
use sea_query::PostgresQueryBuilder;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Column, Pool, Postgres, Row};
use std::collections::{BTreeMap, HashMap, HashSet};

pub struct PostgresConnectionPool {
    pool: Pool<Postgres>,
}

impl PostgresConnectionPool {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Parse a table name that may be schema-qualified (e.g., "schema1.customers" or "customers")
    /// Returns (table_name, schema)
    fn parse_table_name(table_name: &str) -> (String, Option<String>) {
        if let Some(dot_pos) = table_name.rfind('.') {
            let schema = table_name[..dot_pos].to_string();
            let name = table_name[dot_pos + 1..].to_string();
            (name, Some(schema))
        } else {
            (table_name.to_string(), None)
        }
    }

    pub async fn create_pool(
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

                Ok(Box::new(PostgresConnectionPool::new(pool)))
            }
        }
    }

    /// Fetch ENUM values for a given type name
    async fn fetch_enum_values(
        &self,
        type_name: &str,
        type_schema: Option<&str>,
    ) -> Result<Vec<String>, ConnectionPoolError> {
        let query = enum_values(type_name.to_string(), type_schema.map(|s| s.to_string()))
            .to_string(PostgresQueryBuilder);

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::warn!("Failed to fetch enum values for type {}: {}", type_name, e);
                ConnectionPoolError::SqlxError(e)
            })?;

        let enum_values: Vec<String> = rows
            .into_iter()
            .map(|row| row.get::<String, _>("enum_value"))
            .collect();

        Ok(enum_values)
    }

    /// Fetch column information for a specific table
    async fn fetch_table_columns(
        &self,
        table_name: &str,
        table_schema: Option<&str>,
    ) -> Result<Vec<ColumnInfo>, ConnectionPoolError> {
        // Use the modular sea-query builder instead of raw SQL
        let query = table_columns(table_name.to_string(), table_schema.map(|s| s.to_string()))
            .to_string(PostgresQueryBuilder);

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch columns for table {}: {}", table_name, e);
                ConnectionPoolError::SqlxError(e)
            })?;

        let mut columns = Vec::new();
        for row in rows {
            let column_name: String = row.get("column_name");
            let data_type: String = row.get("data_type");
            let udt_name: String = row.get("udt_name");
            let is_nullable: String = row.get("is_nullable");
            let column_default: Option<String> = row.get("column_default");
            let is_primary_key: bool = row.get("is_primary_key");
            let referenced_table_schema: Option<String> = row.get("referenced_table_schema");
            let referenced_table_name: Option<String> = row.get("referenced_table_name");
            let referenced_column_name: Option<String> = row.get("referenced_column_name");

            // Fetch enum values if this is a USER-DEFINED type
            let enum_values = if data_type == "USER-DEFINED" {
                match self.fetch_enum_values(&udt_name, table_schema).await {
                    Ok(values) if !values.is_empty() => Some(values),
                    Ok(_) => {
                        tracing::debug!("No enum values found for {}", udt_name);
                        None
                    }
                    Err(e) => {
                        tracing::warn!("Error fetching enum values for {}: {:?}", udt_name, e);
                        None
                    }
                }
            } else {
                None
            };

            // Build foreign key info if present
            let foreign_key = if let (Some(ref_table), Some(ref_schema), Some(ref_col)) = (
                referenced_table_name,
                referenced_table_schema,
                referenced_column_name,
            ) {
                Some(ForeignKeyInfo {
                    referenced_table: ref_table,
                    referenced_schema: ref_schema,
                    referenced_column: ref_col,
                })
            } else {
                None
            };

            columns.push(ColumnInfo {
                name: column_name,
                data_type,
                is_nullable: is_nullable == "YES",
                is_primary_key,
                default_value: column_default,
                enum_values,
                foreign_key,
            });
        }

        Ok(columns)
    }

    /// Parse query result rows into TableRow structures
    fn parse_rows(rows: Vec<sqlx::postgres::PgRow>) -> Vec<TableRow> {
        let table_rows: Vec<TableRow> = rows
            .into_iter()
            .enumerate()
            .map(|(row_idx, row)| {
                let mut table_row_data: HashMap<String, String> = HashMap::new();
                let mut columns_info: HashSet<String> = HashSet::new();
                let columns = row.columns();

                for column in columns {
                    let column_name = column.name().to_string();
                    columns_info.insert(column_name.clone());

                    let decoded_value = match row.try_get::<Option<String>, _>(column_name.as_str())
                    {
                        Ok(Some(value)) => value,
                        Ok(None) => "".to_string(),
                        Err(e) => {
                            tracing::warn!(
                                "Failed to decode column {} in row {}: {}",
                                column_name,
                                row_idx,
                                e
                            );
                            "[DECODE ERROR]".to_string()
                        }
                    };

                    table_row_data.insert(column_name, decoded_value);
                }

                TableRow {
                    columns: columns_info,
                    data: table_row_data,
                }
            })
            .collect();

        table_rows
    }
}

#[async_trait::async_trait]
impl ConnectionPool for PostgresConnectionPool {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::Postgres
    }

    async fn tables(&self) -> Result<Vec<TableInfo>, ConnectionPoolError> {
        let query = tables().to_string(PostgresQueryBuilder);

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch tables: {}", e);
                ConnectionPoolError::SqlxError(e)
            })?;

        let mut tables: HashMap<String, TableInfo> = HashMap::new();

        for row in rows {
            let table_name: String = row.get("table_name");
            let table_schema: String = row.get("table_schema");
            let column_name: Option<String> = row.get("column_name");
            let table_schema_clone = table_schema.clone();

            let table_key = format!("{}.{}", table_schema, table_name);

            // Initialize table if not exists
            tables
                .entry(table_key.clone())
                .or_insert_with(|| TableInfo {
                    name: table_name,
                    schema: table_schema,
                    columns: Vec::new(),
                });

            // Add column if present
            if let Some(column_name) = column_name {
                let data_type: String = row.get("data_type");
                let udt_name: String = row.get("udt_name");
                let is_nullable: String = row.get("is_nullable");
                let column_default: Option<String> = row.get("column_default");
                let is_primary_key: bool = row.get("is_primary_key");
                let referenced_table_schema: Option<String> = row.get("referenced_table_schema");
                let referenced_table_name: Option<String> = row.get("referenced_table_name");
                let referenced_column_name: Option<String> = row.get("referenced_column_name");

                // Fetch enum values if this is a USER-DEFINED type
                let enum_values = if data_type == "USER-DEFINED" {
                    match self
                        .fetch_enum_values(&udt_name, Some(&table_schema_clone))
                        .await
                    {
                        Ok(values) if !values.is_empty() => Some(values),
                        _ => None,
                    }
                } else {
                    None
                };

                // Build foreign key info if present
                let foreign_key = if let (Some(ref_table), Some(ref_schema), Some(ref_col)) = (
                    referenced_table_name,
                    referenced_table_schema,
                    referenced_column_name,
                ) {
                    Some(ForeignKeyInfo {
                        referenced_table: ref_table,
                        referenced_schema: ref_schema,
                        referenced_column: ref_col,
                    })
                } else {
                    None
                };

                let column_info = ColumnInfo {
                    name: column_name,
                    data_type,
                    is_nullable: is_nullable == "YES",
                    is_primary_key,
                    default_value: column_default,
                    enum_values,
                    foreign_key,
                };

                tables
                    .get_mut(&table_key)
                    .expect("Table should exist after entry")
                    .columns
                    .push(column_info);
            }
        }

        let table_list: Vec<TableInfo> = tables.into_values().collect();

        Ok(table_list)
    }

    async fn table_data(
        &self,
        table_name: String,
        filters: Option<HashMap<String, String>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<TableData, ConnectionPoolError> {
        tracing::debug!(
            "Fetching table data for: {} (filters: {:?}, page: {:?}, page_size: {:?})",
            table_name,
            filters,
            page,
            page_size
        );

        // Parse schema from table name if present
        let (table_name_only, table_schema) = Self::parse_table_name(&table_name);

        // Default to "public" schema if not specified to avoid ambiguity when same table exists in multiple schemas
        let table_schema = table_schema.or(Some("public".to_string()));

        // Fetch column information using the reusable method
        let columns = self
            .fetch_table_columns(&table_name_only, table_schema.as_deref())
            .await?;

        if columns.is_empty() {
            tracing::warn!("No columns found for table: {}", table_name);
            return Ok(TableData {
                rows: Vec::new(),
                total_rows: 0,
                columns: Vec::new(),
            });
        }

        let limit = page_size.unwrap_or(10);
        let offset = page.map(|p| (p - 1) * limit).unwrap_or(0);
        // query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
        let filters_btree_map: Option<BTreeMap<String, String>> = filters.map(|f| {
            let btree_filters: BTreeMap<String, String> = f.into_iter().collect();
            btree_filters
        });

        let query = table_data(
            &columns,
            table_name_only.clone(),
            table_schema.clone(),
            limit,
            offset,
            filters_btree_map,
        )
        .to_string(PostgresQueryBuilder);
        // Execute query and parse results
        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch table data: {}", e);
                ConnectionPoolError::SqlxError(e)
            })?;

        tracing::debug!("Fetched {} rows from database", rows.len());

        let table_data = Self::parse_rows(rows);

        let total_rows = self.table_count(table_name.clone()).await?;
        tracing::debug!("Total rows in table {}: {}", table_name, total_rows);

        Ok(TableData {
            rows: table_data,
            total_rows,
            columns,
        })
    }

    async fn table_count(&self, table_name: String) -> Result<u64, ConnectionPoolError> {
        // Parse schema from table name if present
        let (table_name_only, table_schema) = Self::parse_table_name(&table_name);

        // Default to "public" schema if not specified to avoid ambiguity when same table exists in multiple schemas
        let table_schema = table_schema.or(Some("public".to_string()));

        let query = table_count(table_name_only.clone(), table_schema.clone())
            .to_string(PostgresQueryBuilder);

        let count = sqlx::query(&query)
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get::<i64, _>(0) as u64)
            .map_err(|e| {
                tracing::error!("Failed to fetch table count for {}: {}", table_name, e);
                ConnectionPoolError::SqlxError(e)
            })?;

        Ok(count)
    }

    async fn update_table(&self, update_data: UpdateData) -> Result<(), ConnectionPoolError> {
        // Early return if no changes
        if update_data.changes.is_empty() {
            return Ok(());
        }

        // Parse schema from table name if present
        let (table_name_only, table_schema) = Self::parse_table_name(&update_data.table_name);

        // Default to "public" schema if not specified to avoid ambiguity when same table exists in multiple schemas
        let table_schema = table_schema.or(Some("public".to_string()));

        // Fetch column information for the table
        let columns = self
            .fetch_table_columns(&table_name_only, table_schema.as_deref())
            .await?;

        if columns.is_empty() {
            return Err(ConnectionPoolError::SqlxError(sqlx::Error::Configuration(
                format!(
                    "Table '{}' not found or has no columns",
                    update_data.table_name
                )
                .into(),
            )));
        }

        // Process each change
        for (idx, change) in update_data.changes.iter().enumerate() {
            let mut update_values: BTreeMap<String, String> = BTreeMap::new();

            for col in &columns {
                let col_name = &col.name;
                let updated_value = change.updated_row.get(col_name);
                let original_value = change.original_row.get(col_name);

                // Check if value changed
                let changed = match (original_value, updated_value) {
                    (Some(orig), Some(upd)) => {
                        // Normalize for comparison - treat empty strings as NULL
                        let orig_norm = if orig.is_empty() || orig == "null" || orig == "NULL" {
                            None
                        } else {
                            Some(orig.as_str())
                        };
                        let upd_norm = if upd.is_empty() || upd == "null" || upd == "NULL" {
                            None
                        } else {
                            Some(upd.as_str())
                        };
                        orig_norm != upd_norm
                    }
                    (None, Some(_)) => true,
                    (Some(_), None) => true,
                    (None, None) => false,
                };

                if changed {
                    update_values.insert(col_name.clone(), updated_value.unwrap().clone());
                }
            }

            // Skip if no columns changed
            if update_values.is_empty() {
                tracing::warn!("Change {} has no modified columns, skipping", idx);
                continue;
            }

            // Build UPDATE query
            let update_query = update_table(
                table_name_only.clone(),
                table_schema.clone(),
                change.primary_key_values.clone(),
                update_values,
            )
            .to_string(PostgresQueryBuilder);

            // Execute UPDATE statement
            let _: sqlx::postgres::PgQueryResult = sqlx::query(&update_query)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to execute UPDATE for change {}: {}", idx, e);
                    ConnectionPoolError::SqlxError(e)
                })?;
        }

        Ok(())
    }
}
