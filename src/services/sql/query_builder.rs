use sea_query::{Expr, ExprTrait, Iden, PostgresQueryBuilder, Query, QueryBuilder};

#[derive(Iden)]
pub enum Tables {
    Table,
}

#[derive(Iden)]
pub enum Columns {
    Table,
    ColumnName,
}

#[derive(Iden)]
pub enum TableConstraints {
    Table,
    ConstraintName,
    ConstraintType
}

#[derive(Iden)]
pub enum KeyColumnUsage {
    Table,
    ConstraintName,
    TableName,
    ColumnName,
    TableSchema,
}

pub fn information_schema() -> String {
    "".to_string()
}

pub fn primary_key_constraint<T: QueryBuilder>(builder: T) -> String {
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
            )).equals("PRIMARY KEY"),
        )
        .to_string(builder)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_key_constraint() {
        let expected = r#"SELECT "key_column_usage"."table_name", "key_column_usage"."column_name", "key_column_usage"."table_schema" FROM "information_schema"."table_constraints" INNER JOIN "information_schema"."key_column_usage" ON "information_schema"."table_constraints"."constraint_name" = "information_schema"."key_column_usage"."constraint_name" WHERE "information_schema"."table_constraints"."constraint_type" = "PRIMARY KEY""#;
        assert_eq!(
            primary_key_constraint(PostgresQueryBuilder),
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
