# DevUI Standalone Example

This is a self-contained example that uses the `devui` library from the cargo registry (not from the local workspace).

## Prerequisites

- [Rust](https://rustup.rs/) installed
- [Docker](https://www.docker.com/get-started) and Docker Compose (for the sample database)

## Running the Example

### Step 1: Start the Sample Database (Optional)

If you want to use the SQL editor features, start the PostgreSQL database:

```bash
# From the devui repository root
docker-compose up -d
```

This will start a PostgreSQL database with sample data.

### Step 2: Run the Example

```bash
cd standalone-example
cargo run
```

The server will start on `http://localhost:3000`.

### Available Endpoints

- **Main app**: http://localhost:3000/
- **DevUI**: http://localhost:3000/dev/ui
- **SQL Editor**: http://localhost:3000/dev/ui/sql
- **Health check**: http://localhost:3000/api/health

## Using DevUI from Cargo Registry

This example uses `devui` from [crates.io](https://crates.io/crates/devui) as specified in `Cargo.toml`:

```toml
devui = "0.0.1"
```

To use the latest version, update the version in `Cargo.toml`.

## Frontend Assets

✅ **No additional setup required!** The `devui` library embeds all frontend assets at compile time using `rust_embed`. This means:

- **Self-contained**: All frontend assets are bundled with the library binary
- **No filesystem dependencies**: You don't need a separate `frontend/dist` directory
- **Works out of the box**: Just add the dependency and run—no frontend build steps needed
- **Included in published package**: The frontend assets are included in the published crate source from crates.io

Simply add `devui` to your `Cargo.toml` and start using it:
```toml
[dependencies]
devui = "0.0.1"
```

## Project Structure

```
standalone-example/
├── Cargo.toml          # Uses devui from cargo registry
├── src/
│   └── main.rs         # Example server code
├── docker-compose.yml  # PostgreSQL setup
├── init-db.sql        # Sample database schema
└── README.md          # This file
```

