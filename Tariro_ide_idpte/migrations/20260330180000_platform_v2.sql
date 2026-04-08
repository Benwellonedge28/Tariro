-- migrations/20260330180000_platform_v2.sql

-- Gamification & Reputation
CREATE TABLE IF NOT EXISTS badges (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    icon_url TEXT,
    criteria JSONB -- e.g., { "task_completed": 5, "reputation_score_min": 100 }
);

CREATE TABLE IF NOT EXISTS user_badges (
    user_id UUID REFERENCES users(id),
    badge_id UUID REFERENCES badges(id),
    awarded_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, badge_id)
);

CREATE TABLE IF NOT EXISTS reputation_scores (
    user_id UUID PRIMARY KEY REFERENCES users(id),
    score INTEGER DEFAULT 0,
    last_updated TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- Notifications
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    category TEXT -- e.g., 'job_alert', 'mentorship_update', 'chat_message'
);

-- Career Tools & Verification
ALTER TABLE job_postings ADD COLUMN required_skills TEXT[];
ALTER TABLE job_postings ADD COLUMN ai_scam_probability REAL DEFAULT 0.0;
ALTER TABLE job_postings ADD COLUMN contact_email TEXT;

CREATE TABLE IF NOT EXISTS work_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    employer_name TEXT NOT NULL,
    job_title TEXT NOT NULL,
    start_date DATE NOT NULL,
    end_date DATE,
    description TEXT,
    is_verified BOOLEAN DEFAULT false
);

CREATE TABLE IF NOT EXISTS feedback (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_user_id UUID REFERENCES users(id),
    to_user_id UUID REFERENCES users(id),
    rating INTEGER CHECK (rating >=1 AND rating <= 5),
    comment TEXT,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
