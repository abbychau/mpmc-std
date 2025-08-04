#!/bin/bash

# Preview documentation server script
# Serves the docs folder using Python's built-in HTTP server

PORT=${1:-8080}
DOCS_DIR="docs"

echo "🌐 Starting documentation preview server..."
echo "📁 Serving directory: $DOCS_DIR"
echo "🔗 URL: http://localhost:$PORT"
echo "⏹️  Press Ctrl+C to stop the server"
echo ""

if [ ! -d "$DOCS_DIR" ]; then
    echo "❌ Error: docs directory not found!"
    echo "💡 Run ./copy_benchmarks.sh first to generate documentation"
    exit 1
fi

cd "$DOCS_DIR" || exit 1

# Try Python 3 first, then Python 2 as fallback
if command -v python3 &> /dev/null; then
    echo "🐍 Using Python 3..."
    python3 -m http.server "$PORT"
elif command -v python &> /dev/null; then
    echo "🐍 Using Python 2..."
    python -m SimpleHTTPServer "$PORT"
else
    echo "❌ Error: Python not found!"
    echo "💡 Please install Python to use this preview server"
    exit 1
fi