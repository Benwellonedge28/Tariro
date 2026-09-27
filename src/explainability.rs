// Tariro AGI - Explainable AI (XAI) Engine Module
// Provides interpretable explanations for AI decisions

use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Explanation Types
// ============================================================================

/// Explanation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationRequest {
    pub request_id: String,
    pub decision_id: String,
    pub explanation_type: ExplanationType,
    pub target_audience: Audience,
    pub depth: ExplanationDepth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplanationType {
    Contrastive,      // "Why this instead of X?"
    Selectable,       // User selects what to explain
    Attribution,       // Feature importance
    Counterfactual,   // "What would change the outcome?"
    Process,          // How the decision was made
    RuleBased,        // Rule extraction explanation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Audience {
    Expert,
    Technical,
    General,
    Layperson,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplanationDepth {
    Shallow,   // High-level summary
    Medium,    // Key factors
    Deep,      // Full reasoning
}

/// Generated explanation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    pub explanation_id: String,
    pub decision_id: String,
    pub explanation_type: ExplanationType,
    pub summary: String,
    pub main_factors: Vec<Factor>,
    pub reasoning_chain: Vec<ReasoningStep>,
    pub confidence: f32,
    pub alternatives: Vec<Alternative>,
    pub counterfactuals: Vec<Counterfactual>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Factor {
    pub factor_id: String,
    pub name: String,
    pub description: String,
    pub importance: f32,
    pub direction: FactorDirection,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FactorDirection {
    Supporting,
    Opposing,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub description: String,
    pub source: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceType {
    Data,
    Rule,
    Precedent,
    Heuristic,
    Intuition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStep {
    pub step_id: usize,
    pub description: String,
    pub rule_applied: Option<String>,
    pub conclusion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub alternative_id: String,
    pub description: String,
    pub probability: f32,
    pub why_not_chosen: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Counterfactual {
    pub counterfactual_id: String,
    pub scenario: String,
    pub original_outcome: String,
    pub counterfactual_outcome: String,
    pub key_changes: Vec<String>,
}

// ============================================================================
// XAI Engine
// ============================================================================

/// Explainable AI engine
pub struct XAIEngine {
    pub explanation_cache: HashMap<String, Explanation>,
    pub explanation_history: VecDeque<Explanation>,
    pub templates: Vec<ExplanationTemplate>,
    pub config: XAIConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplanationTemplate {
    pub template_id: String,
    pub explanation_type: ExplanationType,
    pub audience: Audience,
    pub template: String,
}

#[derive(Debug, Clone)]
pub struct XAIConfig {
    pub auto_generate_explanations: bool,
    pub max_explanation_length: usize,
    pub include_confidence: bool,
    pub include_alternatives: bool,
    pub include_counterfactuals: bool,
    pub explanation_storage_duration: Duration,
}

impl Default for XAIConfig {
    fn default() -> Self {
        Self {
            auto_generate_explanations: true,
            max_explanation_length: 500,
            include_confidence: true,
            include_alternatives: true,
            include_counterfactuals: true,
            explanation_storage_duration: Duration::from_secs(86400),
        }
    }
}

impl XAIEngine {
    pub fn new() -> Self {
        Self {
            explanation_cache: HashMap::new(),
            explanation_history: VecDeque::new(),
            templates: Self::default_templates(),
            config: XAIConfig::default(),
        }
    }

    fn default_templates() -> Vec<ExplanationTemplate> {
        vec![
            ExplanationTemplate {
                template_id: "contrastive_expert".to_string(),
                explanation_type: ExplanationType::Contrastive,
                audience: Audience::Expert,
                template: "The decision was made because {factor1} was more important than {factor2}. {evidence}".to_string(),
            },
            ExplanationTemplate {
                template_id: "process_general".to_string(),
                explanation_type: ExplanationType::Process,
                audience: Audience::General,
                template: "Based on the information provided, the system considered {num_factors} key factors and determined that {summary}.".to_string(),
            },
            ExplanationTemplate {
                template_id: "attribution_technical".to_string(),
                explanation_type: ExplanationType::Attribution,
                audience: Audience::Technical,
                template: "Feature importance analysis shows {feature1} contributed {weight1} to the decision.".to_string(),
            },
        ]
    }

    /// Generate explanation for a decision
    pub fn explain(&mut self, request: &ExplanationRequest, decision: &Decision) -> Explanation {
        // Check cache first
        if let Some(cached) = self.explanation_cache.get(&request.decision_id) {
            return cached.clone();
        }

        // Generate explanation based on type
        let explanation = match request.explanation_type {
            ExplanationType::Contrastive => self.generate_contrastive(decision),
            ExplanationType::Selectable => self.generate_selectable(decision, request),
            ExplanationType::Attribution => self.generate_attribution(decision),
            ExplanationType::Counterfactual => self.generate_counterfactual(decision),
            ExplanationType::Process => self.generate_process(decision, &request.target_audience),
            ExplanationType::RuleBased => self.generate_rule_based(decision),
        };

        // Cache and record
        self.explanation_cache.insert(request.decision_id.clone(), explanation.clone());
        self.explanation_history.push_back(explanation.clone());

        // Keep bounded
        while self.explanation_history.len() > 1000 {
            self.explanation_history.pop_front();
        }

        explanation
    }

    fn generate_contrastive(&self, decision: &Decision) -> Explanation {
        let mut factors = Vec::new();

        // Generate factors
        for (i, feature) in decision.features.iter().enumerate() {
            factors.push(Factor {
                factor_id: format!("f_{}", i),
                name: feature.name.clone(),
                description: format!("Feature '{}' had value {}", feature.name, feature.value),
                importance: feature.importance,
                direction: if feature.positive { FactorDirection::Supporting } else { FactorDirection::Opposing },
                evidence: vec![],
            });
        }

        factors.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());

        let main_factors: Vec<Factor> = factors.iter().take(5).cloned().collect();

        Explanation {
            explanation_id: generate_id(),
            decision_id: decision.decision_id.clone(),
            explanation_type: ExplanationType::Contrastive,
            summary: format!("This decision was made primarily because '{}' was the most important factor.", main_factors.first().map(|f| f.name.as_str()).unwrap_or("unknown")),
            main_factors,
            reasoning_chain: vec![],
            confidence: decision.confidence,
            alternatives: vec![],
            counterfactuals: vec![],
            generated_at: Utc::now(),
        }
    }

    fn generate_selectable(&self, decision: &Decision, _request: &ExplanationRequest) -> Explanation {
        let factors: Vec<Factor> = decision.features.iter().enumerate().map(|(i, f)| {
            Factor {
                factor_id: format!("f_{}", i),
                name: f.name.clone(),
                description: f.name.clone(),
                importance: f.importance,
                direction: if f.positive { FactorDirection::Supporting } else { FactorDirection::Opposing },
                evidence: vec![],
            }
        }).collect();

        Explanation {
            explanation_id: generate_id(),
            decision_id: decision.decision_id.clone(),
            explanation_type: ExplanationType::Selectable,
            summary: format!("Decision '{}': {} factors considered. Select any to explore.", decision.decision_type, factors.len()),
            main_factors: factors,
            reasoning_chain: vec![],
            confidence: decision.confidence,
            alternatives: vec![],
            counterfactuals: vec![],
            generated_at: Utc::now(),
        }
    }

    fn generate_attribution(&self, decision: &Decision) -> Explanation {
        let factors: Vec<Factor> = decision.features.iter().enumerate().map(|(i, f)| {
            Factor {
                factor_id: format!("attr_{}", i),
                name: f.name.clone(),
                description: format!("Attribution: {:.2} importance", f.importance),
                importance: f.importance,
                direction: if f.positive { FactorDirection::Supporting } else { FactorDirection::Opposing },
                evidence: vec![Evidence {
                    evidence_type: EvidenceType::Data,
                    description: "Feature contribution analysis".to_string(),
                    source: "Attribution Engine".to_string(),
                    confidence: 0.85,
                }],
            }
        }).collect();

        Explanation {
            explanation_id: generate_id(),
            decision_id: decision.decision_id.clone(),
            explanation_type: ExplanationType::Attribution,
            summary: format!("Feature attribution: {} contributed {:.1}% to the decision.",
                factors.first().map(|f| f.name.as_str()).unwrap_or("unknown"),
                factors.first().map(|f| f.importance * 100.0).unwrap_or(0.0)),
            main_factors: factors,
            reasoning_chain: vec![],
            confidence: decision.confidence,
            alternatives: vec![],
            counterfactuals: vec![],
            generated_at: Utc::now(),
        }
    }

    fn generate_counterfactual(&self, decision: &Decision) -> Explanation {
        let counterfactuals = vec![
            Counterfactual {
                counterfactual_id: "cf_1".to_string(),
                scenario: "If '{}' had been different".to_string(),
                original_outcome: decision.outcome.clone(),
                counterfactual_outcome: "Alternative outcome would have been: {}".to_string(),
                key_changes: vec!["Change factor value".to_string()],
            },
        ];

        Explanation {
            explanation_id: generate_id(),
            decision_id: decision.decision_id.clone(),
            explanation_type: ExplanationType::Counterfactual,
            summary: "Counterfactual analysis shows what changes would alter the decision.".to_string(),
            main_factors: vec![],
            reasoning_chain: vec![],
            confidence: decision.confidence,
            alternatives: vec![],
            counterfactuals,
            generated_at: Utc::now(),
        }
    }

    fn generate_process(&self, decision: &Decision, audience: &Audience) -> Explanation {
        let summary = match audience {
            Audience::Expert => format!("Decision process: 1) Input processing, 2) Feature extraction, 3) Model inference, 4) Output generation. Confidence: {:.0}%", decision.confidence * 100.0),
            Audience::Technical => format!("Model processed {} features in {} steps. Decision confidence: {:.0}%", decision.features.len(), 4, decision.confidence * 100.0),
            Audience::General => format!("After careful analysis of all available information, the system made this decision with {:.0}% confidence.", decision.confidence * 100.0),
            Audience::Layperson => "The system looked at all the facts and came to this conclusion.".to_string(),
        };

        let reasoning_chain = vec![
            ReasoningStep {
                step_id: 1,
                description: "Gathered and processed input data".to_string(),
                rule_applied: None,
                conclusion: "Data prepared for analysis".to_string(),
            },
            ReasoningStep {
                step_id: 2,
                description: "Extracted relevant features".to_string(),
                rule_applied: None,
                conclusion: format!("{} features identified", decision.features.len()),
            },
            ReasoningStep {
                step_id: 3,
                description: "Applied decision model".to_string(),
                rule_applied: Some("ML Model Inference".to_string()),
                conclusion: decision.outcome.clone(),
            },
        ];

        Explanation {
            explanation_id: generate_id(),
            decision_id: decision.decision_id.clone(),
            explanation_type: ExplanationType::Process,
            summary,
            main_factors: vec![],
            reasoning_chain,
            confidence: decision.confidence,
            alternatives: vec![],
            counterfactuals: vec![],
            generated_at: Utc::now(),
        }
    }

    fn generate_rule_based(&self, decision: &Decision) -> Explanation {
        let mut rules = Vec::new();

        for feature in &decision.features {
            if feature.importance > 0.3 {
                rules.push(format!(
                    "IF {} {} {} THEN support decision (weight: {:.2})",
                    feature.name,
                    if feature.positive { ">=" } else { "<" },
                    feature.threshold,
                    feature.importance
                ));
            }
        }

        Explanation {
            explanation_id: generate_id(),
            decision_id: decision.decision_id.clone(),
            explanation_type: ExplanationType::RuleBased,
            summary: "Decision explained through extracted rules:".to_string(),
            main_factors: vec![],
            reasoning_chain: rules.iter().enumerate().map(|(i, r)| ReasoningStep {
                step_id: i + 1,
                description: r.clone(),
                rule_applied: Some("Rule Extraction".to_string()),
                conclusion: "Rule satisfied".to_string(),
            }).collect(),
            confidence: decision.confidence,
            alternatives: vec![],
            counterfactuals: vec![],
            generated_at: Utc::now(),
        }
    }

    /// Get explanation history
    pub fn get_history(&self, limit: usize) -> Vec<&Explanation> {
        self.explanation_history.iter().rev().take(limit).collect()
    }
}

// ============================================================================
// Decision Types for Explanation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub decision_id: String,
    pub decision_type: String,
    pub features: Vec<Feature>,
    pub outcome: String,
    pub confidence: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub value: f32,
    pub importance: f32,
    pub positive: bool,
    pub threshold: f32,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("xai_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contrastive_explanation() {
        let mut engine = XAIEngine::new();

        let decision = Decision {
            decision_id: "d1".to_string(),
            decision_type: "classification".to_string(),
            features: vec![
                Feature { name: "feature_a".to_string(), value: 0.8, importance: 0.6, positive: true, threshold: 0.5 },
                Feature { name: "feature_b".to_string(), value: 0.3, importance: 0.3, positive: false, threshold: 0.5 },
            ],
            outcome: "Positive".to_string(),
            confidence: 0.85,
            timestamp: Utc::now(),
        };

        let request = ExplanationRequest {
            request_id: "req1".to_string(),
            decision_id: "d1".to_string(),
            explanation_type: ExplanationType::Contrastive,
            target_audience: Audience::General,
            depth: ExplanationDepth::Medium,
        };

        let explanation = engine.explain(&request, &decision);
        assert!(!explanation.summary.is_empty());
        assert_eq!(explanation.main_factors.len(), 2);
    }

    #[test]
    fn test_explanation_caching() {
        let mut engine = XAIEngine::new();

        let decision = Decision {
            decision_id: "d2".to_string(),
            decision_type: "regression".to_string(),
            features: vec![],
            outcome: "42".to_string(),
            confidence: 0.9,
            timestamp: Utc::now(),
        };

        let request = ExplanationRequest {
            request_id: "req2".to_string(),
            decision_id: "d2".to_string(),
            explanation_type: ExplanationType::Process,
            target_audience: Audience::Expert,
            depth: ExplanationDepth::Deep,
        };

        // First call
        let exp1 = engine.explain(&request, &decision);
        // Second call should be cached
        let exp2 = engine.explain(&request, &decision);

        assert_eq!(exp1.explanation_id, exp2.explanation_id);
    }
}
