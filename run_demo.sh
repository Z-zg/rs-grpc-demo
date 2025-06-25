#!/bin/bash

echo "🎓 Student Management gRPC Demo"
echo "================================"

# Check if cargo is available
if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo is not installed. Please install Rust and Cargo first."
    exit 1
fi

# Build the project
echo "🔨 Building the project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi

echo "✅ Build successful!"

# Start the server in the background
echo "🚀 Starting gRPC server..."
cargo run --bin server &
SERVER_PID=$!

# Wait a moment for the server to start
sleep 3

# Check if server is running
if ! kill -0 $SERVER_PID 2>/dev/null; then
    echo "❌ Server failed to start"
    exit 1
fi

echo "✅ Server started (PID: $SERVER_PID)"

# Run the client
echo "🎯 Running client demo..."
cargo run --bin client

# Clean up: kill the server
echo "🧹 Cleaning up..."
kill $SERVER_PID 2>/dev/null

echo "✅ Demo completed!"
