use super::{DatabaseIntrospector, TableInfo, ColumnInfo, QueryResult, PostgresConfig};
use sqlx::{PgPool, Row, Column};
use std::collections::HashMap;

/// PostgreSQL database introspector
pub struct PostgresIntrospector {
    pool: PgPool,
}

impl PostgresIntrospector {
    /// Create a new PostgreSQL introspector
    pub async fn new(config: &PostgresConfig) -> Result<Self, sqlx::Error> {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            config.username,
            config.password,
            config.host,
            config.port,
            config.database
        );

        let pool = PgPool::connect(&connection_string).await?;
        Ok(Self { pool })
    }

    /// Create a new introspector with existing pool
    pub fn with_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl DatabaseIntrospector for PostgresIntrospector {
    async fn get_tables(&self) -> Result<Vec<TableInfo>, Box<dyn std::error::Error + Send + Sync>> {
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

        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        let mut tables: HashMap<String, TableInfo> = HashMap::new();

        for row in rows {
            let table_name: String = row.get("table_name");
            let table_schema: String = row.get("table_schema");
            let column_name: Option<String> = row.get("column_name");

            let table_key = format!("{}.{}", table_schema, table_name);

            if !tables.contains_key(&table_key) {
                tables.insert(table_key.clone(), TableInfo {
                    name: table_name,
                    schema: table_schema,
                    columns: Vec::new(),
                });
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

                tables.get_mut(&table_key).unwrap().columns.push(column_info);
            }
        }

        Ok(tables.into_values().collect())
    }

    async fn get_table_schema(&self, table_name: &str) -> Result<TableInfo, Box<dyn std::error::Error + Send + Sync>> {
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
            WHERE t.table_name = $1 AND t.table_schema NOT IN ('information_schema', 'pg_catalog')
            ORDER BY c.ordinal_position
        "#;

        let rows = sqlx::query(query)
            .bind(table_name)
            .fetch_all(&self.pool)
            .await?;

        if rows.is_empty() {
            return Err(format!("Table '{}' not found", table_name).into());
        }

        let mut table_info = TableInfo {
            name: table_name.to_string(),
            schema: String::new(),
            columns: Vec::new(),
        };

        for row in rows {
            if table_info.schema.is_empty() {
                table_info.schema = row.get("table_schema");
            }

            let column_name: Option<String> = row.get("column_name");
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

                table_info.columns.push(column_info);
            }
        }

        Ok(table_info)
    }

    async fn execute_query(&self, query: &str) -> Result<QueryResult, Box<dyn std::error::Error + Send + Sync>> {
        let rows = sqlx::query(query).fetch_all(&self.pool).await?;

        if rows.is_empty() {
            return Ok(QueryResult::new());
        }

        let mut result = QueryResult::new();

        // Get column names from the first row
        for column in rows[0].columns() {
            result.columns.push(column.name().to_string());
        }

        // Convert rows to JSON values
        for row in rows {
            let mut row_values = Vec::new();
            for column in row.columns() {
                let value: serde_json::Value = match row.try_get::<String, _>(column.name()) {
                    Ok(v) => serde_json::Value::String(v),
                    Err(_) => {
                        // Try other types if string fails
                        serde_json::Value::Null
                    }
                };
                row_values.push(value);
            }
            result.rows.push(row_values);
        }

        result.row_count = result.rows.len();
        Ok(result)
    }
}
