use sea_query::JoinType::LeftJoin;
use sea_query::Order::Asc;
use sea_query::{all, Cond, Expr, ExprTrait, Iden, Query, SelectStatement};

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
    UdtName,
    IsNullable,
    ColumnDefault,
    OrdinalPosition,
}

#[derive(Iden)]
enum TableConstraints {
    Table,
    ConstraintName,
    ConstraintType,
    TableSchema,
}

#[derive(Iden)]
enum KeyColumnUsage {
    Table,
    ConstraintName,
    TableName,
    ColumnName,
    TableSchema,
}

#[derive(Iden)]
enum ConstraintColumnUsage {
    Table,
    TableSchema,
    TableName,
    ColumnName,
    ConstraintName,
}

#[derive(Iden)]
enum Pk {
    Table,
    TableName,
    ColumnName,
    TableSchema,
}

#[derive(Iden)]
enum Fk {
    Table,
    TableName,
    ColumnName,
    TableSchema,
    ReferencedTableSchema,
    ReferencedTableName,
    ReferencedColumnName,
}

#[derive(Iden)]
enum InformationSchema {
    Table,
}

#[derive(Iden)]
enum PgCatalog {
    Table,
}

#[derive(Iden)]
enum PgType {
    Table,
    Oid,
    Typnamespace,
    Typname,
}

#[derive(Iden)]
enum PgEnum {
    Table,
    Enumlabel,
    Enumtypid,
    Enumsortorder,
}

#[derive(Iden)]
enum PgNamespace {
    Table,
    Oid,
    Nspname,
}

pub fn primary_key_constraint() -> SelectStatement {
    Query::select()
        .column((KeyColumnUsage::Table, KeyColumnUsage::TableName))
        .column((KeyColumnUsage::Table, KeyColumnUsage::ColumnName))
        .column((KeyColumnUsage::Table, KeyColumnUsage::TableSchema))
        .from((InformationSchema::Table, TableConstraints::Table))
        .inner_join(
            (InformationSchema::Table, KeyColumnUsage::Table),
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
        )
        .and_where(
            Expr::col((
                InformationSchema::Table,
                TableConstraints::Table,
                TableConstraints::ConstraintType,
            ))
            .eq("PRIMARY KEY"),
        )
        .to_owned()
}

