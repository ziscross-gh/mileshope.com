#!/bin/bash

# Development script for MilesHope.com
# Runs Tailwind CSS in watch mode alongside Zola serve

echo "🚀 Starting development environment..."
echo ""
echo "📝 Tailwind CSS will watch for changes in templates/ and content/"
echo "🌐 Zola will serve the site at http://127.0.0.1:1111"
echo ""
echo "Press Ctrl+C to stop both processes"
echo ""

# Function to cleanup on exit
cleanup() {
  echo ""
  echo "👋 Stopping development servers..."
  kill $TAILWIND_PID $ZOLA_PID 2>/dev/null
  exit
}

trap cleanup INT TERM

# Start Tailwind CSS in watch mode
./tailwindcss -i ./styles/input.css -o ./static/css/tailwind.css --watch &
TAILWIND_PID=$!

# Give Tailwind a moment to start
sleep 1

# Start Zola serve
zola serve &
ZOLA_PID=$!

# Wait for both processes
wait
