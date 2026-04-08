-- migrations/20260331000000_ide_features.sql

-- AGI-Enhanced Code Intelligence
CREATE TABLE IF NOT EXISTS code_insights (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    file_path TEXT NOT NULL,
    insight_type TEXT NOT NULL, -- e.g., 'refactoring_suggestion', 'performance_bottleneck', 'security_vulnerability'
    suggestion TEXT NOT NULL,
    confidence REAL,
    agi_id UUID, -- AGI that generated the insight
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- AGI-Powered VCS Operations
CREATE TABLE IF NOT EXISTS git_suggestions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    repository TEXT NOT NULL,
    suggested_message TEXT NOT NULL,
    changes_summary TEXT,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'applied', 'rejected'
    agi_id UUID,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- AGI-Driven Predictive Debugging & Autonomous Testing
CREATE TABLE IF NOT EXISTS debug_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    project_id TEXT NOT NULL,
    session_type TEXT NOT NULL, -- 'predictive', 'autonomous_test', 'digital_twin'
    bug_prediction TEXT,
    test_results JSONB,
    agi_id UUID,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
