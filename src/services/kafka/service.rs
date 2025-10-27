use crate::services::kafka::client_manager::{ClientManager, ClientManagerError};
use crate::services::kafka::config::Config;
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone)]
pub struct Service {
    pub client_manager: Arc<ClientManager>,
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("client manager error")]
    ClientManagerError(#[from] ClientManagerError),
}

impl Service {
    pub fn new(configs: Config) -> Result<Self, ServiceError> {
        let client_manager =
            ClientManager::new(configs).map_err(ServiceError::ClientManagerError)?;
        Ok(Self {
            client_manager: Arc::new(client_manager),
        })
    }
}
