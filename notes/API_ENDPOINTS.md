# API Endpoints Documentation

## Base URL
```
http://localhost:8000
```

## Authentication Endpoints

### 1. Register User
**POST** `/api/register`

**Request Body:**
```json
{
  "username": "john_doe",
  "password": "secure_password123"
}
```

**Response (201 Created):**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "created_at": "2023-12-01T10:00:00Z"
  }
}
```

### 2. Login User
**POST** `/api/login`

**Request Body:**
```json
{
  "username": "john_doe",
  "password": "secure_password123"
}
```

**Response (200 OK):**
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "username": "john_doe",
    "created_at": "2023-12-01T10:00:00Z"
  }
}
```

### 3. Get Current User
**GET** `/api/me`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "username": "john_doe",
  "created_at": "2023-12-01T10:00:00Z"
}
```

## Notes Endpoints

### 1. Create Note
**POST** `/api/notes`

**Headers:**
```
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "title": "My First Note",
  "content": "This is the content of my note."
}
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "My First Note",
  "content": "This is the content of my note.",
  "created_at": "2023-12-01T10:30:00Z",
  "updated_at": "2023-12-01T10:30:00Z"
}
```

### 2. Get All Notes
**GET** `/api/notes`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response (200 OK):**
```json
{
  "notes": [
    {
      "id": "550e8400-e29b-41d4-a716-446655440001",
      "user_id": "550e8400-e29b-41d4-a716-446655440000",
      "title": "My First Note",
      "content": "This is the content of my note.",
      "created_at": "2023-12-01T10:30:00Z",
      "updated_at": "2023-12-01T10:30:00Z"
    }
  ],
  "total": 1
}
```

### 3. Get Single Note
**GET** `/api/notes/{note_id}`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "My First Note",
  "content": "This is the content of my note.",
  "created_at": "2023-12-01T10:30:00Z",
  "updated_at": "2023-12-01T10:30:00Z"
}
```

### 4. Update Note
**PUT** `/api/notes/{note_id}`

**Headers:**
```
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Request Body:**
```json
{
  "title": "Updated Title",
  "content": "Updated content"
}
```

**Response (200 OK):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "user_id": "550e8400-e29b-41d4-a716-446655440000",
  "title": "Updated Title",
  "content": "Updated content",
  "created_at": "2023-12-01T10:30:00Z",
  "updated_at": "2023-12-01T11:00:00Z"
}
```

### 5. Delete Note
**DELETE** `/api/notes/{note_id}`

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response (204 No Content)**

## Health Check

### Health Check
**GET** `/health`

**Response (200 OK):**
```
OK
```

## Error Responses

All error responses follow this format:

```json
{
  "error": "error_code",
  "message": "Human readable error message"
}
```

### Common HTTP Status Codes:
- `200` - Success
- `201` - Created
- `204` - No Content (for deletions)
- `400` - Bad Request (validation errors)
- `401` - Unauthorized (authentication required)
- `404` - Not Found
- `409` - Conflict (duplicate username)
- `500` - Internal Server Error

### Example Error Responses:

**400 Bad Request:**
```json
{
  "error": "validation_error",
  "message": "Validation failed: username must be at least 3 characters"
}
```

**401 Unauthorized:**
```json
{
  "error": "invalid_credentials",
  "message": "Invalid credentials"
}
```

**404 Not Found:**
```json
{
  "error": "note_not_found",
  "message": "Note not found"
}
```

**409 Conflict:**
```json
{
  "error": "user_exists",
  "message": "Username already exists"
}