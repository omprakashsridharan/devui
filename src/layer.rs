use crate::router::devui_router;
use crate::sql::DatabaseConfig;

/// Helper functions for integrating DevUI with Axum routers.
///
/// This module provides utilities to add DevUI routes to existing Axum applications.
/// The recommended approach is to use `Router::nest("/dev/ui", devui_router())` directly.
pub struct DevUiIntegration;

impl DevUiIntegration {
    /// Create a DevUI router with database configuration.
    ///
    /// This is a convenience function that creates a router with all DevUI routes
    /// and the specified database configuration.
    pub fn with_database_config(config: DatabaseConfig) -> axum::Router {
        devui_router(Some(config))
    }

    /// Create a DevUI router without database configuration.
    ///
    /// This creates a router with DevUI routes but no database features.
    pub fn simple() -> axum::Router {
        devui_router(None)
    }
}

/// Legacy DevUiLayer for backward compatibility.
///
/// This is kept for backward compatibility but the recommended approach
/// is to use `Router::nest("/dev/ui", devui_router())` directly.
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

    /// Get the DevUI router for this layer.
    ///
    /// This method returns the router that should be nested at `/dev/ui`.
    pub fn router(&self) -> axum::Router {
        devui_router(self.db_config.clone())
    }
}

impl Default for DevUiLayer {
    fn default() -> Self {
        Self::new()
    }
}
