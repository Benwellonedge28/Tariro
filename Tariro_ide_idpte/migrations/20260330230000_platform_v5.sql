-- migrations/20260330230000_platform_v5.sql

-- Program Synthesis & Intent-Based Development
CREATE TABLE IF NOT EXISTS synthesis_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    intent_hash TEXT NOT NULL, -- Hash of the input intent specification
    target_language TEXT NOT NULL,
    target_architecture TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'synthesizing', 'compiled', 'failed'
    generated_code TEXT,
    compilation_log TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS intent_specifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    name TEXT NOT NULL,
    specification TEXT NOT NULL, -- Natural language or formal spec
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Sovereign Data Fabric & Global Knowledge Graph
CREATE TABLE IF NOT EXISTS semantic_data_nodes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    did TEXT NOT NULL, -- DID of the data owner
    data_type TEXT NOT NULL, -- e.g., 'code_pattern', 'domain_ontology', 'cultural_context'
    content JSONB NOT NULL, -- The semantic data itself (e.g., RDF, OWL, custom JSON)
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS data_fabric_relations (
    source_node_id UUID REFERENCES semantic_data_nodes(id),
    target_node_id UUID REFERENCES semantic_data_nodes(id),
    relation_type TEXT NOT NULL, -- e.g., 'relatesTo', 'inheritsFrom', 'similarTo'
    strength REAL,
    PRIMARY KEY (source_node_id, target_node_id, relation_type)
);

-- AGI Self-Evolution & Constitutional Learning
CREATE TABLE IF NOT EXISTS agi_evolution_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agi_module TEXT NOT NULL, -- e.g., 'EVAS', 'CognitiveArch', 'LearningAlg'
    event_type TEXT NOT NULL, -- e.g., 'self_refinement', 'parameter_update', 'constitutional_amendment'
    description TEXT NOT NULL,
    details JSONB,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS constitutional_parameters (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    last_modified_agi_id UUID -- AGI that proposed the change
);

-- Immersive Neural-Linguistic Programming (NLP) Interface
CREATE TABLE IF NOT EXISTS neural_inputs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    input_type TEXT NOT NULL, -- e.g., 'brainwave_pattern', 'eye_gaze_vector', 'speech_intonation'
    raw_data JSONB NOT NULL,
    processed_intent TEXT,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
