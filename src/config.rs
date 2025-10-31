use crate::services::kafka::config::Config as KafkaConfig;
use crate::SqlConfig;
use derive_builder::Builder;

#[derive(Builder, Default, Clone)]
pub struct DevUIConfig {
    #[builder(default = "None")]
    pub sql_config: Option<SqlConfig>,
    #[builder(default = "None")]
    pub kafka_config: Option<KafkaConfig>,
}
