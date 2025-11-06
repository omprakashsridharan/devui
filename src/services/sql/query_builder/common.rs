use sea_query::{Asterisk, Expr, Query, SelectStatement};


pub fn table_count(table_name: String) -> SelectStatement {
  Query::select()
      .expr(Expr::col(Asterisk).count())
      .from(table_name)
      .to_owned()
}

#[cfg(test)]
mod tests {

  use sea_query::PostgresQueryBuilder;

use super::*;

  #[test]
  fn test_table_count_postgres() {
      let expected = r#"SELECT COUNT(*) FROM "test""#;
      assert_eq!(
          table_count("test".to_string()).to_string(PostgresQueryBuilder),
          expected
      );
  }
}