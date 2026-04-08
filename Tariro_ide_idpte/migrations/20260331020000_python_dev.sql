-- migrations/20260331020000_python_dev.sql

-- Python Development specific tables
CREATE TABLE IF NOT EXISTS python_projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    name TEXT NOT NULL,
    description TEXT,
    framework TEXT, -- e.g., 'Django', 'Flask', 'Pyramid'
    python_version TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS python_modules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID REFERENCES python_projects(id),
    name TEXT NOT NULL,
    file_path TEXT NOT NULL,
    dependencies JSONB, -- Array of other module IDs/names
    last_agi_update TIMESTAMPTZ,
    agi_managed BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS python_learning_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    event_type TEXT NOT NULL, -- 'code_analysis', 'feedback', 'pattern_learned'
    details JSONB,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
