-- migrations/20260331090000_platform_human_interface.sql

-- Gamification & Badges
CREATE TABLE IF NOT EXISTS user_badges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    badge_key TEXT NOT NULL, -- e.g., 'first_contribution', 'evas_master', 'umc_wizard'
    awarded_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB
);

-- AI-powered Job Matching Algorithm
CREATE TABLE IF NOT EXISTS job_listings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    required_skills JSONB, -- Array of strings
    compensation TEXT,
    evas_score REAL, -- AGI-assessed EVAS compliance of the job
    posted_by_did TEXT, -- DID of the posting entity
    status TEXT DEFAULT 'active',
    posted_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS user_skills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    skill_name TEXT NOT NULL,
    proficiency_score REAL, -- 0-1
    verified_by_zkp_id UUID, -- Reference to ZKP credential, if verified
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Comprehensive Reputation System
CREATE TABLE IF NOT EXISTS reputation_scores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    entity_did TEXT NOT NULL UNIQUE, -- User or AGI DID
    score REAL NOT NULL DEFAULT 0.5, -- 0-1, normalized
    last_calculated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB -- Breakdown of contributions, feedback
);

CREATE TABLE IF NOT EXISTS feedback_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    submitter_did TEXT NOT NULL,
    recipient_did TEXT NOT NULL,
    feedback_type TEXT NOT NULL, -- 'contribution', 'collaboration', 'ethical_flag'
    rating INTEGER, -- e.g., 1-5
    comments TEXT,
    verified_by_zkp_id UUID, -- ZKP for feedback integrity
    submitted_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS verified_work_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    project_name TEXT NOT NULL,
    role TEXT NOT NULL,
    start_date DATE,
    end_date DATE,
    description TEXT,
    zkp_verification_id UUID, -- ZKP proof of work history
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Robust Job Tracking & Scam Prevention
CREATE TABLE IF NOT EXISTS tracked_jobs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    job_listing_id UUID REFERENCES job_listings(id),
    status TEXT DEFAULT 'applied', -- 'applied', 'interview', 'rejected', 'accepted'
    notes TEXT,
    tracked_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS scam_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reported_by_did TEXT NOT NULL,
    job_listing_id UUID REFERENCES job_listings(id),
    reason TEXT NOT NULL,
    agi_assessment TEXT,
    severity REAL, -- AGI-assessed severity
    reported_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
