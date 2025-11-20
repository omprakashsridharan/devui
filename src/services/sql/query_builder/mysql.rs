use sea_query::JoinType::LeftJoin;
use sea_query::Order::Asc;
use sea_query::{all, Expr, ExprTrait, Iden, Query, SelectStatement};

#[derive(Iden)]
enum Tables {
    Table,
    TableName,
    TableSchema,
}

#[derive(Iden)]
enum Columns {
    Table,
    TableName,
    TableSchema,
    ColumnName,
    DataType,
    IsNullable,
    ColumnDefault,
    OrdinalPosition,
    ColumnKey,
}

#[derive(Iden)]
enum KeyColumnUsage {
    Table,
    ConstraintName,
    TableName,
    ColumnName,
    TableSchema,
    ReferencedTableName,
    ReferencedColumnName,
    ReferencedTableSchema,
}

#[derive(Iden)]
enum TableConstraints {
    Table,
    ConstraintName,
    ConstraintType,
    TableSchema,
    TableName,
}

#[derive(Iden)]
enum InformationSchema {
    Table,
}

pub fn foreign_key_constraint() -> SelectStatement {
    Query::select()
        .column((KeyColumnUsage::Table, KeyColumnUsage::ColumnName))
        .column((KeyColumnUsage::Table, KeyColumnUsage::TableName))
        .column((KeyColumnUsage::Table, KeyColumnUsage::TableSchema))
        .expr_as(
            Expr::col((KeyColumnUsage::Table, KeyColumnUsage::ReferencedTableSchema)),
            "referenced_table_schema",
        )
        .expr_as(
            Expr::col((KeyColumnUsage::Table, KeyColumnUsage::ReferencedTableName)),
            "referenced_table_name",
        )
        .expr_as(
            Expr::col((KeyColumnUsage::Table, KeyColumnUsage::ReferencedColumnName)),
            "referenced_column_name",
        )
        .from((InformationSchema::Table, TableConstraints::Table))
        .inner_join(
            (InformationSchema::Table, KeyColumnUsage::Table),
            all![
                Expr::col((
                    InformationSchema::Table,
                    TableConstraints::Table,
                    TableConstraints::ConstraintName,
                ))
                .equals((
                    InformationSchema::Table,
                    KeyColumnUsage::Table,
                    KeyColumnUsage::ConstraintName,
                )),
                Expr::col((
                    InformationSchema::Table,
                    TableConstraints::Table,
                    TableConstraints::TableSchema,
                ))
                .equals((
                    InformationSchema::Table,
                    KeyColumnUsage::Table,
                    KeyColumnUsage::TableSchema,
                ))
            ],
        )
        .and_where(
            Expr::col((
                InformationSchema::Table,
                TableConstraints::Table,
                TableConstraints::ConstraintType,
            ))
            .eq("FOREIGN KEY"),
        )
        .to_owned()
}

/// Builds the base column information query with common SELECT and JOINs.
/// This helper function returns a query builder that can be further customized.
fn build_column_info_base() -> SelectStatement {
    Query::select()
        .expr_as(
            Expr::col((Columns::Table, Columns::ColumnName)),
            "column_name",
        )
        .expr_as(Expr::col((Columns::Table, Columns::DataType)), "data_type")
        .expr_as(
            Expr::col((Columns::Table, Columns::IsNullable)),
            "is_nullable",
        )
        .expr_as(
            Expr::col((Columns::Table, Columns::ColumnDefault)),
            "column_default",
        )
        .expr_as(
            Expr::case(
                Expr::col((Columns::Table, Columns::ColumnKey)).eq("PRI"),
                true,
            )
            .finally(false),
            "is_primary_key",
        )
        .expr_as(
            Expr::col((KeyColumnUsage::Table, KeyColumnUsage::ReferencedTableSchema)),
            "referenced_table_schema",
        )
        .expr_as(
            Expr::col((KeyColumnUsage::Table, KeyColumnUsage::ReferencedTableName)),
            "referenced_table_name",
        )
        .expr_as(
            Expr::col((KeyColumnUsage::Table, KeyColumnUsage::ReferencedColumnName)),
            "referenced_column_name",
        )
        .from((InformationSchema::Table, Columns::Table))
        .join_subquery(
            LeftJoin,
            foreign_key_constraint(),
            KeyColumnUsage::Table,
            all![
                Expr::col((InformationSchema::Table, Columns::Table, Columns::TableName))
                    .equals((KeyColumnUsage::Table, KeyColumnUsage::TableName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::ColumnName
                ))
                .equals((KeyColumnUsage::Table, KeyColumnUsage::ColumnName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::TableSchema
                ))
                .equals((KeyColumnUsage::Table, KeyColumnUsage::TableSchema)),
            ],
        )
        .to_owned()
}

