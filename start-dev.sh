#!/bin/bash

# MICCAI 2025 Papers Visualization - Development Startup Script

echo "🚀 Starting MICCAI 2025 Papers Visualization Development Environment"
echo ""

# Check if we're in the right directory
if [ ! -f "CLAUDE.md" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

# Check dependencies
echo "🔍 Checking dependencies..."

if ! command -v python3 &> /dev/null; then
    echo "❌ Python 3 is required but not installed"
    exit 1
fi

if ! command -v node &> /dev/null; then
    echo "❌ Node.js is required but not installed"
    exit 1
fi

echo "✅ Dependencies check passed"
echo ""

# Start backend
echo "🐍 Starting Backend (FastAPI)..."
cd backend

if [ ! -d ".venv" ]; then
    echo "📦 Creating Python virtual environment..."
    python3 -m venv .venv
fi

echo "📦 Activating virtual environment..."
source .venv/bin/activate

if [ ! -f ".venv/pyvenv.cfg" ] || [ requirements.txt -nt .venv/pyvenv.cfg ]; then
    echo "📦 Installing Python dependencies..."
    pip install -r requirements.txt
fi

echo "🚀 Starting backend server on http://localhost:8000"
python main.py &
BACKEND_PID=$!

# Give backend time to start
sleep 3

# Start frontend
cd ../frontend

if [ ! -d "node_modules" ]; then
    echo "📦 Installing Node.js dependencies..."
    npm install
fi

echo "⚛️ Starting Frontend (React + Vite)..."
echo "🌐 Frontend will be available at http://localhost:5173"
echo ""
echo "🎉 Both servers are starting up!"
echo ""
echo "📱 Quick Actions:"
echo "   • Open http://localhost:5173 to view the app"
echo "   • API docs at http://localhost:8000/docs"
echo "   • Press Ctrl+C to stop both servers"
echo ""

# Start frontend (this will keep script running)
npm run dev

# Cleanup when script exits
echo ""
echo "🛑 Stopping servers..."
kill $BACKEND_PID 2>/dev/null
echo "✅ Development servers stopped"