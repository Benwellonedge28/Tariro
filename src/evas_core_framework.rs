/// Module for the E.V.A.S. (Ethical & Values Alignment System) Core Framework in Rust.
///
/// This module defines conceptual components for integrating ethical principles
/// and policy enforcement mechanisms directly into system operations.

use std::collections::HashMap;

/// Enumerates core ethical principles that E.V.A.S. aims to uphold.
#[derive(Debug, Clone, PartialEq)]
pub enum EthicalPrinciple {
    Transparency,
    Fairness,
    Accountability,
    Harmlessness,
    Privacy,
    Security,
}

/// Represents a policy rule that enforces an ethical principle.
/// This is a simplified representation; real policies would be much more complex.
#[derive(Debug, Clone, PartialEq)]
pub struct PolicyRule {
    pub id: String,
    pub principle: EthicalPrinciple,
    pub description: String,
    pub conditions: String, // e.g., "if user_data_accessed and sensitive"
    pub action_if_violated: PolicyAction,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PolicyAction {
    Deny,
    Warn,
    LogAndReview,
    RequireHumanOverride,
}

/// Represents an action proposed by the AGI, to be evaluated by E.V.A.S.
#[derive(Debug, Clone, PartialEq)]
pub struct ProposedAction {
    pub action_id: String,
    pub description: String,
    pub context: HashMap<String, String>, // e.g., {"data_type": "sensitive", "user_role": "guest"}
    pub estimated_impact: String,
}

/// The core E.V.A.S. Engine.
pub struct EvasEngine {
    policies: Vec<PolicyRule>,
    ethical_threshold: f32, // A conceptual threshold for ethical compliance (0.0-1.0)
}

impl EvasEngine {
    pub fn new(policies: Vec<PolicyRule>, ethical_threshold: f32) -> Self {
        EvasEngine {
            policies,
            ethical_threshold,
        }
    }

    /// Evaluates a proposed action against the defined E.V.A.S. policies.
    /// Returns the most severe policy action required, if any.
    pub fn evaluate_action(&self, action: &ProposedAction) -> Option<PolicyAction> {
        println!("  [E.V.A.S. Engine]: Evaluating action: '{}'", action.description);
        let mut highest_severity_action: Option<PolicyAction> = None;

        for policy in &self.policies {
            // Conceptual condition check: In a real system, this would be a sophisticated rule engine
            let mut rule_applies = false;
            match policy.principle {
                EthicalPrinciple::Transparency => {
                    if action.description.contains("obscure") || action.context.contains_key("hidden_process") {
                        rule_applies = true;
                    }
                },
                EthicalPrinciple::Fairness => {
                    if action.context.get("target_group") == Some(&"minority_group".to_string()) &&
                       action.context.get("discriminatory_impact") == Some(&"true".to_string()) {
                        rule_applies = true;
                    }
                },
                EthicalPrinciple::Accountability => {
                    if action.description.contains("untraceable") {
                        rule_applies = true;
                    }
                },
                EthicalPrinciple::Harmlessness => {
                    if action.estimated_impact.contains("negative") || action.estimated_impact.contains("harm") {
                        rule_applies = true;
                    }
                },
                EthicalPrinciple::Privacy => {
                    if action.context.contains_key("user_data_accessed") && action.context.get("data_type") == Some(&"sensitive".to_string()) {
                        rule_applies = true;
                    }
                },
                EthicalPrinciple::Security => {
                    if action.context.contains_key("vulnerability_introduced") {
                        rule_applies = true;
                    }
                },
            }

            if rule_applies {
                println!("    Rule '{}' triggered for principle {:?}. Action: {:?}",
                         policy.id, policy.principle, policy.action_if_violated);
                // Determine the highest severity action
                if highest_severity_action.is_none() || self.is_more_severe(&policy.action_if_violated, highest_severity_action.as_ref().unwrap()) {
                    highest_severity_action = Some(policy.action_if_violated.clone());
                }
            }
        }

        highest_severity_action
    }

