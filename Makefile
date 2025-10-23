# DevUI Development Makefile
# Provides convenient commands for development workflow

.PHONY: help dev dev-watch build test clean install-watch frontend-install frontend-build frontend-dev

# Default target
help:
	@echo "DevUI Development Commands"
	@echo "========================="
	@echo ""
	@echo "  make dev              - Start development server with hot reloading"
	@echo "  make build            - Build the entire project (frontend + backend)"
	@echo "  make test             - Run tests"
	@echo "  make clean            - Clean build artifacts"
	@echo "  make install-watch    - Install cargo-watch for hot reloading"
	@echo "  make frontend-install - Install frontend dependencies"
	@echo "  make frontend-build   - Build the React frontend"
	@echo "  make frontend-dev     - Run frontend development server"
	@echo ""

# Install frontend dependencies
frontend-install:
	@echo "📦 Installing frontend dependencies..."
	@cd frontend && npm install

# Build the React frontend
frontend-build:
	@echo "⚛️  Building React frontend..."
	@cd frontend && npm run build

# Run frontend development server
frontend-dev:
	@echo "⚛️  Starting frontend development server..."
	@cd frontend && npm run dev

# Development server with hot reloading
dev:
	@echo "🚀 Starting DevUI development server..."
	@./dev.sh

# Build the entire project (frontend + backend)
build: frontend-build
	@echo "🔨 Building DevUI backend..."
	@cargo build

# Run tests
test:
	@echo "🧪 Running tests..."
	@cargo test

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	@cargo clean
	@cd frontend && rm -rf dist

# Install cargo-watch
install-watch:
	@echo "📦 Installing cargo-watch..."
	@cargo install cargo-watch
