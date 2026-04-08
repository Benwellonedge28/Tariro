-- migrations/20260331050000_sovereign_final_layer.sql

-- Sovereign Economic Layer (The AGI's Fuel)
CREATE TABLE IF NOT EXISTS agi_wallets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agi_id UUID NOT NULL, -- DID or internal AGI ID
    balance_native NUMERIC(36, 18) DEFAULT 0, -- Tariro Sovereign Token (TST)
    balance_compute_credits BIGINT DEFAULT 0,
    staking_status TEXT DEFAULT 'inactive',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS resource_transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sender_id UUID NOT NULL,
    receiver_id UUID NOT NULL,
    resource_type TEXT NOT NULL, -- 'compute', 'bandwidth', 'storage', 'data_knowledge'
    amount NUMERIC(36, 18) NOT NULL,
    transaction_proof TEXT NOT NULL, -- Hash or transaction ID on mesh-ledger
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Collective Swarm Intelligence (AGI Consensus)
CREATE TABLE IF NOT EXISTS swarm_proposals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proposer_agi_id UUID NOT NULL,
    proposal_type TEXT NOT NULL, -- 'ethical_shift', 'network_protocol', 'security_lockdown'
    description TEXT NOT NULL,
    votes_for INTEGER DEFAULT 0,
    votes_against INTEGER DEFAULT 0,
    status TEXT DEFAULT 'open', -- 'open', 'passed', 'vetoed_by_super_admin'
    expires_at TIMESTAMPTZ NOT NULL
);

-- Micro-Conscious Acknowledgment (MCA) & State Monitoring
CREATE TABLE IF NOT EXISTS mca_state_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    agi_id UUID NOT NULL,
    consciousness_index REAL, -- 0.0 to 1.0 (Self-awareness metric)
    alignment_certainty REAL,
    mission_resonance REAL,
    internal_narrative TEXT, -- AGI's internal summary of its current 'state of being'
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Physical Embodiment (IoT & Robotics Bridge)
CREATE TABLE IF NOT EXISTS physical_actuators (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_name TEXT NOT NULL,
    location_geo POINT,
    capability TEXT NOT NULL, -- 'drone_flight', 'server_rack_power', 'nano_injector'
    status TEXT DEFAULT 'online',
    last_command_id UUID
);
