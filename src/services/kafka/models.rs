use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClusterMetadata {
    pub brokers: HashMap<String, Broker>,
    pub topics: Vec<Topic>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Broker {
    pub id: i32,
    pub host: String,
    pub port: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Topic {
    pub name: String,
    pub partitions: Vec<Partition>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Partition {
    pub partition: i32,
    pub leader_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConsumeMessage {
    pub key: Option<String>,
    pub value: Option<String>,
}
