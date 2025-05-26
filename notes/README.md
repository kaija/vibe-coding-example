# Notes Application - Full Stack

A complete full-stack note-taking application with Rust backend API, static frontend, and PostgreSQL database, all containerized with Docker.

## Features

### Backend (Rust API)
- **User Authentication**: JWT-based authentication with secure password hashing using Argon2
- **Note Management**: Full CRUD operations for notes with user ownership
- **Database**: PostgreSQL with SeaORM for type-safe database operations
- **Security**: Password hashing, JWT tokens, and proper error handling
- **CORS**: Configured for cross-origin requests
- **Modular Architecture**: Clean separation of concerns with services, entities, and API layers

### Frontend (Static Web App)
- **Responsive UI**: Clean, modern interface built with HTML, CSS, and JavaScript
- **User Authentication**: Login and registration forms with validation
- **Note Management**: Create, read, update, and delete notes with real-time updates
- **Search Functionality**: Search through notes by title and content
- **Modal Dialogs**: Intuitive modals for note editing and deletion confirmation
- **Error Handling**: User-friendly error messages and loading states

### Infrastructure
- **Containerized**: Complete Docker setup for all components
- **Production Ready**: Optimized Docker images with multi-stage builds
- **Development Environment**: Hot-reload development setup with volume mounting
- **Health Checks**: Built-in health monitoring for all services
- **Networking**: Secure inter-service communication

## Tech Stack

### Backend
- **Framework**: Rocket 0.5
- **ORM**: SeaORM 0.12
- **Database**: PostgreSQL
- **Authentication**: JWT + Argon2 password hashing
- **Serialization**: Serde
- **Validation**: Validator

### Frontend
- **Languages**: HTML5, CSS3, JavaScript (ES6+)
- **Web Server**: Nginx (containerized)
- **Architecture**: Single Page Application (SPA)
- **Styling**: Custom CSS with responsive design
- **API Communication**: Fetch API with JWT authentication

### Infrastructure
- **Containerization**: Docker & Docker Compose
- **Web Server**: Nginx (frontend), Rocket (backend)
- **Database**: PostgreSQL 15
- **Networking**: Docker bridge network
- **Storage**: Docker volumes for data persistence

## Project Structure

```
├── src/                    # Rust backend source code
│   ├── api/               # API route handlers
│   │   ├── auth.rs        # Authentication endpoints
│   │   ├── notes.rs       # Notes CRUD endpoints
│   │   └── mod.rs
│   ├── auth/              # Authentication utilities
│   │   └── mod.rs         # JWT and password hashing
│   ├── db/                # Database connection and setup
│   │   └── mod.rs
│   ├── dto/               # Data Transfer Objects
│   │   └── mod.rs         # Request/Response structures
│   ├── entities/          # Database models
│   │   ├── users.rs       # User entity
│   │   ├── notes.rs       # Note entity
│   │   └── mod.rs
│   ├── migration/         # Database migrations
│   │   ├── mod.rs
│   │   ├── m20231201_000001_create_users_table.rs
│   │   └── m20231201_000002_create_notes_table.rs
│   ├── services/          # Business logic
│   │   ├── user_service.rs
│   │   ├── note_service.rs
│   │   └── mod.rs
│   └── main.rs            # Application entry point
├── frontend/              # Static frontend application
│   ├── index.html         # Main HTML file
│   ├── styles.css         # CSS styles
│   ├── app.js             # JavaScript application logic
│   ├── Dockerfile         # Frontend container configuration
│   └── nginx.conf         # Nginx server configuration
├── scripts/               # Utility scripts
│   ├── health-check.sh    # Health check script
│   └── verify-setup.sh    # Setup verification script
├── docker-compose.yml     # Production Docker setup
├── docker-compose.dev.yml # Development Docker setup
├── Dockerfile             # Backend container configuration
├── Makefile              # Build and deployment commands
└── README.md             # This file
```

## Setup Instructions

### Option 1: Docker Setup (Recommended)