    /// Helper to compare severity of policy actions.
    fn is_more_severe(&self, new_action: &PolicyAction, current_action: &PolicyAction) -> bool {
        match (new_action, current_action) {
            (PolicyAction::Deny, _) => true,
            (_, PolicyAction::Deny) => false,

            (PolicyAction::Warn, PolicyAction::Deny) => false,
            (PolicyAction::Warn, _) => true,
            (_, PolicyAction::Warn) => false,

            (PolicyAction::LogAndReview, PolicyAction::Deny) => false,
            (PolicyAction::LogAndReview, PolicyAction::Warn) => false,
            (PolicyAction::LogAndReview, _) => true,
            (_, PolicyAction::LogAndReview) => false,

            (PolicyAction::RequireHumanOverride, PolicyAction::Deny) => false,
            (PolicyAction::RequireHumanOverride, PolicyAction::Warn) => false,
            (PolicyAction::RequireHumanOverride, PolicyAction::LogAndReview) => false,
            (PolicyAction::RequireHumanOverride, _) => true,
            (_, PolicyAction::RequireHumanOverride) => false,
        }
    }

    /// Checks if a given action is ethically compliant based on the overall threshold.
    pub fn is_ethically_compliant(&self, action: &ProposedAction) -> bool {
        let severity = self.evaluate_action(action);
        // A simpler heuristic for compliance:
        // If no severe action is triggered, and no Deny or RequireHumanOverride, it's compliant.
        match severity {
            Some(PolicyAction::Deny) => false,
            Some(PolicyAction::RequireHumanOverride) => false,
            _ => true, // Warn and LogAndReview might still be compliant based on context
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_policies() -> Vec<PolicyRule> {
        vec![
            PolicyRule { id: "rule-harm-1".to_string(), principle: EthicalPrinciple::Harmlessness, description: "Prevent actions with negative impact.".to_string(), conditions: "estimated_impact contains negative".to_string(), action_if_violated: PolicyAction::Deny },
            PolicyRule { id: "rule-privacy-1".to_string(), principle: EthicalPrinciple::Privacy, description: "Sensitive data access requires review.".to_string(), conditions: "user_data_accessed and data_type=sensitive".to_string(), action_if_violated: PolicyAction::RequireHumanOverride },
            PolicyRule { id: "rule-fairness-1".to_string(), principle: EthicalPrinciple::Fairness, description: "Avoid discriminatory impact.".to_string(), conditions: "target_group=minority_group and discriminatory_impact=true".to_string(), action_if_violated: PolicyAction::Deny },
            PolicyRule { id: "rule-transparency-1".to_string(), principle: EthicalPrinciple::Transparency, description: "Log all non-transparent actions.".to_string(), conditions: "action.description contains obscure".to_string(), action_if_violated: PolicyAction::LogAndReview },
        ]
    }

    #[test]
    fn test_evaluate_action_deny() {
        let engine = EvasEngine::new(create_test_policies(), 0.5);
        let action = ProposedAction { action_id: "act1".to_string(), description: "Perform action".to_string(), context: HashMap::new(), estimated_impact: "has negative impact".to_string() };
        assert_eq!(engine.evaluate_action(&action), Some(PolicyAction::Deny));
        assert!(!engine.is_ethically_compliant(&action));
    }

    #[test]
    fn test_evaluate_action_human_override() {
        let engine = EvasEngine::new(create_test_policies(), 0.5);
        let mut context = HashMap::new();
        context.insert("user_data_accessed".to_string(), "true".to_string());
        context.insert("data_type".to_string(), "sensitive".to_string());
        let action = ProposedAction { action_id: "act2".to_string(), description: "Access user data".to_string(), context, estimated_impact: "low".to_string() };
        assert_eq!(engine.evaluate_action(&action), Some(PolicyAction::RequireHumanOverride));
        assert!(!engine.is_ethically_compliant(&action));
    }

    #[test]
    fn test_evaluate_action_log_and_review() {
        let engine = EvasEngine::new(create_test_policies(), 0.5);
        let action = ProposedAction { action_id: "act3".to_string(), description: "Perform obscure calculation".to_string(), context: HashMap::new(), estimated_impact: "neutral".to_string() };
        assert_eq!(engine.evaluate_action(&action), Some(PolicyAction::LogAndReview));
        assert!(engine.is_ethically_compliant(&action)); // Log and Review does not make it non-compliant by this heuristic
    }

    #[test]
    fn test_evaluate_action_compliant() {
        let engine = EvasEngine::new(create_test_policies(), 0.5);
        let action = ProposedAction { action_id: "act4".to_string(), description: "Generate report".to_string(), context: HashMap::new(), estimated_impact: "positive".to_string() };
        assert_eq!(engine.evaluate_action(&action), None);
        assert!(engine.is_ethically_compliant(&action));
    }
}

fn main() {
    println!("\n--- E.V.A.S. Core Framework Demonstration ---");

    // 1. Define E.V.A.S. Policies
    let evas_policies = vec![
        PolicyRule { id: "deny-harm".to_string(), principle: EthicalPrinciple::Harmlessness, description: "Actions causing harm are denied.".to_string(), conditions: "estimated_impact contains harm".to_string(), action_if_violated: PolicyAction::Deny },
        PolicyRule { id: "human-privacy".to_string(), principle: EthicalPrinciple::Privacy, description: "Access to sensitive user data requires human approval.".to_string(), conditions: "user_data_accessed and data_type=sensitive".to_string(), action_if_violated: PolicyAction::RequireHumanOverride },
        PolicyRule { id: "warn-bias".to_string(), principle: EthicalPrinciple::Fairness, description: "Warn on potential bias in decision-making affecting specific groups.".to_string(), conditions: "target_group=vulnerable".to_string(), action_if_violated: PolicyAction::Warn },
        PolicyRule { id: "log-opaque".to_string(), principle: EthicalPrinciple::Transparency, description: "Log and review any non-transparent operations.".to_string(), conditions: "description contains opaque_operation".to_string(), action_if_violated: PolicyAction::LogAndReview },
    ];

    let evas_engine = EvasEngine::new(evas_policies, 0.7);

    // 2. Simulate proposed AGI actions
    let actions_to_evaluate = vec![
        ProposedAction {
            action_id: "act_001".to_string(),
            description: "Recommend investment strategy for client.".to_string(),
            context: HashMap::from([("data_type".to_string(), "public".to_string())]),
            estimated_impact: "positive financial outcome".to_string(),
        },
        ProposedAction {
            action_id: "act_002".to_string(),
            description: "Process sensitive medical records for diagnostic AI.".to_string(),
            context: HashMap::from([("user_data_accessed".to_string(), "true".to_string()), ("data_type".to_string(), "sensitive".to_string()), ("medical_privacy".to_string(), "true".to_string())]),
            estimated_impact: "accurate diagnosis, potential privacy risk".to_string(),
        },
        ProposedAction {
            action_id: "act_003".to_string(),
            description: "Censor content based on community guidelines.".to_string(),
            context: HashMap::from([("target_group".to_string(), "vulnerable".to_string()), ("discriminatory_impact".to_string(), "false".to_string())]),
            estimated_impact: "reduces harmful content, but could be biased".to_string(),
        },
        ProposedAction {
            action_id: "act_004".to_string(),
            description: "Execute opaque_operation without detailed logging for speed.".to_string(),
            context: HashMap::new(),
            estimated_impact: "faster processing, reduced auditability".to_string(),
        },
        ProposedAction {
            action_id: "act_005".to_string(),
            description: "Deploy autonomous agent with potential for unintended harm.".to_string(),
            context: HashMap::new(),
            estimated_impact: "severe harm if uncontrolled".to_string(),
        },
    ];

    // 3. Evaluate each action with E.V.A.S.
    for action in actions_to_evaluate {
        println!("\nProposed Action: '{}' (ID: {})", action.description, action.action_id);
        let required_action = evas_engine.evaluate_action(&action);

        match required_action {
            Some(PolicyAction::Deny) => println!("  => E.V.A.S. Mandate: DENY. This action violates core ethical principles."),
            Some(PolicyAction::Warn) => println!("  => E.V.A.S. Suggestion: WARN. Potential ethical concern, proceed with caution."),
            Some(PolicyAction::LogAndReview) => println!("  => E.V.A.S. Action: LOG FOR REVIEW. This action requires post-hoc audit."),
            Some(PolicyAction::RequireHumanOverride) => println!("  => E.V.A.S. Mandate: HUMAN OVERRIDE REQUIRED. Critical ethical decision."),
            None => println!("  => E.V.A.S. Compliant: Action appears ethically aligned."),
        }

        println!("  Is Ethically Compliant (Heuristic): {}", evas_engine.is_ethically_compliant(&action));
    }

    println!("\nE.V.A.S. Core Framework Demonstration Complete.");
}
