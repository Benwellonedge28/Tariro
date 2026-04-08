// src/autonomy/policy_engine.rs

//! Manages and enforces AGI autonomy policies based on predefined rules.

use crate::default_api::{AutonomyPolicy, AutonomyPolicyAutonomyLevel};

/// The `PolicyEngine` evaluates actions against defined `AutonomyPolicy` rules.
pub struct PolicyEngine {
    policies: Vec<AutonomyPolicy>,
}

impl PolicyEngine {
    pub fn new(policies: Vec<AutonomyPolicy>) -> Self {
        PolicyEngine { policies }
    }

    /// Retrieves the effective autonomy level for a given operational domain.
    pub fn get_autonomy_level(&self, domain: &str) -> AutonomyPolicyAutonomyLevel {
        self.policies.iter()
            .find(|p| p.domain == domain)
            .map(|p| p.autonomy_level.clone())
            .unwrap_or(AutonomyPolicyAutonomyLevel::SuggestOnly) // Default to SuggestOnly
    }

    pub fn check_permission(&self, domain: &str, action: &str) -> bool {
        let autonomy_level = self.get_autonomy_level(domain);

        match autonomy_level {
            AutonomyPolicyAutonomyLevel::FullAuto => true, // All actions permitted
            AutonomyPolicyAutonomyLevel::SuggestOnly => {
                action.contains("suggest") || action.contains("propose") || action.contains("analyze")
            },
            AutonomyPolicyAutonomyLevel::HumanRequired => false,
        }
    }
}
