use crate::handlers::{api::tables::get_tables, spa::serve_spa};
use crate::sql::DatabaseConfig;
use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

/// Create a DevUI router with all the development tools routes
///
/// This router handles:
/// - `/assets/*path` - Static assets (JS, CSS, etc.)
/// - `/api/tables` - Database tables API (only if db_config is provided)
/// - `/*path` - SPA fallback (catch-all for React app)
///
/// # Arguments
/// * `db_config` - Optional database configuration for API endpoints
///
/// # Example
/// ```rust
/// use devui::devui_router;
/// use axum::Router;
///
/// let app = Router::new()
///     .nest("/dev/ui", devui_router(Some(db_config)));
/// ```
pub fn devui_router(db_config: Option<DatabaseConfig>) -> Router {
    match db_config {
        Some(config) => {
            Router::new()
                .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
                .route("/api/tables", get(get_tables))
                .route("/*path", get(serve_spa))
                .with_state(Arc::new(config))
        }
        None => {
            Router::new()
                .nest_service("/assets", ServeDir::new("frontend/dist/assets"))
                .route("/*path", get(serve_spa))
        }
    }
}

/// Create a DevUI router without database configuration
///
/// This is a convenience function for when database features are not needed.
pub fn devui_router_simple() -> Router {
    devui_router(None)
}
