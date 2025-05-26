#!/bin/bash

# Docker setup verification script for Note API
# This script verifies that the Docker containerization is working correctly

set -e

echo "🐳 Note API Docker Setup Verification"
echo "======================================"

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    echo "❌ Docker is not installed or not in PATH"
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    echo "❌ Docker Compose is not installed or not in PATH"
    exit 1
fi

echo "✅ Docker and Docker Compose are installed"

# Check if required files exist
required_files=(
    "Dockerfile"
    "docker-compose.yml"
    "docker-compose.dev.yml"
    "init.sql"
    ".dockerignore"
    "Makefile"
)

for file in "${required_files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ Required file missing: $file"
        exit 1
    fi
done

echo "✅ All required Docker files are present"

# Build the Docker image
echo "🔨 Building Docker image..."
if docker-compose build --quiet; then
    echo "✅ Docker image built successfully"
else
    echo "❌ Failed to build Docker image"
    exit 1
fi

# Start services
echo "🚀 Starting services..."
if docker-compose up -d; then
    echo "✅ Services started successfully"
else
    echo "❌ Failed to start services"
    exit 1
fi

# Wait for services to be ready
echo "⏳ Waiting for services to be ready..."
sleep 10

# Check if backend is responding
echo "🔍 Checking backend health..."
max_attempts=30
attempt=1

while [ $attempt -le $max_attempts ]; do
    if curl -f -s http://localhost:8000/health > /dev/null; then
        echo "✅ Backend is responding to health checks"
        break
    fi
    
    if [ $attempt -eq $max_attempts ]; then
        echo "❌ Backend failed to respond after $max_attempts attempts"
        echo "📋 Backend logs:"
        docker-compose logs backend
        docker-compose down
        exit 1
    fi
    
    echo "⏳ Attempt $attempt/$max_attempts - waiting for backend..."
    sleep 2
    ((attempt++))
done

# Check if database is accessible
echo "🔍 Checking database connectivity..."
if docker-compose exec -T postgres pg_isready -U postgres -d note_api > /dev/null; then
    echo "✅ Database is accessible"
else
    echo "❌ Database is not accessible"
    docker-compose down
    exit 1
fi

# Test API endpoint
echo "🔍 Testing API endpoints..."
response=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:8000/health)
if [ "$response" = "200" ]; then
    echo "✅ Health endpoint is working (HTTP $response)"
else
    echo "❌ Health endpoint returned HTTP $response"
    docker-compose down
    exit 1
fi

# Clean up
echo "🧹 Cleaning up..."
docker-compose down

echo ""
echo "🎉 Docker setup verification completed successfully!"
echo ""
echo "📋 Quick start commands:"
echo "  Production:  docker-compose up -d"
echo "  Development: docker-compose -f docker-compose.dev.yml up -d"
echo "  With Make:   make up (production) or make dev (development)"
echo ""
echo "🔗 API will be available at: http://localhost:8000"
echo "📊 Health check endpoint: http://localhost:8000/health"