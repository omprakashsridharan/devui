use sqlx::{Pool, Postgres};
use crate::handlers::api::sql::DatabaseType;

#[derive(Clone)]
pub enum ConnectionPool {
    Postgres(Pool<Postgres>),
}

impl Into<DatabaseType> for ConnectionPool {
    fn into(self) -> DatabaseType {
        match self {
            ConnectionPool::Postgres(_) => DatabaseType::POSTGRES,
        }
    }
}