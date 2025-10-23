//! DevUI - Axum-native development tools UI
//!
//! This crate provides Axum routers and utilities for adding development tools UI
//! to any Axum application. It serves a React-based development interface at `/dev/ui`.

pub mod components;
pub mod handlers;
pub mod layer;
pub mod router;
pub mod sql;

// Re-export the main types for easy access
pub use layer::{DevUiLayer, DevUiIntegration};
pub use router::{devui_router, devui_router_simple};

/// Create a DevUI router with database configuration.
///
/// This is the recommended way to integrate DevUI into your Axum application.
///
/// # Example
///
/// ```rust
/// use devui::{devui_router, sql::DatabaseConfig};
/// use axum::Router;
///
/// let app = Router::new()
///     .nest("/dev/ui", devui_router(Some(db_config)));
/// ```
pub fn dev_ui_router(db_config: Option<crate::sql::DatabaseConfig>) -> axum::Router {
    devui_router(db_config)
}

/// Create a simple DevUI router without database features.
///
/// # Example
///
/// ```rust
/// use devui::dev_ui_router_simple;
/// use axum::Router;
///
/// let app = Router::new()
///     .nest("/dev/ui", dev_ui_router_simple());
/// ```
pub fn dev_ui_router_simple() -> axum::Router {
    devui_router_simple()
}

/// Legacy function for backward compatibility.
///
/// This is kept for backward compatibility but the recommended approach
/// is to use `dev_ui_router()` directly.
#[deprecated(note = "Use dev_ui_router() instead")]
pub fn dev_ui_layer() -> DevUiLayer {
    DevUiLayer::new()
}
