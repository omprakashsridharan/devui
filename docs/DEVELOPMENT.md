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

### Option 3: Backend Development Only (Using Published Package)
```bash
# Install cargo-watch if you haven't already
cargo install cargo-watch

# Start Rust development server with hot reloading
cd standalone-example && cargo watch -x "run"
```

### Option 4: Local Development Server (Using Workspace Library)
```bash
# Build frontend first (required)
cd frontend
npm install
npm run build
cd ..

# Run development server using local library code
cd dev-server
cargo run

# Or with hot reloading
cargo install cargo-watch
cargo watch -x "run"
```

**When to use `dev-server` vs `standalone-example`:**
- **`dev-server`**: Use when developing the library itself. It uses the local workspace code (`devui = { path = ".." }`), so your changes are immediately available.
- **`standalone-example`**: Use when testing the published package. It uses `devui` from crates.io (`devui = "0.0.1"`), simulating how users would use the library.

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

#### Using Published Package (standalone-example)
```bash
cargo install cargo-watch
cd standalone-example && cargo watch -x "run"
```

#### Using Local Library Code (dev-server)
```bash
cargo install cargo-watch
cd dev-server && cargo watch -x "run"
```

This automatically rebuilds and restarts when you change:
- Library code in `../src/`
- Dev server code in `dev-server/src/`

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
paths = ["src", "standalone-example/src", "Cargo.toml", "Cargo.lock"]
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
- `src/` - Library source code (use `dev-server` to test changes)
- `dev-server/src/main.rs` - Development server code
- `standalone-example/src/main.rs` - Standalone example server (uses published package)

### 3. Automatic Reloading
- **Frontend**: Vite hot module replacement updates instantly
- **Backend**: Cargo watch rebuilds and restarts server
- **Full Stack**: Both frontend and backend changes are handled

## 🔧 Advanced Configuration

### Custom Watch Patterns
```bash
# Watch only specific files
cd standalone-example && cargo watch -x "run" --watch src/

# Watch with custom delay
cd standalone-example && cargo watch -x "run" --delay 1.0

# Watch with custom shell command
cd standalone-example && cargo watch --shell "cargo build && cargo run"
```

### Environment Variables
```bash
# Set custom port
cd standalone-example && PORT=8080 cargo watch -x "run"

# Enable debug logging
cd standalone-example && RUST_LOG=debug cargo watch -x "run"
```

## 🐛 Troubleshooting

### Common Issues

1. **Port already in use**
   ```bash
   # Kill existing processes
   pkill -f "devui-example"
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
cd standalone-example && RUST_LOG=debug cargo watch -x "run"
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
   cd standalone-example && cargo watch --delay 0.1 -x "run"
   ```

3. **Use incremental builds**
   ```bash
   # Only rebuild changed parts
   cd standalone-example && cargo watch -x "build" -x "run"
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
cd standalone-example && cargo watch -x "run"

# Production
cd standalone-example && cargo run --release
```
