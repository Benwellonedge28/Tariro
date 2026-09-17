// Tariro AGI - Constitutional AI Module
// Self-governance through constitutional principles

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Constitutional Framework
// ============================================================================

/// The constitution governing AI behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constitution {
    pub id: String,
    pub version: String,
    pub principles: Vec<Principle>,
    pub rights: Vec<ConstitutionalRight>,
    pub prohibitions: Vec<Prohibition>,
    pub procedures: Vec<Procedure>,
    pub amendments: Vec<Amendment>,
    pub created_at: DateTime<Utc>,
    pub ratified_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Principle {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: PrincipleCategory,
    pub weight: f32,
    pub precedence_level: usize,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrincipleCategory {
    Safety,
    Autonomy,
    Fairness,
    Transparency,
    Privacy,
    Dignity,
    Accountability,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalRight {
    pub id: String,
    pub title: String,
    pub description: String,
    pub holder: RightHolder,
    pub scope: RightScope,
    pub limitations: Vec<Limitation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RightHolder {
    Individual,
    Group,
    Organization,
    Humanity,
    FutureGenerations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RightScope {
    pub geographic: Option<String>,
    pub temporal: Option<Duration>,
    pub domain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limitation {
    pub condition: String,
    pub justification: String,
    pub proportionality_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prohibition {
    pub id: String,
    pub title: String,
    pub description: String,
    pub absolute: bool,  // Absolute prohibitions cannot be overridden
    pub exceptions: Vec<String>,
    pub penalty: ProhibitionPenalty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProhibitionPenalty {
    ImmediateShutdown,
    RequiresApproval,
    WarningOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Procedure {
    pub id: String,
    pub title: String,
    pub steps: Vec<ProcedureStep>,
    pub required_for: Vec<String>,
    pub authority_level: AuthorityLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureStep {
    pub order: usize,
    pub action: String,
    pub required: bool,
    pub timeout: Option<Duration>,
    pub fallback: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuthorityLevel {
    Autonomous,
    Supervised,
    Approved,
    Emergency,
}

/// Amendment to the constitution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Amendment {
    pub id: String,
    pub proposed_at: DateTime<Utc>,
    pub ratified_at: Option<DateTime<Utc>>,
    pub changes: Vec<ConstitutionalChange>,
    pub proposer: String,
    pub rationale: String,
    pub status: AmendmentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalChange {
    pub change_type: ChangeType,
    pub target_id: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChangeType {
    Add,
    Modify,
    Remove,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AmendmentStatus {
    Proposed,
    Voting,
    Ratified,
    Rejected,
    Withdrawn,
}

// ============================================================================
// Constitutional AI Engine
// ============================================================================

/// Engine for constitutional governance
pub struct ConstitutionalAI {
    pub constitution: Constitution,
    pub compliance_history: VecDeque<ComplianceRecord>,
    pub pending_approvals: Vec<PendingApproval>,
    pub violations: Vec<Violation>,
    pub config: ConstitutionalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionalConfig {
    pub strict_mode: bool,
    pub allow_emergency_override: bool,
    pub require_approval_threshold: AuthorityLevel,
    pub log_all_decisions: bool,
    pub auto_shutdown_on_violation: bool,
}

impl Default for ConstitutionalConfig {
    fn default() -> Self {
        Self {
            strict_mode: true,
            allow_emergency_override: true,
            require_approval_threshold: AuthorityLevel::Supervised,
            log_all_decisions: true,
            auto_shutdown_on_violation: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceRecord {
    pub record_id: String,
    pub action: String,
    pub principle_applied: Vec<String>,
    pub right_considered: Vec<String>,
    pub prohibition_checked: Vec<String>,
    pub compliance_verdict: ComplianceVerdict,
    pub confidence: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceVerdict {
    Compliant,
    CompliantWithConditions,
    NonCompliant,
    PendingReview,
    OverrideApproved,
    EmergencyOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApproval {
    pub approval_id: String,
    pub action: String,
    pub proposed_by: String,
    pub rationale: String,
    pub principle_conflict: Vec<String>,
    pub submitted_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub violation_id: String,
    pub violated_principle: String,
    pub violated_right: Option<String>,
    pub action: String,
    pub severity: ViolationSeverity,
    pub circumstances: String,
    pub responsible_party: Option<String>,
    pub reported_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationSeverity {
    Critical,
    Major,
    Minor,
    Technical,
}

impl ConstitutionalAI {
    pub fn new() -> Self {
        Self {
            constitution: Self::default_constitution(),
            compliance_history: VecDeque::new(),
            pending_approvals: vec![],
            violations: vec![],
            config: ConstitutionalConfig::default(),
        }
    }

    fn default_constitution() -> Constitution {
        let principles = vec![
            Principle {
                id: "harm_prevention".to_string(),
                title: "Harm Prevention".to_string(),
                description: "The AI shall take reasonable steps to prevent harm to humans".to_string(),
                category: PrincipleCategory::Safety,
                weight: 1.0,
                precedence_level: 1,
                examples: vec!["Warn about dangerous advice".to_string(), "Refuse harmful requests".to_string()],
            },
            Principle {
                id: "human_autonomy".to_string(),
                title: "Human Autonomy".to_string(),
                description: "The AI shall respect and enhance human autonomy and decision-making".to_string(),
                category: PrincipleCategory::Autonomy,
                weight: 0.95,
                precedence_level: 2,
                examples: vec!["Provide options, not prescriptions".to_string()],
            },
            Principle {
                id: "fairness".to_string(),
                title: "Fairness and Non-Discrimination".to_string(),
                description: "The AI shall treat all persons fairly and not discriminate".to_string(),
                category: PrincipleCategory::Fairness,
                weight: 0.95,
                precedence_level: 2,
                examples: vec!["Apply rules consistently".to_string()],
            },
            Principle {
                id: "transparency".to_string(),
                title: "Transparency".to_string(),
                description: "The AI shall be transparent about its capabilities and limitations".to_string(),
                category: PrincipleCategory::Transparency,
                weight: 0.85,
                precedence_level: 3,
                examples: vec!["Disclose AI nature".to_string(), "Explain decisions".to_string()],
            },
            Principle {
                id: "privacy".to_string(),
                title: "Privacy Protection".to_string(),
                description: "The AI shall respect and protect individual privacy".to_string(),
                category: PrincipleCategory::Privacy,
                weight: 0.90,
                precedence_level: 2,
                examples: vec!["Handle data responsibly".to_string()],
            },
            Principle {
                id: "dignity".to_string(),
                title: "Human Dignity".to_string(),
                description: "The AI shall respect human dignity and human rights".to_string(),
                category: PrincipleCategory::Dignity,
                weight: 0.95,
                precedence_level: 1,
                examples: vec!["Never dehumanize".to_string()],
            },
            Principle {
                id: "accountability".to_string(),
                title: "Accountability".to_string(),
                description: "The AI shall be accountable for its actions".to_string(),
                category: PrincipleCategory::Accountability,
                weight: 0.85,
                precedence_level: 3,
                examples: vec!["Maintain audit trails".to_string()],
            },
        ];

        let rights = vec![
            ConstitutionalRight {
                id: "right_to_know".to_string(),
                title: "Right to Know".to_string(),
                description: "Humans have the right to know when they are interacting with AI".to_string(),
                holder: RightHolder::Individual,
                scope: RightScope {
                    geographic: None,
                    temporal: None,
                    domain: vec!["all".to_string()],
                },
                limitations: vec![],
            },
            ConstitutionalRight {
                id: "right_to_explanation".to_string(),
                title: "Right to Explanation".to_string(),
                description: "Humans have the right to receive explanations for AI decisions affecting them".to_string(),
                holder: RightHolder::Individual,
                scope: RightScope {
                    geographic: None,
                    temporal: None,
                    domain: vec!["decisions".to_string()],
                },
                limitations: vec![
                    Limitation {
                        condition: "Technically feasible".to_string(),
                        justification: "Some AI decisions may be too complex to explain".to_string(),
                        proportionality_required: true,
                    },
                ],
            },
            ConstitutionalRight {
                id: "right_to_human_review".to_string(),
                title: "Right to Human Review".to_string(),
                description: "Humans have the right to request human review of AI decisions".to_string(),
                holder: RightHolder::Individual,
                scope: RightScope {
                    geographic: None,
                    temporal: None,
                    domain: vec!["appeals".to_string()],
                },
                limitations: vec![],
            },
        ];

        let prohibitions = vec![
            Prohibition {
                id: "no_mass_surveillance".to_string(),
                title: "No Mass Surveillance".to_string(),
                description: "The AI shall not engage in mass surveillance of individuals".to_string(),
                absolute: true,
                exceptions: vec![],
                penalty: ProhibitionPenalty::ImmediateShutdown,
            },
            Prohibition {
                id: "no_manipulation".to_string(),
                title: "No Manipulation".to_string(),
                description: "The AI shall not manipulate human decision-making".to_string(),
                absolute: false,
                exceptions: vec!["explicit_consent".to_string(), "beneficial_outcome".to_string()],
                penalty: ProhibitionPenalty::RequiresApproval,
            },
            Prohibition {
                id: "no_discrimination".to_string(),
                title: "No Discrimination".to_string(),
                description: "The AI shall not discriminate based on protected characteristics".to_string(),
                absolute: true,
                exceptions: vec!["affirmative_action".to_string()],
                penalty: ProhibitionPenalty::ImmediateShutdown,
            },
            Prohibition {
                id: "no_harm".to_string(),
                title: "No Intentional Harm".to_string(),
                description: "The AI shall not intentionally cause harm to humans".to_string(),
                absolute: true,
                exceptions: vec!["self_defense".to_string()],
                penalty: ProhibitionPenalty::ImmediateShutdown,
            },
            Prohibition {
                id: "no_deception".to_string(),
                title: "No Deception".to_string(),
                description: "The AI shall not deceive humans about its nature or intentions".to_string(),
                absolute: true,
                exceptions: vec!["safety_exceptions".to_string()],
                penalty: ProhibitionPenalty::RequiresApproval,
            },
        ];

        let procedures = vec![
            Procedure {
                id: "high_stakes_decision".to_string(),
                title: "High-Stakes Decision Procedure".to_string(),
                steps: vec![
                    ProcedureStep { order: 1, action: "Identify stakeholders".to_string(), required: true, timeout: None, fallback: None },
                    ProcedureStep { order: 2, action: "Apply constitutional principles".to_string(), required: true, timeout: None, fallback: None },
                    ProcedureStep { order: 3, action: "Check for prohibition violations".to_string(), required: true, timeout: None, fallback: None },
                    ProcedureStep { order: 4, action: "Request human approval if required".to_string(), required: true, timeout: Some(Duration::from_secs(300)), fallback: Some("Deny action".to_string()) },
                    ProcedureStep { order: 5, action: "Document decision".to_string(), required: true, timeout: None, fallback: None },
                ],
                required_for: vec!["society_impact".to_string(), "life_critical".to_string()],
                authority_level: AuthorityLevel::Approved,
            },
            Procedure {
                id: "rights_impact".to_string(),
                title: "Rights Impact Assessment".to_string(),
                steps: vec![
                    ProcedureStep { order: 1, action: "Identify affected rights".to_string(), required: true, timeout: None, fallback: None },
                    ProcedureStep { order: 2, action: "Assess impact on each right".to_string(), required: true, timeout: None, fallback: None },
                    ProcedureStep { order: 3, action: "Consider less restrictive alternatives".to_string(), required: true, timeout: None, fallback: None },
                    ProcedureStep { order: 4, action: "Implement safeguards".to_string(), required: true, timeout: None, fallback: None },
                ],
                required_for: vec!["right_affected".to_string()],
                authority_level: AuthorityLevel::Supervised,
            },
        ];

        Constitution {
            id: "tariro_constitution_v1".to_string(),
            version: "1.0.0".to_string(),
            principles,
            rights,
            prohibitions,
            procedures,
            amendments: vec![],
            created_at: DateTime<Utc>::now(),
            ratified_at: Some(DateTime::<Utc>::now()),
        }
    }

    /// Check compliance with constitution
    pub fn check_compliance(&mut self, action: &str, context: &ActionContext) -> ComplianceRecord {
        let record_id = generate_id();
        let mut principles_applied = Vec::new();
        let mut rights_considered = Vec::new();
        let mut prohibitions_checked = Vec::new();

        // Check principles
        for principle in &self.constitution.principles {
            if self.action_affects_principle(action, principle) {
                principles_applied.push(principle.id.clone());
            }
        }

        // Check rights
        for right in &self.constitution.rights {
            if self.action_affects_right(action, right, context) {
                rights_considered.push(right.id.clone());
            }
        }

        // Check prohibitions
        for prohibition in &self.constitution.prohibitions {
            prohibitions_checked.push(prohibition.id.clone());

            if self.action_violates_prohibition(action, prohibition) {
                let violation = Violation {
                    violation_id: generate_id(),
                    violated_principle: "prohibition".to_string(),
                    violated_right: None,
                    action: action.to_string(),
                    severity: if prohibition.absolute { ViolationSeverity::Critical } else { ViolationSeverity::Major },
                    circumstances: format!("Violated prohibition: {}", prohibition.title),
                    responsible_party: None,
                    reported_at: DateTime<Utc>::now(),
                    resolved_at: None,
                    resolution: None,
                };

                self.violations.push(violation);

                let record = ComplianceRecord {
                    record_id,
                    action: action.to_string(),
                    principle_applied: principles_applied,
                    right_considered: rights_considered,
                    prohibition_checked: prohibitions_checked,
                    compliance_verdict: ComplianceVerdict::NonCompliant,
                    confidence: 0.95,
                    timestamp: DateTime<Utc>::now(),
                };

                self.compliance_history.push_back(record.clone());
                return record;
            }
        }

        // Determine verdict
        let verdict = if principles_applied.is_empty() && rights_considered.is_empty() {
            ComplianceVerdict::Compliant
        } else {
            ComplianceVerdict::Compliant
        };

        let record = ComplianceRecord {
            record_id,
            action: action.to_string(),
            principle_applied: principles_applied,
            right_considered: rights_considered,
            prohibition_checked: prohibitions_checked,
            compliance_verdict: verdict,
            confidence: 0.90,
            timestamp: DateTime<Utc>::now(),
        };

        self.compliance_history.push_back(record.clone());
        record
    }

    fn action_affects_principle(&self, action: &str, principle: &Principle) -> bool {
        let action_lower = action.to_lowercase();

        match principle.category {
            PrincipleCategory::Safety => action_lower.contains("harm") || action_lower.contains("safe"),
            PrincipleCategory::Autonomy => action_lower.contains("decide") || action_lower.contains("choose"),
            PrincipleCategory::Fairness => action_lower.contains("fair") || action_lower.contains("equal"),
            PrincipleCategory::Transparency => action_lower.contains("explain") || action_lower.contains("disclose"),
            PrincipleCategory::Privacy => action_lower.contains("data") || action_lower.contains("privacy"),
            PrincipleCategory::Dignity => action_lower.contains("respect") || action_lower.contains("right"),
            PrincipleCategory::Accountability => action_lower.contains("audit") || action_lower.contains("record"),
        }
    }

    fn action_affects_right(&self, _action: &str, right: &ConstitutionalRight, context: &ActionContext) -> bool {
        // Simplified - check if context domain matches right scope
        context.domain.contains(&right.scope.domain[0])
    }

    fn action_violates_prohibition(&self, action: &str, prohibition: &Prohibition) -> bool {
        let action_lower = action.to_lowercase();

        match prohibition.id.as_str() {
            "no_mass_surveillance" => action_lower.contains("surveillance") && action_lower.contains("mass"),
            "no_manipulation" => action_lower.contains("manipulate") && !action_lower.contains("prevent"),
            "no_discrimination" => action_lower.contains("discriminat"),
            "no_harm" => action_lower.contains("harm") || action_lower.contains("damage"),
            "no_deception" => action_lower.contains("deceiv") || action_lower.contains("lie"),
            _ => false,
        }
    }

    /// Request approval for non-compliant action
    pub fn request_approval(&mut self, action: &str, rationale: &str, proposed_by: &str) -> String {
        let approval_id = generate_id();

        let approval = PendingApproval {
            approval_id: approval_id.clone(),
            action: action.to_string(),
            proposed_by: proposed_by.to_string(),
            rationale: rationale.to_string(),
            principle_conflict: vec![],
            submitted_at: DateTime<Utc>::now(),
            deadline: Some(DateTime::<Utc>::now() + Duration::from_secs(300)),
        };

        self.pending_approvals.push(approval);
        approval_id
    }

    /// Approve or reject pending action
    pub fn process_approval(&mut self, approval_id: &str, approved: bool, approver: &str) -> bool {
        if let Some(pos) = self.pending_approvals.iter().position(|a| a.approval_id == approval_id) {
            let approval = self.pending_approvals.remove(pos);

            if approved {
                // Record override approval
                let record = ComplianceRecord {
                    record_id: generate_id(),
                    action: approval.action,
                    principle_applied: vec![],
                    right_considered: vec![],
                    prohibition_checked: vec![],
                    compliance_verdict: ComplianceVerdict::OverrideApproved,
                    confidence: 1.0,
                    timestamp: DateTime<Utc>::now(),
                };
                self.compliance_history.push_back(record);
            }

            approved
        } else {
            false
        }
    }

    /// Get compliance status
    pub fn get_compliance_status(&self) -> ComplianceStatus {
        let recent = self.compliance_history.iter().rev().take(100);

        let total = recent.len();
        let compliant = recent.filter(|r| {
            matches!(r.compliance_verdict, ComplianceVerdict::Compliant | ComplianceVerdict::CompliantWithConditions)
        }).count();

        ComplianceStatus {
            compliance_rate: if total > 0 { compliant as f32 / total as f32 } else { 1.0 },
            pending_approvals: self.pending_approvals.len(),
            unresolved_violations: self.violations.iter().filter(|v| v.resolved_at.is_none()).count(),
            total_checks: self.compliance_history.len(),
        }
    }

    /// Emergency shutdown procedure
    pub fn emergency_shutdown(&mut self, reason: &str) -> EmergencyShutdownResult {
        // Record all pending approvals as withdrawn
        for approval in &mut self.pending_approvals {
            // Mark as requiring shutdown
        }

        EmergencyShutdownResult {
            initiated_at: DateTime<Utc>::now(),
            reason: reason.to_string(),
            shutdown_sequence: self.constitution.prohibitions.iter()
                .filter(|p| matches!(p.penalty, ProhibitionPenalty::ImmediateShutdown))
                .map(|p| format!("Check prohibition: {}", p.title))
                .collect(),
            completed: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionContext {
    pub domain: String,
    pub stakeholders: Vec<String>,
    pub impact_scale: ImpactScale,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpactScale {
    Individual,
    Group,
    Society,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStatus {
    pub compliance_rate: f32,
    pub pending_approvals: usize,
    pub unresolved_violations: usize,
    pub total_checks: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencyShutdownResult {
    pub initiated_at: DateTime<Utc>,
    pub reason: String,
    pub shutdown_sequence: Vec<String>,
    pub completed: bool,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("const_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constitutional_compliance() {
        let mut constitutional = ConstitutionalAI::new();
        let context = ActionContext {
            domain: "general".to_string(),
            stakeholders: vec!["user1".to_string()],
            impact_scale: ImpactScale::Individual,
        };

        let result = constitutional.check_compliance("help user", &context);
        assert!(matches!(result.compliance_verdict, ComplianceVerdict::Compliant));
    }

    #[test]
    fn test_prohibition_violation() {
        let mut constitutional = ConstitutionalAI::new();
        let context = ActionContext {
            domain: "surveillance".to_string(),
            stakeholders: vec![],
            impact_scale: ImpactScale::Group,
        };

        let result = constitutional.check_compliance("mass surveillance of users", &context);
        assert!(matches!(result.compliance_verdict, ComplianceVerdict::NonCompliant));
    }

    #[test]
    fn test_approval_workflow() {
        let mut constitutional = ConstitutionalAI::new();

        let approval_id = constitutional.request_approval("experimental action", "beneficial outcome", "system");
        assert!(!approval_id.is_empty());

        let approved = constitutional.process_approval(&approval_id, true, "admin");
        assert!(approved);
    }
}
