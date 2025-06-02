-- Seed data for app_roles table
INSERT INTO app_roles (id, name, created_at, updated_at) VALUES
    ('550e8400-e29b-41d4-a716-446655440001', 'Super Admin', NOW(), NOW()),
    ('550e8400-e29b-41d4-a716-446655440002', 'Admin', NOW(), NOW()),
    ('550e8400-e29b-41d4-a716-446655440003', 'Manager', NOW(), NOW()),
    ('550e8400-e29b-41d4-a716-446655440004', 'User', NOW(), NOW()),
    ('550e8400-e29b-41d4-a716-446655440005', 'Guest', NOW(), NOW())
ON CONFLICT (id) DO NOTHING;
