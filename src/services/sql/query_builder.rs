use sea_query::BinOper::As;
use sea_query::JoinOn::Condition;
use sea_query::JoinType::LeftJoin;
use sea_query::Order::Asc;
use sea_query::{
    all, Cond, Expr, ExprTrait, Iden, OrderedStatement, PostgresQueryBuilder, Query, QueryBuilder,
    SelectStatement,
};

#[derive(Iden)]
pub enum Tables {
    Table,
    TableName,
    TableSchema,
}

#[derive(Iden)]
pub enum Columns {
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
pub enum TableConstraints {
    Table,
    ConstraintName,
    ConstraintType,
    TableSchema,
}

#[derive(Iden)]
pub enum KeyColumnUsage {
    Table,
    ConstraintName,
    TableName,
    ColumnName,
    TableSchema,
}

#[derive(Iden)]
pub enum ConstraintColumnUsage {
    Table,
    TableSchema,
    TableName,
    ColumnName,
    ConstraintName,
}

#[derive(Iden)]
pub enum Pk {
    Table,
    TableName,
    ColumnName,
    TableSchema,
}

#[derive(Iden)]
pub enum Fk {
    Table,
    TableName,
    ColumnName,
    TableSchema,
    ReferencedTableSchema,
    ReferencedTableName,
    ReferencedColumnName,
}

pub fn information_schema() -> String {
    "".to_string()
}

pub fn primary_key_constraint() -> SelectStatement {
    Query::select()
        .column((KeyColumnUsage::Table, KeyColumnUsage::TableName))
        .column((KeyColumnUsage::Table, KeyColumnUsage::ColumnName))
        .column((KeyColumnUsage::Table, KeyColumnUsage::TableSchema))
        .from(("information_schema", TableConstraints::Table))
        .inner_join(
            ("information_schema", KeyColumnUsage::Table),
            Expr::col((
                "information_schema",
                TableConstraints::Table,
                TableConstraints::ConstraintName,
            ))
            .equals((
                "information_schema",
                KeyColumnUsage::Table,
                KeyColumnUsage::ConstraintName,
            )),
        )
        .and_where(
            Expr::col((
                "information_schema",
                TableConstraints::Table,
                TableConstraints::ConstraintType,
            ))
            .equals("PRIMARY KEY"),
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
            "referenced_table_schema",
        )
        .expr_as(
            Expr::col((
                ConstraintColumnUsage::Table,
                ConstraintColumnUsage::TableName,
            )),
            "referenced_table_name",
        )
        .expr_as(
            Expr::col((
                ConstraintColumnUsage::Table,
                ConstraintColumnUsage::ColumnName,
            )),
            "referenced_column_name",
        )
        .from(("information_schema", TableConstraints::Table))
        .inner_join(
            ("information_schema", KeyColumnUsage::Table),
            all![
                Expr::col((
                    "information_schema",
                    TableConstraints::Table,
                    TableConstraints::ConstraintName,
                ))
                .equals((
                    "information_schema",
                    KeyColumnUsage::Table,
                    KeyColumnUsage::ConstraintName,
                )),
                Expr::col((
                    "information_schema",
                    TableConstraints::Table,
                    TableConstraints::TableSchema,
                ))
                .equals((
                    "information_schema",
                    KeyColumnUsage::Table,
                    KeyColumnUsage::TableSchema,
                ))
            ],
        )
        .inner_join(
            ("information_schema", ConstraintColumnUsage::Table),
            all![
                Expr::col((
                    "information_schema",
                    TableConstraints::Table,
                    TableConstraints::ConstraintName,
                ))
                .equals((
                    "information_schema",
                    ConstraintColumnUsage::Table,
                    ConstraintColumnUsage::ConstraintName,
                )),
                Expr::col((
                    "information_schema",
                    TableConstraints::Table,
                    TableConstraints::TableSchema,
                ))
                .equals((
                    "information_schema",
                    ConstraintColumnUsage::Table,
                    ConstraintColumnUsage::TableSchema,
                ))
            ],
        )
        .and_where(
            Expr::col((
                "information_schema",
                TableConstraints::Table,
                TableConstraints::ConstraintType,
            ))
            .equals("FOREIGN KEY"),
        )
        .to_owned()
}

pub fn table_data() -> SelectStatement {
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
        .from(("information_schema", TableConstraints::Table))
        .left_join(
            ("information_schema", Columns::Table),
            all![
                Expr::col(("information_schema", Tables::Table, Tables::TableName,)).equals((
                    "information_schema",
                    Columns::Table,
                    Columns::TableName,
                )),
                Expr::col(("information_schema", Tables::Table, Tables::TableSchema,)).equals((
                    "information_schema",
                    Columns::Table,
                    Columns::TableSchema,
                ))
            ],
        )
        .join_subquery(
            LeftJoin,
            primary_key_constraint(),
            Pk::Table,
            all![
                Expr::col(("information_schema", Columns::Table, Columns::TableName))
                    .equals((Pk::Table, Pk::TableName)),
                Expr::col(("information_schema", Columns::Table, Columns::ColumnName))
                    .equals((Pk::Table, Pk::ColumnName)),
                Expr::col(("information_schema", Columns::Table, Columns::TableSchema))
                    .equals((Pk::Table, Pk::TableSchema)),
            ],
        )
        .join_subquery(
            LeftJoin,
            foreign_key_constraint(),
            Fk::Table,
            all![
                Expr::col(("information_schema", Columns::Table, Columns::TableName))
                    .equals((Fk::Table, Fk::TableName)),
                Expr::col(("information_schema", Columns::Table, Columns::ColumnName))
                    .equals((Fk::Table, Fk::ColumnName)),
                Expr::col(("information_schema", Columns::Table, Columns::TableSchema))
                    .equals((Fk::Table, Fk::TableSchema)),
            ],
        )
        .and_where(
            Expr::col(("information_schema", Tables::Table, Tables::TableSchema))
                .is_not_in(["information_schema"]),
        )
        .order_by_columns(vec![
            (
                ("information_schema", Tables::Table, Tables::TableSchema),
                Asc,
            ),
            (
                ("information_schema", Tables::Table, Tables::TableName),
                Asc,
            ),
        ])
        .order_by(
            (
                "information_schema",
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

    #[test]
    fn test_primary_key_constraint_postgres() {
        let expected = r#"SELECT "key_column_usage"."table_name", "key_column_usage"."column_name", "key_column_usage"."table_schema" FROM "information_schema"."table_constraints" INNER JOIN "information_schema"."key_column_usage" ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name" WHERE "information_schema"."table_constraints"."constraint_type" = "PRIMARY KEY""#;
        assert_eq!(
            primary_key_constraint().to_string(PostgresQueryBuilder),
            expected
        );
    }

    #[test]
    fn test_foreign_key_constraint_postgres() {
        let expected = r#"SELECT "key_column_usage"."column_name", "key_column_usage"."table_name", "key_column_usage"."table_schema", "constraint_column_usage"."table_schema" AS "referenced_table_schema", "constraint_column_usage"."table_name" AS "referenced_table_name", "constraint_column_usage"."column_name" AS "referenced_column_name" FROM "information_schema"."table_constraints" INNER JOIN "information_schema"."key_column_usage" ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name" AND "information_schema"."table_constraints"."table_schema" = "information_schema"."key_column_usage"."table_schema" INNER JOIN "information_schema"."constraint_column_usage" ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."constraint_column_usage"."constraint_name" AND "information_schema"."table_constraints"."table_schema" = "information_schema"."constraint_column_usage"."table_schema" WHERE "information_schema"."table_constraints"."constraint_type" = "FOREIGN KEY""#;
        assert_eq!(
            foreign_key_constraint().to_string(PostgresQueryBuilder),
            expected
        );
    }
    //
    // #[test]
    // fn test_information_schema() {
    //     let expected = r#"
    //         SELECT c.column_name, c.data_type, c.udt_name, c.is_nullable, c.column_default, CASE WHEN pk.column_name IS NOT NULL THEN true ELSE false END as is_primary_key, fk.referenced_table_schema, fk.referenced_table_name, fk.referenced_column_name
    //         FROM information_schema.columns c
    //         // LEFT JOIN (
    //         //     SELECT ku.table_name, ku.column_name, ku.table_schema
    //         //     FROM information_schema.table_constraints tc
    //         //     JOIN information_schema.key_column_usage ku ON tc.constraint_name = ku.constraint_name
    //         //     WHERE tc.constraint_type = 'PRIMARY KEY'
    //         // ) pk ON c.table_name = pk.table_name
    //         //     AND c.column_name = pk.column_name
    //         //     AND c.table_schema = pk.table_schema
    //         // LEFT JOIN (
    //         //     SELECT
    //         //         kcu.column_name,
    //         //         kcu.table_name,
    //         //         kcu.table_schema,
    //         //         ccu.table_schema AS referenced_table_schema,
    //         //         ccu.table_name AS referenced_table_name,
    //         //         ccu.column_name AS referenced_column_name
    //         //     FROM information_schema.table_constraints AS tc
    //         //     JOIN information_schema.key_column_usage AS kcu
    //         //         ON tc.constraint_name = kcu.constraint_name
    //         //         AND tc.table_schema = kcu.table_schema
    //         //     JOIN information_schema.constraint_column_usage AS ccu
    //         //         ON ccu.constraint_name = tc.constraint_name
    //         //         AND ccu.table_schema = tc.table_schema
    //         //     WHERE tc.constraint_type = 'FOREIGN KEY'
    //         // ) fk ON c.table_name = fk.table_name
    //         //     AND c.column_name = fk.column_name
    //         //     AND c.table_schema = fk.table_schema
    //         // WHERE c.table_name = '{}'
    //         // {}
    //         // ORDER BY c.ordinal_position
    //         "#;
    //     assert_eq!(
    //         information_schema(),
    //         expected.replace("\n", " ").replace("\t", " ").trim()
    //     );
    // }
}
