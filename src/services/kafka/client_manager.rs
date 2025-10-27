use crate::services::kafka::config::Config;
use rdkafka::admin::AdminClient;
use rdkafka::client::DefaultClientContext;
use rdkafka::error::KafkaError;
use rdkafka::ClientConfig;
use std::collections::HashMap;
use thiserror::Error;

pub struct ClientManager {
    clients: HashMap<String, AdminClient<DefaultClientContext>>,
}

#[derive(Error, Debug)]
pub enum ClientManagerError {
    #[error("rdkafka error")]
    KafkaLibError(#[from] KafkaError),
    #[error("client with name \"{0}\" already exists")]
    ClientWithNameExists(String),
}

impl ClientManager {
    pub fn new(configs: Config) -> Result<Self, ClientManagerError> {
        let mut clients = HashMap::new();
        for cluster_config in configs.cluster_configs {
            if clients.contains_key(&cluster_config.name) {
                return Err(ClientManagerError::ClientWithNameExists(
                    cluster_config.name,
                ));
            } else {
                let client: AdminClient<DefaultClientContext> = ClientConfig::new()
                    .set(
                        "bootstrap.servers",
                        cluster_config.bootstrap_servers.as_str(),
                    )
                    .set("message.timeout.ms", "5000")
                    .create()
                    .map_err(ClientManagerError::KafkaLibError)?;
                clients.insert(cluster_config.name.clone(), client);
            }
        }
        Ok(Self { clients })
    }
}
