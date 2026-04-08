-- migrations/20260330220000_umc.sql
CREATE TABLE IF NOT EXISTS compiler_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    project_id UUID,
    source_language TEXT NOT NULL,
    target_architecture TEXT NOT NULL,
    optimization_tier INTEGER DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'queued', -- 'queued', 'lexing', 'parsing', 'optimizing', 'completed', 'failed'
    output_log TEXT,
    binary_url TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS optimization_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID REFERENCES compiler_jobs(id),
    tier INTEGER,
    technique TEXT,
    reduction_pct REAL,
    performance_gain_pct REAL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