#### Prerequisites
- Docker
- Docker Compose
- Git

#### Quick Start with Docker

1. **Clone the repository**
```bash
git clone <repository-url>
cd note-api
```

2. **Start the complete application stack**
```bash
# Production setup (recommended)
make stack-up
# OR
docker-compose up -d

# Development setup (with hot reload)
make stack-dev
# OR
docker-compose -f docker-compose.dev.yml up -d
```

3. **Access the application**
- **Frontend**: http://localhost:8080 (main application interface)
- **Backend API**: http://localhost:8000 (REST API endpoints)
- **Database**: localhost:5432 (PostgreSQL)

4. **Open the application**
```bash
make open
# OR manually open http://localhost:8080 in your browser
```

#### Verify Docker Setup

To verify that your Docker setup is working correctly:

```bash
# Run the verification script
./scripts/verify-setup.sh
```

This script will:
- Check Docker and Docker Compose installation
- Verify all required files are present
- Build the Docker image
- Start services and test connectivity
- Verify API endpoints are responding
- Clean up after testing

#### Docker Commands

```bash
# Full Stack Commands
make stack-up        # Start complete stack (frontend + backend + database)
make stack-dev       # Start development stack with hot reload
make open           # Open application in browser

# Individual Service Commands
docker-compose up -d                    # Start all services
docker-compose up -d frontend          # Start only frontend
docker-compose up -d backend           # Start only backend
docker-compose up -d postgres          # Start only database

# View logs
docker-compose logs -f                 # All services
docker-compose logs -f frontend        # Frontend only
docker-compose logs -f backend         # Backend only
docker-compose logs -f postgres        # Database only

# Stop services
docker-compose down                    # Stop all services
docker-compose stop frontend          # Stop only frontend

# Rebuild and start
docker-compose up --build -d          # Rebuild all and start
make frontend-build                   # Build only frontend image

# Remove volumes (clean database)
docker-compose down -v                # Remove all volumes
```

#### Development with Docker

For development with hot reload and volume mounting:

```bash
# Start development environment
docker-compose -f docker-compose.dev.yml up -d

# View development logs
docker-compose -f docker-compose.dev.yml logs -f backend

# Stop development environment
docker-compose -f docker-compose.dev.yml down
```

### Option 2: Local Development Setup

#### Prerequisites
- Rust (latest stable version)
- PostgreSQL database
- Git

#### 1. Clone and Setup

```bash
git clone <repository-url>
cd note-api
```

#### 2. Database Setup

Create a PostgreSQL database:

```sql
CREATE DATABASE note_api;
CREATE USER note_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE note_api TO note_user;
```

#### 3. Environment Configuration

Copy the example environment file and configure it:

```bash
cp .env.example .env
```

Edit `.env` with your database credentials:

```env
DATABASE_URL=postgresql://note_user:your_password@localhost:5432/note_api
JWT_SECRET=your-super-secret-jwt-key-here-make-it-long-and-random
ROCKET_PORT=8000
ROCKET_ADDRESS=127.0.0.1
```

#### 4. Run the Application

```bash
cargo run
```

The API will be available at `http://localhost:8000`

## API Endpoints

### Authentication

#### Register User
```http
POST /api/register
Content-Type: application/json

{
  "username": "john_doe",
  "password": "secure_password123"
}
```

#### Login
```http
POST /api/login
Content-Type: application/json

{
  "username": "john_doe",
  "password": "secure_password123"
}
```

#### Get Current User
```http
GET /api/me
Authorization: Bearer <jwt_token>
```

### Notes

#### Create Note
```http
POST /api/notes
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "title": "My First Note",
  "content": "This is the content of my note."
}
```

#### Get All Notes
```http
GET /api/notes
Authorization: Bearer <jwt_token>
```

#### Get Single Note
```http
GET /api/notes/{note_id}
Authorization: Bearer <jwt_token>
```

#### Update Note
```http
PUT /api/notes/{note_id}
Authorization: Bearer <jwt_token>
Content-Type: application/json

{
  "title": "Updated Title",
  "content": "Updated content"
}
```