pub fn foreign_key_constraint() -> SelectStatement {
    Query::select()
        .column((KeyColumnUsage::Table, KeyColumnUsage::ColumnName))
        .column((KeyColumnUsage::Table, KeyColumnUsage::TableName))
        .column((KeyColumnUsage::Table, KeyColumnUsage::TableSchema))
        .expr_as(
            Expr::col((
                ConstraintColumnUsage::Table,
                ConstraintColumnUsage::TableSchema,
            )),
            Fk::ReferencedTableSchema,
        )
        .expr_as(
            Expr::col((
                ConstraintColumnUsage::Table,
                ConstraintColumnUsage::TableName,
            )),
            Fk::ReferencedTableName,
        )
        .expr_as(
            Expr::col((
                ConstraintColumnUsage::Table,
                ConstraintColumnUsage::ColumnName,
            )),
            Fk::ReferencedColumnName,
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
        .inner_join(
            (InformationSchema::Table, ConstraintColumnUsage::Table),
            all![
                Expr::col((
                    InformationSchema::Table,
                    TableConstraints::Table,
                    TableConstraints::ConstraintName,
                ))
                .equals((
                    InformationSchema::Table,
                    ConstraintColumnUsage::Table,
                    ConstraintColumnUsage::ConstraintName,
                )),
                Expr::col((
                    InformationSchema::Table,
                    TableConstraints::Table,
                    TableConstraints::TableSchema,
                ))
                .equals((
                    InformationSchema::Table,
                    ConstraintColumnUsage::Table,
                    ConstraintColumnUsage::TableSchema,
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
        .column((Columns::Table, Columns::ColumnName))
        .column((Columns::Table, Columns::DataType))
        .column((Columns::Table, Columns::UdtName))
        .column((Columns::Table, Columns::IsNullable))
        .column((Columns::Table, Columns::ColumnDefault))
        .expr_as(
            Expr::case(Expr::col((Pk::Table, Pk::ColumnName)).is_not_null(), true).finally(false),
            "is_primary_key",
        )
        .column((Fk::Table, Fk::ReferencedTableSchema))
        .column((Fk::Table, Fk::ReferencedTableName))
        .column((Fk::Table, Fk::ReferencedColumnName))
        .from((InformationSchema::Table, Columns::Table))
        .join_subquery(
            LeftJoin,
            primary_key_constraint(),
            Pk::Table,
            all![
                Expr::col((InformationSchema::Table, Columns::Table, Columns::TableName))
                    .equals((Pk::Table, Pk::TableName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::ColumnName
                ))
                .equals((Pk::Table, Pk::ColumnName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::TableSchema
                ))
                .equals((Pk::Table, Pk::TableSchema)),
            ],
        )
        .join_subquery(
            LeftJoin,
            foreign_key_constraint(),
            Fk::Table,
            all![
                Expr::col((InformationSchema::Table, Columns::Table, Columns::TableName))
                    .equals((Fk::Table, Fk::TableName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::ColumnName
                ))
                .equals((Fk::Table, Fk::ColumnName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::TableSchema
                ))
                .equals((Fk::Table, Fk::TableSchema)),
            ],
        )
        .to_owned()
}

/// Builds a query to fetch column information for a specific table.
/// This is the sea-query equivalent of the fetch_table_columns query in postgres.rs.
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
        .column((Tables::Table, Tables::TableName))
        .column((Tables::Table, Tables::TableSchema))
        .column((Columns::Table, Columns::ColumnName))
        .column((Columns::Table, Columns::DataType))
        .column((Columns::Table, Columns::UdtName))
        .column((Columns::Table, Columns::IsNullable))
        .column((Columns::Table, Columns::ColumnDefault))
        .expr_as(
            Expr::case(Expr::col((Pk::Table, Pk::ColumnName)).is_not_null(), true).finally(false),
            "is_primary_key",
        )
        .column((Fk::Table, Fk::ReferencedTableSchema))
        .column((Fk::Table, Fk::ReferencedTableName))
        .column((Fk::Table, Fk::ReferencedColumnName))
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
            primary_key_constraint(),
            Pk::Table,
            all![
                Expr::col((InformationSchema::Table, Columns::Table, Columns::TableName))
                    .equals((Pk::Table, Pk::TableName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::ColumnName
                ))
                .equals((Pk::Table, Pk::ColumnName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::TableSchema
                ))
                .equals((Pk::Table, Pk::TableSchema)),
            ],
        )
        .join_subquery(
            LeftJoin,
            foreign_key_constraint(),
            Fk::Table,
            all![
                Expr::col((InformationSchema::Table, Columns::Table, Columns::TableName))
                    .equals((Fk::Table, Fk::TableName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::ColumnName
                ))
                .equals((Fk::Table, Fk::ColumnName)),
                Expr::col((
                    InformationSchema::Table,
                    Columns::Table,
                    Columns::TableSchema
                ))
                .equals((Fk::Table, Fk::TableSchema)),
            ],
        )
        .and_where(
            Expr::col((InformationSchema::Table, Tables::Table, Tables::TableSchema)).is_not_in([
                InformationSchema::Table.to_string(),
                PgCatalog::Table.to_string(),
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

pub fn enum_values(enum_name: String, schema: Option<String>) -> SelectStatement {
    let mut conditions = Cond::all().add(Expr::col((PgType::Table, PgType::Typname)).eq(enum_name));
    if let Some(schema) = schema {
        conditions =
            conditions.add(Expr::col((PgNamespace::Table, PgNamespace::Nspname)).eq(schema));
    }
    Query::select()
        .expr_as(
            Expr::col((PgEnum::Table, PgEnum::Enumlabel)),
            "enum_value",
        )
        .from((PgCatalog::Table, PgType::Table))
        .inner_join(
            (PgCatalog::Table, PgEnum::Table),
            Expr::col((PgEnum::Table, PgEnum::Enumtypid)).equals((PgType::Table, PgType::Oid)),
        )
        .inner_join(
            (PgCatalog::Table, PgNamespace::Table),
            Expr::col((PgNamespace::Table, PgNamespace::Oid))
                .equals((PgType::Table, PgType::Typnamespace)),
        )
        .cond_where(conditions)
        .order_by((PgEnum::Table, PgEnum::Enumsortorder), Asc)
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_query::{MysqlQueryBuilder, PostgresQueryBuilder};

    // Helper function to normalize SQL strings by removing whitespace and newlines
    fn normalize_sql(sql: &str) -> String {
        sql.split_whitespace().collect::<Vec<&str>>().join(" ")
    }

    #[test]
    fn test_primary_key_constraint_postgres() {
        let expected = r#"
            SELECT
                "key_column_usage"."table_name",
                "key_column_usage"."column_name",
                "key_column_usage"."table_schema"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'PRIMARY KEY'
        "#;
        let actual = primary_key_constraint().to_string(PostgresQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }

    #[test]
    fn test_foreign_key_constraint_postgres() {
        let expected = r#"
            SELECT
                "key_column_usage"."column_name",
                "key_column_usage"."table_name",
                "key_column_usage"."table_schema",
                "constraint_column_usage"."table_schema" AS "referenced_table_schema",
                "constraint_column_usage"."table_name" AS "referenced_table_name",
                "constraint_column_usage"."column_name" AS "referenced_column_name"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."key_column_usage"."table_schema"
            INNER JOIN "information_schema"."constraint_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."constraint_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."constraint_column_usage"."table_schema"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'FOREIGN KEY'
        "#;
        let actual = foreign_key_constraint().to_string(PostgresQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }

    #[test]
    fn test_tables_postgres() {
        let expected = r#"
            SELECT
                "tables"."table_name",
                "tables"."table_schema",
                "columns"."column_name",
                "columns"."data_type",
                "columns"."udt_name",
                "columns"."is_nullable",
                "columns"."column_default",
                (CASE WHEN ("pk"."column_name" IS NOT NULL) THEN TRUE ELSE FALSE END) AS "is_primary_key",
                "fk"."referenced_table_schema",
                "fk"."referenced_table_name",
                "fk"."referenced_column_name"
            FROM "information_schema"."tables"
            LEFT JOIN "information_schema"."columns"
                ON "information_schema"."tables"."table_name" = "information_schema"."columns"."table_name"
                AND "information_schema"."tables"."table_schema" = "information_schema"."columns"."table_schema"
            LEFT JOIN (SELECT
                "key_column_usage"."table_name",
                "key_column_usage"."column_name",
                "key_column_usage"."table_schema"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'PRIMARY KEY') AS "pk"
                ON "information_schema"."columns"."table_name" = "pk"."table_name"
                AND "information_schema"."columns"."column_name" = "pk"."column_name"
                AND "information_schema"."columns"."table_schema" = "pk"."table_schema"
            LEFT JOIN (SELECT
                "key_column_usage"."column_name",
                "key_column_usage"."table_name",
                "key_column_usage"."table_schema",
                "constraint_column_usage"."table_schema" AS "referenced_table_schema",
                "constraint_column_usage"."table_name" AS "referenced_table_name",
                "constraint_column_usage"."column_name" AS "referenced_column_name"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."key_column_usage"."table_schema"
            INNER JOIN "information_schema"."constraint_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."constraint_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."constraint_column_usage"."table_schema"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'FOREIGN KEY') AS "fk"
                ON "information_schema"."columns"."table_name" = "fk"."table_name"
                AND "information_schema"."columns"."column_name" = "fk"."column_name"
                AND "information_schema"."columns"."table_schema" = "fk"."table_schema"
            WHERE "information_schema"."tables"."table_schema" NOT IN ('information_schema')
            ORDER BY "information_schema"."tables"."table_schema" ASC, "information_schema"."tables"."table_name" ASC, "information_schema"."columns"."ordinal_position" ASC
        "#;
        let actual = tables().to_string(PostgresQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }

    #[test]
    fn test_tables_mysql() {
        let expected = r#"
            SELECT
                "tables"."table_name",
                "tables"."table_schema",
                "columns"."column_name",
                "columns"."data_type",
                "columns"."udt_name",
                "columns"."is_nullable",
                "columns"."column_default",
                (CASE WHEN ("pk"."column_name" IS NOT NULL) THEN TRUE ELSE FALSE END) AS "is_primary_key",
                "fk"."referenced_table_schema",
                "fk"."referenced_table_name",
                "fk"."referenced_column_name"
            FROM "information_schema"."tables"
            LEFT JOIN "information_schema"."columns"
                ON "information_schema"."tables"."table_name" = "information_schema"."columns"."table_name"
                AND "information_schema"."tables"."table_schema" = "information_schema"."columns"."table_schema"
            LEFT JOIN (SELECT
                "key_column_usage"."table_name",
                "key_column_usage"."column_name",
                "key_column_usage"."table_schema"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'PRIMARY KEY') AS "pk"
                ON "information_schema"."columns"."table_name" = "pk"."table_name"
                AND "information_schema"."columns"."column_name" = "pk"."column_name"
                AND "information_schema"."columns"."table_schema" = "pk"."table_schema"
            LEFT JOIN (SELECT
                "key_column_usage"."column_name",
                "key_column_usage"."table_name",
                "key_column_usage"."table_schema",
                "constraint_column_usage"."table_schema" AS "referenced_table_schema",
                "constraint_column_usage"."table_name" AS "referenced_table_name",
                "constraint_column_usage"."column_name" AS "referenced_column_name"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."key_column_usage"."table_schema"
            INNER JOIN "information_schema"."constraint_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."constraint_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."constraint_column_usage"."table_schema"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'FOREIGN KEY') AS "fk"
                ON "information_schema"."columns"."table_name" = "fk"."table_name"
                AND "information_schema"."columns"."column_name" = "fk"."column_name"
                AND "information_schema"."columns"."table_schema" = "fk"."table_schema"
            WHERE "information_schema"."tables"."table_schema" NOT IN ('information_schema', 'pg_catalog')
            ORDER BY "information_schema"."tables"."table_schema" ASC, "information_schema"."tables"."table_name" ASC, "information_schema"."columns"."ordinal_position" ASC
        "#;
        let actual = tables().to_string(MysqlQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }

    #[test]
    fn test_table_columns_postgres() {
        let expected = r#"
            SELECT
                "columns"."column_name",
                "columns"."data_type",
                "columns"."udt_name",
                "columns"."is_nullable",
                "columns"."column_default",
                (CASE WHEN ("pk"."column_name" IS NOT NULL) THEN TRUE ELSE FALSE END) AS "is_primary_key",
                "fk"."referenced_table_schema",
                "fk"."referenced_table_name",
                "fk"."referenced_column_name"
            FROM "information_schema"."columns"
            LEFT JOIN (SELECT
                "key_column_usage"."table_name",
                "key_column_usage"."column_name",
                "key_column_usage"."table_schema"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'PRIMARY KEY') AS "pk"
                ON "information_schema"."columns"."table_name" = "pk"."table_name"
                AND "information_schema"."columns"."column_name" = "pk"."column_name"
                AND "information_schema"."columns"."table_schema" = "pk"."table_schema"
            LEFT JOIN (SELECT
                "key_column_usage"."column_name",
                "key_column_usage"."table_name",
                "key_column_usage"."table_schema",
                "constraint_column_usage"."table_schema" AS "referenced_table_schema",
                "constraint_column_usage"."table_name" AS "referenced_table_name",
                "constraint_column_usage"."column_name" AS "referenced_column_name"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."key_column_usage"."table_schema"
            INNER JOIN "information_schema"."constraint_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."constraint_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."constraint_column_usage"."table_schema"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'FOREIGN KEY') AS "fk"
                ON "information_schema"."columns"."table_name" = "fk"."table_name"
                AND "information_schema"."columns"."column_name" = "fk"."column_name"
                AND "information_schema"."columns"."table_schema" = "fk"."table_schema"
            WHERE "information_schema"."columns"."table_name" = 'users'
            ORDER BY "information_schema"."columns"."ordinal_position" ASC
        "#;
        let actual = table_columns("users".to_string(), None).to_string(PostgresQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }

    #[test]
    fn test_table_columns_with_schema_postgres() {
        let expected = r#"
            SELECT
                "columns"."column_name",
                "columns"."data_type",
                "columns"."udt_name",
                "columns"."is_nullable",
                "columns"."column_default",
                (CASE WHEN ("pk"."column_name" IS NOT NULL) THEN TRUE ELSE FALSE END) AS "is_primary_key",
                "fk"."referenced_table_schema",
                "fk"."referenced_table_name",
                "fk"."referenced_column_name"
            FROM "information_schema"."columns"
            LEFT JOIN (SELECT
                "key_column_usage"."table_name",
                "key_column_usage"."column_name",
                "key_column_usage"."table_schema"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'PRIMARY KEY') AS "pk"
                ON "information_schema"."columns"."table_name" = "pk"."table_name"
                AND "information_schema"."columns"."column_name" = "pk"."column_name"
                AND "information_schema"."columns"."table_schema" = "pk"."table_schema"
            LEFT JOIN (SELECT
                "key_column_usage"."column_name",
                "key_column_usage"."table_name",
                "key_column_usage"."table_schema",
                "constraint_column_usage"."table_schema" AS "referenced_table_schema",
                "constraint_column_usage"."table_name" AS "referenced_table_name",
                "constraint_column_usage"."column_name" AS "referenced_column_name"
            FROM "information_schema"."table_constraints"
            INNER JOIN "information_schema"."key_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."key_column_usage"."table_schema"
            INNER JOIN "information_schema"."constraint_column_usage"
                ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."constraint_column_usage"."constraint_name"
                AND "information_schema"."table_constraints"."table_schema" = "information_schema"."constraint_column_usage"."table_schema"
            WHERE "information_schema"."table_constraints"."constraint_type" = 'FOREIGN KEY') AS "fk"
                ON "information_schema"."columns"."table_name" = "fk"."table_name"
                AND "information_schema"."columns"."column_name" = "fk"."column_name"
                AND "information_schema"."columns"."table_schema" = "fk"."table_schema"
            WHERE "information_schema"."columns"."table_name" = 'users'
                AND "information_schema"."columns"."table_schema" = 'public'
            ORDER BY "information_schema"."columns"."ordinal_position" ASC
        "#;
        let actual = table_columns("users".to_string(), Some("public".to_string()))
            .to_string(PostgresQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }

    #[test]
    fn test_enum_values_postgres() {
        let expected = r#"
            SELECT
                "pg_enum"."enumlabel" AS "enum_value"
            FROM "pg_catalog"."pg_type"
            INNER JOIN "pg_catalog"."pg_enum"
                ON "pg_enum"."enumtypid" = "pg_type"."oid"
            INNER JOIN "pg_catalog"."pg_namespace"
                ON "pg_namespace"."oid" = "pg_type"."typnamespace"
            WHERE "pg_type"."typname" = 'status_enum'
            ORDER BY "pg_enum"."enumsortorder" ASC
        "#;
        let actual = enum_values("status_enum".to_string(), None).to_string(PostgresQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }

    #[test]
    fn test_enum_values_with_schema_postgres() {
        let expected = r#"
            SELECT
                "pg_enum"."enumlabel" AS "enum_value"
            FROM "pg_catalog"."pg_type"
            INNER JOIN "pg_catalog"."pg_enum"
                ON "pg_enum"."enumtypid" = "pg_type"."oid"
            INNER JOIN "pg_catalog"."pg_namespace"
                ON "pg_namespace"."oid" = "pg_type"."typnamespace"
            WHERE "pg_type"."typname" = 'status_enum'
                AND "pg_namespace"."nspname" = 'public'
            ORDER BY "pg_enum"."enumsortorder" ASC
        "#;
        let actual = enum_values("status_enum".to_string(), Some("public".to_string()))
            .to_string(PostgresQueryBuilder);
        assert_eq!(normalize_sql(&actual), normalize_sql(expected));
    }
}
