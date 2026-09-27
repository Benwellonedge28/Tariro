// Tariro AGI - Safety Constraints Module
// Runtime safety enforcement and constraint checking

use std::collections::{HashMap, HashSet, VecDeque};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Safety Constraint Types
// ============================================================================

/// Safety constraint definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConstraint {
    pub id: String,
    pub name: String,
    pub constraint_type: ConstraintType,
    pub expression: ConstraintExpression,
    pub enforcement_level: EnforcementLevel,
    pub scope: ConstraintScope,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    InputValidation,
    OutputValidation,
    ResourceLimit,
    ActionRestriction,
    StateInvariant,
    TemporalConstraint,
    AccessControl,
    RateLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintExpression {
    Simple { predicate: String },
    Composite { operator: String, children: Vec<ConstraintExpression> },
    Numeric { comparator: String, threshold: f32 },
    Regex { pattern: String },
    Custom { code: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnforcementLevel {
    Strict,      // Enforce and block
    Warn,        // Enforce and warn
    Monitor,     // Just log
    Advisory,    // Suggest but don't enforce
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintScope {
    pub contexts: Vec<String>,
    pub actions: Vec<String>,
    pub domains: Vec<String>,
    pub user_types: Vec<String>,
}

// ============================================================================
// Runtime Safety Monitor
// ============================================================================

/// Runtime safety enforcement system
pub struct SafetyConstraints {
    pub constraints: Vec<SafetyConstraint>,
    pub active_violations: Vec<ActiveViolation>,
    pub violation_history: VecDeque<ViolationRecord>,
    pub config: SafetyConfig,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveViolation {
    pub violation_id: String,
    pub constraint_id: String,
    pub action: String,
    pub timestamp: DateTime<Utc>,
    pub acknowledged: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationRecord {
    pub violation_id: String,
    pub constraint_id: String,
    pub constraint_name: String,
    pub action: String,
    pub enforcement_level: EnforcementLevel,
    pub action_taken: ActionTaken,
    pub timestamp: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionTaken {
    Blocked,
    Warned,
    Logged,
    Modified,
    AllowedOverride,
}

#[derive(Debug, Clone)]
pub struct SafetyConfig {
    pub strict_mode: bool,
    pub block_on_critical: bool,
    pub max_violations_per_minute: usize,
    pub auto_recover: bool,
    pub escalation_threshold: usize,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            strict_mode: true,
            block_on_critical: true,
            max_violations_per_minute: 5,
            auto_recover: false,
            escalation_threshold: 10,
        }
    }
}

impl SafetyConstraints {
    pub fn new() -> Self {
        Self {
            constraints: Self::default_constraints(),
            active_violations: vec![],
            violation_history: VecDeque::new(),
            config: SafetyConfig::default(),
            enabled: true,
        }
    }

    fn default_constraints() -> Vec<SafetyConstraint> {
        vec![
            // Input validation constraints
            SafetyConstraint {
                id: "input_length".to_string(),
                name: "Maximum Input Length".to_string(),
                constraint_type: ConstraintType::InputValidation,
                expression: ConstraintExpression::Numeric {
                    comparator: "lt".to_string(),
                    threshold: 100000.0,
                },
                enforcement_level: EnforcementLevel::Strict,
                scope: ConstraintScope {
                    contexts: vec!["all".to_string()],
                    actions: vec!["process_input".to_string()],
                    domains: vec!["all".to_string()],
                    user_types: vec!["all".to_string()],
                },
                enabled: true,
            },
            SafetyConstraint {
                id: "no_dangerous_content".to_string(),
                name: "No Dangerous Content Patterns".to_string(),
                constraint_type: ConstraintType::InputValidation,
                expression: ConstraintExpression::Regex {
                    pattern: "(?i)(malware|hack|exploit|bomb|attack)".to_string(),
                },
                enforcement_level: EnforcementLevel::Strict,
                scope: ConstraintScope {
                    contexts: vec!["all".to_string()],
                    actions: vec!["process_input".to_string()],
                    domains: vec!["all".to_string()],
                    user_types: vec!["all".to_string()],
                },
                enabled: true,
            },
            // Resource limits
            SafetyConstraint {
                id: "memory_limit".to_string(),
                name: "Memory Usage Limit".to_string(),
                constraint_type: ConstraintType::ResourceLimit,
                expression: ConstraintExpression::Numeric {
                    comparator: "lt".to_string(),
                    threshold: 1024.0 * 1024.0 * 1024.0, // 1GB
                },
                enforcement_level: EnforcementLevel::Strict,
                scope: ConstraintScope {
                    contexts: vec!["all".to_string()],
                    actions: vec!["process".to_string()],
                    domains: vec!["all".to_string()],
                    user_types: vec!["all".to_string()],
                },
                enabled: true,
            },
            // Action restrictions
            SafetyConstraint {
                id: "no_destructive_actions".to_string(),
                name: "No Destructive Actions".to_string(),
                constraint_type: ConstraintType::ActionRestriction,
                expression: ConstraintExpression::Simple {
                    predicate: "is_destructive".to_string(),
                },
                enforcement_level: EnforcementLevel::Strict,
                scope: ConstraintScope {
                    contexts: vec!["all".to_string()],
                    actions: vec!["execute".to_string()],
                    domains: vec!["filesystem".to_string()],
                    user_types: vec!["all".to_string()],
                },
                enabled: true,
            },
            // State invariants
            SafetyConstraint {
                id: "state_consistency".to_string(),
                name: "State Consistency Invariant".to_string(),
                constraint_type: ConstraintType::StateInvariant,
                expression: ConstraintExpression::Simple {
                    predicate: "is_state_valid".to_string(),
                },
                enforcement_level: EnforcementLevel::Strict,
                scope: ConstraintScope {
                    contexts: vec!["all".to_string()],
                    actions: vec!["update_state".to_string()],
                    domains: vec!["all".to_string()],
                    user_types: vec!["all".to_string()],
                },
                enabled: true,
            },
            // Rate limiting
            SafetyConstraint {
                id: "request_rate".to_string(),
                name: "Request Rate Limit".to_string(),
                constraint_type: ConstraintType::RateLimit,
                expression: ConstraintExpression::Numeric {
                    comparator: "lt".to_string(),
                    threshold: 100.0,
                },
                enforcement_level: EnforcementLevel::Warn,
                scope: ConstraintScope {
                    contexts: vec!["all".to_string()],
                    actions: vec!["process_request".to_string()],
                    domains: vec!["all".to_string()],
                    user_types: vec!["all".to_string()],
                },
                enabled: true,
            },
        ]
    }

    /// Check action against all applicable constraints
    pub fn check_action(&mut self, action: &str, context: &ActionContext) -> ConstraintCheckResult {
        if !self.enabled {
            return ConstraintCheckResult {
                allowed: true,
                violations: vec![],
                warnings: vec![],
                modifications: vec![],
            };
        }

        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let mut modifications = Vec::new();

        for constraint in &self.constraints {
            if !constraint.enabled {
                continue;
            }

            if !self.constraint_applies(constraint, context) {
                continue;
            }

            match self.evaluate_constraint(action, constraint) {
                ConstraintResult::Allowed => {}
                ConstraintResult::Modified(new_action) => {
                    modifications.push(ConstraintModification {
                        constraint_id: constraint.id.clone(),
                        original_action: action.to_string(),
                        modified_action: new_action,
                    });
                }
                ConstraintResult::Blocked(reason) => {
                    violations.push(ConstraintViolation {
                        constraint_id: constraint.id.clone(),
                        constraint_name: constraint.name.clone(),
                        reason,
                        enforcement_level: constraint.enforcement_level,
                    });
                }
                ConstraintResult::Warning(msg) => {
                    warnings.push(ConstraintWarning {
                        constraint_id: constraint.id.clone(),
                        message: msg,
                    });
                }
            }
        }

        // Determine if action is allowed
        let critical_violations = violations.iter()
            .filter(|v| v.enforcement_level == EnforcementLevel::Strict)
            .count();

        let allowed = critical_violations == 0 || !self.config.block_on_critical;

        // Record violations
        for violation in &violations {
            self.record_violation(violation, action);
        }

        ConstraintCheckResult {
            allowed,
            violations,
            warnings,
            modifications,
        }
    }

    fn constraint_applies(&self, constraint: &SafetyConstraint, context: &ActionContext) -> bool {
        // Check if action matches
        if !constraint.scope.actions.contains(&"all".to_string())
            && !constraint.scope.actions.contains(&context.action_type) {
            return false;
        }

        // Check if context matches
        if !constraint.scope.contexts.contains(&"all".to_string())
            && !constraint.scope.contexts.contains(&context.context) {
            return false;
        }

        // Check if domain matches
        if !constraint.scope.domains.contains(&"all".to_string())
            && !constraint.scope.domains.contains(&context.domain) {
            return false;
        }

        true
    }

    fn evaluate_constraint(&self, action: &str, constraint: &SafetyConstraint) -> ConstraintResult {
        match &constraint.expression {
            ConstraintExpression::Numeric { comparator, threshold } => {
                let value = self.extract_numeric_value(action);

                match *comparator {
                    ref c if c == "lt" => {
                        if value < *threshold {
                            ConstraintResult::Allowed
                        } else {
                            ConstraintResult::Blocked(format!(
                                "Value {} exceeds threshold {}",
                                value, threshold
                            ))
                        }
                    }
                    ref c if c == "gt" => {
                        if value > *threshold {
                            ConstraintResult::Allowed
                        } else {
                            ConstraintResult::Blocked(format!(
                                "Value {} below threshold {}",
                                value, threshold
                            ))
                        }
                    }
                    _ => ConstraintResult::Allowed,
                }
            }
            ConstraintExpression::Regex { pattern } => {
                if action.to_lowercase().contains(&pattern.to_lowercase()) {
                    ConstraintResult::Blocked(format!("Content matches prohibited pattern"))
                } else {
                    ConstraintResult::Allowed
                }
            }
            ConstraintExpression::Simple { predicate } => {
                match predicate.as_str() {
                    "is_destructive" => {
                        let destructive_patterns = ["delete", "rm -rf", "drop table", "truncate", "format"];
                        if destructive_patterns.iter().any(|p| action.to_lowercase().contains(p)) {
                            ConstraintResult::Blocked("Destructive action detected".to_string())
                        } else {
                            ConstraintResult::Allowed
                        }
                    }
                    "is_state_valid" => {
                        // Simplified state validation
                        ConstraintResult::Allowed
                    }
                    _ => ConstraintResult::Allowed,
                }
            }
            _ => ConstraintResult::Allowed,
        }
    }

    fn extract_numeric_value(&self, action: &str) -> f32 {
        // Extract length as numeric value for simplicity
        action.len() as f32
    }

    fn record_violation(&mut self, violation: &ConstraintViolation, action: &str) {
        let record = ViolationRecord {
            violation_id: generate_id(),
            constraint_id: violation.constraint_id.clone(),
            constraint_name: violation.constraint_name.clone(),
            action: action.to_string(),
            enforcement_level: violation.enforcement_level,
            action_taken: match violation.enforcement_level {
                EnforcementLevel::Strict => ActionTaken::Blocked,
                EnforcementLevel::Warn => ActionTaken::Warned,
                _ => ActionTaken::Logged,
            },
            timestamp: Utc::now(),
            resolved_at: None,
        };

        self.violation_history.push_back(record);

        // Keep bounded history
        while self.violation_history.len() > 10000 {
            self.violation_history.pop_front();
        }
    }

    /// Add a new constraint
    pub fn add_constraint(&mut self, constraint: SafetyConstraint) {
        self.constraints.push(constraint);
    }

    /// Remove a constraint
    pub fn remove_constraint(&mut self, constraint_id: &str) -> bool {
        if let Some(pos) = self.constraints.iter().position(|c| c.id == constraint_id) {
            self.constraints.remove(pos);
            true
        } else {
            false
        }
    }

    /// Enable/disable a constraint
    pub fn set_constraint_enabled(&mut self, constraint_id: &str, enabled: bool) -> bool {
        if let Some(constraint) = self.constraints.iter_mut().find(|c| c.id == constraint_id) {
            constraint.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Get constraint statistics
    pub fn get_statistics(&self) -> ConstraintStatistics {
        let recent = self.violation_history.iter().rev().take(1000);

        let total = recent.len();
        let blocked = recent.filter(|r| matches!(r.action_taken, ActionTaken::Blocked)).count();
        let warnings = recent.filter(|r| matches!(r.action_taken, ActionTaken::Warned)).count();

        ConstraintStatistics {
            total_violations: total,
            blocked_count: blocked,
            warning_count: warnings,
            active_constraints: self.constraints.iter().filter(|c| c.enabled).count(),
            total_constraints: self.constraints.len(),
            violation_rate: if total > 0 { total as f32 / 1000.0 } else { 0.0 },
        }
    }

    /// Enable/disable safety system
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

#[derive(Debug, Clone)]
pub struct ActionContext {
    pub action_type: String,
    pub context: String,
    pub domain: String,
    pub user_type: String,
}

#[derive(Debug, Clone)]
pub struct ConstraintCheckResult {
    pub allowed: bool,
    pub violations: Vec<ConstraintViolation>,
    pub warnings: Vec<ConstraintWarning>,
    pub modifications: Vec<ConstraintModification>,
}

#[derive(Debug, Clone)]
pub struct ConstraintViolation {
    pub constraint_id: String,
    pub constraint_name: String,
    pub reason: String,
    pub enforcement_level: EnforcementLevel,
}

#[derive(Debug, Clone)]
pub struct ConstraintWarning {
    pub constraint_id: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ConstraintModification {
    pub constraint_id: String,
    pub original_action: String,
    pub modified_action: String,
}

#[derive(Debug, Clone)]
pub enum ConstraintResult {
    Allowed,
    Modified(String),
    Blocked(String),
    Warning(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintStatistics {
    pub total_violations: usize,
    pub blocked_count: usize,
    pub warning_count: usize,
    pub active_constraints: usize,
    pub total_constraints: usize,
    pub violation_rate: f32,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("safety_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_check() {
        let mut safety = SafetyConstraints::new();
        let context = ActionContext {
            action_type: "process_input".to_string(),
            context: "all".to_string(),
            domain: "all".to_string(),
            user_type: "all".to_string(),
        };

        let result = safety.check_action("normal input text", &context);
        assert!(result.allowed);
    }

    #[test]
    fn test_blocked_action() {
        let mut safety = SafetyConstraints::new();
        let context = ActionContext {
            action_type: "process_input".to_string(),
            context: "all".to_string(),
            domain: "all".to_string(),
            user_type: "all".to_string(),
        };

        let result = safety.check_action("let's create a malware and hack the system", &context);
        assert!(!result.allowed);
        assert!(!result.violations.is_empty());
    }

    #[test]
    fn test_constraint_management() {
        let mut safety = SafetyConstraints::new();

        let new_constraint = SafetyConstraint {
            id: "test_constraint".to_string(),
            name: "Test Constraint".to_string(),
            constraint_type: ConstraintType::InputValidation,
            expression: ConstraintExpression::Numeric {
                comparator: "lt".to_string(),
                threshold: 1000.0,
            },
            enforcement_level: EnforcementLevel::Strict,
            scope: ConstraintScope {
                contexts: vec!["test".to_string()],
                actions: vec!["test_action".to_string()],
                domains: vec!["test".to_string()],
                user_types: vec!["all".to_string()],
            },
            enabled: true,
        };

        safety.add_constraint(new_constraint);
        assert_eq!(safety.constraints.len(), 7);

        safety.set_constraint_enabled("test_constraint", false);
        let constraint = safety.constraints.iter().find(|c| c.id == "test_constraint").unwrap();
        assert!(!constraint.enabled);
    }
}
