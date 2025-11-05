use crate::handlers::sql::DatabaseType;
use crate::services::sql::config::DatabaseConfig;
use crate::services::sql::connection_pool::{ConnectionPool, ConnectionPoolError};
use crate::services::sql::field_decoder::FieldDecoder;
use crate::services::sql::filter_handler::FilterHandler;
use crate::services::sql::models::{ColumnInfo, ForeignKeyInfo, TableData, TableInfo, TableRow, UpdateData};
use crate::services::sql::query_builder::table_count;
use sea_query::PostgresQueryBuilder;
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
                CASE WHEN pk.column_name IS NOT NULL THEN true ELSE false END as is_primary_key,
                fk.referenced_table_schema,
                fk.referenced_table_name,
                fk.referenced_column_name
            FROM information_schema.columns c
            LEFT JOIN (
                SELECT ku.table_name, ku.column_name, ku.table_schema
                FROM information_schema.table_constraints tc
                JOIN information_schema.key_column_usage ku ON tc.constraint_name = ku.constraint_name
                WHERE tc.constraint_type = 'PRIMARY KEY'
            ) pk ON c.table_name = pk.table_name
                AND c.column_name = pk.column_name
                AND c.table_schema = pk.table_schema
            LEFT JOIN (
                SELECT
                    kcu.column_name,
                    kcu.table_name,
                    kcu.table_schema,
                    ccu.table_schema AS referenced_table_schema,
                    ccu.table_name AS referenced_table_name,
                    ccu.column_name AS referenced_column_name
                FROM information_schema.table_constraints AS tc
                JOIN information_schema.key_column_usage AS kcu
                    ON tc.constraint_name = kcu.constraint_name
                    AND tc.table_schema = kcu.table_schema
                JOIN information_schema.constraint_column_usage AS ccu
                    ON ccu.constraint_name = tc.constraint_name
                    AND ccu.table_schema = tc.table_schema
                WHERE tc.constraint_type = 'FOREIGN KEY'
            ) fk ON c.table_name = fk.table_name
                AND c.column_name = fk.column_name
                AND c.table_schema = fk.table_schema
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
            let referenced_table_schema: Option<String> = row.get("referenced_table_schema");
            let referenced_table_name: Option<String> = row.get("referenced_table_name");
            let referenced_column_name: Option<String> = row.get("referenced_column_name");

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

    /// Escape SQL value to prevent injection
    fn escape_sql_value(value: &str) -> String {
        value.replace("'", "''")
    }

    /// Check if a string represents a numeric value
    fn is_numeric(value: &str) -> bool {
        value.parse::<f64>().is_ok()
    }

    /// Parse boolean value
    fn parse_boolean(value: &str) -> Result<bool, String> {
        match value.to_lowercase().as_str() {
            "true" | "t" | "1" | "yes" | "y" | "on" => Ok(true),
            "false" | "f" | "0" | "no" | "n" | "off" => Ok(false),
            _ => Err(format!("Invalid boolean value: '{}'", value)),
        }
    }

    /// Format SQL value based on data type for UPDATE statements
    fn format_sql_value(value: &str, data_type: &str) -> String {
        // Handle NULL values
        if value.is_empty() || value == "null" || value == "NULL" {
            return "NULL".to_string();
        }

        match data_type.to_lowercase().as_str() {
            // Numeric types - no quotes
            "integer" | "bigint" | "smallint" | "numeric" | "decimal" | "real"
            | "double precision" | "int2" | "int4" | "int8" | "float4" | "float8" => {
                if Self::is_numeric(value) {
                    value.to_string()
                } else {
                    // If not numeric, treat as text
                    let escaped = Self::escape_sql_value(value);
                    format!("'{}'", escaped)
                }
            }

            // Boolean types - no quotes
            "boolean" | "bool" => {
                match Self::parse_boolean(value) {
                    Ok(true) => "true".to_string(),
                    Ok(false) => "false".to_string(),
                    Err(_) => {
                        // Invalid boolean, treat as text
                        let escaped = Self::escape_sql_value(value);
                        format!("'{}'", escaped)
                    }
                }
            }

            // UUID - quoted
            "uuid" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // Text types - quoted and escaped
            "text" | "varchar" | "char" | "character" | "character varying" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // Date/time types - quoted
            "timestamp" | "timestamptz" | "date" | "time" | "timetz" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // JSON types - quoted
            "json" | "jsonb" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // User-defined types - quoted (will be cast to text if needed)
            "user-defined" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // Default - quoted as text
            _ => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }
        }
    }

    /// Build SET clause for UPDATE statement from changed columns
    fn build_update_set_clause(
        columns: &[ColumnInfo],
        updated_row: &HashMap<String, String>,
        original_row: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut set_parts = Vec::new();

        for col in columns {
            let col_name = &col.name;
            let updated_value = updated_row.get(col_name);
            let original_value = original_row.get(col_name);

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
                let formatted_value = if let Some(upd_val) = updated_value {
                    Self::format_sql_value(upd_val, &col.data_type)
                } else {
                    "NULL".to_string()
                };

                set_parts.push(format!("{} = {}", col_name, formatted_value));
            }
        }

        set_parts
    }

    /// Build WHERE clause for UPDATE statement using primary key values
    fn build_update_where_clause(
        columns: &[ColumnInfo],
        primary_key_values: &HashMap<String, String>,
    ) -> Result<String, ConnectionPoolError> {
        let primary_key_columns: Vec<&ColumnInfo> = columns
            .iter()
            .filter(|col| col.is_primary_key)
            .collect();

        if primary_key_columns.is_empty() {
            return Err(ConnectionPoolError::SqlxError(sqlx::Error::Configuration(
                "Table has no primary key columns".into(),
            )));
        }

        let mut where_parts = Vec::new();

        for pk_col in &primary_key_columns {
            let pk_value = primary_key_values.get(&pk_col.name).ok_or_else(|| {
                ConnectionPoolError::SqlxError(sqlx::Error::Configuration(format!(
                    "Missing primary key value for column: {}",
                    pk_col.name
                ).into()))
            })?;

            let formatted_value = Self::format_sql_value(pk_value, &pk_col.data_type);
            where_parts.push(format!("{} = {}", pk_col.name, formatted_value));
        }

        Ok(where_parts.join(" AND "))
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
                    CASE WHEN pk.column_name IS NOT NULL THEN true ELSE false END as is_primary_key,
                    fk.referenced_table_schema,
                    fk.referenced_table_name,
                    fk.referenced_column_name
                FROM information_schema.tables t
                LEFT JOIN information_schema.columns c ON t.table_name = c.table_name AND t.table_schema = c.table_schema
                LEFT JOIN (
                    SELECT ku.table_name, ku.column_name, ku.table_schema
                    FROM information_schema.table_constraints tc
                    JOIN information_schema.key_column_usage ku ON tc.constraint_name = ku.constraint_name
                    WHERE tc.constraint_type = 'PRIMARY KEY'
                ) pk ON c.table_name = pk.table_name AND c.column_name = pk.column_name AND c.table_schema = pk.table_schema
                LEFT JOIN (
                    SELECT
                        kcu.column_name,
                        kcu.table_name,
                        kcu.table_schema,
                        ccu.table_schema AS referenced_table_schema,
                        ccu.table_name AS referenced_table_name,
                        ccu.column_name AS referenced_column_name
                    FROM information_schema.table_constraints AS tc
                    JOIN information_schema.key_column_usage AS kcu
                        ON tc.constraint_name = kcu.constraint_name
                        AND tc.table_schema = kcu.table_schema
                    JOIN information_schema.constraint_column_usage AS ccu
                        ON ccu.constraint_name = tc.constraint_name
                        AND ccu.table_schema = tc.table_schema
                    WHERE tc.constraint_type = 'FOREIGN KEY'
                ) fk ON c.table_name = fk.table_name
                    AND c.column_name = fk.column_name
                    AND c.table_schema = fk.table_schema
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
        let query = table_count(table_name.clone()).to_string(PostgresQueryBuilder);

        sqlx::query(&query)
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get::<i64, _>(0) as u64)
            .map_err(|e| {
                tracing::error!("Failed to fetch table count for {}: {}", table_name, e);
                ConnectionPoolError::SqlxError(e)
            })
    }

    async fn update_table(&self, update_data: UpdateData) -> Result<(), ConnectionPoolError> {
        // Early return if no changes
        if update_data.changes.is_empty() {
            tracing::debug!("No changes to apply");
            return Ok(());
        }

        // Fetch column information for the table
        let columns = self.fetch_table_columns(&update_data.table_name, None).await?;

        if columns.is_empty() {
            tracing::warn!("No columns found for table: {}", update_data.table_name);
            return Err(ConnectionPoolError::SqlxError(sqlx::Error::Configuration(
                format!("Table '{}' not found or has no columns", update_data.table_name).into(),
            )));
        }

        // Escape table name to prevent SQL injection
        let escaped_table_name = update_data.table_name.replace("'", "''");

        // Process each change
        for (idx, change) in update_data.changes.iter().enumerate() {
            // Build WHERE clause from primary key values
            let where_clause = Self::build_update_where_clause(&columns, &change.primary_key_values)?;

            // Build SET clause from changed columns
            let set_parts = Self::build_update_set_clause(
                &columns,
                &change.updated_row,
                &change.original_row,
            );

            // Skip if no columns changed
            if set_parts.is_empty() {
                tracing::warn!(
                    "Change {} has no modified columns, skipping",
                    idx
                );
                continue;
            }

            // Build UPDATE query
            let update_query = format!(
                "UPDATE {} SET {} WHERE {}",
                escaped_table_name,
                set_parts.join(", "),
                where_clause
            );

            tracing::debug!("Executing UPDATE query (change {}): {}", idx, update_query);

            // Execute UPDATE statement
            sqlx::query(&update_query)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    tracing::error!(
                        "Failed to execute UPDATE for change {}: {}",
                        idx,
                        e
                    );
                    ConnectionPoolError::SqlxError(e)
                })?;
        }

        Ok(())
    }
}
