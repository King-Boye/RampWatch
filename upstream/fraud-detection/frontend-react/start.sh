#!/bin/bash

# Fraud Detection System - React Frontend Startup Script

echo "🛡️  Fraud Detection System - React Frontend"
echo "==========================================="
echo ""

# Check if node_modules exists
if [ ! -d "node_modules" ]; then
    echo "📦 Installing dependencies..."
    npm install
    echo ""
fi

# Check if .env exists
if [ ! -f ".env" ]; then
    echo "⚙️  Creating .env file from template..."
    cp .env.example .env
    echo "✓ Created .env file"
    echo ""
fi

echo "🚀 Starting development server..."
echo ""
echo "Frontend will be available at: http://localhost:5173"
echo "Make sure the API is running at: http://localhost:8000"
echo ""
echo "Press Ctrl+C to stop the server"
echo ""

npm run dev