pub fn table_columns(table_name: String, table_schema: Option<String>) -> SelectStatement {
    let mut query = build_column_info_base();

    query.and_where(
        Expr::col((InformationSchema::Table, Columns::Table, Columns::TableName)).eq(table_name),
    );

    if let Some(schema) = table_schema {
        query.and_where(
            Expr::col((
                InformationSchema::Table,
                Columns::Table,
                Columns::TableSchema,
            ))
            .eq(schema),
        );
    }

    query.order_by(
        (
            InformationSchema::Table,
            Columns::Table,
            Columns::OrdinalPosition,
        ),
        Asc,
    );

    query
}

pub fn tables() -> SelectStatement {
    Query::select()
        .expr_as(Expr::col((Tables::Table, Tables::TableName)), "table_name")
        .expr_as(
            Expr::col((Tables::Table, Tables::TableSchema)),
            "table_schema",
        )
        .expr_as(
            Expr::col((Columns::Table, Columns::ColumnName)),
            "column_name",
        )
        .expr_as(Expr::col((Columns::Table, Columns::DataType)), "data_type")
        .expr_as(
            Expr::col((Columns::Table, Columns::IsNullable)),
            "is_nullable",
        )
        .expr_as(
            Expr::col((Columns::Table, Columns::ColumnDefault)),
            "column_default",
        )
        .expr_as(
            Expr::case(
                Expr::col((Columns::Table, Columns::ColumnKey)).eq("PRI"),
                true,
            )
            .finally(false),
            "is_primary_key",
        )
        .expr_as(
            Expr::col((KeyColumnUsage::Table, KeyColumnUsage::ReferencedTableSchema)),
            "referenced_table_schema",
        )
        .expr_as(
            Expr::col((KeyColumnUsage::Table, KeyColumnUsage::ReferencedTableName)),
            "referenced_table_name",
        )
        .expr_as(
            Expr::col((KeyColumnUsage::Table, KeyColumnUsage::ReferencedColumnName)),
            "referenced_column_name",
        )
        .from((InformationSchema::Table, Tables::Table))
        .left_join(
            (InformationSchema::Table, Columns::Table),
            all![
                Expr::col((InformationSchema::Table, Tables::Table, Tables::TableName,)).equals((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::TableName,
                )),
                Expr::col((InformationSchema::Table, Tables::Table, Tables::TableSchema,)).equals(
                    (
                        InformationSchema::Table,
                        Columns::Table,
                        Columns::TableSchema,
                    )
                )
            ],
        )
        .join_subquery(
            LeftJoin,
            foreign_key_constraint(),
            KeyColumnUsage::Table,
            all![
                Expr::col((InformationSchema::Table, Columns::Table, Columns::TableName))
                    .equals((KeyColumnUsage::Table, KeyColumnUsage::TableName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::ColumnName
                ))
                .equals((KeyColumnUsage::Table, KeyColumnUsage::ColumnName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::TableSchema
                ))
                .equals((KeyColumnUsage::Table, KeyColumnUsage::TableSchema)),
            ],
        )
        .and_where(
            Expr::col((InformationSchema::Table, Tables::Table, Tables::TableSchema)).is_not_in([
                "information_schema",
                "mysql",
                "performance_schema",
                "sys",
            ]),
        )
        .order_by_columns(vec![
            (
                (InformationSchema::Table, Tables::Table, Tables::TableSchema),
                Asc,
            ),
            (
                (InformationSchema::Table, Tables::Table, Tables::TableName),
                Asc,
            ),
        ])
        .order_by(
            (
                InformationSchema::Table,
                Columns::Table,
                Columns::OrdinalPosition,
            ),
            Asc,
        )
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_query::MysqlQueryBuilder;

    // Helper function to normalize SQL strings by removing whitespace and newlines
    fn normalize_sql(sql: &str) -> String {
        sql.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    #[test]
    fn test_table_columns_mysql() {
        let expected = r#"
            SELECT
                `columns`.`column_name` AS `column_name`,
                `columns`.`data_type` AS `data_type`,
                `columns`.`is_nullable` AS `is_nullable`,
                `columns`.`column_default` AS `column_default`,
                (CASE WHEN (`columns`.`column_key` = 'PRI') THEN TRUE ELSE FALSE END) AS `is_primary_key`,
                `key_column_usage`.`referenced_table_schema` AS `referenced_table_schema`,
                `key_column_usage`.`referenced_table_name` AS `referenced_table_name`,
                `key_column_usage`.`referenced_column_name` AS `referenced_column_name`
            FROM `information_schema`.`columns`
            LEFT JOIN (SELECT
                `key_column_usage`.`column_name`,
                `key_column_usage`.`table_name`,
                `key_column_usage`.`table_schema`,
                `key_column_usage`.`referenced_table_schema` AS `referenced_table_schema`,
                `key_column_usage`.`referenced_table_name` AS `referenced_table_name`,
                `key_column_usage`.`referenced_column_name` AS `referenced_column_name`
            FROM `information_schema`.`table_constraints`
            INNER JOIN `information_schema`.`key_column_usage`
                ON `information_schema`.`table_constraints`.`constraint_name` = `information_schema`.`key_column_usage`.`constraint_name`
                AND `information_schema`.`table_constraints`.`table_schema` = `information_schema`.`key_column_usage`.`table_schema`
            WHERE `information_schema`.`table_constraints`.`constraint_type` = 'FOREIGN KEY') AS `key_column_usage`
                ON `information_schema`.`columns`.`table_name` = `key_column_usage`.`table_name`
                AND `information_schema`.`columns`.`column_name` = `key_column_usage`.`column_name`
                AND `information_schema`.`columns`.`table_schema` = `key_column_usage`.`table_schema`
            WHERE `information_schema`.`columns`.`table_name` = 'users'
            ORDER BY `information_schema`.`columns`.`ordinal_position` ASC
        "#;
        let actual = table_columns("users".to_string(), None).to_string(MysqlQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }

    #[test]
    fn test_table_columns_with_schema_mysql() {
        let expected = r#"
            SELECT
                `columns`.`column_name` AS `column_name`,
                `columns`.`data_type` AS `data_type`,
                `columns`.`is_nullable` AS `is_nullable`,
                `columns`.`column_default` AS `column_default`,
                (CASE WHEN (`columns`.`column_key` = 'PRI') THEN TRUE ELSE FALSE END) AS `is_primary_key`,
                `key_column_usage`.`referenced_table_schema` AS `referenced_table_schema`,
                `key_column_usage`.`referenced_table_name` AS `referenced_table_name`,
                `key_column_usage`.`referenced_column_name` AS `referenced_column_name`
            FROM `information_schema`.`columns`
            LEFT JOIN (SELECT
                `key_column_usage`.`column_name`,
                `key_column_usage`.`table_name`,
                `key_column_usage`.`table_schema`,
                `key_column_usage`.`referenced_table_schema` AS `referenced_table_schema`,
                `key_column_usage`.`referenced_table_name` AS `referenced_table_name`,
                `key_column_usage`.`referenced_column_name` AS `referenced_column_name`
            FROM `information_schema`.`table_constraints`
            INNER JOIN `information_schema`.`key_column_usage`
                ON `information_schema`.`table_constraints`.`constraint_name` = `information_schema`.`key_column_usage`.`constraint_name`
                AND `information_schema`.`table_constraints`.`table_schema` = `information_schema`.`key_column_usage`.`table_schema`
            WHERE `information_schema`.`table_constraints`.`constraint_type` = 'FOREIGN KEY') AS `key_column_usage`
                ON `information_schema`.`columns`.`table_name` = `key_column_usage`.`table_name`
                AND `information_schema`.`columns`.`column_name` = `key_column_usage`.`column_name`
                AND `information_schema`.`columns`.`table_schema` = `key_column_usage`.`table_schema`
            WHERE `information_schema`.`columns`.`table_name` = 'users'
                AND `information_schema`.`columns`.`table_schema` = 'my_db'
            ORDER BY `information_schema`.`columns`.`ordinal_position` ASC
        "#;
        let actual = table_columns("users".to_string(), Some("my_db".to_string()))
            .to_string(MysqlQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }
}
