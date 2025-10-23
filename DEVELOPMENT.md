# Development Guide - React + Rust Development

This guide explains how to set up development for the DevUI library with React frontend and Rust backend.

## 🚀 Quick Start

### Option 1: Full Stack Development
```bash
# Install frontend dependencies
make frontend-install

# Build the entire project (frontend + backend)
make build

# Start development server with hot reloading
make dev
```

### Option 2: Frontend Development Only
```bash
# Start React development server with hot reloading
make frontend-dev
```

### Option 3: Backend Development Only
```bash
# Install cargo-watch if you haven't already
cargo install cargo-watch

# Start Rust development server with hot reloading
cargo watch -x "run --example axum_server"
```

## 🛠️ Development Tools

### 1. Frontend Development (React + Vite)
- **Hot Module Replacement**: Instant updates without page refresh
- **TypeScript Support**: Full type checking and IntelliSense
- **Fast Builds**: Vite's lightning-fast development server

```bash
# Start frontend development server
make frontend-dev
```

### 2. Backend Development (Rust + Cargo Watch)
Automatically rebuilds and restarts the server when files change:

```bash
cargo install cargo-watch
cargo watch -x "run --example axum_server"
```

### 3. Development Scripts

#### `make dev` - Full stack development
- Builds React frontend
- Starts Rust backend with hot reloading
- Watches both frontend and backend changes

#### `make frontend-dev` - Frontend only
- Starts Vite development server
- Hot module replacement for React components
- TypeScript compilation

#### `make frontend-build` - Build frontend
- Builds React app for production
- Outputs to `frontend/dist/`
- Optimized for embedding in Rust binary

## 📁 File Watching Configuration

### Backend (Rust)
The `.cargo-watch.toml` file configures what to watch:

```toml
[watch]
paths = ["src", "examples", "Cargo.toml", "Cargo.lock"]
ignore = ["target/**", ".git/**", "*.tmp", "*.log", "frontend/**"]
delay = 0.5
clear = true
notify = true
```

### Frontend (React)
Vite automatically watches for changes in:
- `frontend/src/` - React components
- `frontend/public/` - Static assets
- `frontend/index.html` - HTML template

## 🎯 Development Workflow

### 1. Start Development Server
```bash
make dev
```

### 2. Make Changes
Edit any file:
- `frontend/src/components/` - React components
- `frontend/src/App.tsx` - Main React app
- `src/middleware.rs` - Tower middleware
- `src/layer.rs` - Tower layer
- `examples/axum_server.rs` - Example server

### 3. Automatic Reloading
- **Frontend**: Vite hot module replacement updates instantly
- **Backend**: Cargo watch rebuilds and restarts server
- **Full Stack**: Both frontend and backend changes are handled

## 🔧 Advanced Configuration

### Custom Watch Patterns
```bash
# Watch only specific files
cargo watch -x "run --example axum_server" --watch src/components/

# Watch with custom delay
cargo watch -x "run --example axum_server" --delay 1.0

# Watch with custom shell command
cargo watch --shell "cargo build && cargo run --example axum_server"
```

### Environment Variables
```bash
# Set custom port
PORT=8080 cargo watch -x "run --example axum_server"

# Enable debug logging
RUST_LOG=debug cargo watch -x "run --example axum_server"
```

## 🐛 Troubleshooting

### Common Issues

1. **Port already in use**
   ```bash
   # Kill existing processes
   pkill -f "axum_server"
   ```

2. **cargo-watch not found**
   ```bash
   cargo install cargo-watch
   ```

3. **Browser doesn't open**
   - Manually navigate to `http://localhost:3000/dev/ui`
   - Check if `open` or `xdg-open` commands are available

4. **Changes not detected**
   - Ensure files are being watched: `cargo watch --help`
   - Check `.cargo-watch.toml` configuration
   - Verify file permissions

### Debug Mode
```bash
# Run with debug logging
RUST_LOG=debug cargo watch -x "run --example axum_server"
```

## 📊 Performance Tips

1. **Exclude unnecessary files**
   ```toml
   # In .cargo-watch.toml
   ignore = ["target/**", ".git/**", "*.tmp", "*.log", "node_modules/**"]
   ```

2. **Optimize watch delay**
   ```bash
   # Faster restart (0.1s delay)
   cargo watch --delay 0.1 -x "run --example axum_server"
   ```

3. **Use incremental builds**
   ```bash
   # Only rebuild changed parts
   cargo watch -x "build" -x "run --example axum_server"
   ```

## 🔄 Integration with IDEs

### VS Code
Install the "Rust" extension and configure:
```json
{
  "rust-analyzer.cargo.features": ["hot-reload"],
  "rust-analyzer.checkOnSave.command": "clippy"
}
```

### IntelliJ IDEA / CLion
- Enable "Build project automatically"
- Configure external tools for cargo-watch
- Set up file watchers for `.rs` files

## 📝 Best Practices

1. **Keep components small** - Faster rebuilds
2. **Use incremental compilation** - Enable `incremental = true` in Cargo.toml
3. **Watch only necessary files** - Exclude `target/`, `.git/`
4. **Use development profiles** - Separate dev/prod configurations
5. **Monitor build times** - Use `cargo watch --help` for timing info

## 🚀 Production vs Development

### Development
- Hot reloading enabled
- Debug symbols included
- Verbose logging
- Fast rebuilds

### Production
- Optimized builds
- No debug symbols
- Minimal logging
- Full optimizations

Switch between modes:
```bash
# Development
cargo watch -x "run --example axum_server"

# Production
cargo run --example axum_server --release
```
