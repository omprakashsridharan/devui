use crate::services::kafka::config::Config as KafkaConfig;
use crate::SqlConfig;
use derive_builder::Builder;

#[derive(Builder)]
pub struct DevUIConfig {
    pub sql_config: SqlConfig,
    pub kafka_config: KafkaConfig,
}
