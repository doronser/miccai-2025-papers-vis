#!/bin/bash

# MICCAI 2025 Papers Visualization - Production Build Script

echo "🏗️  Building MICCAI 2025 Papers Visualization for Production"
echo ""

# Check if we're in the right directory
if [ ! -f "CLAUDE.md" ]; then
    echo "❌ Error: Please run this script from the project root directory"
    exit 1
fi

echo "🔍 Running pre-build checks..."

# Backend checks
cd backend
echo "📋 Running backend tests..."
source .venv/bin/activate
python -m pytest tests/ -q --tb=no

if [ $? -ne 0 ]; then
    echo "❌ Backend tests failed. Please fix issues before building."
    exit 1
fi

echo "✅ Backend tests passed"

# Frontend checks
cd ../frontend
echo "📋 Running frontend linting..."
npm run lint --silent

if [ $? -ne 0 ]; then
    echo "❌ Frontend linting failed. Please fix issues before building."
    exit 1
fi

echo "✅ Frontend linting passed"

echo "📋 Running frontend tests..."
npm test -- --watchAll=false --silent

if [ $? -ne 0 ]; then
    echo "❌ Frontend tests failed. Please fix issues before building."
    exit 1
fi

echo "✅ Frontend tests passed"

# Build frontend
echo "🏗️  Building frontend for production..."
npm run build

if [ $? -ne 0 ]; then
    echo "❌ Frontend build failed."
    exit 1
fi

echo "✅ Frontend build complete: frontend/dist/"

# Backend is ready as-is (no build step needed for Python)
echo "✅ Backend ready for deployment"

echo ""
echo "🎉 Production build completed successfully!"
echo ""
echo "📦 Deployment artifacts:"
echo "   • Backend: backend/ directory (with .venv/)"
echo "   • Frontend: frontend/dist/ directory"
echo ""
echo "🚀 Next steps:"
echo "   1. Deploy backend to Railway/Render/Heroku"
echo "   2. Deploy frontend/dist to Vercel/Netlify"
echo "   3. Update CORS settings with production frontend URL"
echo "   4. Set VITE_API_BASE_URL to production backend URL"