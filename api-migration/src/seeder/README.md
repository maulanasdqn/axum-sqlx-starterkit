# Database Seeders

This directory contains SQL seeder files to populate your database with initial data for development and testing.

## 📁 Seeder Files

- **`seed_role.sql`** - Creates default user roles (Super Admin, Admin, Manager, User, Guest)
- **`seed_permission.sql`** - Creates system permissions for CRUD operations
- **`seed_role_permissions.sql`** - Maps permissions to roles (RBAC setup)
- **`seed_user.sql`** - Creates sample users with different roles
- **`run_all_seeders.sql`** - Master file that runs all seeders in correct order

## 🚀 How to Use Seeders

### Method 1: Using PostgreSQL CLI (Recommended)

1. **Make sure your database is running and migrations are applied:**

   ```bash
   # Run migrations first
   sqlx migrate run --source api-migration/src
   ```

2. **Run all seeders at once:**

   ```bash
   # From project root directory
   psql -d your_database_name -f api-migration/src/seeder/run_all_seeders.sql
   ```

3. **Or run individual seeders:**
   ```bash
   # Run seeders in this order:
   psql -d your_database_name -f api-migration/src/seeder/seed_role.sql
   psql -d your_database_name -f api-migration/src/seeder/seed_permission.sql
   psql -d your_database_name -f api-migration/src/seeder/seed_role_permissions.sql
   psql -d your_database_name -f api-migration/src/seeder/seed_user.sql
   ```

### Method 2: Using Database URL

```bash
# Replace with your actual DATABASE_URL
export DATABASE_URL="postgresql://username:password@localhost/database_name"

# Run all seeders
psql $DATABASE_URL -f api-migration/src/seeder/run_all_seeders.sql
```

### Method 3: Copy and Paste

1. Connect to your database using your preferred SQL client
2. Copy the contents of each seeder file and execute them in order:
   - `seed_role.sql`
   - `seed_permission.sql`
   - `seed_role_permissions.sql`
   - `seed_user.sql`

## 📊 Seeded Data Overview

### Roles Created

- **Super Admin** - Full system access
- **Admin** - User and role management
- **Manager** - User read/update permissions
- **User** - Basic profile permissions
- **Guest** - Read-only profile access

### Permissions Created

- **User Management**: `users.create`, `users.read`, `users.update`, `users.delete`
- **Role Management**: `roles.create`, `roles.read`, `roles.update`, `roles.delete`
- **Permission Management**: `permissions.create`, `permissions.read`, `permissions.update`, `permissions.delete`
- **System**: `system.admin`
- **Profile**: `profile.read`, `profile.update`

### Sample Users Created

| Email                  | Password    | Role        | Full Name           |
| ---------------------- | ----------- | ----------- | ------------------- |
| superadmin@example.com | password123 | Super Admin | Super Administrator |
| admin@example.com      | password123 | Admin       | System Admin        |
| manager@example.com    | password123 | Manager     | John Manager        |
| user@example.com       | password123 | User        | Jane User           |
| guest@example.com      | password123 | Guest       | Guest User          |

## 🔒 Security Notes

- **Change default passwords** in production
- All seeded passwords are hashed using bcrypt
- The hash `$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/VcSAg9S6O` represents "password123"
- Use proper password policies in production

## 🔄 Re-running Seeders

The seeders use `ON CONFLICT (id) DO NOTHING` to prevent duplicate entries. You can safely re-run them multiple times.

To reset and re-seed:

```sql
-- Clear existing data (be careful in production!)
TRUNCATE app_role_permissions, app_users, app_permissions, app_roles RESTART IDENTITY CASCADE;

-- Then re-run seeders
\i api-migration/src/seeder/run_all_seeders.sql
```

## 🛠️ Customizing Seeders

1. **Adding new roles**: Edit `seed_role.sql`
2. **Adding new permissions**: Edit `seed_permission.sql`
3. **Changing role-permission mappings**: Edit `seed_role_permissions.sql`
4. **Adding new users**: Edit `seed_user.sql`

Remember to use unique UUIDs for new entries and maintain referential integrity.

## 📝 Verification

After running seeders, verify the data:

```sql
-- Check roles
SELECT * FROM app_roles;

-- Check permissions
SELECT * FROM app_permissions;

-- Check role-permission mappings
SELECT r.name as role, p.name as permission
FROM app_role_permissions rp
JOIN app_roles r ON rp.role_id = r.id
JOIN app_permissions p ON rp.permission_id = p.id
ORDER BY r.name, p.name;

-- Check users
SELECT u.fullname, u.email, r.name as role
FROM app_users u
JOIN app_roles r ON u.role_id = r.id;
```

## 🚨 Troubleshooting

**Error: "relation does not exist"**

- Make sure you've run migrations first: `sqlx migrate run --source api-migration/src`

**Error: "duplicate key value violates unique constraint"**

- The seeders are designed to handle duplicates. This error suggests the data already exists.

**Error: "permission denied"**

- Make sure your database user has INSERT permissions on all tables.

**Error: "foreign key constraint violation"**

- Run seeders in the correct order (roles → permissions → role_permissions → users)
