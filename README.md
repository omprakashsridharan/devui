# DevUI - Development Tools UI

A Tower middleware that adds a `/dev/ui` route to any HTTP service, serving a React-based development tools interface.

## Features

- 🛠️ **Tower Middleware**: Drop-in middleware for any Tower-compatible HTTP service
- ⚛️ **React SPA**: Modern React TypeScript single-page application
- 🚀 **Axum Integration**: Works seamlessly with Axum applications
- 📱 **Responsive UI**: Modern, clean interface for development tools
- 🔧 **Embedded Assets**: React app is embedded in the Rust binary at compile time

## Quick Start

### 1. Add to your Cargo.toml

```toml
[dependencies]
devui = "0.1.0"
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
```

### 2. Apply the middleware to your Axum app

```rust
use axum::{response::Html, routing::get, Router};
use devui::DevUiLayer;
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Html("<h1>My App</h1>") }))
        .layer(ServiceBuilder::new().layer(DevUiLayer::new()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### 3. Access your development tools

- **Main app**: http://localhost:3000/
- **DevUI**: http://localhost:3000/dev/ui

## Example

Run the included example:

```bash
cargo run --example axum_server
```

Then visit:
- http://localhost:3000/ - Your main application
- http://localhost:3000/dev/ui - Development tools interface

## How it works

The `DevUiLayer` middleware:

1. **Intercepts** requests to `/dev/ui`
2. **Serves** the React SPA for non-API routes
3. **Handles** API endpoints for backend communication
4. **Passes through** all other requests to your application

## Architecture

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   HTTP Request  │───▶│   DevUiLayer    │───▶│  Your Service   │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                              │
                              ▼
                       ┌──────────────────┐
                       │  React SPA       │
                       │  /dev/ui route   │
                       │  + API endpoints │
                       └──────────────────┘
```

## Future Development Tools

The interface is designed to be extensible with tools like:

- **SQL Editor**: Query and manage databases
- **Kafka UI**: Monitor Kafka topics and messages
- **API Explorer**: Test and explore API endpoints
- **Log Viewer**: View and search application logs
- **Metrics Dashboard**: Monitor application performance

## Development

For hot module reloading during development, see [DEVELOPMENT.md](DEVELOPMENT.md).

### Quick Development Setup

```bash
# Install frontend dependencies
make frontend-install

# Build the entire project (frontend + backend)
make build

# Start development server with hot reloading
make dev
```

### Frontend Development

For React development with hot reloading:

```bash
# Start frontend development server
make frontend-dev
```

## License

MIT
