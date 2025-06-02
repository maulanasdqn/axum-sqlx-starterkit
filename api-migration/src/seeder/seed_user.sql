-- Seed data for app_users table
-- Note: Passwords should be hashed in a real application
INSERT INTO app_users (id, fullname, email, phone_number, password, role_id, created_at, updated_at) VALUES
    ('880e8400-e29b-41d4-a716-446655440001', 'Super Administrator', 'superadmin@example.com', '+1234567890', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/VcSAg9S6O', '550e8400-e29b-41d4-a716-446655440001', NOW(), NOW()),
    ('880e8400-e29b-41d4-a716-446655440002', 'System Admin', 'admin@example.com', '+1234567891', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/VcSAg9S6O', '550e8400-e29b-41d4-a716-446655440002', NOW(), NOW()),
    ('880e8400-e29b-41d4-a716-446655440003', 'John Manager', 'manager@example.com', '+1234567892', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/VcSAg9S6O', '550e8400-e29b-41d4-a716-446655440003', NOW(), NOW()),
    ('880e8400-e29b-41d4-a716-446655440004', 'Jane User', 'user@example.com', '+1234567893', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/VcSAg9S6O', '550e8400-e29b-41d4-a716-446655440004', NOW(), NOW()),
    ('880e8400-e29b-41d4-a716-446655440005', 'Guest User', 'guest@example.com', '+1234567894', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/VcSAg9S6O', '550e8400-e29b-41d4-a716-446655440005', NOW(), NOW())
ON CONFLICT (id) DO NOTHING;

-- Note: All passwords above are hashed version of "password123"
-- In production, use proper password hashing with unique salts
