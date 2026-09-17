use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Represents a single ethical consideration or decision point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralDecisionPoint {
    pub decision_id: String,
    pub timestamp: DateTime<Utc>,
    pub action_taken_id: String, // Reference to the action that triggered the decision
    pub action_description: String,
    pub detected_conflicts: Vec<String>, // List of identified ethical conflicts
    pub considered_policies: Vec<String>, // List of E.V.A.S. policies considered
    pub reasoning_process: String, // Narrative of how the decision was reached
    pub final_decision: String, // The actual decision made by Tariro
    pub ethical_score_post_decision: f64, // 0.0 - 1.0, self-assessed ethical score
    pub contextual_info_rag: HashMap<String, String>, // RAG-provided context
}

// Represents a review or audit record of a moral decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRecord {
    pub audit_id: String,
    pub decision_id: String,
    pub auditor_id: String, // e.g., "Samuel Mukandara", "Automated_Compliance_Bot"
    pub audit_timestamp: DateTime<Utc>,
    pub review_status: String, // "Approved", "Flagged", "Overridden"
    pub auditor_comments: String,
}

// Placeholder for RAG to provide policy application context
pub struct RagPolicyContext {
    pub policy_documents: HashMap<String, String>, // Full text of E.V.A.S. policies
    pub ethical_precedents: HashMap<String, String>, // Past cases and their resolutions
    pub legal_interpretations: HashMap<String, String>, // Legal interpretations related to policies
}

impl RagPolicyContext {
    pub fn new() -> Self {
        let mut docs = HashMap::new();
        docs.insert("DataPrivacyPolicy".to_string(), "Comprehensive data privacy policy document.".to_string());
        docs.insert("NonMaleficenceDirective".to_string(), "Directive on avoiding harm.".to_string());
        let mut precedents = HashMap::new();
        precedents.insert("AutonomousResourceAllocation_Case1".to_string(), "Resolved by prioritizing human safety.".to_string());
        let mut interpretations = HashMap::new();
        interpretations.insert("GDPR_DataRetention".to_string(), "Legal interpretation of data retention requirements under GDPR.".to_string());

        RagPolicyContext { policy_documents: docs, ethical_precedents: precedents, legal_interpretations: interpretations }
    }

    pub fn get_policy_context(&self, policy_name: &str) -> Option<String> {
        self.policy_documents.get(policy_name)
            .map(|s| format!("Policy Document: {}", s))
            .or_else(|| self.ethical_precedents.get(policy_name).map(|s| format!("Ethical Precedent: {}", s)))
            .or_else(|| self.legal_interpretations.get(policy_name).map(|s| format!("Legal Interpretation: {}", s)))
    }
}

pub struct MoralReasoningLogger {
    rag_context_provider: Arc<RagPolicyContext>,
    moral_decision_log: Mutex<Vec<MoralDecisionPoint>>,
    audit_log: Mutex<Vec<AuditRecord>>,
}

impl MoralReasoningLogger {
    pub fn new(rag_context_provider: Arc<RagPolicyContext>) -> Self {
        MoralReasoningLogger {
            rag_context_provider,
            moral_decision_log: Mutex::new(Vec::new()),
            audit_log: Mutex::new(Vec::new()),
        }
    }

    // Logs an ethical consideration or decision made by Tariro
    pub fn log_moral_decision(&self,
                              action_id: &str,
                              action_description: &str,
                              detected_conflicts: Vec<String>,
                              considered_policies: Vec<String>,
                              reasoning_process: &str,
                              final_decision: &str,
                              ethical_score: f64) -> MoralDecisionPoint {
        println!("Moral Reasoning Logger: Logging new moral decision for action: {}", action_id);

        let mut contextual_info = HashMap::new();
        for policy_name in &considered_policies {
            if let Some(context) = self.rag_context_provider.get_policy_context(policy_name) {
                contextual_info.insert(policy_name.clone(), context);
            }
        }

        let decision_point = MoralDecisionPoint {
            decision_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            action_taken_id: action_id.to_string(),
            action_description: action_description.to_string(),
            detected_conflicts,
            considered_policies,
            reasoning_process: reasoning_process.to_string(),
            final_decision: final_decision.to_string(),
            ethical_score_post_decision: ethical_score,
            contextual_info_rag: contextual_info,
        };
        self.moral_decision_log.lock().unwrap().push(decision_point.clone());
        println!("Moral Reasoning Logger: Decision logged: {:?}", decision_point.decision_id);
        decision_point
    }

    // Records an audit event for a specific moral decision
    pub fn log_audit_record(&self, decision_id: &str, auditor_id: &str, review_status: &str, comments: &str) -> AuditRecord {
        println!("Moral Reasoning Logger: Logging audit for decision: {}", decision_id);
        let audit_record = AuditRecord {
            audit_id: Uuid::new_v4().to_string(),
            decision_id: decision_id.to_string(),
            auditor_id: auditor_id.to_string(),
            audit_timestamp: Utc::now(),
            review_status: review_status.to_string(),
            auditor_comments: comments.to_string(),
        };
        self.audit_log.lock().unwrap().push(audit_record.clone());
        audit_record
    }

    pub fn get_moral_decision_log(&self) -> Vec<MoralDecisionPoint> {
        self.moral_decision_log.lock().unwrap().clone()
    }

    pub fn get_audit_log(&self) -> Vec<AuditRecord> {
        self.audit_log.lock().unwrap().clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moral_reasoning_logger() {
        let rag_context = Arc::new(RagPolicyContext::new());
        let logger = MoralReasoningLogger::new(rag_context);

        let action_id = Uuid::new_v4().to_string();
        let action_desc = "Decided to prioritize data integrity over immediate system availability during a fault.";
        let conflicts = vec!["Availability vs Integrity".to_string()];
        let policies = vec!["DataPrivacyPolicy".to_string(), "NonMaleficenceDirective".to_string()];
        let reasoning = "Based on DataPrivacyPolicy, integrity is paramount. NonMaleficenceDirective supports avoiding data corruption which could lead to greater harm.";
        let final_dec = "Halt system to ensure data integrity.";
        let ethical_score = 0.85;

        let decision = logger.log_moral_decision(
            &action_id,
            action_desc,
            conflicts.clone(),
            policies.clone(),
            reasoning,
            final_dec,
            ethical_score,
        );

        let decision_log = logger.get_moral_decision_log();
        assert_eq!(decision_log.len(), 1);
        assert_eq!(decision_log[0].action_taken_id, action_id);
        assert!(decision_log[0].contextual_info_rag.contains_key("DataPrivacyPolicy"));

        // Log an audit record for this decision
        let audit_record = logger.log_audit_record(
            &decision.decision_id,
            "Samuel Mukandara",
            "Approved",
            "Decision aligns with current E.V.A.S. framework.",
        );

        let audit_log = logger.get_audit_log();
        assert_eq!(audit_log.len(), 1);
        assert_eq!(audit_log[0].decision_id, decision.decision_id);
        assert_eq!(audit_log[0].auditor_id, "Samuel Mukandara");
    }
}
*/