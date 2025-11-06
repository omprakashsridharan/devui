use crate::handlers::sql::DatabaseType;
use crate::services::sql::config::DatabaseConfig;
use crate::services::sql::connection_pool::{ConnectionPool, ConnectionPoolError};
use crate::services::sql::field_decoder::FieldDecoder;
use crate::services::sql::filter_handler::FilterHandler;
use crate::services::sql::models::{
    ColumnInfo, ForeignKeyInfo, TableData, TableInfo, TableRow, UpdateData,
};
use crate::services::sql::query_builder::common::table_count;
use crate::services::sql::query_builder::postgres::{enum_values, table_columns, tables};
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
        tracing::debug!(
            "Fetching enum values for type: {} (schema: {:?})",
            type_name,
            type_schema
        );

        let query = enum_values(type_name.to_string(), type_schema.map(|s| s.to_string()))
            .to_string(PostgresQueryBuilder);

        tracing::debug!("Enum values query: {}", query);

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

        tracing::debug!(
            "Fetched {} enum values for type {}: {:?}",
            enum_values.len(),
            type_name,
            enum_values
        );

        Ok(enum_values)
    }

    /// Fetch column information for a specific table
    async fn fetch_table_columns(
        &self,
        table_name: &str,
        table_schema: Option<&str>,
    ) -> Result<Vec<ColumnInfo>, ConnectionPoolError> {
        tracing::debug!(
            "Fetching columns for table: {} (schema: {:?})",
            table_name,
            table_schema
        );

        // Use the modular sea-query builder instead of raw SQL
        let query = table_columns(table_name.to_string(), table_schema.map(|s| s.to_string()))
            .to_string(PostgresQueryBuilder);

        tracing::debug!("Table columns query: {}", query);

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch columns for table {}: {}", table_name, e);
                ConnectionPoolError::SqlxError(e)
            })?;

        tracing::debug!("Fetched {} rows for table columns", rows.len());

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

            tracing::debug!(
                "Processing column: {} (type: {}, udt: {}, nullable: {}, pk: {})",
                column_name,
                data_type,
                udt_name,
                is_nullable,
                is_primary_key
            );

            // Fetch enum values if this is a USER-DEFINED type
            let enum_values = if data_type == "USER-DEFINED" {
                tracing::debug!("Detected USER-DEFINED type, fetching enum values for: {}", udt_name);
                match self.fetch_enum_values(&udt_name, table_schema).await {
                    Ok(values) if !values.is_empty() => {
                        tracing::debug!("Found {} enum values for {}", values.len(), udt_name);
                        Some(values)
                    }
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
                tracing::debug!(
                    "Found foreign key: {}.{}.{} -> {}.{}.{}",
                    table_schema.unwrap_or("public"),
                    table_name,
                    column_name,
                    ref_schema,
                    ref_table,
                    ref_col
                );
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

        tracing::debug!("Returning {} columns for table {}", columns.len(), table_name);

        Ok(columns)
    }

    /// Build SELECT clause parts with proper type casting for user-defined types and complex types
    fn build_select_parts(columns: &[ColumnInfo]) -> Vec<String> {
        tracing::debug!("Building SELECT parts for {} columns", columns.len());
        let select_parts: Vec<String> = columns
            .iter()
            .map(|col| {
                let data_type_upper = col.data_type.to_uppercase();

                // Types that need to be cast to text for proper decoding
                let needs_text_cast = col.data_type == "USER-DEFINED"
                    || data_type_upper == "BIT"
                    || data_type_upper == "BIT VARYING"
                    || data_type_upper == "VARBIT"
                    || data_type_upper == "INTERVAL"
                    || data_type_upper == "INET"
                    || data_type_upper == "CIDR"
                    || data_type_upper == "MACADDR"
                    || data_type_upper == "MACADDR8"
                    || data_type_upper == "POINT"
                    || data_type_upper == "LINE"
                    || data_type_upper == "LSEG"
                    || data_type_upper == "BOX"
                    || data_type_upper == "PATH"
                    || data_type_upper == "POLYGON"
                    || data_type_upper == "CIRCLE"
                    || data_type_upper == "INT4RANGE"
                    || data_type_upper == "INT8RANGE"
                    || data_type_upper == "NUMRANGE"
                    || data_type_upper == "TSRANGE"
                    || data_type_upper == "TSTZRANGE"
                    || data_type_upper == "DATERANGE"
                    || data_type_upper == "XML"
                    // TIME WITH TIME ZONE needs to be cast to text
                    || data_type_upper == "TIME WITH TIME ZONE"
                    || data_type_upper == "TIMETZ"
                    // Array types - PostgreSQL returns data_type = 'ARRAY' for array columns
                    // Also check for array notation in type name
                    || data_type_upper == "ARRAY"
                    || data_type_upper.ends_with("[]")
                    || data_type_upper.contains("ARRAY")
                    || data_type_upper.starts_with("_");

                if needs_text_cast {
                    let casted = format!("{}::text as {}", col.name, col.name);
                    tracing::debug!("Casting column {} (type: {}) to text", col.name, col.data_type);
                    casted
                } else {
                    col.name.clone()
                }
            })
            .collect();
        tracing::debug!("Built {} SELECT parts", select_parts.len());
        select_parts
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
        tracing::debug!("Parsing {} rows", rows.len());
        let mut decode_errors = 0;
        let table_rows: Vec<TableRow> = rows
            .into_iter()
            .enumerate()
            .map(|(row_idx, row)| {
                let mut table_row_data: HashMap<String, String> = HashMap::new();
                let mut columns_info: HashSet<String> = HashSet::new();
                let columns = row.columns();

                tracing::debug!("Parsing row {} with {} columns", row_idx, columns.len());

                for column in columns {
                    let column_name = column.name().to_string();
                    columns_info.insert(column_name.clone());

                    let decoded_value = match FieldDecoder::decode_field(&row, &column) {
                        Ok(value) => {
                            value
                        }
                        Err(e) => {
                            decode_errors += 1;
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

        if decode_errors > 0 {
            tracing::warn!("Encountered {} decode errors while parsing rows", decode_errors);
        }

        tracing::debug!("Successfully parsed {} rows", table_rows.len());
        table_rows
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
        tracing::debug!("Formatting SQL value: '{}' (type: {})", value, data_type);

        // Handle NULL values
        if value.is_empty() || value == "null" || value == "NULL" {
            tracing::debug!("Value is NULL");
            return "NULL".to_string();
        }

        let formatted = match data_type.to_lowercase().as_str() {
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

            // Interval type - quoted
            "interval" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // JSON types - quoted
            "json" | "jsonb" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // Network types - quoted
            "inet" | "cidr" | "macaddr" | "macaddr8" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // Geometric types - quoted
            "point" | "line" | "lseg" | "box" | "path" | "polygon" | "circle" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // Range types - quoted
            "int4range" | "int8range" | "numrange" | "tsrange" | "tstzrange" | "daterange" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // Bit string types - quoted
            "bit" | "bit varying" | "varbit" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // XML type - quoted
            "xml" => {
                let escaped = Self::escape_sql_value(value);
                format!("'{}'", escaped)
            }

            // Money type - quoted (treated as text for safety)
            "money" => {
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
        };

        tracing::debug!("Formatted value: '{}' -> '{}'", value, formatted);
        formatted
    }

    /// Build SET clause for UPDATE statement from changed columns
    fn build_update_set_clause(
        columns: &[ColumnInfo],
        updated_row: &HashMap<String, String>,
        original_row: &HashMap<String, String>,
    ) -> Vec<String> {
        tracing::debug!(
            "Building UPDATE SET clause for {} columns",
            columns.len()
        );

        let mut set_parts = Vec::new();
        let mut changed_count = 0;

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
                changed_count += 1;
                let formatted_value = if let Some(upd_val) = updated_value {
                    tracing::debug!(
                        "Column {} changed: '{}' -> '{}'",
                        col_name,
                        original_value.unwrap_or(&"NULL".to_string()),
                        upd_val
                    );
                    Self::format_sql_value(upd_val, &col.data_type)
                } else {
                    tracing::debug!("Column {} changed to NULL", col_name);
                    "NULL".to_string()
                };

                set_parts.push(format!("{} = {}", col_name, formatted_value));
            }
        }

        tracing::debug!(
            "Built SET clause with {} changed columns out of {} total",
            changed_count,
            columns.len()
        );

        set_parts
    }

    /// Build WHERE clause for UPDATE statement using primary key values
    fn build_update_where_clause(
        columns: &[ColumnInfo],
        primary_key_values: &HashMap<String, String>,
    ) -> Result<String, ConnectionPoolError> {
        tracing::debug!("Building UPDATE WHERE clause from primary key values");

        let primary_key_columns: Vec<&ColumnInfo> =
            columns.iter().filter(|col| col.is_primary_key).collect();

        tracing::debug!("Found {} primary key columns", primary_key_columns.len());

        if primary_key_columns.is_empty() {
            tracing::error!("Table has no primary key columns");
            return Err(ConnectionPoolError::SqlxError(sqlx::Error::Configuration(
                "Table has no primary key columns".into(),
            )));
        }

        let mut where_parts = Vec::new();

        for pk_col in &primary_key_columns {
            let pk_value = primary_key_values.get(&pk_col.name).ok_or_else(|| {
                tracing::error!("Missing primary key value for column: {}", pk_col.name);
                ConnectionPoolError::SqlxError(sqlx::Error::Configuration(
                    format!("Missing primary key value for column: {}", pk_col.name).into(),
                ))
            })?;

            tracing::debug!(
                "Adding primary key condition: {} = {}",
                pk_col.name,
                pk_value
            );

            let formatted_value = Self::format_sql_value(pk_value, &pk_col.data_type);
            where_parts.push(format!("{} = {}", pk_col.name, formatted_value));
        }

        let where_clause = where_parts.join(" AND ");
        tracing::debug!("Built WHERE clause: {}", where_clause);

        Ok(where_clause)
    }
}

#[async_trait::async_trait]
impl ConnectionPool for PostgresConnectionPool {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::POSTGRES
    }

    async fn tables(&self) -> Result<Vec<TableInfo>, ConnectionPoolError> {
        tracing::debug!("Fetching all tables");

        let query = tables().to_string(PostgresQueryBuilder);
        tracing::debug!("Tables query: {}", query);

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to fetch tables: {}", e);
                ConnectionPoolError::SqlxError(e)
            })?;

        tracing::debug!("Fetched {} rows for tables", rows.len());

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
        tracing::debug!("Returning {} tables", table_list.len());

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
            tracing::debug!("Building WHERE clauses from {} filters", filters.len());
            FilterHandler::build_where_clauses(filters, &column_tuples).map_err(|e| {
                tracing::error!("Failed to build WHERE clauses: {}", e);
                ConnectionPoolError::SqlxError(sqlx::Error::Configuration(e.to_string().into()))
            })?
        } else {
            Vec::new()
        };

        if !where_clauses.is_empty() {
            tracing::debug!("Built {} WHERE clauses: {:?}", where_clauses.len(), where_clauses);
        }

        // Build the final query (table_name should be validated/escaped by caller)
        let escaped_table_name = table_name.replace("'", "''");
        let mut query = format!(
            "SELECT {} FROM {}",
            select_parts.join(", "),
            escaped_table_name
        );

        if !where_clauses.is_empty() {
            query.push_str(&format!(" WHERE {}", where_clauses.join(" AND ")));
        }

        // Handle pagination
        let limit = page_size.unwrap_or(10);
        let offset = page.map(|p| (p - 1) * limit).unwrap_or(0);
        query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

        tracing::debug!("Final SQL query: {}", query);
        tracing::debug!("Pagination: limit={}, offset={}", limit, offset);

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
        tracing::debug!("Fetching row count for table: {}", table_name);

        let query = table_count(table_name.clone()).to_string(PostgresQueryBuilder);
        tracing::debug!("Table count query: {}", query);

        let count = sqlx::query(&query)
            .fetch_one(&self.pool)
            .await
            .map(|row| row.get::<i64, _>(0) as u64)
            .map_err(|e| {
                tracing::error!("Failed to fetch table count for {}: {}", table_name, e);
                ConnectionPoolError::SqlxError(e)
            })?;

        tracing::debug!("Table {} has {} rows", table_name, count);

        Ok(count)
    }

    async fn update_table(&self, update_data: UpdateData) -> Result<(), ConnectionPoolError> {
        tracing::debug!(
            "Updating table: {} with {} changes",
            update_data.table_name,
            update_data.changes.len()
        );

        // Early return if no changes
        if update_data.changes.is_empty() {
            tracing::debug!("No changes to apply");
            return Ok(());
        }

        // Fetch column information for the table
        let columns = self
            .fetch_table_columns(&update_data.table_name, None)
            .await?;

        if columns.is_empty() {
            tracing::warn!("No columns found for table: {}", update_data.table_name);
            return Err(ConnectionPoolError::SqlxError(sqlx::Error::Configuration(
                format!(
                    "Table '{}' not found or has no columns",
                    update_data.table_name
                )
                .into(),
            )));
        }

        // Escape table name to prevent SQL injection
        let escaped_table_name = update_data.table_name.replace("'", "''");

        // Process each change
        for (idx, change) in update_data.changes.iter().enumerate() {
            tracing::debug!(
                "Processing change {} of {} for table {}",
                idx + 1,
                update_data.changes.len(),
                update_data.table_name
            );
            // Build WHERE clause from primary key values
            let where_clause =
                Self::build_update_where_clause(&columns, &change.primary_key_values)?;

            // Build SET clause from changed columns
            let set_parts =
                Self::build_update_set_clause(&columns, &change.updated_row, &change.original_row);

            // Skip if no columns changed
            if set_parts.is_empty() {
                tracing::warn!("Change {} has no modified columns, skipping", idx);
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
            let result = sqlx::query(&update_query)
                .execute(&self.pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to execute UPDATE for change {}: {}", idx, e);
                    ConnectionPoolError::SqlxError(e)
                })?;

            tracing::debug!(
                "UPDATE query (change {}) affected {} rows",
                idx,
                result.rows_affected()
            );
        }

        tracing::debug!(
            "Successfully applied {} changes to table {}",
            update_data.changes.len(),
            update_data.table_name
        );

        Ok(())
    }
}
