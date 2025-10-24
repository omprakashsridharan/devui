use crate::state::{DevUIState, SqlState};
use axum::extract::FromRef;
use std::sync::Arc;

pub struct SqlService {
    pub sql_state: Option<SqlState>,
}

impl FromRef<Arc<DevUIState>> for SqlService {
    fn from_ref(dev_ui_state: &Arc<DevUIState>) -> Self {
        if dev_ui_state.sql_state.is_none() {
            SqlService {
                sql_state: None,
            }
        } else {
            SqlService {
                sql_state: dev_ui_state.sql_state.clone(),
            }
        }

    }
}