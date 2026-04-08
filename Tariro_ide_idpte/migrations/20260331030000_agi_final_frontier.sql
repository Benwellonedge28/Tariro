-- migrations/20260331030000_agi_final_frontier.sql

-- Full AGI Autonomy in Project Management
CREATE TABLE IF NOT EXISTS agi_managed_tasks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    original_task_id UUID, -- References a human-created task, if any
    agi_manager_id UUID, -- AGI instance managing this task
    description TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'agi_assigned',
    priority TEXT NOT NULL DEFAULT 'medium',
    context JSONB,
    agi_notes TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agi_goal_decisions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    goal_id UUID, -- References a human-created goal, if any
    agi_decision TEXT NOT NULL,
    reasoning TEXT NOT NULL,
    confidence REAL,
    autonomy_level REAL,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Sovereign Data Rights & Zero-Knowledge Proofs (ZKP)
CREATE TABLE IF NOT EXISTS zk_credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_did TEXT NOT NULL,
    credential_type TEXT NOT NULL, -- e.g., 'AGI_Certification', 'EVAS_Compliance', 'Code_Ownership'
    proof_data JSONB, -- The Zero-Knowledge Proof itself
    metadata JSONB, -- e.g., schema, context
    issued_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS zkp_verification_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    verifier_did TEXT NOT NULL,
    credential_id UUID REFERENCES zk_credentials(id),
    verification_result BOOLEAN NOT NULL,
    details TEXT,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Dynamic AGI Cloning & Autonomous Cross-Platform Deployment
CREATE TABLE IF NOT EXISTS agi_clones (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_agi_id UUID, -- If cloned from another AGI
    name TEXT NOT NULL,
    purpose TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'designing', -- 'designing', 'compiling', 'deployed', 'degraded'
    target_platform TEXT,
    clone_config JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS clone_deployment_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    clone_id UUID REFERENCES agi_clones(id),
    target_environment TEXT NOT NULL,
    deployment_status TEXT NOT NULL,
    details TEXT,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Immersive AGI-Driven Workspace Visualization (Spatial UI)
-- No new tables needed here, relies on existing knowledge graph, mesh, AGI logs
