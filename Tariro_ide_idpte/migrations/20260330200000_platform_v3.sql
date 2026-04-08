-- migrations/20260330200000_platform_v3.sql

-- Content Moderation
CREATE TABLE IF NOT EXISTS content_flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    content_type TEXT NOT NULL, -- e.g., 'job_posting', 'mentorship_topic', 'chat_message'
    content_id TEXT NOT NULL, -- UUID of the content flagged
    reason TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'reviewed', 'resolved', 'rejected'
    agi_assessment JSONB, -- AGI's autonomous assessment of the flag
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Decentralized Identity (DID)
CREATE TABLE IF NOT EXISTS decentralized_identities (
    did TEXT PRIMARY KEY, -- e.g., did:tariro:12345...
    user_id UUID REFERENCES users(id) UNIQUE,
    public_key_jwk JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS did_attributes (
    did TEXT REFERENCES decentralized_identities(did),
    attribute_key TEXT NOT NULL,
    attribute_value TEXT NOT NULL,
    is_verified BOOLEAN DEFAULT FALSE,
    PRIMARY KEY (did, attribute_key)
);

-- Wellbeing Monitoring
CREATE TABLE IF NOT EXISTS wellbeing_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    mood TEXT, -- e.g., 'excellent', 'stressed', 'neutral'
    energy_level INTEGER, -- 1-10
    focus_quality INTEGER, -- 1-10
    agi_recommendation TEXT,
    logged_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
