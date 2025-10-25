use std::sync::Arc;
use axum::extract::FromRef;
use crate::services::sql::service::Service as SqlService;

#[derive(Clone)]
pub struct SqlServiceState(pub(crate) SqlService);

impl FromRef<Arc<DevUIState>> for SqlServiceState {
    fn from_ref(dev_ui_state: &Arc<DevUIState>) -> Self {
        SqlServiceState(dev_ui_state.sql_service.clone())
    }
}

#[derive(Clone)]
pub struct DevUIState {
    pub sql_service: SqlService,
}

