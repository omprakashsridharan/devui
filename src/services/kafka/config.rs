#[derive(Clone)]
pub struct Config {
    pub cluster_configs: Vec<ClusterConfig>,
}

#[derive(Clone)]
pub struct ClusterConfig {
    pub name: String,
    pub bootstrap_servers: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            cluster_configs: Vec::new(),
        }
    }

    pub fn with_cluster(mut self, cluster: ClusterConfig) -> Self {
        self.cluster_configs.push(cluster);
        self
    }
}
