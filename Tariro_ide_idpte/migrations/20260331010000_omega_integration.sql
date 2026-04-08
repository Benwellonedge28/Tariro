-- migrations/20260331010000_omega_integration.sql

-- News & Feed Engine
CREATE TABLE IF NOT EXISTS sovereign_news_feeds (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    summary TEXT,
    source_url TEXT,
    category TEXT, -- 'tech', 'sovereignty', 'omega_updates'
    evas_verified BOOLEAN DEFAULT TRUE,
    published_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Simulator & Device Logs
CREATE TABLE IF NOT EXISTS simulator_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    device_type TEXT NOT NULL, -- 'phone', 'desktop', 'nano', 'iot'
    status TEXT DEFAULT 'running',
    captured_logs TEXT,
    started_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Kernel Metadata (Syscalls)
CREATE TABLE IF NOT EXISTS omega_syscalls (
    syscall_number INTEGER PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    category TEXT -- 'file_io', 'process_mgmt', 'mesh_comm'
);

INSERT INTO omega_syscalls (syscall_number, name, description, category) VALUES
(0, 'SYS_SHONA_READ', 'Sovereign read with Shona language priority', 'file_io'),
(1, 'SYS_EVAS_AUDIT', 'Real-time ethical audit of calling process', 'process_mgmt'),
(2, 'SYS_MESH_HANDSHAKE', 'Initiate P2P mesh handshake', 'mesh_comm');
