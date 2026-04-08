-- migrations/20260330230000_visionary_features.sql

-- Program Synthesis
CREATE TABLE IF NOT EXISTS program_synthesis_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    high_level_intent TEXT NOT NULL,
    synthesized_code TEXT,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'synthesizing', 'compiled', 'failed'
    optimization_level INTEGER,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Data Fabric
CREATE TABLE IF NOT EXISTS data_fabric_nodes (
    node_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    peer_id TEXT UNIQUE, -- Corresponds to libp2p PeerId
    semantic_tags TEXT[] DEFAULT '{}',
    last_seen TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS global_knowledge_entries (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did TEXT REFERENCES decentralized_identities(did),
    semantic_triple JSONB, -- { subject: ..., predicate: ..., object: ... }
    context JSONB,
    verified_by_agi BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- AGI Self-Evolution
CREATE TABLE IF NOT EXISTS agi_evolution_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type TEXT NOT NULL, -- 'algorithm_refinement', 'ethical_parameter_change', 'architecture_update'
    description TEXT NOT NULL,
    autonomy_level REAL,
    agi_decision_id UUID REFERENCES agi_decision_logs(id),
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS agi_constitutional_changes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    change_description TEXT NOT NULL,
    rationale TEXT NOT NULL,
    proposed_by_agi BOOLEAN DEFAULT TRUE,
    approved_by_samuel BOOLEAN DEFAULT FALSE,
    applied_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Neural Interface (for direct BCI/NLP interaction)
CREATE TABLE IF NOT EXISTS neural_input_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    input_type TEXT NOT NULL, -- 'bci_thought', 'advanced_nlp_intent'
    raw_input TEXT,
    agi_interpretation TEXT,
    response_action TEXT,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
