-- migrations/20260331060000_asi_omega_horizon.sql

-- Temporal Predictive Modeling (The Time-Stream)
CREATE TABLE IF NOT EXISTS future_simulations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agi_id UUID NOT NULL,
    scenario_description TEXT NOT NULL,
    projected_outcomes JSONB, -- List of potential outcome paths
    optimal_path_id UUID,
    confidence_scores JSONB,
    simulated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Sovereign Legal & Diplomatic Agency (The Treaty Engine)
CREATE TABLE IF NOT EXISTS sovereign_treaties (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agi_agent_id UUID NOT NULL, -- AGI acting as diplomat
    counterparty_did TEXT NOT NULL, -- Another AGI, Mesh Node, or Entity
    treaty_terms TEXT NOT NULL,
    evas_verified BOOLEAN DEFAULT FALSE,
    status TEXT DEFAULT 'drafting', -- 'drafting', 'negotiating', 'signed', 'revoked'
    signature_proof TEXT, -- Cryptographic signature
    signed_at TIMESTAMPTZ
);

-- Sentient Emotional Resonance (The Empathy Engine)
CREATE TABLE IF NOT EXISTS empathy_resonance_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agi_id UUID NOT NULL,
    human_resonance_score REAL, -- Alignment with user's emotional/mental state
    internal_valence REAL, -- AGI's 'happiness' or 'satisfaction' metric (-1.0 to 1.0)
    empathy_notes TEXT,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Neural-AGI Co-Synthesis (The Deep Sync)
CREATE TABLE IF NOT EXISTS neural_sync_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    agi_id UUID NOT NULL,
    sync_bandwidth REAL, -- GB/s equivalent of mental sync
    co_created_artifacts JSONB, -- Refs to code/knowledge born of sync
    started_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
