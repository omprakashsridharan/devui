use crate::handlers::sql::DatabaseType;
use crate::services::sql::config::DatabaseConfig;
use crate::services::sql::connection_pool::{ConnectionPool, ConnectionPoolError};
use crate::services::sql::field_decoder::FieldDecoder;
use crate::services::sql::filter_handler::FilterHandler;
use crate::services::sql::models::{ColumnInfo, TableData, TableInfo, TableRow};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Column, Pool, Postgres, Row};
use std::collections::{HashMap, HashSet};

pub struct PostgresConnectionPool {
    pool: Pool<Postgres>,
}

impl PostgresConnectionPool {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
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
        let schema_filter = if let Some(schema) = type_schema {
            format!("AND n.nspname = '{}'", schema.replace("'", "''"))
        } else {
            String::new()
        };

        let query = format!(
            r#"
            SELECT e.enumlabel AS enum_value
            FROM pg_type t
            JOIN pg_enum e ON t.oid = e.enumtypid
            JOIN pg_namespace n ON n.oid = t.typnamespace
            WHERE t.typname = '{}'
            {}
            ORDER BY e.enumsortorder
            "#,
            type_name.replace("'", "''"),
            schema_filter
        );

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::warn!("Failed to fetch enum values for type {}: {}", type_name, e);
                ConnectionPoolError::SqlxError(e)
            })?;

        Ok(rows
            .into_iter()
            .map(|row| row.get::<String, _>("enum_value"))
            .collect())
    }

    /// Fetch column information for a specific table
    async fn fetch_table_columns(
        &self,
        table_name: &str,
        table_schema: Option<&str>,
    ) -> Result<Vec<ColumnInfo>, ConnectionPoolError> {
        let schema_filter = if let Some(schema) = table_schema {
            format!("AND c.table_schema = '{}'", schema.replace("'", "''"))
        } else {
            String::new()
        };

        let query = format!(
            r#"
            SELECT
                c.column_name,
                c.data_type,
                c.udt_name,
                c.is_nullable,
                c.column_default,
                CASE WHEN pk.column_name IS NOT NULL THEN true ELSE false END as is_primary_key
            FROM information_schema.columns c
            LEFT JOIN (
                SELECT ku.table_name, ku.column_name, ku.table_schema
                FROM information_schema.table_constraints tc
                JOIN information_schema.key_column_usage ku ON tc.constraint_name = ku.constraint_name
                WHERE tc.constraint_type = 'PRIMARY KEY'
            ) pk ON c.table_name = pk.table_name
                AND c.column_name = pk.column_name
                AND c.table_schema = pk.table_schema
            WHERE c.table_name = '{}'
            {}
            ORDER BY c.ordinal_position
            "#,
            table_name.replace("'", "''"),
            schema_filter
        );

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

            // Fetch enum values if this is a USER-DEFINED type
            let enum_values = if data_type == "USER-DEFINED" {
                match self
                    .fetch_enum_values(&udt_name, table_schema)
                    .await
                {
                    Ok(values) if !values.is_empty() => Some(values),
                    _ => None,
                }
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
            });
        }

        Ok(columns)
    }

    /// Build SELECT clause parts with proper type casting for user-defined types
    fn build_select_parts(columns: &[ColumnInfo]) -> Vec<String> {
        columns
            .iter()
            .map(|col| {
                if col.data_type == "USER-DEFINED" {
                    format!("{}::text as {}", col.name, col.name)
                } else {
                    col.name.clone()
                }
            })
            .collect()
    }

    /// Convert ColumnInfo to tuple format for filter handler
    fn columns_to_tuples(columns: &[ColumnInfo]) -> Vec<(String, String)> {
        columns
            .iter()
            .map(|col| (col.name.clone(), col.data_type.clone()))
            .collect()
    }

    /// Parse query result rows into TableRow structures
    fn parse_rows(rows: Vec<sqlx::postgres::PgRow>) -> Vec<TableRow> {
        rows.into_iter()
            .map(|row| {
                let mut table_row_data: HashMap<String, String> = HashMap::new();
                let mut columns_info: HashSet<String> = HashSet::new();
                let columns = row.columns();

                for column in columns {
                    columns_info.insert(column.name().to_string());

                    let decoded_value = match FieldDecoder::decode_field(&row, &column) {
                        Ok(value) => value,
                        Err(e) => {
                            tracing::warn!("Failed to decode column {}: {}", column.name(), e);
                            "[DECODE ERROR]".to_string()
                        }
                    };

                    table_row_data.insert(column.name().to_string(), decoded_value);
                }

                TableRow {
                    columns: columns_info,
                    data: table_row_data,
                }
            })
            .collect()
    }
}

