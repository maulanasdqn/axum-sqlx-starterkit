# 🌱 Database Seeder Usage Guide

This guide shows you how to use the simplified Rust seeder for your Axum SQLx Starter Kit.

## 🚀 Quick Start

### Option 1: Use the Shell Scripts (Recommended)

**Linux/macOS:**

```bash
chmod +x run-seeder.sh
./run-seeder.sh
```

**Windows PowerShell:**

```powershell
.\run-seeder.ps1
```

### Option 2: Use Cargo Directly

```bash
# Run migrations first
sqlx migrate run --source api-migration/src

# Run the seeder
cargo run -p api-migration --bin seeder
```

## 📋 What the Seeder Does

1. **Connects to your database** using the `DATABASE_URL` from your `.env` file
2. **Seeds roles** - Creates 5 default roles (Super Admin, Admin, Manager, User, Guest)
3. **Seeds permissions** - Creates 15 granular permissions for CRUD operations
4. **Seeds role-permission mappings** - Sets up RBAC (Role-Based Access Control)
5. **Seeds sample users** - Creates 5 test users with different roles
6. **Shows a summary** - Displays what was created and login credentials

## 🔑 Sample Login Credentials

After seeding, you can use these credentials to test your API:

| Email                  | Password    | Role        | Permissions          |
| ---------------------- | ----------- | ----------- | -------------------- |
| superadmin@example.com | password123 | Super Admin | All permissions      |
| admin@example.com      | password123 | Admin       | User/role management |
| manager@example.com    | password123 | Manager     | User read/update     |
| user@example.com       | password123 | User        | Profile management   |
| guest@example.com      | password123 | Guest       | Profile read only    |

## 🛠️ Prerequisites

- Your `.env` file must contain a valid `DATABASE_URL`
- PostgreSQL database must be running
- SQLx CLI must be installed: `cargo install sqlx-cli --no-default-features --features postgres`

## 🔄 Re-running the Seeder

The seeder is safe to run multiple times. It uses `ON CONFLICT DO NOTHING` to prevent duplicate entries.

## 📊 Example Output

```
🌱 Starting database seeding...
✅ Connected to database
📝 Seeding roles...
✅ Roles seeded
🔐 Seeding permissions...
✅ Permissions seeded
🔗 Seeding role-permission mappings...
✅ Role-permission mappings seeded
👥 Seeding users...
✅ Users seeded
🎉 All seeders completed successfully!

📊 Seeded data summary:
  - 5 roles created
  - 15 permissions created
  - 5 users created
  - 32 role-permission mappings created

🔑 Sample login credentials:
  Super Admin: superadmin@example.com / password123
  Admin:       admin@example.com / password123
  Manager:     manager@example.com / password123
  User:        user@example.com / password123
  Guest:       guest@example.com / password123
```

## 🚨 Troubleshooting

**Error: "DATABASE_URL must be set"**

- Make sure your `.env` file exists and contains `DATABASE_URL=postgresql://...`

**Error: "Failed to connect to database"**

- Check that PostgreSQL is running
- Verify your database credentials in the `DATABASE_URL`

**Error: "relation does not exist"**

- Run migrations first: `sqlx migrate run --source api-migration/src`

## 🎯 Next Steps

After seeding:

1. Start your API server: `cargo run --bin api`
2. Test the endpoints with the seeded user credentials
3. Build your authentication system using the seeded roles and permissions
