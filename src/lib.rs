//! DevUI - A Tower middleware for adding development tools UI to any HTTP service
//!
//! This crate provides a Tower layer that intercepts `/dev/ui` requests and serves
//! a React-based development tools interface.

pub mod components;
pub mod layer;
pub mod middleware;
pub mod sql;

// Re-export the main types for easy access
pub use layer::DevUiLayer;
pub use middleware::DevUiService;

/// Create a new DevUI layer that can be applied to any Tower service.
///
/// # Example
///
/// ```rust
/// use devui::DevUiLayer;
/// use tower::ServiceBuilder;
///
/// let layer = ServiceBuilder::new()
///     .layer(DevUiLayer::new())
///     .into_inner();
/// ```
pub fn dev_ui_layer() -> DevUiLayer {
    DevUiLayer::new()
}
