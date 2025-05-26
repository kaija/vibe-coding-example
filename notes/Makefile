# Makefile for Note API Docker operations

.PHONY: help build up down logs clean test dev dev-down dev-logs frontend-build frontend-logs stack-up stack-dev open

# Default target
help:
	@echo "Available commands:"
	@echo ""
	@echo "Full Stack:"
	@echo "  stack-up       - Start complete stack (frontend + backend + database)"
	@echo "  stack-dev      - Start development stack with hot reload"
	@echo "  open           - Open application in browser"
	@echo ""
	@echo "Production:"
	@echo "  build          - Build Docker images"
	@echo "  up             - Start production services"
	@echo "  down           - Stop production services"
	@echo "  logs           - View production logs"
	@echo "  clean          - Remove containers and volumes"
	@echo ""
	@echo "Development:"
	@echo "  dev            - Start development services"
	@echo "  dev-down       - Stop development services"
	@echo "  dev-logs       - View development logs"
	@echo ""
	@echo "Frontend:"
	@echo "  frontend-build - Build only frontend image"
	@echo "  frontend-logs  - View frontend logs"
	@echo ""
	@echo "Other:"
	@echo "  test           - Run tests in Docker"

# Production commands
build:
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down

logs:
	docker-compose logs -f

clean:
	docker-compose down -v --remove-orphans
	docker system prune -f

test:
	docker-compose exec backend cargo test

# Development commands
dev:
	docker-compose -f docker-compose.dev.yml up -d

dev-down:
	docker-compose -f docker-compose.dev.yml down

dev-logs:
	docker-compose -f docker-compose.dev.yml logs -f

# Database commands
db-shell:
	docker-compose exec postgres psql -U postgres -d note_api

dev-db-shell:
	docker-compose -f docker-compose.dev.yml exec postgres psql -U postgres -d note_api

# Backend shell access
shell:
	docker-compose exec backend /bin/bash

dev-shell:
	docker-compose -f docker-compose.dev.yml exec backend /bin/bash

# Frontend commands
frontend-build:
	docker-compose build frontend

frontend-logs:
	docker-compose logs -f frontend

# Open application URLs
open:
	@echo "Opening application URLs..."
	@echo "Frontend: http://localhost:8080"
	@echo "Backend API: http://localhost:8000"
	@which open >/dev/null 2>&1 && open http://localhost:8080 || echo "Please open http://localhost:8080 in your browser"

# Full stack commands
stack-up:
	@echo "Starting full stack (database + backend + frontend)..."
	docker-compose up -d
	@echo "Services started!"
	@echo "Frontend: http://localhost:8080"
	@echo "Backend API: http://localhost:8000"
	@echo "Database: localhost:5432"

stack-dev:
	@echo "Starting development stack..."
	docker-compose -f docker-compose.dev.yml up -d
	@echo "Development services started!"
	@echo "Frontend: http://localhost:8080"
	@echo "Backend API: http://localhost:8000"
	@echo "Database: localhost:5432"