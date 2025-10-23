use crate::middleware::DevUiService;
use crate::sql::DatabaseConfig;
use tower::Layer;

/// A Tower layer that wraps services with the DevUI middleware.
/// This layer intercepts `/dev/ui` requests and renders the DevUI page.
#[derive(Clone)]
pub struct DevUiLayer {
    db_config: Option<DatabaseConfig>,
}

impl DevUiLayer {
    /// Create a new DevUI layer.
    pub fn new() -> Self {
        Self {
            db_config: None,
        }
    }

    /// Create a new DevUI layer with database configuration.
    pub fn with_database_config(config: DatabaseConfig) -> Self {
        Self {
            db_config: Some(config),
        }
    }
}

impl<S> Layer<S> for DevUiLayer {
    type Service = DevUiService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        if let Some(config) = &self.db_config {
            DevUiService::new(inner).with_database_config(config.clone())
        } else {
            DevUiService::new(inner)
        }
    }
}

impl Default for DevUiLayer {
    fn default() -> Self {
        Self::new()
    }
}
