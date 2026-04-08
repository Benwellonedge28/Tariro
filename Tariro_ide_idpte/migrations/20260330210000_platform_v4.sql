-- migrations/20260330210000_platform_v4.sql

-- Community & Marketplace
CREATE TABLE IF NOT EXISTS marketplace_plugins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID REFERENCES users(id),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    version TEXT NOT NULL DEFAULT '1.0.0',
    price NUMERIC(10, 2) DEFAULT 0.0,
    is_approved BOOLEAN DEFAULT FALSE,
    downloads INTEGER DEFAULT 0,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS community_projects (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID REFERENCES users(id),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    visibility TEXT NOT NULL DEFAULT 'public', -- 'public', 'private', 'mesh_only'
    members JSONB DEFAULT '[]', -- Array of user_ids
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS discussion_forums (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS forum_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    thread_id UUID REFERENCES discussion_forums(id),
    posted_by UUID REFERENCES users(id),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- AGI Autonomy & Scaling
CREATE TABLE IF NOT EXISTS agi_decision_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    decision TEXT NOT NULL,
    reasoning TEXT NOT NULL,
    autonomy_level REAL NOT NULL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS resource_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    component TEXT NOT NULL, -- e.g., 'API Server', 'AGI Core', 'Database Shard'
    cpu_usage REAL,
    memory_usage REAL,
    network_io REAL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
