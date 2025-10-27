use crate::services::kafka::cluster_manager::{ClientManager, ClusterManagerError};
use crate::services::kafka::config::Config;
use crate::services::kafka::models::{Broker, ClusterMetadata, Partition, Topic};
use rdkafka::consumer::Consumer;
use rdkafka::error::KafkaError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;

#[derive(Clone)]
pub struct Service {
    pub cluster_manager: Arc<ClientManager>,
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("client manager error")]
    ClientManagerError(#[from] ClusterManagerError),
    #[error("metadata fetch error")]
    MetadataFetchError(#[from] KafkaError),
}

impl Service {
    pub fn new(configs: Config) -> Result<Self, ServiceError> {
        let client_manager =
            ClientManager::new(configs).map_err(ServiceError::ClientManagerError)?;
        Ok(Self {
            cluster_manager: Arc::new(client_manager),
        })
    }

    pub fn get_clusters(&self) -> Vec<String> {
        self.cluster_manager.get_clusters()
    }

    pub fn metadata(&self, cluster_name: String) -> Result<ClusterMetadata, ServiceError> {
        let base_consumer = self
            .cluster_manager
            .get_cluster_base_consumer(&cluster_name)
            .map_err(ServiceError::ClientManagerError)?;

        let metadata = base_consumer
            .fetch_metadata(None, Duration::from_secs(5))
            .map_err(ServiceError::MetadataFetchError)?;

        let mut brokers = HashMap::new();
        let mut topics = Vec::new();

        for broker in metadata.brokers() {
            brokers.insert(
                broker.id().to_string(),
                Broker {
                    id: broker.id(),
                    host: broker.host().to_string(),
                    port: broker.port(),
                },
            );
        }

        for topic in metadata.topics() {
            let mut partitions = Vec::new();
            for partition in topic.partitions() {
                partitions.push(Partition {
                    partition: partition.id(),
                    leader_id: partition.leader(),
                });
            }
            topics.push(Topic {
                name: topic.name().to_string(),
                partitions,
            })
        }

        let cluster_metadata = ClusterMetadata { brokers, topics };

        Ok(cluster_metadata)
    }
}
