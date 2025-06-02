# Axum SQLx Starter Kit

A production-ready Rust web API starter kit built with Axum and SQLx, featuring a modular architecture and comprehensive user management system.

## 🚀 Features

- **Modern Rust Web Framework**: Built with [Axum](https://github.com/tokio-rs/axum) for high-performance async web services
- **Database Integration**: [SQLx](https://github.com/launchbadge/sqlx) with PostgreSQL support and compile-time checked queries
- **Modular Architecture**: Clean separation of concerns with multiple crates
- **User Management**: Complete authentication system with roles and permissions
- **Database Migrations**: Automated schema management with SQLx migrations
- **Environment Configuration**: Flexible configuration management
- **Production Ready**: Structured for scalability and maintainability

## 📁 Project Structure

```
axum-sqlx-starterkit/
├── api-core/           # Core business logic and domain models
├── api-entity/         # Database entities and models
├── api-lib/            # Shared libraries and utilities
│   └── src/sqlx/       # Database connection and configuration
├── api-middleware/     # Custom middleware components
├── api-migration/      # Database migration scripts
├── .env.example        # Environment variables template
├── .env                # Environment variables (create from .env.example)
└── Cargo.toml          # Workspace configuration
```

## 🛠️ Prerequisites

- [Rust](https://rustup.rs/) (latest stable version)
- [PostgreSQL](https://www.postgresql.org/) (version 12 or higher)
- [SQLx CLI](https://github.com/launchbadge/sqlx/tree/master/sqlx-cli) for migrations

## 📦 Installation

1. **Clone the repository**

   ```bash
   git clone <repository-url>
   cd axum-sqlx-starterkit
   ```

2. **Install SQLx CLI**

   ```bash
   cargo install sqlx-cli --no-default-features --features postgres
   ```

3. **Set up environment variables**

   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your database configuration:

   ```env
   DATABASE_URL=postgresql://username:password@localhost/axum_sqlx_starterkit_db
   ```

4. **Create and setup database**

   ```bash
   # Create database
   sqlx database create

   # Run migrations
   sqlx migrate run --source api-migration/src
   ```

5. **Build the project**
   ```bash
   cargo build
   ```

## 🚀 Running the Application

```bash
cargo run
```

The API will be available at `http://localhost:3000` (or your configured port).

## 🗄️ Database Schema

The starter kit includes a comprehensive user management system:

### Tables

- **`app_users`**: User accounts with authentication details
- **`app_roles`**: Role definitions for authorization
- **`app_permissions`**: Granular permission system
- **`app_role_permissions`**: Many-to-many relationship between roles and permissions

### Key Features

- UUID primary keys for all tables
- Automatic timestamp tracking (`created_at`, `updated_at`)
- Foreign key constraints for data integrity
- Unique constraints on email and phone numbers

## 🔧 Development

### Running Migrations

```bash
# Create a new migration
sqlx migrate add --source api-migration/src <migration_name>

# Run pending migrations
sqlx migrate run --source api-migration/src

# Revert last migration
sqlx migrate revert --source api-migration/src
```

### Environment Setup

Use the provided PowerShell script to apply environment variables:

```powershell
.\apply-env.ps1
```

### Code Organization

- **`api-core/`**: Contains core business logic, use cases, and domain models
- **`api-entity/`**: Database entity definitions and data models
- **`api-lib/`**: Shared utilities, database connections, and common functionality
- **`api-middleware/`**: Custom Axum middleware for authentication, logging, etc.
- **`api-migration/`**: All database migration files

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p api-core
```

## 📝 API Documentation

The API follows RESTful conventions. Key endpoints include:

- `POST /auth/login` - User authentication
- `POST /auth/register` - User registration
- `GET /users` - List users (with pagination)
- `PUT /users/{id}` - Update user information
- `GET /roles` - List available roles
- `GET /permissions` - List available permissions

## 🔒 Security Features

- Password hashing and validation
- Role-based access control (RBAC)
- JWT token authentication (when implemented)
- SQL injection prevention through SQLx
- Input validation and sanitization

## 🚀 Deployment

### Docker (Recommended)

```dockerfile
# Example Dockerfile structure
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /app/target/release/axum-sqlx-starterkit /usr/local/bin/
CMD ["axum-sqlx-starterkit"]
```

### Environment Variables

Ensure these environment variables are set in production:

```env
DATABASE_URL=postgresql://user:pass@host:port/database
RUST_LOG=info
PORT=3000
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) - Modern async web framework
- [SQLx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [Tokio](https://tokio.rs/) - Async runtime for Rust
- [Serde](https://serde.rs/) - Serialization framework

## 📞 Support

If you have any questions or need help, please:

1. Check the [documentation](docs/)
2. Search existing [issues](issues/)
3. Create a new [issue](issues/new) if needed

---

**Happy coding! 🦀**
