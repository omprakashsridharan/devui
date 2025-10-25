use axum::{response::Html, routing::get, Router};
use devui::{dev_ui_router, DevUIConfigBuilder, DevUIConfigBuilderError, PostgresConfig, SqlConfig};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create database configuration (optional)
    let sql_config = SqlConfig::new().with_postgres(
        "afp-local".to_string(),
        PostgresConfig {
            host: "localhost".to_string(),
            port: 5432,
            database: "afp_onboarding".to_string(),
            username: "postgres".to_string(),
            password: "postgres".to_string(),
            ssl_mode: Some("disable".to_string()),
        },
    );

    let dev_ui_config = DevUIConfigBuilder::default().sql_config(sql_config).build().expect("Error building dev ui config");

    let dev_ui_router = dev_ui_router(dev_ui_config)
        .await
        .expect("error constructing dev_ui_router");

    // Create a basic Axum router with some example routes
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Welcome to My App!</h1><p>Visit <a href='/dev/ui'>/dev/ui</a> for development tools.</p>") }))
        .route("/api/health", get(|| async { "OK" }))
        .route("/api/status", get(|| async { "{\"status\": \"running\"}" }))
        // Add DevUI routes using the new Axum-native approach
        .nest("/dev/ui", dev_ui_router)
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
                .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PUT, axum::http::Method::DELETE])
                .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
        )
        .layer(TraceLayer::new_for_http());

    // Convert Axum router to a Tower service
    let app = app.into_make_service();

    println!("🚀 Server starting on http://localhost:3000");
    println!("📱 Main app: http://localhost:3000/");
    println!("🛠️  DevUI: http://localhost:3000/dev/ui");
    println!("🗄️  SQL Editor: http://localhost:3000/dev/ui/sql");
    println!("🔍 Health check: http://localhost:3000/api/health");

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
