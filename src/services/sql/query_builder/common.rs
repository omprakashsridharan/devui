use crate::services::sql::models::ColumnInfo;
use sea_query::{Alias, Asterisk, Cond, Expr, ExprTrait, Query, SelectStatement, UpdateStatement};
use std::collections::{BTreeMap, HashMap};

pub fn table_count(table_name: String, table_schema: Option<String>) -> SelectStatement {
    let mut query = Query::select();
    query.expr(Expr::col(Asterisk).count());

    if let Some(schema) = table_schema {
        query.from((schema, table_name));
    } else {
        query.from(table_name);
    }

    query.to_owned()
}

pub fn table_data(
    columns: &[ColumnInfo],
    table_name: String,
    table_schema: Option<String>,
    limit: u64,
    offset: u64,
    filters: Option<BTreeMap<String, String>>,
    cast_type: &str,
) -> SelectStatement {
    let mut query = Query::select();

    for col in columns {
        let column_name = col.name.clone();
        query.expr_as(
            Expr::col(column_name.clone()).cast_as(cast_type.to_string()),
            Alias::new(column_name),
        );
    }

    if let Some(schema) = table_schema {
        query.from((schema, table_name));
    } else {
        query.from(table_name);
    }

    if let Some(filters) = filters {
        if !filters.is_empty() {
            let mut conditions = Cond::all();
            for (column_name, filter_value) in filters {
                conditions = conditions.add(
                    Expr::col(column_name.clone())
                        .cast_as(cast_type.to_string())
                        .eq(filter_value.clone()),
                );
            }
            query.cond_where(conditions);
        }
    }

    query.limit(limit).offset(offset);

    query.to_owned()
}

pub fn update_table(
    table_name: String,
    table_schema: Option<String>,
    primary_key_values: HashMap<String, String>,
    update_values: BTreeMap<String, String>,
    cast_type: &str,
) -> UpdateStatement {
    let mut query = Query::update();

    if let Some(schema) = table_schema {
        query.table((schema, table_name));
    } else {
        query.table(table_name);
    }

    let mut conditions = Cond::all();
    for (primary_column_name, primary_column_value) in primary_key_values {
        conditions = conditions.add(
            Expr::col(primary_column_name.clone())
                .cast_as(cast_type.to_string())
                .eq(primary_column_value.clone()),
        );
    }
    query.values(update_values.iter().map(|(column_name, update_value)| {
        (column_name.clone(), update_value.clone().to_owned().into())
    }));
    query.cond_where(conditions);

    query.to_owned()
}

#[cfg(test)]
mod tests {

    use sea_query::PostgresQueryBuilder;

    use super::*;

    #[test]
    fn test_table_count_postgres() {
        let expected = r#"SELECT COUNT(*) FROM "test""#;
        assert_eq!(
            table_count("test".to_string(), None).to_string(PostgresQueryBuilder),
            expected
        );
    }

    #[test]
    fn test_table_count_with_schema_postgres() {
        let expected = r#"SELECT COUNT(*) FROM "schema1"."test""#;
        assert_eq!(
            table_count("test".to_string(), Some("schema1".to_string()))
                .to_string(PostgresQueryBuilder),
            expected
        );
    }

    #[test]
    fn test_table_data_postgres() {
        let columns = vec![
            ColumnInfo {
                name: "id".to_string(),
                data_type: "INTEGER".to_string(),
                is_nullable: false,
                is_primary_key: true,
                default_value: None,
                enum_values: None,
                foreign_key: None,
            },
            ColumnInfo {
                name: "name".to_string(),
                data_type: "NUMRANGE".to_string(),
                is_nullable: false,
                is_primary_key: true,
                default_value: None,
                enum_values: None,
                foreign_key: None,
            },
        ];
        let expected = r#"SELECT
         CAST("id" AS TEXT) AS "id",
         CAST("name" AS TEXT) AS "name"
         FROM "test"
         WHERE CAST("id" AS TEXT) = '1' AND CAST("name" AS TEXT) = '2'
         LIMIT 10 OFFSET 0
         "#;
        assert_eq!(
            table_data(
                &columns,
                "test".to_string(),
                None,
                10,
                0,
                Some(BTreeMap::from([
                    ("id".to_string(), "1".to_string()),
                    ("name".to_string(), "2".to_string()),
                ])),
                "TEXT"
            )
            .to_string(PostgresQueryBuilder)
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" "),
            expected.split_whitespace().collect::<Vec<&str>>().join(" ")
        );
    }
}