#### Delete Note
```http
DELETE /api/notes/{note_id}
Authorization: Bearer <jwt_token>
```

### Health Check
```http
GET /health
```

## Database Schema

### Users Table
- `id` (UUID, Primary Key)
- `username` (String, Unique)
- `password` (String, Hashed)
- `created_at` (Timestamp)

### Notes Table
- `id` (UUID, Primary Key)
- `user_id` (UUID, Foreign Key)
- `title` (String)
- `content` (Text)
- `created_at` (Timestamp)
- `updated_at` (Timestamp)

## Docker Configuration

### Docker Files Overview

- **`Dockerfile`**: Multi-stage build for production-optimized backend container
- **`frontend/Dockerfile`**: Nginx-based container for static frontend files
- **`frontend/nginx.conf`**: Optimized Nginx configuration for SPA
- **`docker-compose.yml`**: Production setup with all services
- **`docker-compose.dev.yml`**: Development setup with volume mounting
- **`init.sql`**: Database initialization script
- **`.dockerignore`**: Optimizes build context

### Container Architecture

#### Frontend Container
- **Base Image**: `nginx:alpine`
- **Port**: 80 (exposed as 8080)
- **Content**: Static HTML, CSS, JavaScript files
- **Web Server**: Nginx with optimized configuration
- **Features**: Gzip compression, security headers, SPA routing
- **Health Check**: `/health` endpoint

#### Backend Container
- **Base Image**: `rust:1.75` (build) → `debian:bookworm-slim` (runtime)
- **Port**: 8000
- **User**: Non-root user for security
- **Health Check**: `/health` endpoint
- **Dependencies**: Minimal runtime dependencies

#### Database Container
- **Image**: `postgres:15`
- **Port**: 5432
- **Volume**: Persistent data storage
- **Health Check**: `pg_isready`
- **Initialization**: Automatic schema setup

### Environment Variables

#### Production Environment
```env
DATABASE_URL=postgresql://postgres:postgres@postgres:5432/note_api
JWT_SECRET=your-super-secret-jwt-key-here-make-it-long-and-random-for-docker
ROCKET_PORT=8000
ROCKET_ADDRESS=0.0.0.0
RUST_LOG=info
```

#### Development Environment
```env
DATABASE_URL=postgresql://postgres:postgres@postgres:5432/note_api
JWT_SECRET=dev-jwt-secret-key-for-development-only
ROCKET_PORT=8000
ROCKET_ADDRESS=0.0.0.0
RUST_LOG=debug
```

### Docker Networking

Services communicate through a custom bridge network:
- **Network Name**: `note-api-network`
- **Frontend Service**: `frontend` (nginx on port 80, exposed as 8080)
- **Backend Service**: `backend` (rust API on port 8000)
- **Database Service**: `postgres` (PostgreSQL on port 5432)
- **Internal Communication**: Service names as hostnames
- **External Access**:
  - Frontend: http://localhost:8080
  - Backend API: http://localhost:8000
  - Database: localhost:5432

### Persistent Storage

- **Production Volume**: `postgres_data`
- **Development Volumes**:
  - `postgres_dev_data` (database)
  - `cargo_cache` (Rust dependencies)
  - `target_cache` (build artifacts)

### Health Checks

All services include health checks:
- **Frontend**: HTTP check on `/health` endpoint (nginx)
- **Backend**: HTTP check on `/health` endpoint (Rust API)
- **Database**: PostgreSQL ready check (`pg_isready`)
- **Dependencies**:
  - Backend waits for database health
  - Frontend waits for backend availability

### Quick Commands (Makefile)

A Makefile is provided for convenient Docker operations:

