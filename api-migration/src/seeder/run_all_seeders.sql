-- Master seeder file - Run all seeders in correct order
-- This file executes all seeder files in the proper dependency order

-- 1. First seed roles (no dependencies)
\i api-migration/src/seeder/seed_role.sql

-- 2. Then seed permissions (no dependencies)
\i api-migration/src/seeder/seed_permission.sql

-- 3. Then seed role_permissions (depends on roles and permissions)
\i api-migration/src/seeder/seed_role_permissions.sql

-- 4. Finally seed users (depends on roles)
\i api-migration/src/seeder/seed_user.sql

-- Display completion message
SELECT 'All seeders executed successfully!' as message;
