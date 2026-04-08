-- migrations/20260330153000_init.sql
CREATE TABLE IF NOT EXISTS files (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    extension TEXT NOT NULL,
    language TEXT NOT NULL,
    path TEXT NOT NULL,
    content_hash TEXT NOT NULL,
    last_maintained_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS evas_audits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    module TEXT NOT NULL,
    action TEXT NOT NULL,
    safety_score REAL NOT NULL,
    alignment_status TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
