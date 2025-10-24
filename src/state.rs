use crate::services::sql::Config;
use crate::services::sql::connection_manager::ConnectionManager;

#[derive(Clone)]
pub struct SqlState {
    pub config: Config,
    pub connection_manager: ConnectionManager,
}

#[derive(Clone)]
pub struct DevUIState {
    pub sql_state: SqlState,
}