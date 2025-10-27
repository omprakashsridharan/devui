#[derive(Clone)]
pub struct Config {
    pub cluster_configs: Vec<ClusterConfig>,
}

#[derive(Clone)]
pub struct ClusterConfig {
    pub name: String,
    pub bootstrap_servers: String,
}
