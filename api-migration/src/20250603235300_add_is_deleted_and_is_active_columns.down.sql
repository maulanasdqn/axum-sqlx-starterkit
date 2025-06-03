ALTER TABLE app_roles
    DROP COLUMN IF EXISTS is_deleted;

ALTER TABLE app_permissions
    DROP COLUMN IF EXISTS is_deleted;

ALTER TABLE app_users
    DROP COLUMN IF EXISTS is_deleted,
    DROP COLUMN IF EXISTS is_active;

ALTER TABLE app_role_permissions
    DROP COLUMN IF EXISTS is_deleted;
