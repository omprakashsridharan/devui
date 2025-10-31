# DevUI Development Server

A local development server that uses the `devui` library code directly from the workspace (not from crates.io). This is useful for:

- 🛠️ **Testing local changes**: Quickly test changes to the devui library code
- 🔄 **Hot reloading**: Use with `cargo watch` for automatic rebuilds on code changes
- 🐛 **Debugging**: Easier debugging with direct access to library source code
- ⚡ **Faster iteration**: No need to publish to crates.io to test changes

## Prerequisites

- [Rust](https://rustup.rs/) installed
- [Docker](https://www.docker.com/get-started) and Docker Compose (for the sample database, optional)

## Quick Start

### 1. Build Frontend (Required)

The devui library embeds frontend assets at compile time. You need to build the frontend first:

```bash
# From the repository root
cd frontend
npm install
npm run build
cd ..
```

### 2. Start the Database (Optional)

If you want to use the SQL editor features, start the PostgreSQL database:

```bash
# From the repository root
cd dev-server
docker-compose up -d

# Verify the database is running
docker-compose ps
```

The database will be initialized with:
- **Database**: `devui_sample_db`
- **User**: `devui_user`
- **Password**: `devui_password`
- **Port**: `5432`

### 3. Run the Development Server

```bash
# From the repository root
cd dev-server
cargo run
```

The server will start on `http://localhost:3000` with:
- Main application at `/`
- DevUI interface at `/dev/ui`
- SQL editor at `/dev/ui/sql` (if database is running)
- Health check at `/api/health`

## Available Endpoints

- **Main app**: http://localhost:3000/
- **DevUI**: http://localhost:3000/dev/ui
- **SQL Editor**: http://localhost:3000/dev/ui/sql
- **Health check**: http://localhost:3000/api/health

## Development Workflow

### With Hot Reloading

Use `cargo watch` to automatically rebuild when you change library code:

```bash
# Install cargo-watch if you haven't already
cargo install cargo-watch

# Run with hot reloading
cd dev-server
cargo watch -x "run"
```

Now any changes to the `devui` library code in `../src/` will trigger a rebuild.

### Manual Build

```bash
cd dev-server
cargo run
```

## How It Works

Unlike `standalone-example` which uses `devui` from crates.io:

```toml
# standalone-example/Cargo.toml
devui = "0.0.1"  # From crates.io
```

This dev server uses the local workspace library:

```toml
# dev-server/Cargo.toml
devui = { path = ".." }  # From local workspace
```

This means:
- ✅ Changes to library code in `../src/` are immediately available
- ✅ No need to publish to test changes
- ✅ Faster development iteration
- ✅ Better debugging experience

## Rebuilding Frontend Assets

When you make changes to the frontend, you need to rebuild it:

```bash
# From the repository root
cd frontend
npm run build
cd ..
```

Then restart the dev server. The rebuilt `frontend/dist` will be embedded in the library during compilation.

## Cleaning Up

When you're done, stop the database:

```bash
# From dev-server directory
docker-compose down

# Remove volumes (optional - removes all data)
docker-compose down -v
```

## Project Structure

```
dev-server/
├── Cargo.toml          # Uses devui from local workspace (path = "..")
├── src/
│   └── main.rs         # Development server code
├── docker-compose.yml  # PostgreSQL setup
├── init-db.sql        # Sample database schema
└── README.md          # This file
```

## Differences from standalone-example

| Feature | standalone-example | dev-server |
|---------|-------------------|------------|
| Library Source | crates.io | Local workspace |
| Use Case | Testing published package | Local development |
| Frontend Build | Built before publishing | Required before running |
| Cargo.toml | `devui = "0.0.1"` | `devui = { path = ".." }` |

## Troubleshooting

### Frontend assets not found

If you see errors about missing frontend assets, make sure you've built the frontend:

```bash
cd frontend
npm run build
```

### Database connection errors

If SQL editor features don't work:
1. Make sure Docker is running
2. Start the database: `docker-compose up -d`
3. Verify it's running: `docker-compose ps`

