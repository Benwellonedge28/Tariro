-- migrations/20260330170000_platform.sql
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    two_factor_secret TEXT,
    role TEXT NOT NULL DEFAULT 'developer',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS job_postings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    employer_id UUID REFERENCES users(id),
    title TEXT NOT NULL,
    company TEXT NOT NULL,
    salary_min INTEGER,
    salary_max INTEGER,
    currency TEXT DEFAULT 'USD',
    description TEXT,
    is_active BOOLEAN DEFAULT true
);

CREATE TABLE IF NOT EXISTS mentorship_programs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    mentor_id UUID REFERENCES users(id),
    topic TEXT NOT NULL,
    capacity INTEGER DEFAULT 5,
    joined_count INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id),
    amount NUMERIC(10, 2) NOT NULL,
    status TEXT NOT NULL,
    payment_method TEXT, -- e.g. 'EcoCash', 'Stripe'
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
