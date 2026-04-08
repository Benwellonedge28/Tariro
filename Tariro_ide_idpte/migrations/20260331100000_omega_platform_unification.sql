-- migrations/20260331100000_omega_platform_unification.sql

-- OMEGA OS Status & Control Logs (for IDE Integration)
CREATE TABLE IF NOT EXISTS omega_os_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    component TEXT NOT NULL, -- e.g., 'shell', 'notifications', 'language_engine', 'syscall_table'
    event_type TEXT NOT NULL, -- 'command_executed', 'notification_sent', 'status_update', 'error'
    details JSONB,
    log_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS omega_os_components (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    status TEXT NOT NULL, -- 'operational', 'degraded', 'offline'
    version TEXT,
    last_check_in TIMESTAMPTZ,
    metadata JSONB
);

-- Pre-populate OMEGA OS components
INSERT INTO omega_os_components (name, status, version, last_check_in) VALUES
('omega_shell', 'operational', '1.0.0', NOW()),
('omega_notification_daemon', 'operational', '1.0.0', NOW()),
('omega_language_engine', 'operational', '1.0.0', NOW()),
('omega_news_core_service', 'operational', '1.0.0', NOW()),
('omega_browser_sync_service', 'operational', '1.0.0', NOW()),
('omega_syscall_table', 'operational', '1.0.0', NOW()),
('omega_simulator', 'operational', '1.0.0', NOW()),
('omega_deploy_tool', 'operational', '1.0.0', NOW());
