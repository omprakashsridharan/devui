use crate::services::kafka::config::Config;
use rdkafka::consumer::BaseConsumer;
use rdkafka::error::KafkaError;
use rdkafka::ClientConfig;
use std::collections::HashMap;
use thiserror::Error;

pub struct ClientManager {
    base_consumers: HashMap<String, BaseConsumer>,
}

#[derive(Error, Debug)]
pub enum ClientManagerError {
    #[error("rdkafka error")]
    KafkaLibError(#[from] KafkaError),
    #[error("client with name \"{0}\" already exists")]
    ClusterWithNameExists(String),
    #[error("client with name \"{0}\" does not exist")]
    ClusterNotFound(String),
}

impl ClientManager {
    pub fn new(configs: Config) -> Result<Self, ClientManagerError> {
        let mut base_consumers = HashMap::new();
        for cluster_config in configs.cluster_configs {
            if base_consumers.contains_key(&cluster_config.name) {
                return Err(ClientManagerError::ClusterWithNameExists(
                    cluster_config.name,
                ));
            } else {
                let base_consumer: BaseConsumer = ClientConfig::new()
                    .set(
                        "bootstrap.servers",
                        cluster_config.bootstrap_servers.as_str(),
                    )
                    .create()
                    .map_err(ClientManagerError::KafkaLibError)?;

                base_consumers.insert(cluster_config.name.clone(), base_consumer);
                tracing::info!(
                    "client with name \"{0}\" bootstrap servers \"{1}\" created",
                    cluster_config.name,
                    cluster_config.bootstrap_servers
                );
            }
        }
        Ok(Self { base_consumers })
    }

    pub fn get_cluster_base_consumer(&self, name: &str) -> Result<&BaseConsumer, ClientManagerError> {
        self.base_consumers
            .get(name)
            .ok_or(ClientManagerError::ClusterNotFound(name.to_string()))
    }
}
