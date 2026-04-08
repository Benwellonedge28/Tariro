-- migrations/20260330160000_plugins.sql
CREATE TABLE IF NOT EXISTS plugins (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    is_active BOOLEAN DEFAULT false,
    config JSONB DEFAULT '{}',
    installed_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO plugins (name, is_active, config) VALUES 
('EcoCash Pay Integration', false, '{"provider": "Steward Bank", "currency": "ZWL"}'),
('Shona Semantic Analyzer', true, '{"dialect": "Standard"}'),
('Omega Solar Optimizer', false, '{"threshold": 0.8}');
