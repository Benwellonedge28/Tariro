// Tariro_ide_idpte/src/compliance_advisor/compliance_policy_manager.rs

//! Manages and enforces legal and ethical compliance policies across OMEGA OS.
//! This module aggregates rules and mandates, and provides interfaces for checking compliance.

use crate::compliance_advisor::legal_framework_models::{
    LegalRule, EthicalMandate, ComplianceStatus, ComplianceViolation, ViolationSeverity
};
use crate::compliance_advisor::ethical_decision_engine::EthicalDecisionEngine;
use anyhow::Result;
use std::collections::HashMap;
use tracing::{info, warn};

/// Manages a comprehensive set of legal rules and ethical mandates.
pub struct CompliancePolicyManager {
    legal_rules: HashMap<String, LegalRule>,
    ethical_mandates: HashMap<String, EthicalMandate>,
    ethical_engine: EthicalDecisionEngine,
}

impl CompliancePolicyManager {
    pub fn new() -> Self {
        println!("Compliance Policy Manager initialized.");
        CompliancePolicyManager {
            legal_rules: HashMap::new(),
            ethical_mandates: HashMap::new(),
            ethical_engine: EthicalDecisionEngine::new(),
        }
    }

    /// Updates the internal store of legal rules.
    pub async fn update_legal_rules(&mut self, rules: Vec<LegalRule>) {
        for rule in rules {
            info!("Compliance Policy Manager: Updating legal rule {}: {}", rule.id, rule.title);
            self.legal_rules.insert(rule.id.clone(), rule);
        }
    }

    /// Updates the internal store of ethical mandates and the ethical decision engine.
    pub async fn update_ethical_mandates(&mut self, mandates: Vec<EthicalMandate>) {
        self.ethical_engine.update_mandates(mandates.clone()).await;
        for mandate in mandates {
            info!("Compliance Policy Manager: Updating ethical mandate {}: {}", mandate.id, mandate.title);
            self.ethical_mandates.insert(mandate.id.clone(), mandate);
        }
    }

    /// Checks the compliance status of a specific OMEGA OS operation or module.
    pub async fn check_compliance(
        &self,
        entity_id: &str,
        entity_type: &str,
        action_description: &str,
        relevant_agi_module: &str,
        affected_data_domains: &[String],
    ) -> Result<ComplianceStatus> {
        info!("Compliance Policy Manager: Checking compliance for {} ({}).
", entity_id, entity_type);
        let mut violations = Vec::new();

        // Check against Legal Rules
        for rule in self.legal_rules.values() {
            let is_relevant_jurisdiction = rule.jurisdiction == "Global" || rule.relevant_domains.iter().any(|d| affected_data_domains.contains(d));
            if is_relevant_jurisdiction && action_description.contains("process user data") {
                // Simplified check: A real check would involve deep contextual analysis
                violations.push(ComplianceViolation {
                    rule_id: rule.id.clone(),
                    severity: ViolationSeverity::High,
                    description: format!("Potential violation of legal rule: {}. Action: '{}'.", rule.title, action_description),
                    recommended_action: Some("Review data handling procedures.".to_string()),
                });
            }
        }

        // Check against Ethical Mandates via Ethical Decision Engine
        let ethical_violations = self.ethical_engine.evaluate_action(
            action_description,
            relevant_agi_module,
            affected_data_domains,
        ).await?;
        violations.extend(ethical_violations);

        let is_compliant = violations.is_empty();
        Ok(ComplianceStatus {
            entity_id: entity_id.to_string(),
            entity_type: entity_type.to_string(),
            is_compliant,
            violations,
            last_checked: Utc::now(),
            confidence: 0.95, // High confidence if all checks pass
        })
    }
}
