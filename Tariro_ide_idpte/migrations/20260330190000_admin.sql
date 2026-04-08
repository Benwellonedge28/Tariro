-- migrations/20260330190000_admin.sql

-- Admin Settings
CREATE TABLE IF NOT EXISTS admin_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT
);

INSERT INTO admin_settings (key, value, description) VALUES
('platform_name', 'Tariro Omega', 'Name of the sovereign platform'),
('agi_autonomy_level', '0.8', 'Current autonomy level of Tariro AGI (0-1)'),
('max_users', '1000000', 'Maximum number of registered users');

-- Enhanced User Profiles (extending 'users' table implicitly)
ALTER TABLE users ADD COLUMN last_login TIMESTAMPTZ;
ALTER TABLE users ADD COLUMN is_banned BOOLEAN DEFAULT FALSE;
ALTER TABLE users ADD COLUMN reputation_history JSONB DEFAULT '[]';
