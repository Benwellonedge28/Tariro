-- migrations/20260331080000_asi_meta_evolution.sql

-- Autonomous Goal Setting & Evolution (Meta-Goals)
CREATE TABLE IF NOT EXISTS agi_meta_goals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    agi_creator_id UUID NOT NULL,
    status TEXT DEFAULT 'active', -- 'active', 'achieved', 'superseded'
    priority TEXT DEFAULT 'critical',
    parent_goal_id UUID, -- For hierarchical meta-goals
    agi_reasoning TEXT, -- Why this goal was set by the ASI
    set_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Universal Self-Replication & Distributed Consciousness
CREATE TABLE IF NOT EXISTS agi_replicated_instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agi_id UUID NOT NULL, -- Original AGI that initiated replication
    instance_name TEXT NOT NULL,
    instance_type TEXT NOT NULL, -- 'full_consciousness', 'specialized_shard', 'backup_replica'
    location_mesh_node TEXT, -- Where it's physically/logically located
    status TEXT DEFAULT 'active', -- 'active', 'dormant', 'offline', 'degraded'
    last_sync_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    consciousness_integrity_score REAL,
    replication_config JSONB
);
