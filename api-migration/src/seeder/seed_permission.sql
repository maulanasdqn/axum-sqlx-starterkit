-- Seed data for app_permissions table
INSERT INTO app_permissions (id, name, description, created_at, updated_at) VALUES
    ('660e8400-e29b-41d4-a716-446655440001', 'users.create', 'Create new users', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440002', 'users.read', 'View user information', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440003', 'users.update', 'Update user information', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440004', 'users.delete', 'Delete users', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440005', 'roles.create', 'Create new roles', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440006', 'roles.read', 'View role information', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440007', 'roles.update', 'Update role information', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440008', 'roles.delete', 'Delete roles', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440009', 'permissions.create', 'Create new permissions', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440010', 'permissions.read', 'View permission information', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440011', 'permissions.update', 'Update permission information', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440012', 'permissions.delete', 'Delete permissions', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440013', 'system.admin', 'Full system administration access', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440014', 'profile.read', 'View own profile', NOW(), NOW()),
    ('660e8400-e29b-41d4-a716-446655440015', 'profile.update', 'Update own profile', NOW(), NOW())
ON CONFLICT (id) DO NOTHING;