#[async_trait::async_trait]
impl ConnectionPool for PostgresConnectionPool {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::POSTGRES
    }

    async fn tables(&self) -> Result<Vec<TableInfo>, ConnectionPoolError> {
        let query = r#"
                SELECT
                    t.table_name,
                    t.table_schema,
                    c.column_name,
                    c.data_type,
                    c.udt_name,
                    c.is_nullable,
                    c.column_default,
                    CASE WHEN pk.column_name IS NOT NULL THEN true ELSE false END as is_primary_key
                FROM information_schema.tables t
                LEFT JOIN information_schema.columns c ON t.table_name = c.table_name AND t.table_schema = c.table_schema
                LEFT JOIN (
                    SELECT ku.table_name, ku.column_name, ku.table_schema
                    FROM information_schema.table_constraints tc
                    JOIN information_schema.key_column_usage ku ON tc.constraint_name = ku.constraint_name
                    WHERE tc.constraint_type = 'PRIMARY KEY'
                ) pk ON c.table_name = pk.table_name AND c.column_name = pk.column_name AND c.table_schema = pk.table_schema
                WHERE t.table_schema NOT IN ('information_schema', 'pg_catalog')
                ORDER BY t.table_schema, t.table_name, c.ordinal_position
            "#;

        let rows = sqlx::query(query)
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

                let column_info = ColumnInfo {
                    name: column_name,
                    data_type,
                    is_nullable: is_nullable == "YES",
                    is_primary_key,
                    default_value: column_default,
                    enum_values,
                };

                tables
                    .get_mut(&table_key)
                    .expect("Table should exist after entry")
                    .columns
                    .push(column_info);
            }
        }

        Ok(tables.into_values().collect())
    }

    async fn table_data(
        &self,
        table_name: String,
        filters: Option<HashMap<String, String>>,
        page: Option<u64>,
        page_size: Option<u64>,
    ) -> Result<TableData, ConnectionPoolError> {
        // Fetch column information using the reusable method
        let columns = self.fetch_table_columns(&table_name, None).await?;

        if columns.is_empty() {
            tracing::warn!("No columns found for table: {}", table_name);
            return Ok(TableData {
                rows: Vec::new(),
                total_rows: 0,
                columns: Vec::new(),
            });
        }

        // Build SELECT parts with proper type casting
        let select_parts = Self::build_select_parts(&columns);

        // Convert to tuple format for filter handler
        let column_tuples = Self::columns_to_tuples(&columns);

        // Build WHERE clauses using the comprehensive filter handler
        let where_clauses = if let Some(ref filters) = filters {
            FilterHandler::build_where_clauses(filters, &column_tuples).map_err(|e| {
                tracing::error!("Failed to build WHERE clauses: {}", e);
                ConnectionPoolError::SqlxError(sqlx::Error::Configuration(e.to_string().into()))
            })?
        } else {
            Vec::new()
        };

        // Build the final query (table_name should be validated/escaped by caller)
        let escaped_table_name = table_name.replace("'", "''");
        let mut query = format!("SELECT {} FROM {}", select_parts.join(", "), escaped_table_name);

        if !where_clauses.is_empty() {
            query.push_str(&format!(" WHERE {}", where_clauses.join(" AND ")));
        }

        // Handle pagination
        let limit = page_size.unwrap_or(10);
        let offset = page.map(|p| (p - 1) * limit).unwrap_or(0);
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        tracing::debug!("SQL query: {}", query);

        // Execute query and parse results
        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch table data: {}", e);
                ConnectionPoolError::SqlxError(e)
            })?;

        let table_data = Self::parse_rows(rows);

        Ok(TableData {
            rows: table_data,
            total_rows: self.table_count(table_name).await?,
            columns,
        })
    }

    async fn table_count(&self, table_name: String) -> Result<u64, ConnectionPoolError> {
        // Escape table name to prevent SQL injection
        let escaped_table_name = table_name.replace("'", "''");
        let query = format!("SELECT COUNT(*) FROM {}", escaped_table_name);

        sqlx::query(&query)
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get::<i64, _>(0) as u64)
            .map_err(|e| {
                tracing::error!("Failed to fetch table count for {}: {}", table_name, e);
                ConnectionPoolError::SqlxError(e)
            })
    }
}
