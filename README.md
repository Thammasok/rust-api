# Rust API - Clean Architecture Example

A production-ready REST API built with Rust and Axum, following clean architecture principles.

## Project Structure

```
rust-api/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config/              # Configuration management
│   │   └── mod.rs
│   ├── models/              # Data structures
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── response.rs
│   ├── errors/              # Error handling
│   │   └── mod.rs
│   ├── repositories/        # Data access layer
│   │   ├── mod.rs
│   │   └── user_repository.rs
│   ├── services/            # Business logic layer
│   │   ├── mod.rs
│   │   └── user_service.rs
│   ├── handlers/            # HTTP handlers (controllers)
│   │   ├── mod.rs
│   │   ├── health.rs
│   │   └── user_handler.rs
│   ├── routes/              # Route configuration
│   │   └── mod.rs
│   └── middleware/          # Custom middleware
│       ├── mod.rs
│       ├── logging.rs
│       └── auth.rs
├── Cargo.toml
├── .env.example
└── README.md
```

## Architecture Layers

### 1. **Handlers** (HTTP Layer)
- Handle HTTP requests/responses
- Parse request bodies and path parameters
- Return appropriate status codes
- Located in `src/handlers/`

### 2. **Services** (Business Logic Layer)
- Implement business rules and validation
- Coordinate between handlers and repositories
- Error handling and business logic
- Located in `src/services/`

### 3. **Repositories** (Data Access Layer)
- Manage data persistence
- Abstract database operations
- PostgreSQL database with sqlx
- Located in `src/repositories/`

### 4. **Models** (Data Structures)
- Define request/response structures
- Domain entities
- Located in `src/models/`

## API Endpoints

### Health Checks
- `GET /` - Root endpoint
- `GET /health` - Health check endpoint

### User Management
- `GET /api/users` - Get all users
- `POST /api/users` - Create a new user
- `GET /api/users/:id` - Get user by ID
- `PUT /api/users/:id` - Update user
- `DELETE /api/users/:id` - Delete user

## Getting Started

### Prerequisites
- Rust 1.70 or higher
- Cargo
- PostgreSQL 12 or higher

### Installation

1. Clone the repository

```bash
git clone <your-repo-url>
cd rust-api
```

2. Setup PostgreSQL database

```bash
# Create a new database
createdb rust_api_db

# Or using psql
psql -U postgres -c "CREATE DATABASE rust_api_db;"
```

3. Copy environment variables

```bash
cp .env.example .env
```

4. Update the DATABASE_URL in .env file

```
DATABASE_URL=postgres://username:password@localhost/rust_api_db
```

5. Build the project

```bash
cargo build
```

6. Run the server (migrations will run automatically)

```bash
cargo run
```

The server will start on `http://0.0.0.0:3000`

## Example Requests

### Create a user
```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "name": "John Doe",
    "email": "john@example.com"
  }'
```

### Get all users
```bash
curl http://localhost:3000/api/users
```

### Get user by ID
```bash
# Replace {uuid} with actual user UUID from create response
curl http://localhost:3000/api/users/{uuid}
```

### Update user
```bash
# Replace {uuid} with actual user UUID
curl -X PUT http://localhost:3000/api/users/{uuid} \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jane Doe",
    "email": "jane@example.com"
  }'
```

### Delete user
```bash
# Replace {uuid} with actual user UUID
curl -X DELETE http://localhost:3000/api/users/{uuid}
```

## Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SERVER_HOST` | Server bind address | `0.0.0.0` |
| `SERVER_PORT` | Server port | `3000` |
| `DATABASE_URL` | Database connection string | `postgres://localhost/mydb` |
| `JWT_SECRET` | JWT secret key | `your-secret-key` |

## Features

- ✅ Clean Architecture (Handlers → Services → Repositories)
- ✅ RESTful API design
- ✅ Error handling with custom error types
- ✅ Request/Response validation
- ✅ Logging middleware
- ✅ Authentication middleware (example)
- ✅ PostgreSQL database with sqlx
- ✅ Automatic database migrations
- ✅ UUID-based primary keys
- ✅ Type-safe with Rust
- ✅ Async/await with Tokio

## Database Schema

### Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

## Next Steps

To make this production-ready, consider adding:

1. **Authentication & Authorization**
   - Implement JWT authentication
   - Add role-based access control

2. **Testing**
   - Unit tests for services
   - Integration tests for API endpoints

3. **Documentation**
   - Add OpenAPI/Swagger documentation
   - API documentation with `utoipa`

4. **Observability**
   - Structured logging with `tracing`
   - Metrics with Prometheus
   - Distributed tracing

5. **Deployment**
   - Docker containerization
   - Kubernetes manifests
   - CI/CD pipeline

## License

MIT
