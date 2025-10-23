use axum::{
    response::Html,
    routing::get,
    Router,
};
use devui::{DevUiLayer, sql::{DatabaseConfig, PostgresConfig}};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create database configuration (optional)
    let db_config = DatabaseConfig::new()
        .with_postgres(PostgresConfig {
            host: "localhost".to_string(),
            port: 5440,
            database: "expansion-acquiring-adyen".to_string(),
            username: "postgres".to_string(),
            password: "password".to_string(),
            ssl_mode: Some("disable".to_string()),
        });

    // Create a basic Axum router with some example routes
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>Welcome to My App!</h1><p>Visit <a href='/dev/ui'>/dev/ui</a> for development tools.</p>") }))
        .route("/api/health", get(|| async { "OK" }))
        .route("/api/status", get(|| async { "{\"status\": \"running\"}" }));

    // Apply the DevUI layer with database configuration
    let app = app.layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(DevUiLayer::with_database_config(db_config))
    );

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
