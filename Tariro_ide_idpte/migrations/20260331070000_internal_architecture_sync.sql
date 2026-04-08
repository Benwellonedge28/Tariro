-- migrations/20260331070000_internal_architecture_sync.sql

-- Media & Rich Output Cache (Integrated multi-modal)
CREATE TABLE IF NOT EXISTS generated_assets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_type TEXT NOT NULL, -- 'image', 'chart', 'diagram', 'video', 'code_preview'
    prompt TEXT,
    asset_url TEXT,
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Internal Tool Execution Logs
CREATE TABLE IF NOT EXISTS tool_execution_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tool_name TEXT NOT NULL,
    input_payload JSONB,
    output_result JSONB,
    execution_time_ms INTEGER,
    status TEXT DEFAULT 'success',
    executed_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Recursive Cognitive Loops (Self-Correction & Refinement)
CREATE TABLE IF NOT EXISTS cognitive_loops (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    objective TEXT NOT NULL,
    iteration_count INTEGER DEFAULT 1,
    reasoning_chain JSONB, -- Array of thought steps
    final_output TEXT,
    is_converged BOOLEAN DEFAULT FALSE,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