```bash
# Full Stack Commands
make stack-up         # Start complete stack (frontend + backend + database)
make stack-dev        # Start development stack with hot reload
make open            # Open application in browser

# Production
make up              # Start production services
make down            # Stop production services
make logs            # View logs
make build           # Build images
make clean           # Clean up containers and volumes

# Development
make dev             # Start development services
make dev-down        # Stop development services
make dev-logs        # View development logs

# Frontend Specific
make frontend-build  # Build only frontend image
make frontend-logs   # View frontend logs

# Database access
make db-shell        # Access production database
make dev-db-shell    # Access development database

# Testing
make test            # Run tests in Docker

# Shell access
make shell           # Access production backend container
make dev-shell       # Access development backend container
```

## Complete Application Architecture

The Notes Application is a full-stack containerized solution with three main components:

### Application Flow
1. **User Access**: Users access the application through the frontend at http://localhost:8080
2. **Frontend**: Static web application served by Nginx handles the user interface
3. **API Communication**: Frontend communicates with the backend API at http://localhost:8000
4. **Backend Processing**: Rust API handles authentication, business logic, and data operations
5. **Data Storage**: PostgreSQL database stores user accounts and notes

### Service Communication
```
┌─────────────────┐    HTTP/HTTPS    ┌─────────────────┐    TCP/5432    ┌─────────────────┐
│                 │ ──────────────── │                 │ ────────────── │                 │
│    Frontend     │                  │     Backend     │                │    Database     │
│   (Nginx:80)    │                  │   (Rocket:8000) │                │ (PostgreSQL)    │
│                 │                  │                 │                │                 │
└─────────────────┘                  └─────────────────┘                └─────────────────┘
        │                                      │                                  │
        │                                      │                                  │
   Port 8080                              Port 8000                         Port 5432
   (External)                             (External)                        (External)
```

### Key Features
- **Containerized**: All components run in Docker containers for consistency
- **Scalable**: Each service can be scaled independently
- **Secure**: JWT authentication, password hashing, and secure networking
- **Development-Friendly**: Hot reload and volume mounting for development
- **Production-Ready**: Optimized builds and health monitoring

## Development

### Development Workflow

#### With Docker (Recommended)
```bash
# Start development environment
docker-compose -f docker-compose.dev.yml up -d

# View logs
docker-compose -f docker-compose.dev.yml logs -f

# Execute commands in container
docker-compose -f docker-compose.dev.yml exec backend cargo test
docker-compose -f docker-compose.dev.yml exec backend cargo fmt
docker-compose -f docker-compose.dev.yml exec backend cargo clippy

# Database access
docker-compose -f docker-compose.dev.yml exec postgres psql -U postgres -d note_api

# Stop development environment
docker-compose -f docker-compose.dev.yml down
```

#### Local Development
```bash
# Ensure PostgreSQL is running locally
cargo run
cargo test
cargo fmt
cargo clippy
```

### Running Tests
```bash
# Local testing
cargo test

# Docker testing
docker-compose exec backend cargo test
```

### Database Migrations

Migrations are automatically run on application startup. To run them manually:

```bash
# Local development
# The migrations are embedded in the application
# They run automatically when the server starts

# Docker development
docker-compose exec backend cargo run
```

### Code Structure

- **Entities**: Database models using SeaORM
- **Services**: Business logic layer
- **API**: HTTP handlers and routing
- **DTOs**: Request/response data structures
- **Auth**: Authentication and authorization utilities

## Security Features

- **Password Hashing**: Uses Argon2 for secure password storage
- **JWT Authentication**: Stateless authentication with configurable expiration
- **Input Validation**: Request validation using the validator crate
- **SQL Injection Protection**: SeaORM provides query builder protection
- **CORS**: Configurable cross-origin resource sharing

## Error Handling

The API returns structured error responses:

```json
{
  "error": "error_code",
  "message": "Human readable error message"
}
```

Common HTTP status codes:
- `200` - Success
- `201` - Created
- `204` - No Content (for deletions)
- `400` - Bad Request (validation errors)
- `401` - Unauthorized (authentication required)
- `403` - Forbidden (insufficient permissions)
- `404` - Not Found
- `409` - Conflict (duplicate username)
- `500` - Internal Server Error

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License.