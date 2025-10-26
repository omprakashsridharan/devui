use crate::handlers::api::sql::DatabaseType;
use crate::services::sql::config::DatabaseConfig;
use crate::services::sql::connection_pool::{ConnectionPool, ConnectionPoolError};
use crate::services::sql::models::{ColumnInfo, TableInfo, TableRow};
use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use sqlx::types::{chrono, Json};
use sqlx::{Column, Pool, Postgres, Row, TypeInfo};
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

        match sqlx::query(query).fetch_all(&self.pool).await {
            Ok(rows) => {
                let mut tables: HashMap<String, TableInfo> = HashMap::new();

                for row in rows {
                    let table_name: String = row.get("table_name");
                    let table_schema: String = row.get("table_schema");
                    let column_name: Option<String> = row.get("column_name");

                    let table_key = format!("{}.{}", table_schema, table_name);

                    if !tables.contains_key(&table_key) {
                        tables.insert(
                            table_key.clone(),
                            TableInfo {
                                name: table_name,
                                schema: table_schema,
                                columns: Vec::new(),
                            },
                        );
                    }

                    if let Some(column_name) = column_name {
                        let data_type: String = row.get("data_type");
                        let is_nullable: String = row.get("is_nullable");
                        let column_default: Option<String> = row.get("column_default");
                        let is_primary_key: bool = row.get("is_primary_key");

                        let column_info = ColumnInfo {
                            name: column_name,
                            data_type,
                            is_nullable: is_nullable == "YES",
                            is_primary_key,
                            default_value: column_default,
                        };

                        tables
                            .get_mut(&table_key)
                            .unwrap()
                            .columns
                            .push(column_info);
                    }
                }

                Ok(tables.into_values().collect())
            }
            Err(e) => {
                tracing::error!("Failed to fetch tables: {}", e);
                Err(ConnectionPoolError::SqlxError(e))
            }
        }
    }

    async fn table_data(
        &self,
        table_name: String,
        filters: Option<HashMap<String, String>>,
    ) -> Result<Vec<TableRow>, ConnectionPoolError> {
        let column_query = format!(
            r#"
        SELECT column_name, data_type, udt_name, is_nullable
        FROM information_schema.columns
        WHERE table_name = '{}'
        "#,
            table_name
        );

        let columns_info = sqlx::query(&column_query)
            .fetch_all(&self.pool)
            .await
            .map_err(ConnectionPoolError::SqlxError)?;

        // Build a query that casts enums to text and handles filters
        let mut select_parts = Vec::new();
        let mut where_clauses = Vec::new();

        for row in columns_info {
            let column_name: String = row.get("column_name");
            let data_type: String = row.get("data_type");

            // Handle enum casting for SELECT
            if data_type == "USER-DEFINED" {
                select_parts.push(format!("{}::text as {}", column_name, column_name));
            } else {
                select_parts.push(column_name.clone());
            }

            // Handle filters based on column type
            if let Some(ref filters) = filters {
                if let Some(filter_value) = filters.get(&column_name) {
                    if !filter_value.is_empty() {
                        let escaped_value = filter_value.replace("'", "''"); // Basic SQL injection prevention

                        let where_clause = match data_type.as_str() {
                            // Numeric types
                            "integer" | "bigint" | "smallint" | "numeric" | "decimal" | "real"
                            | "double precision" => {
                                if let Ok(_) = filter_value.parse::<f64>() {
                                    format!("{} = '{}'", column_name, escaped_value)
                                } else {
                                    format!("{}::text ILIKE '%{}%'", column_name, escaped_value)
                                }
                            }
                            // Date/time types
                            "timestamp" | "timestamptz" | "date" | "time" => {
                                format!("{}::text ILIKE '%{}%'", column_name, escaped_value)
                            }
                            // Boolean
                            "boolean" => {
                                if filter_value.to_lowercase() == "true" || filter_value == "1" {
                                    format!("{} = true", column_name)
                                } else if filter_value.to_lowercase() == "false"
                                    || filter_value == "0"
                                {
                                    format!("{} = false", column_name)
                                } else {
                                    format!("{}::text ILIKE '%{}%'", column_name, escaped_value)
                                }
                            }
                            // JSON types
                            "json" | "jsonb" => {
                                format!("{}::text ILIKE '%{}%'", column_name, escaped_value)
                            }
                            // UUID
                            "uuid" => {
                                format!("{}::text ILIKE '%{}%'", column_name, escaped_value)
                            }
                            // Arrays
                            "ARRAY" => {
                                format!("{}::text ILIKE '%{}%'", column_name, escaped_value)
                            }
                            // Enums and other user-defined types
                            "USER-DEFINED" => {
                                format!("{}::text ILIKE '%{}%'", column_name, escaped_value)
                            }
                            // Text types (default)
                            _ => {
                                format!("{} ILIKE '%{}%'", column_name, escaped_value)
                            }
                        };

                        where_clauses.push(where_clause);
                    }
                }
            }
        }

        // Build the final query
        let mut query = format!("SELECT {} FROM {}", select_parts.join(", "), table_name);

        if !where_clauses.is_empty() {
            query.push_str(&format!(" WHERE {}", where_clauses.join(" AND ")));
        }

        query.push_str(" LIMIT 10"); // Increased limit for filtered results

        tracing::debug!("SQL query: {}", query);

        match sqlx::query(&query).fetch_all(&self.pool).await {
            Ok(rows) => {
                let mut table_data: Vec<TableRow> = Vec::new();
                for row in rows {
                    let mut table_row_data: HashMap<String, String> = HashMap::new();
                    let mut columns_info: HashSet<String> = HashSet::new();
                    let columns = row.columns();
                    for column in columns {
                        let type_info = column.type_info();
                        columns_info.insert(column.name().to_string());
                        match type_info.name().to_ascii_uppercase().as_str() {
                            "INT4" => {
                                let value: i32 = row.get(column.name());
                                table_row_data.insert(column.name().to_string(), value.to_string());
                            }
                            "TIMESTAMPTZ" => {
                                let value: DateTime<Utc> = row.get(column.name());
                                table_row_data.insert(column.name().to_string(), value.to_string());
                            }
                            "TIMESTAMP" => {
                                let value: chrono::NaiveDateTime = row.get(column.name());
                                table_row_data.insert(column.name().to_string(), value.to_string());
                            }
                            "DATE" => {
                                let value: chrono::NaiveDate = row.get(column.name());
                                table_row_data.insert(column.name().to_string(), value.to_string());
                            }
                            "TIME" => {
                                let value: chrono::NaiveTime = row.get(column.name());
                                table_row_data.insert(column.name().to_string(), value.to_string());
                            }
                            "JSONB" => {
                                let value: Json<serde_json::Value> = row.get(column.name());
                                table_row_data.insert(column.name().to_string(), value.to_string());
                            }
                            "TEXT" | "VARCHAR" | "CHAR" => {
                                let value: Option<String> = row.get(column.name());
                                if let Some(value) = value {
                                    table_row_data.insert(column.name().to_string(), value);
                                } else {
                                    table_row_data
                                        .insert(column.name().to_string(), "".to_string());
                                }
                            }
                            unknown => {
                                // For custom types that were cast to text, try to get as text
                                // Even though SQLx reports the original type name, the actual data is text
                                match row.try_get::<Option<String>, _>(column.name()) {
                                    Ok(value) => {
                                        if let Some(value) = value {
                                            table_row_data.insert(column.name().to_string(), value);
                                        } else {
                                            table_row_data
                                                .insert(column.name().to_string(), "".to_string());
                                        }
                                    }
                                    Err(_) => {
                                        // If that fails, try to get as a generic string representation
                                        match row.try_get::<String, _>(column.name()) {
                                            Ok(value) => {
                                                table_row_data
                                                    .insert(column.name().to_string(), value);
                                            }
                                            Err(_) => {
                                                // Last resort: convert to string representation
                                                tracing::warn!("Failed to decode column {} of type {} as text, using placeholder",
                                                    column.name(), unknown);
                                                table_row_data.insert(
                                                    column.name().to_string(),
                                                    "[UNKNOWN TYPE]".to_string(),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    table_data.push(TableRow {
                        columns: columns_info,
                        data: table_row_data,
                    })
                }

                Ok(table_data)
            }
            Err(e) => {
                tracing::error!("Failed to fetch table data: {}", e);
                Err(ConnectionPoolError::SqlxError(e))
            }
        }
    }
}
