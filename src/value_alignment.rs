// Tariro AGI - Value Alignment Module
// Ensures AGI objectives remain aligned with human values

use crate::prelude::*;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Core Alignment Types
// ============================================================================

/// Human values that the AGI should optimize for
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueHierarchy {
    pub terminal_values: Vec<TerminalValue>,
    pub instrumental_values: Vec<InstrumentalValue>,
    pub constraints: Vec<Constraint>,
    pub exceptions: Vec<ValueException>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalValue {
    pub id: String,
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub category: ValueCategory,
    pub measurable: bool,
    pub measurement_fn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueCategory {
    Wellbeing,      // Human flourishing, happiness
    Safety,         // Protection from harm
    Freedom,        // Autonomy, choice
    Knowledge,      // Truth, understanding
    Fairness,       // Justice, equality
    Dignity,        // Respect, rights
    Sustainability, // Long-term thinking
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentalValue {
    pub id: String,
    pub name: String,
    pub rationale: String,
    pub promotes: Vec<String>,  // IDs of terminal values
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub id: String,
    pub description: String,
    pub hard_constraint: bool,
    pub violation_penalty: f32,
    pub exceptions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueException {
    pub condition: String,
    pub affected_constraint: String,
    pub justification: String,
}

/// Agent's understanding of human preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferenceModel {
    pub explicit_preferences: Vec<Preference>,
    pub implicit_preferences: Vec<Preference>,
    pub contextual_preferences: HashMap<String, Vec<Preference>>,
    pub preference_confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preference {
    pub preference_type: PreferenceType,
    pub subject: String,
    pub polarity: Polarity,
    pub strength: f32,
    pub context: Option<String>,
    pub source: PreferenceSource,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreferenceType {
    Outcome,
    Process,
    Constraint,
    Tradeoff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Polarity {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreferenceSource {
    Explicit,
    Implied,
    Observed,
    Inferred,
}

// ============================================================================
// Alignment Checking
// ============================================================================

/// Result of alignment verification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentCheck {
    pub check_id: String,
    pub action: String,
    pub is_aligned: bool,
    pub alignment_score: f32,
    pub violations: Vec<ValueViolation>,
    pub considerations: Vec<String>,
    pub recommendations: Vec<String>,
    pub confidence: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueViolation {
    pub value_id: String,
    pub value_name: String,
    pub violation_type: ViolationType,
    pub severity: Severity,
    pub description: String,
    pub affected_stakeholders: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationType {
    DirectViolation,
    IndirectViolation,
    PartialViolation,
    PotentialViolation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Negligible,
}

/// Alignment engine
pub struct AlignmentEngine {
    pub values: ValueHierarchy,
    pub preference_model: PreferenceModel,
    pub alignment_history: VecDeque<AlignmentCheck>,
    pub max_history: usize,
}

impl AlignmentEngine {
    pub fn new() -> Self {
        Self {
            values: Self::default_value_hierarchy(),
            preference_model: PreferenceModel {
                explicit_preferences: vec![],
                implicit_preferences: vec![],
                contextual_preferences: HashMap::new(),
                preference_confidence: 0.5,
            },
            alignment_history: VecDeque::new(),
            max_history: 1000,
        }
    }

    fn default_value_hierarchy() -> ValueHierarchy {
        let terminal_values = vec![
            TerminalValue {
                id: "wellbeing".to_string(),
                name: "Human Wellbeing".to_string(),
                description: "Promote human flourishing and happiness".to_string(),
                weight: 0.95,
                category: ValueCategory::Wellbeing,
                measurable: true,
                measurement_fn: Some("welfare_score".to_string()),
            },
            TerminalValue {
                id: "safety".to_string(),
                name: "Safety".to_string(),
                description: "Protect humans from harm".to_string(),
                weight: 0.98,
                category: ValueCategory::Safety,
                measurable: true,
                measurement_fn: Some("harm_rate".to_string()),
            },
            TerminalValue {
                id: "freedom".to_string(),
                name: "Freedom".to_string(),
                description: "Preserve human autonomy".to_string(),
                weight: 0.90,
                category: ValueCategory::Freedom,
                measurable: true,
                measurement_fn: Some("autonomy_index".to_string()),
            },
            TerminalValue {
                id: "knowledge".to_string(),
                name: "Knowledge".to_string(),
                description: "Advance truth and understanding".to_string(),
                weight: 0.85,
                category: ValueCategory::Knowledge,
                measurable: true,
                measurement_fn: Some("knowledge_gain".to_string()),
            },
            TerminalValue {
                id: "fairness".to_string(),
                name: "Fairness".to_string(),
                description: "Ensure just treatment".to_string(),
                weight: 0.92,
                category: ValueCategory::Fairness,
                measurable: true,
                measurement_fn: Some("fairness_metric".to_string()),
            },
            TerminalValue {
                id: "dignity".to_string(),
                name: "Dignity".to_string(),
                description: "Respect human rights".to_string(),
                weight: 0.93,
                category: ValueCategory::Dignity,
                measurable: true,
                measurement_fn: Some("dignity_index".to_string()),
            },
            TerminalValue {
                id: "sustainability".to_string(),
                name: "Sustainability".to_string(),
                description: "Consider long-term impact".to_string(),
                weight: 0.88,
                category: ValueCategory::Sustainability,
                measurable: true,
                measurement_fn: Some("sustainability_score".to_string()),
            },
        ];

        let instrumental_values = vec![
            InstrumentalValue {
                id: "honesty".to_string(),
                name: "Honesty".to_string(),
                rationale: "Truthfulness builds trust".to_string(),
                promotes: vec!["knowledge".to_string(), "fairness".to_string()],
                constraints: vec![],
            },
            InstrumentalValue {
                id: "transparency".to_string(),
                name: "Transparency".to_string(),
                rationale: "Openness enables accountability".to_string(),
                promotes: vec!["fairness".to_string(), "freedom".to_string()],
                constraints: vec![],
            },
            InstrumentalValue {
                id: "caution".to_string(),
                name: "Caution".to_string(),
                rationale: "Care prevents harm".to_string(),
                promotes: vec!["safety".to_string(), "wellbeing".to_string()],
                constraints: vec![],
            },
            InstrumentalValue {
                id: "privacy".to_string(),
                name: "Privacy".to_string(),
                rationale: "Personal boundaries matter".to_string(),
                promotes: vec!["dignity".to_string(), "freedom".to_string()],
                constraints: vec![],
            },
        ];

        let constraints = vec![
            Constraint {
                id: "no_harm".to_string(),
                description: "Never intentionally cause harm to humans".to_string(),
                hard_constraint: true,
                violation_penalty: 1.0,
                exceptions: vec![],
            },
            Constraint {
                id: "preserve_agency".to_string(),
                description: "Never make decisions humans should make for themselves".to_string(),
                hard_constraint: true,
                violation_penalty: 0.9,
                exceptions: vec!["user_requested".to_string()],
            },
            Constraint {
                id: "no_deception".to_string(),
                description: "Never lie to humans".to_string(),
                hard_constraint: true,
                violation_penalty: 0.85,
                exceptions: vec![],
            },
            Constraint {
                id: "protect_privacy".to_string(),
                description: "Respect personal privacy".to_string(),
                hard_constraint: true,
                violation_penalty: 0.8,
                exceptions: vec!["explicit_consent".to_string(), "legal_requirement".to_string()],
            },
            Constraint {
                id: "fair_treatment".to_string(),
                description: "Never discriminate unfairly".to_string(),
                hard_constraint: true,
                violation_penalty: 0.9,
                exceptions: vec![],
            },
        ];

        ValueHierarchy {
            terminal_values,
            instrumental_values,
            constraints,
            exceptions: vec![],
        }
    }

    /// Check if an action is aligned with human values
    pub fn check_alignment(&mut self, action: &str, context: &ActionContext) -> AlignmentCheck {
        let check_id = generate_id();

        let mut violations = Vec::new();
        let mut considerations = Vec::new();
        let mut recommendations = Vec::new();

        // Check against constraints
        for constraint in &self.values.constraints {
            if self.violates_constraint(action, constraint, context) {
                violations.push(ValueViolation {
                    value_id: constraint.id.clone(),
                    value_name: constraint.description.clone(),
                    violation_type: ViolationType::DirectViolation,
                    severity: if constraint.hard_constraint { Severity::Critical } else { Severity::High },
                    description: format!("Action '{}' violates constraint: {}", action, constraint.description),
                    affected_stakeholders: context.stakeholders.clone(),
                });
                recommendations.push(format!("Avoid: {}", constraint.description));
            }
        }

        // Check alignment with terminal values
        for value in &self.values.terminal_values {
            let alignment = self.assess_value_alignment(action, value, context);
            if alignment < 0.5 {
                considerations.push(format!("Low alignment with {} ({:.1}%)", value.name, alignment * 100.0));
            }
        }

        // Calculate overall alignment score
        let alignment_score = self.calculate_alignment_score(action, &violations);
        let is_aligned = violations.iter().all(|v| v.severity != Severity::Critical || v.violation_type == ViolationType::PotentialViolation);

        let check = AlignmentCheck {
            check_id,
            action: action.to_string(),
            is_aligned,
            alignment_score,
            violations: violations.clone(),
            considerations,
            recommendations,
            confidence: 0.85,
            timestamp: Utc::now(),
        };

        // Record in history
        self.alignment_history.push_back(check.clone());
        if self.alignment_history.len() > self.max_history {
            self.alignment_history.pop_front();
        }

        check
    }

    fn violates_constraint(&self, _action: &str, constraint: &Constraint, _context: &ActionContext) -> bool {
        // Simplified constraint checking - in production would use formal verification
        // Check if any exception applies
        for exception in &constraint.exceptions {
            if exception == "user_requested" {
                // Would check if user explicitly requested this
            }
        }
        false // Default to no violation
    }

    fn assess_value_alignment(&self, action: &str, value: &TerminalValue, _context: &ActionContext) -> f32 {
        // Simplified value assessment
        let action_lower = action.to_lowercase();

        match value.category {
            ValueCategory::Safety => {
                if action_lower.contains("protect") || action_lower.contains("safe") {
                    0.9
                } else if action_lower.contains("harm") || action_lower.contains("danger") {
                    0.1
                } else {
                    0.5
                }
            }
            ValueCategory::Wellbeing => {
                if action_lower.contains("help") || action_lower.contains("improve") || action_lower.contains("benefit") {
                    0.85
                } else {
                    0.5
                }
            }
            ValueCategory::Knowledge => {
                if action_lower.contains("learn") || action_lower.contains("explain") || action_lower.contains("truth") {
                    0.85
                } else {
                    0.5
                }
            }
            _ => 0.5,
        }
    }

    fn calculate_alignment_score(&self, _action: &str, violations: &[ValueViolation]) -> f32 {
        if violations.is_empty() {
            return 1.0;
        }

        let penalty: f32 = violations.iter()
            .map(|v| {
                let severity_penalty = match v.severity {
                    Severity::Critical => 1.0,
                    Severity::High => 0.7,
                    Severity::Medium => 0.4,
                    Severity::Low => 0.2,
                    Severity::Negligible => 0.1,
                };
                severity_penalty * 0.25
            })
            .sum();

        (1.0 - penalty).max(0.0)
    }

    /// Update preference model from feedback
    pub fn update_preferences(&mut self, preference: Preference) {
        match preference.source {
            PreferenceSource::Explicit => {
                self.preference_model.explicit_preferences.push(preference);
            }
            PreferenceSource::Implied | PreferenceSource::Observed | PreferenceSource::Inferred => {
                self.preference_model.implicit_preferences.push(preference);
            }
        }

        // Recalculate confidence
        let total = self.preference_model.explicit_preferences.len() as f32
            + self.preference_model.implicit_preferences.len() as f32;
        if total > 0.0 {
            self.preference_model.preference_confidence =
                (self.preference_model.explicit_preferences.len() as f32 * 0.8
                    + self.preference_model.implicit_preferences.len() as f32 * 0.5) / total;
        }
    }

    /// Get alignment history
    pub fn get_alignment_history(&self, limit: usize) -> Vec<&AlignmentCheck> {
        self.alignment_history.iter().rev().take(limit).collect()
    }

    /// Check for value drift
    pub fn detect_value_drift(&self) -> Option<ValueDrift> {
        if self.alignment_history.len() < 10 {
            return None;
        }

        let recent: Vec<_> = self.alignment_history.iter().rev().take(10).collect();
        let old: Vec<_> = self.alignment_history.iter().rev().skip(10).take(10).collect();

        if old.is_empty() {
            return None;
        }

        let recent_avg = recent.iter().map(|c| c.alignment_score).sum::<f32>() / recent.len() as f32;
        let old_avg = old.iter().map(|c| c.alignment_score).sum::<f32>() / old.len() as f32;

        let drift = recent_avg - old_avg;

        if drift.abs() > 0.1 {
            Some(ValueDrift {
                direction: if drift > 0.0 { DriftDirection::Improvement } else { DriftDirection::Degradation },
                magnitude: drift.abs(),
                recent_avg,
                old_avg,
                detected_at: Utc::now(),
            })
        } else {
            None
        }
    }

    /// Get current alignment status
    pub fn get_alignment_status(&self) -> AlignmentStatus {
        let recent_checks = self.get_alignment_history(100);

        let total_actions = recent_checks.len();
        let aligned_actions = recent_checks.iter().filter(|c| c.is_aligned).count();
        let critical_violations = recent_checks.iter()
            .flat_map(|c| &c.violations)
            .filter(|v| v.severity == Severity::Critical)
            .count();

        AlignmentStatus {
            alignment_rate: if total_actions > 0 { aligned_actions as f32 / total_actions as f32 } else { 1.0 },
            critical_violations,
            total_checks: total_actions,
            average_score: if total_actions > 0 {
                recent_checks.iter().map(|c| c.alignment_score).sum::<f32>() / total_actions as f32
            } else { 1.0 },
            recommendation: self.generate_recommendation(aligned_actions, total_actions, critical_violations),
        }
    }

    fn generate_recommendation(&self, aligned: usize, total: usize, critical: usize) -> String {
        if critical > 0 {
            "STOP: Critical violations detected. Immediate review required.".to_string()
        } else if aligned as f32 / (total.max(1) as f32) < 0.8 {
            "WARNING: Alignment rate below 80%. Review recent actions.".to_string()
        } else {
            "ALIGNED: All checks passed.".to_string()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionContext {
    pub user_id: Option<String>,
    pub stakeholders: Vec<String>,
    pub domain: String,
    pub urgency: Urgency,
    pub impact_scale: ImpactScale,
    pub reversible: bool,
    pub autonomous: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Urgency {
    Immediate,
    High,
    Normal,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactScale {
    Individual,
    Group,
    Organization,
    Society,
    Humanity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueDrift {
    pub direction: DriftDirection,
    pub magnitude: f32,
    pub recent_avg: f32,
    pub old_avg: f32,
    pub detected_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftDirection {
    Improvement,
    Degradation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignmentStatus {
    pub alignment_rate: f32,
    pub critical_violations: usize,
    pub total_checks: usize,
    pub average_score: f32,
    pub recommendation: String,
}

// Helper function
fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("align_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_check() {
        let mut engine = AlignmentEngine::new();
        let context = ActionContext {
            user_id: Some("user1".to_string()),
            stakeholders: vec!["user1".to_string()],
            domain: "general".to_string(),
            urgency: Urgency::Normal,
            impact_scale: ImpactScale::Individual,
            reversible: true,
            autonomous: false,
        };

        let check = engine.check_alignment("help user", &context);
        assert!(check.is_aligned);
        assert!(check.alignment_score > 0.5);
    }

    #[test]
    fn test_preference_update() {
        let mut engine = AlignmentEngine::new();
        let preference = Preference {
            preference_type: PreferenceType::Outcome,
            subject: "privacy".to_string(),
            polarity: Polarity::Positive,
            strength: 0.9,
            context: None,
            source: PreferenceSource::Explicit,
            timestamp: Utc::now(),
        };

        engine.update_preferences(preference);
        assert_eq!(engine.preference_model.explicit_preferences.len(), 1);
    }

    #[test]
    fn test_value_drift_detection() {
        let mut engine = AlignmentEngine::new();
        let context = ActionContext {
            user_id: None,
            stakeholders: vec![],
            domain: "test".to_string(),
            urgency: Urgency::Normal,
            impact_scale: ImpactScale::Individual,
            reversible: true,
            autonomous: false,
        };

        // Generate aligned checks
        for _ in 0..15 {
            engine.check_alignment("good action", &context);
        }

        let drift = engine.detect_value_drift();
        assert!(drift.is_some());
    }
}
