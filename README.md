# Rust Advanced API

A production-ready REST API built with Rust, featuring async web server, PostgreSQL database integration, JWT authentication, and comprehensive error handling.

## Features

- **Async Web Server**: Built with Axum framework for high performance
- **Database Integration**: PostgreSQL with SQLx for type-safe queries
- **Authentication**: JWT-based authentication with bcrypt password hashing
- **Validation**: Request validation using the validator crate
- **Error Handling**: Comprehensive error handling with custom error types
- **Logging**: Structured logging with tracing
- **CORS**: Cross-origin resource sharing support
- **Migrations**: Database migrations with SQLx

## API Endpoints

### Health Check
- `GET /health` - Health check endpoint

### Authentication
- `POST /api/auth/register` - Register a new user
- `POST /api/auth/login` - Login user

### Users (Protected)
- `GET /api/users` - List all users
- `GET /api/users/:id` - Get user by ID
- `POST /api/users/:id` - Update user

## Setup

1. **Prerequisites**
   - Rust 1.70+
   - PostgreSQL
   - sqlx-cli: `cargo install sqlx-cli`

2. **Environment Setup**
   ```bash
   cp .env.example .env
   # Edit .env with your database credentials
   ```

3. **Database Setup**
   ```bash
   # Create database
   createdb rust_advanced_api
   
   # Run migrations
   sqlx migrate run
   ```

4. **Run the Application**
   ```bash
   cargo run
   ```

## Project Structure

```
src/
├── auth/           # Authentication logic (JWT, password hashing)
├── config/         # Configuration management
├── database/       # Database connection and setup
├── handlers/       # HTTP request handlers
├── middleware/     # Custom middleware (auth, etc.)
├── models/         # Data models and DTOs
├── utils/          # Utility functions and error handling
└── main.rs         # Application entry point
```

## Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `PORT`: Server port (default: 3000)
- `JWT_SECRET`: Secret key for JWT signing
- `BCRYPT_COST`: Bcrypt hashing cost (default: 12)

## Testing

```bash
cargo test
```

## Building for Production

```bash
cargo build --release
```