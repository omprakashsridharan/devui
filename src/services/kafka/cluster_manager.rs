use crate::services::kafka::config::Config;
use rdkafka::consumer::BaseConsumer;
use rdkafka::error::KafkaError;
use rdkafka::producer::{BaseProducer, FutureProducer};
use rdkafka::ClientConfig;
use std::collections::HashMap;
use thiserror::Error;

pub struct ClientManager {
    base_consumers: HashMap<String, BaseConsumer>,
    producers: HashMap<String, FutureProducer>,
}

#[derive(Error, Debug)]
pub enum ClusterManagerError {
    #[error("rdkafka error")]
    KafkaLibError(#[from] KafkaError),
    #[error("client with name \"{0}\" already exists")]
    ClusterWithNameExists(String),
    #[error("consumer for cluster with name \"{0}\" does not exist")]
    ClusterConsumerNotFound(String),
    #[error("producer for cluster with name \"{0}\" does not exist")]
    ClusterProducerNotFound(String),
}

impl ClientManager {
    pub fn new(configs: Config) -> Result<Self, ClusterManagerError> {
        let mut base_consumers = HashMap::new();
        let mut producers = HashMap::new();
        for cluster_config in configs.cluster_configs {
            if base_consumers.contains_key(&cluster_config.name) {
                return Err(ClusterManagerError::ClusterWithNameExists(
                    cluster_config.name,
                ));
            } else {
                let future_producer = ClientConfig::new()
                    .set(
                        "bootstrap.servers",
                        cluster_config.bootstrap_servers.as_str(),
                    )
                    .create()
                    .map_err(ClusterManagerError::KafkaLibError)?;
                producers.insert(cluster_config.name.clone(), future_producer);
                let base_consumer: BaseConsumer = ClientConfig::new()
                    .set(
                        "bootstrap.servers",
                        cluster_config.bootstrap_servers.as_str(),
                    )
                    .create()
                    .map_err(ClusterManagerError::KafkaLibError)?;

                base_consumers.insert(cluster_config.name.clone(), base_consumer);
                tracing::info!(
                    "client for cluster with name \"{0}\" bootstrap servers \"{1}\" created",
                    cluster_config.name,
                    cluster_config.bootstrap_servers
                );
            }
        }
        Ok(Self {
            base_consumers,
            producers,
        })
    }

    pub fn get_clusters(&self) -> Vec<String> {
        self.base_consumers.keys().cloned().collect()
    }

    pub fn get_cluster_base_consumer(
        &self,
        name: &str,
    ) -> Result<&BaseConsumer, ClusterManagerError> {
        self.base_consumers
            .get(name)
            .ok_or(ClusterManagerError::ClusterConsumerNotFound(
                name.to_string(),
            ))
    }

    pub fn get_cluster_producer(&self, name: &str) -> Result<&FutureProducer, ClusterManagerError> {
        self.producers
            .get(name)
            .ok_or(ClusterManagerError::ClusterProducerNotFound(
                name.to_string(),
            ))
    }
}
