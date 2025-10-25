use derive_builder::Builder;
use crate::SqlConfig;

#[derive(Builder)]
pub struct DevUIConfig {
    pub sql_config: SqlConfig,
}