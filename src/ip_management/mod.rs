pub mod models;
pub mod registration;
pub mod security;
pub mod compliance;
pub mod discovery;
pub mod analytics;
pub mod lifecycle;

use models::{IpAsset, IpType, IpDomain, IpStatus};

/// The ultimate nexus for managing, protecting, and evolving all forms of intellectual property.
pub struct IpManagementPanel {
    pub registration_engine: registration::UniversalIpRegistrationEngine,
    pub security_vault: security::QuantumChiffredVault,
    pub compliance_checker: compliance::EvasComplianceChecker,
    pub discovery_engine: discovery::AutonomousDiscoveryEngine,
    pub lifecycle_manager: lifecycle::IpLifecycleManager,
    pub analytics_engine: analytics::IpAnalyticsEngine,
}

impl IpManagementPanel {
    pub fn new() -> Self {
        Self {
            registration_engine: registration::UniversalIpRegistrationEngine::default(),
            security_vault: security::QuantumChiffredVault::new(),
            compliance_checker: compliance::EvasComplianceChecker::default(),
            discovery_engine: discovery::AutonomousDiscoveryEngine::default(),
            lifecycle_manager: lifecycle::IpLifecycleManager,
            analytics_engine: analytics::IpAnalyticsEngine::default(),
        }
    }
}
