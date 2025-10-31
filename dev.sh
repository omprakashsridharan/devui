#!/bin/bash

# Advanced development server with hot reloading and browser refresh
# This script provides a complete development experience

echo "🔥 DevUI Development Server with Hot Reloading"
echo "=============================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to cleanup on exit
cleanup() {
    echo -e "\n${YELLOW}🛑 Shutting down development server...${NC}"
    pkill -f "devui-example" 2>/dev/null || true
    pkill -f "cargo watch" 2>/dev/null || true
    exit 0
}

# Set up signal handlers
trap cleanup SIGINT SIGTERM

# Check if cargo-watch is installed
if ! command -v cargo-watch &> /dev/null; then
    echo -e "${RED}❌ cargo-watch not found. Installing...${NC}"
    cargo install cargo-watch
fi

# Kill any existing processes
echo -e "${BLUE}🧹 Cleaning up existing processes...${NC}"
pkill -f "devui-example" 2>/dev/null || true

# Start the development server from standalone-example
echo -e "${GREEN}🚀 Starting development server with hot reloading...${NC}"
echo -e "${BLUE}📁 Watching: standalone-example/src/, src/, Cargo.toml${NC}"
echo -e "${BLUE}🔄 Auto-restarting on file changes${NC}"
echo -e "${BLUE}🌐 Server will be available at: http://localhost:3000${NC}"
echo -e "${BLUE}🛠️  DevUI will be available at: http://localhost:3000/dev/ui${NC}"
echo ""

# Start cargo watch with enhanced options
cd standalone-example
cargo watch \
  --watch src/ \
  --watch ../src/ \
  --watch Cargo.toml \
  --watch ../Cargo.toml \
  --clear \
  --delay 0.5 \
  --notify \
  --exec "run" \
  --shell "echo '🔄 Rebuilding and restarting server...' && cargo run"
