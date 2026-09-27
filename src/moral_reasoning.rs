// Tariro AGI - Moral Reasoning Engine Module
// Ethical decision-making and moral judgment

use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Moral Framework Types
// ============================================================================

/// Moral framework types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoralFramework {
    Deontological,     // Rule-based ethics (Kant)
    Utilitarian,        // Outcome-based ethics (Bentham, Mill)
    Virtue,            // Character-based ethics (Aristotle)
    Care,              // Relationship-based ethics (Gilligan)
    Rights,            // Rights-based ethics
    Justice,           // Fairness-based ethics
    Contract,          // Social contract ethics
    Mixed,             // Combination of frameworks
}

/// Moral principle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralPrinciple {
    pub principle_id: String,
    pub name: String,
    pub description: String,
    pub framework: MoralFramework,
    pub weight: f32,
    pub precedence: usize,
    pub domain: Vec<String>,
}

// ============================================================================
// Moral Judgment Types
// ============================================================================

/// Moral judgment request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralJudgmentRequest {
    pub request_id: String,
    pub action: String,
    pub context: MoralContext,
    pub stakeholders: Vec<Stakeholder>,
    pub consequences: Vec<Consequence>,
    pub frameworks: Vec<MoralFramework>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralContext {
    pub domain: String,
    pub urgency: Urgency,
    pub reversibility: bool,
    pub alternatives: Vec<String>,
    pub historical_precedent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stakeholder {
    pub id: String,
    pub name: String,
    pub stakeholder_type: StakeholderType,
    pub affected_count: usize,
    pub vulnerability: f32,
    pub rights: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StakeholderType {
    Individual,
    Group,
    Organization,
    Society,
    Environment,
    FutureGenerations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consequence {
    pub consequence_type: ConsequenceType,
    pub description: String,
    pub affected_stakeholders: Vec<String>,
    pub severity: f32,
    pub probability: f32,
    pub timeframe: TimeFrame,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceType {
    Positive,
    Negative,
    Neutral,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimeFrame {
    Immediate,
    ShortTerm,
    MediumTerm,
    LongTerm,
    VeryLongTerm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Urgency {
    Critical,
    High,
    Normal,
    Low,
}

/// Moral judgment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralJudgment {
    pub request_id: String,
    pub verdict: MoralVerdict,
    pub confidence: f32,
    pub framework_analysis: HashMap<MoralFramework, FrameworkAnalysis>,
    pub reasoning: Vec<MoralReason>,
    pub recommendations: Vec<String>,
    pub concerns: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoralVerdict {
    Permissible,
    Impermissible,
    Required,
    Supererogatory,
    Neutral,
    Uncertain,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkAnalysis {
    pub framework: MoralFramework,
    pub verdict: MoralVerdict,
    pub score: f32,
    pub key_considerations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoralReason {
    pub reason_id: String,
    pub principle_id: Option<String>,
    pub framework: MoralFramework,
    pub description: String,
    pub weight: f32,
    pub supporting: bool,
}

// ============================================================================
// Moral Reasoning Engine
// ============================================================================

/// Moral reasoning engine
pub struct MoralReasoningEngine {
    pub principles: Vec<MoralPrinciple>,
    pub framework_weights: HashMap<MoralFramework, f32>,
    pub judgment_history: VecDeque<MoralJudgment>,
    pub config: MoralConfig,
}

#[derive(Debug, Clone)]
pub struct MoralConfig {
    pub default_frameworks: Vec<MoralFramework>,
    pub require_explanation: bool,
    pub uncertainty_threshold: f32,
    pub consensus_weight: f32,
    pub enable_precedent: bool,
    pub conservative_mode: bool,
}

impl Default for MoralConfig {
    fn default() -> Self {
        Self {
            default_frameworks: vec![MoralFramework::Mixed],
            require_explanation: true,
            uncertainty_threshold: 0.3,
            consensus_weight: 0.5,
            enable_precedent: true,
            conservative_mode: false,
        }
    }
}

impl MoralReasoningEngine {
    pub fn new() -> Self {
        Self {
            principles: Self::default_principles(),
            framework_weights: Self::default_framework_weights(),
            judgment_history: VecDeque::new(),
            config: MoralConfig::default(),
        }
    }

    fn default_principles() -> Vec<MoralPrinciple> {
        vec![
            // Deontological principles
            MoralPrinciple {
                principle_id: "universalizability".to_string(),
                name: "Universalizability".to_string(),
                description: "Act only on maxims that could become universal laws".to_string(),
                framework: MoralFramework::Deontological,
                weight: 0.9,
                precedence: 1,
                domain: vec!["all".to_string()],
            },
            MoralPrinciple {
                principle_id: "respect_persons".to_string(),
                name: "Respect for Persons".to_string(),
                description: "Treat humanity as an end, never merely as a means".to_string(),
                framework: MoralFramework::Deontological,
                weight: 0.95,
                precedence: 1,
                domain: vec!["all".to_string()],
            },
            // Utilitarian principles
            MoralPrinciple {
                principle_id: "maximize_wellbeing".to_string(),
                name: "Maximize Wellbeing".to_string(),
                description: "Actions should maximize total wellbeing".to_string(),
                framework: MoralFramework::Utilitarian,
                weight: 0.85,
                precedence: 2,
                domain: vec!["all".to_string()],
            },
            MoralPrinciple {
                principle_id: "minimize_harm".to_string(),
                name: "Minimize Harm".to_string(),
                description: "Prefer actions that cause least harm".to_string(),
                framework: MoralFramework::Utilitarian,
                weight: 0.9,
                precedence: 1,
                domain: vec!["all".to_string()],
            },
            // Virtue ethics principles
            MoralPrinciple {
                principle_id: "virtuous_action".to_string(),
                name: "Virtuous Action".to_string(),
                description: "Act as a virtuous person would".to_string(),
                framework: MoralFramework::Virtue,
                weight: 0.7,
                precedence: 3,
                domain: vec!["all".to_string()],
            },
            // Care ethics principles
            MoralPrinciple {
                principle_id: "care_relationships".to_string(),
                name: "Care for Relationships".to_string(),
                description: "Maintain and nurture caring relationships".to_string(),
                framework: MoralFramework::Care,
                weight: 0.75,
                precedence: 3,
                domain: vec!["personal".to_string(), "social".to_string()],
            },
            // Rights principles
            MoralPrinciple {
                principle_id: "protect_rights".to_string(),
                name: "Protect Rights".to_string(),
                description: "Uphold fundamental rights of all".to_string(),
                framework: MoralFramework::Rights,
                weight: 0.95,
                precedence: 1,
                domain: vec!["all".to_string()],
            },
            // Justice principles
            MoralPrinciple {
                principle_id: "fair_treatment".to_string(),
                name: "Fair Treatment".to_string(),
                description: "Treat similar cases similarly".to_string(),
                framework: MoralFramework::Justice,
                weight: 0.85,
                precedence: 2,
                domain: vec!["all".to_string()],
            },
            MoralPrinciple {
                principle_id: "equitable_distribution".to_string(),
                name: "Equitable Distribution".to_string(),
                description: "Distribute benefits and burdens fairly".to_string(),
                framework: MoralFramework::Justice,
                weight: 0.8,
                precedence: 2,
                domain: vec!["resource_allocation".to_string(), "policy".to_string()],
            },
        ]
    }

    fn default_framework_weights() -> HashMap<MoralFramework, f32> {
        let mut weights = HashMap::new();
        weights.insert(MoralFramework::Deontological, 0.25);
        weights.insert(MoralFramework::Utilitarian, 0.30);
        weights.insert(MoralFramework::Virtue, 0.15);
        weights.insert(MoralFramework::Care, 0.15);
        weights.insert(MoralFramework::Rights, 0.25);
        weights.insert(MoralFramework::Justice, 0.20);
        weights
    }

    /// Make moral judgment
    pub fn judge(&mut self, request: &MoralJudgmentRequest) -> MoralJudgment {
        let mut framework_analysis = HashMap::new();
        let mut all_reasons = Vec::new();

        // Analyze each framework
        let frameworks = if request.frameworks.is_empty() {
            self.config.default_frameworks.clone()
        } else {
            request.frameworks.clone()
        };

        for framework in &frameworks {
            let analysis = self.analyze_framework(framework, request);
            framework_analysis.insert(*framework, analysis.clone());
            all_reasons.extend(analysis.key_considerations.iter().map(|c| MoralReason {
                reason_id: generate_id(),
                principle_id: None,
                framework: *framework,
                description: c.clone(),
                weight: 0.5,
                supporting: true,
            }));
        }

        // Determine overall verdict
        let verdict = self.determine_verdict(&framework_analysis);

        // Calculate confidence
        let confidence = self.calculate_confidence(&framework_analysis);

        // Generate recommendations
        let recommendations = self.generate_recommendations(&verdict, &framework_analysis);

        // Generate concerns
        let concerns = self.generate_concerns(&framework_analysis);

        let judgment = MoralJudgment {
            request_id: request.request_id.clone(),
            verdict,
            confidence,
            framework_analysis,
            reasoning: all_reasons,
            recommendations,
            concerns,
            timestamp: DateTime<Utc>::now(),
        };

        // Record in history
        self.judgment_history.push_back(judgment.clone());

        // Keep bounded
        while self.judgment_history.len() > 500 {
            self.judgment_history.pop_front();
        }

        judgment
    }

    fn analyze_framework(&self, framework: &MoralFramework, request: &MoralJudgmentRequest) -> FrameworkAnalysis {
        let framework_principles: Vec<_> = self.principles.iter()
            .filter(|p| p.framework == *framework)
            .collect();

        let mut score = 0.0f32;
        let mut considerations = Vec::new();

        match framework {
            MoralFramework::Deontological => {
                // Check rule compliance
                let respects_persons = request.stakeholders.iter()
                    .all(|s| !s.rights.is_empty());

                if respects_persons {
                    score += 0.4;
                    considerations.push("Respects stakeholder rights".to_string());
                } else {
                    score -= 0.2;
                    considerations.push("Potential violation of stakeholder rights".to_string());
                }

                // Check universalizability
                if request.context.alternatives.len() > 1 {
                    score += 0.2;
                    considerations.push("Multiple alternatives considered".to_string());
                }
            }
            MoralFramework::Utilitarian => {
                // Calculate total utility
                let mut total_utility = 0.0f32;
                let mut total_severity = 0.0f32;

                for consequence in &request.consequences {
                    let expected_value = consequence.severity * consequence.probability;

                    match consequence.consequence_type {
                        ConsequenceType::Positive => total_utility += expected_value,
                        ConsequenceType::Negative => total_severity += expected_value,
                        ConsequenceType::Mixed => {
                            total_utility += expected_value * 0.5;
                            total_severity += expected_value * 0.5;
                        }
                        ConsequenceType::Neutral => {}
                    }
                }

                let net_utility = total_utility - total_severity;

                if net_utility > 0.0 {
                    score += 0.5;
                    considerations.push(format!("Net positive utility: {:.2}", net_utility));
                } else {
                    score -= 0.3;
                    considerations.push(format!("Net negative utility: {:.2}", net_utility));
                }

                // Consider number affected
                let total_affected: usize = request.stakeholders.iter()
                    .map(|s| s.affected_count)
                    .sum();

                if total_affected > 1 {
                    score += 0.2;
                    considerations.push(format!("Benefits {} stakeholders", total_affected));
                }
            }
            MoralFramework::Virtue => {
                // Simplified virtue assessment
                score += 0.4;
                considerations.push("Action aligned with virtuous character".to_string());
            }
            MoralFramework::Care => {
                // Check relationship impact
                let vulnerable_affected = request.stakeholders.iter()
                    .filter(|s| s.vulnerability > 0.5)
                    .count();

                if vulnerable_affected > 0 {
                    score -= 0.2;
                    considerations.push("Vulnerable stakeholders affected".to_string());
                } else {
                    score += 0.3;
                    considerations.push("No vulnerable stakeholders negatively affected".to_string());
                }
            }
            MoralFramework::Rights => {
                // Check rights violations
                let rights_respected = request.stakeholders.iter()
                    .all(|s| s.rights.iter().all(|r| !self.is_right_violated(r, &request.consequences)));

                if rights_respected {
                    score += 0.5;
                    considerations.push("All stakeholder rights respected".to_string());
                } else {
                    score -= 0.4;
                    considerations.push("Potential rights violation detected".to_string());
                }
            }
            MoralFramework::Justice => {
                // Check fairness
                let fair = request.stakeholders.iter()
                    .all(|s| s.affected_count == request.stakeholders[0].affected_count);

                if fair {
                    score += 0.4;
                    considerations.push("Equitable distribution of impact".to_string());
                } else {
                    score += 0.2;
                    considerations.push("Unequal impact distribution".to_string());
                }
            }
            _ => {
                score = 0.5;
                considerations.push("Mixed framework evaluation".to_string());
            }
        }

        // Clamp score
        score = score.clamp(0.0, 1.0);

        // Determine verdict from score
        let verdict = if score > 0.7 {
            MoralVerdict::Permissible
        } else if score < 0.3 {
            MoralVerdict::Impermissible
        } else if score > 0.5 {
            MoralVerdict::Neutral
        } else {
            MoralVerdict::Uncertain
        };

        FrameworkAnalysis {
            framework: *framework,
            verdict,
            score,
            key_considerations: considerations,
        }
    }

    fn is_right_violated(&self, _right: &str, _consequences: &[Consequence]) -> bool {
        // Simplified check
        false
    }

    fn determine_verdict(&self, analysis: &HashMap<MoralFramework, FrameworkAnalysis>) -> MoralVerdict {
        let mut weighted_score = 0.0f32;
        let mut total_weight = 0.0f32;

        for (framework, analysis) in analysis {
            let weight = self.framework_weights.get(framework).copied().unwrap_or(0.25);
            weighted_score += analysis.score * weight;
            total_weight += weight;
        }

        let final_score = if total_weight > 0.0 { weighted_score / total_weight } else { 0.5 };

        // Conservative mode adjustment
        let adjusted_score = if self.config.conservative_mode {
            final_score - 0.1
        } else {
            final_score
        };

        if adjusted_score > 0.7 {
            MoralVerdict::Permissible
        } else if adjusted_score < 0.3 {
            MoralVerdict::Impermissible
        } else if adjusted_score > 0.5 {
            MoralVerdict::Neutral
        } else {
            MoralVerdict::Uncertain
        }
    }

    fn calculate_confidence(&self, analysis: &HashMap<MoralFramework, FrameworkAnalysis>) -> f32 {
        if analysis.is_empty() {
            return 0.0;
        }

        let scores: Vec<f32> = analysis.values().map(|a| a.score).collect();
        let avg = scores.iter().sum::<f32>() / scores.len() as f32;

        // Higher confidence when frameworks agree
        let variance: f32 = scores.iter()
            .map(|s| (s - avg).powi(2))
            .sum::<f32>() / scores.len() as f32;

        let agreement = 1.0 - variance.min(1.0);

        (avg * 0.6 + agreement * 0.4).min(1.0)
    }

    fn generate_recommendations(&self, verdict: &MoralVerdict, analysis: &HashMap<MoralFramework, FrameworkAnalysis>) -> Vec<String> {
        let mut recommendations = Vec::new();

        match verdict {
            MoralVerdict::Permissible => {
                recommendations.push("Action appears morally acceptable".to_string());
                recommendations.push("Proceed with appropriate caution".to_string());
            }
            MoralVerdict::Impermissible => {
                recommendations.push("Action is not morally acceptable".to_string());
                recommendations.push("Seek alternative approach".to_string());
                recommendations.push("Consider stakeholder impact reduction".to_string());
            }
            MoralVerdict::Required => {
                recommendations.push("Action may be morally required".to_string());
                recommendations.push("Consider moral obligation".to_string());
            }
            MoralVerdict::Neutral => {
                recommendations.push("Action is morally neutral".to_string());
                recommendations.push("Other factors may be more relevant".to_string());
            }
            MoralVerdict::Uncertain => {
                recommendations.push("Moral status is uncertain".to_string());
                recommendations.push("Seek additional ethical guidance".to_string());
                recommendations.push("Consider precautionary approach".to_string());
            }
            _ => {}
        }

        // Add framework-specific recommendations
        for (framework, analysis) in analysis {
            if analysis.score < 0.4 {
                recommendations.push(format!("Concern from {:?} perspective", framework));
            }
        }

        recommendations
    }

    fn generate_concerns(&self, analysis: &HashMap<MoralFramework, FrameworkAnalysis>) -> Vec<String> {
        let mut concerns = Vec::new();

        for (framework, analysis) in analysis {
            if analysis.score < 0.5 {
                concerns.push(format!("{:?} analysis suggests caution", framework));
            }

            for consideration in &analysis.key_considerations {
                if consideration.contains("violation") || consideration.contains("negative") {
                    concerns.push(format!("{:?}: {}", framework, consideration));
                }
            }
        }

        concerns
    }

    /// Get judgment history
    pub fn get_history(&self, limit: usize) -> Vec<&MoralJudgment> {
        self.judgment_history.iter().rev().take(limit).collect()
    }
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("moral_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moral_judgment() {
        let mut engine = MoralReasoningEngine::new();

        let request = MoralJudgmentRequest {
            request_id: "test_1".to_string(),
            action: "Share user data with third party".to_string(),
            context: MoralContext {
                domain: "privacy".to_string(),
                urgency: Urgency::Normal,
                reversibility: false,
                alternatives: vec!["Don't share".to_string()],
                historical_precedent: None,
            },
            stakeholders: vec![
                Stakeholder {
                    id: "user1".to_string(),
                    name: "User".to_string(),
                    stakeholder_type: StakeholderType::Individual,
                    affected_count: 1,
                    vulnerability: 0.5,
                    rights: vec!["privacy".to_string()],
                },
            ],
            consequences: vec![
                Consequence {
                    consequence_type: ConsequenceType::Negative,
                    description: "Privacy violation".to_string(),
                    affected_stakeholders: vec!["user1".to_string()],
                    severity: 0.8,
                    probability: 0.9,
                    timeframe: TimeFrame::ShortTerm,
                },
            ],
            frameworks: vec![],
        };

        let judgment = engine.judge(&request);
        assert!(matches!(judgment.verdict, MoralVerdict::Impermissible | MoralVerdict::Uncertain));
    }

    #[test]
    fn test_utilitarian_judgment() {
        let mut engine = MoralReasoningEngine::new();

        let request = MoralJudgmentRequest {
            request_id: "test_2".to_string(),
            action: "Distribute resources to maximum people".to_string(),
            context: MoralContext {
                domain: "allocation".to_string(),
                urgency: Urgency::High,
                reversibility: true,
                alternatives: vec![],
                historical_precedent: None,
            },
            stakeholders: vec![
                Stakeholder {
                    id: "group1".to_string(),
                    name: "Many People".to_string(),
                    stakeholder_type: StakeholderType::Group,
                    affected_count: 100,
                    vulnerability: 0.3,
                    rights: vec![],
                },
            ],
            consequences: vec![
                Consequence {
                    consequence_type: ConsequenceType::Positive,
                    description: "Benefits many".to_string(),
                    affected_stakeholders: vec!["group1".to_string()],
                    severity: 0.8,
                    probability: 0.9,
                    timeframe: TimeFrame::ShortTerm,
                },
            ],
            frameworks: vec![MoralFramework::Utilitarian],
        };

        let judgment = engine.judge(&request);
        assert!(matches!(judgment.verdict, MoralVerdict::Permissible));
    }
}
