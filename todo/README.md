# Rust Todo List Service

A full-stack todo list application built with Rust, featuring a REST API with JWT authentication and a responsive web interface.

## Features

- 🔐 **JWT Authentication** - Secure user registration and login
- 📝 **Todo Management** - Create, read, update, and delete todos
- ✅ **Status Tracking** - Mark todos as completed/pending with checkboxes
- 🕒 **Timestamps** - Track creation and update times
- 🐳 **Containerized** - Docker and Docker Compose support
- 🗄️ **PostgreSQL Database** - Persistent data storage
- 🎨 **Responsive UI** - Modern web interface that works on all devices
- 🏗️ **Modular Architecture** - Clean separation of concerns

## Tech Stack

### Backend
- **Rust** - Systems programming language
- **Axum** - Modern web framework
- **SQLx** - Async SQL toolkit
- **PostgreSQL** - Database
- **JWT** - Authentication tokens
- **bcrypt** - Password hashing

### Frontend
- **HTML5/CSS3** - Modern web standards
- **Vanilla JavaScript** - No framework dependencies
- **Responsive Design** - Mobile-first approach

## Quick Start

### Prerequisites
- Docker and Docker Compose
- Or: Rust (1.75+) and PostgreSQL

### Option 1: Docker Compose (Recommended)

1. **Clone and navigate to the project:**
   ```bash
   git clone <repository-url>
   cd todo-service
   ```

2. **Start the services:**
   ```bash
   docker-compose up --build
   ```

3. **Access the application:**
   - Web Interface: http://localhost:3000
   - API: http://localhost:3000/api

### Option 2: Local Development

1. **Start PostgreSQL:**
   ```bash
   docker run --name postgres -e POSTGRES_DB=todoapp -e POSTGRES_USER=todouser -e POSTGRES_PASSWORD=todopass -p 5432:5432 -d postgres:15
   ```

2. **Initialize the database:**
   ```bash
   psql -h localhost -U todouser -d todoapp -f init.sql
   ```

3. **Run the Rust application:**
   ```bash
   cargo run
   ```

4. **Access the application:**
   - Web Interface: http://localhost:3000
   - API: http://localhost:3000/api

## API Documentation

### Authentication Endpoints

#### Register User
```http
POST /api/auth/register
Content-Type: application/json

{
  "username": "testuser",
  "email": "test@example.com",
  "password": "password123"
}
```

#### Login User
```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "testuser",
  "password": "password123"
}
```

### Todo Endpoints

All todo endpoints require authentication via `Authorization: Bearer <token>` header.

#### Create Todo
```http
POST /api/todos
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Learn Rust",
  "description": "Complete the Rust tutorial",
  "completed": false
}
```

#### Get All Todos
```http
GET /api/todos
Authorization: Bearer <token>
```

#### Get Single Todo
```http
GET /api/todos/{id}
Authorization: Bearer <token>
```

#### Update Todo
```http
PUT /api/todos/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Learn Rust Advanced",
  "description": "Complete advanced Rust concepts",
  "completed": true
}
```

#### Delete Todo
```http
DELETE /api/todos/{id}
Authorization: Bearer <token>
```

## Project Structure

```
todo-service/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── error.rs             # Error handling
│   ├── database.rs          # Database operations
│   ├── models.rs            # Data models and DTOs
│   ├── auth.rs              # Authentication logic
│   ├── routes.rs            # Route definitions
│   └── handlers/
│       ├── mod.rs           # Handler module exports
│       ├── auth.rs          # Authentication handlers
│       └── todo.rs          # Todo CRUD handlers
├── static/
│   ├── index.html           # Main web page
│   ├── styles.css           # Styling
│   └── script.js            # Frontend JavaScript
├── Dockerfile               # Container build instructions
├── docker-compose.yml       # Multi-container setup
├── init.sql                 # Database schema
├── .env                     # Environment variables
└── Cargo.toml              # Rust dependencies
```

## Environment Variables

Create a `.env` file or set these environment variables:

```env
DATABASE_URL=postgres://todouser:todopass@localhost:5432/todoapp
JWT_SECRET=your-super-secret-jwt-key-change-in-production
RUST_LOG=info
SERVER_HOST=0.0.0.0
SERVER_PORT=3000
```

## Development

### Running Tests
```bash
cargo test
```

### Database Migrations
The application uses SQLx for database operations. The schema is initialized via `init.sql`.

### Code Structure
- **Modular Design**: Each component has its own module
- **Error Handling**: Centralized error handling with custom error types
- **Authentication**: JWT-based authentication with middleware
- **Database**: Async database operations with connection pooling
- **Validation**: Input validation using the validator crate

## Security Features

- 🔒 **Password Hashing**: bcrypt with salt
- 🎫 **JWT Tokens**: Secure authentication tokens
- 🛡️ **Input Validation**: Server-side validation
- 🚫 **SQL Injection Protection**: Parameterized queries
- 🌐 **CORS**: Configurable cross-origin requests

## Default Test User

For testing purposes, a default user is created:
- **Username**: `testuser`
- **Email**: `test@example.com`
- **Password**: `password123`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is open source and available under the [MIT License](LICENSE).