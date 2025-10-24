use std::collections::HashMap;
use axum::Json;
use http::StatusCode;
use serde::Serialize;
use sqlx::Row;
use crate::extractors::postgres_pool::PostgresPool;
use crate::handlers::api::sql::{ColumnInfo, TableInfo};

pub async fn tables(PostgresPool { connection_name, pool }: PostgresPool) -> Result<Json<Vec<TableInfo>>, StatusCode> {
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

    match sqlx::query(query).fetch_all(&pool).await {
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

            Ok(Json(tables.into_values().collect()))
        }
        Err(e) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
