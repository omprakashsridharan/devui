use axum::{response::Html, routing::get, Router};
use devui::{
    dev_ui_router, DevUIConfigBuilder, PostgresConfig, SqlConfig,
};
use tower_http::{
    cors::{AllowOrigin, AllowMethods, AllowHeaders, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    // Initialize tracing with DEBUG level by default
    // You can override via RUST_LOG environment variable: RUST_LOG=info, RUST_LOG=debug, etc.
    // Examples:
    //   RUST_LOG=debug - Show all debug logs
    //   RUST_LOG=info - Show info and above
    //   RUST_LOG=devui::services::sql=debug - Show debug only for SQL services
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create database configuration with sample database from docker-compose
    // Make sure to start the database first: docker-compose up -d
    let sql_config = SqlConfig::new()
        .with_postgres(
            "sample-db".to_string(),
            PostgresConfig {
                host: "localhost".to_string(),
                port: 5432,
                database: "devui_sample_db".to_string(),
                username: "devui_user".to_string(),
                password: "devui_password".to_string(),
                ssl_mode: Some("disable".to_string()),
            },
        );

    let dev_ui_config = DevUIConfigBuilder::default()
        .sql_config(Some(sql_config))
        .build()
        .expect("Error building dev ui config");

    let dev_ui_router = dev_ui_router(dev_ui_config)
        .await
        .expect("error constructing dev_ui_router");

    // Create a basic Axum router with some example routes
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Welcome to DevUI Dev Server!</h1><p>Visit <a href='/dev/ui'>/dev/ui</a> for development tools.</p>") }))
        .route("/api/health", get(|| async { "OK" }))
        .route("/api/status", get(|| async { "{\"status\": \"running\"}" }))
        // Add DevUI routes using the local workspace library
        .nest("/dev/ui", dev_ui_router)
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()))
                .allow_methods(AllowMethods::list([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                    axum::http::Method::OPTIONS,
                ]))
                .allow_headers(AllowHeaders::list([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]))
                .allow_credentials(true)
        )
        .layer(TraceLayer::new_for_http());

    // Convert Axum router to a Tower service
    let app = app.into_make_service();

    println!("🚀 Dev Server starting on http://localhost:3000");
    println!("📱 Main app: http://localhost:3000/");
    println!("🛠️  DevUI: http://localhost:3000/dev/ui");
    println!("🗄️  SQL Editor: http://localhost:3000/dev/ui/sql");
    println!("🔍 Health check: http://localhost:3000/api/health");
    println!("");
    println!("ℹ️  This server uses the local devui library code from the workspace");

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

