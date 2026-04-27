/// Module for Context-Aware Reasoning & Adaptive Decision-Making in Rust.
///
/// This module conceptually outlines a system where the AGI leverages fused
/// multi-modal information to perform advanced inference, evaluate various
/// action alternatives, and make optimal decisions. These decisions adapt
/// dynamically to changing contexts and long-term goals, moving beyond simple
/// rule-based systems.

use std::collections::HashMap;
use std::fmt;
use std::thread;
use std::time::Duration;
use rand::Rng;

/// Represents the current operational context, including environmental and internal states.
#[derive(Debug, Clone)]
pub struct Context {
    pub id: String,
    pub environment_state: HashMap<String, String>, // e.g., "temperature": "25C", "network_status": "normal"
    pub internal_state: HashMap<String, String>,    // e.g., "cpu_load": "50%", "memory_usage": "60%"
    pub active_threats: Vec<String>,
    pub user_sentiment: String,
}

/// Defines an objective or desired state for the AGI.
#[derive(Debug, Clone, PartialEq)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub target_state: HashMap<String, String>, // e.g., "system_uptime": "99.9%", "cost_optimization": "active"
    pub priority: u8,
}

/// Represents a possible action the AGI can take.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    ScaleService { service_id: String, instances: u32 },
    AdjustPolicy { policy_id: String, parameter: String, value: String },
    SendAlert { recipient: String, message: String },
    OptimizeResource { resource_type: String, strategy: String },
    CollectMoreData { data_type: String, source: String },
    NoAction,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The chosen action, its rationale, and predicted outcome.
#[derive(Debug, Clone)]
pub struct Decision {
    pub chosen_action: Action,
    pub rationale: String,
    pub predicted_outcome: String,
    pub confidence: f32,
}

/// The Core Reasoning Engine for Context-Aware Decision-Making.
pub struct ReasoningEngine {
    pub known_goals: HashMap<String, Goal>,
    pub inference_models: HashMap<String, String>, // Conceptual: Model names -> paths/IDs
}

impl ReasoningEngine {
    pub fn new() -> Self {
        let mut known_goals = HashMap::new();
        known_goals.insert("g1".to_string(), Goal {
            id: "g1".to_string(),
            description: "Maintain system stability and uptime.".to_string(),
            target_state: HashMap::from([
                ("system_uptime".to_string(), "99.9%+".to_string()),
                ("error_rate".to_string(), "<0.1%".to_string()),
            ]),
            priority: 10,
        });
        known_goals.insert("g2".to_string(), Goal {
            id: "g2".to_string(),
            description: "Optimize operational costs.".to_string(),
            target_state: HashMap::from([
                ("resource_cost".to_string(), "reduced".to_string()),
                ("cpu_utilization".to_string(), "optimal".to_string()),
            ]),
            priority: 7,
        });

        ReasoningEngine {
            known_goals,
            inference_models: HashMap::from([
                ("anomaly_detection_model".to_string(), "/models/ad_v2.ml".to_string()),
                ("risk_assessment_model".to_string(), "/models/risk_v1.ml".to_string()),
            ]),
        }
    }

    /// Analyzes the current context to extract key features and potential issues.
    pub fn analyze_context(&self, context: &Context) -> HashMap<String, String> {
        println!("  [ReasoningEngine]: Analyzing context ID: {}.", context.id);
        let mut features = HashMap::new();
        features.insert("cpu_status".to_string(), context.internal_state.get("cpu_load").cloned().unwrap_or("unknown".to_string()));
        features.insert("network_health".to_string(), context.environment_state.get("network_status").cloned().unwrap_or("unknown".to_string()));

        if !context.active_threats.is_empty() {
            features.insert("security_threat".to_string(), context.active_threats.join(", "));
        }
        if context.user_sentiment == "negative" {
            features.insert("user_dissatisfaction".to_string(), "true".to_string());
        }
        thread::sleep(Duration::from_millis(50));
        features
    }

    /// Infers implications and potential future states based on context and models.
    pub fn infer_implications(&self, features: &HashMap<String, String>) -> Vec<String> {
        println!("  [ReasoningEngine]: Inferring implications...");
        let mut implications = Vec::new();

        if features.get("cpu_status").map_or(false, |s| s.contains("90%")) {
            implications.push("Potential system overload imminent.".to_string());
            implications.push("Performance degradation likely.".to_string());
        }
        if features.get("security_threat").is_some() {
            implications.push("Data breach risk elevated.".to_string());
            implications.push("Compliance violation possible.".to_string());
        }
        if features.get("user_dissatisfaction").is_some() {
            implications.push("User churn risk.".to_string());
        }
        thread::sleep(Duration::from_millis(70));
        implications
    }

    /// Evaluates possible actions against current goals and predicted outcomes.
    pub fn evaluate_actions(&self, context: &Context, current_goals: &[&Goal], implications: &[String]) -> HashMap<Action, f32> {
        println!("  [ReasoningEngine]: Evaluating possible actions against goals...");
        let mut action_scores: HashMap<Action, f32> = HashMap::new();

        let possible_actions = vec![
            Action::ScaleService { service_id: "web_app".to_string(), instances: 2 },
            Action::AdjustPolicy { policy_id: "cpu_throttle".to_string(), parameter: "limit".to_string(), value: "85".to_string() },
            Action::SendAlert { recipient: "ops_team".to_string(), message: "High CPU usage predicted".to_string() },
            Action::OptimizeResource { resource_type: "database".to_string(), strategy: "shard".to_string() },
            Action::NoAction,
        ];

        for action in possible_actions {
            let mut score = 0.5; // Base score

            // Heuristic scoring based on implications and goals
            for implication in implications {
                if implication.contains("overload") {
                    if matches!(action, Action::ScaleService { .. } | Action::OptimizeResource { .. }) { score += 0.3; }
                }
                if implication.contains("threat") {
                    if matches!(action, Action::AdjustPolicy { .. } | Action::SendAlert { .. }) { score += 0.4; }
                }
            }

            for goal in current_goals {
                if goal.id == "g1" { // Maintain system stability
                    if matches!(action, Action::ScaleService { .. } | Action::AdjustPolicy { .. }) { score += 0.2; }
                } else if goal.id == "g2" { // Optimize costs
                    if matches!(action, Action::OptimizeResource { .. }) { score += 0.2; }
                }
            }
            action_scores.insert(action, score);
        }
        thread::sleep(Duration::from_millis(100));
        action_scores
    }

    /// Makes the optimal decision based on evaluated actions.
    pub fn make_decision(&self, action_scores: HashMap<Action, f32>, rng: &mut impl Rng) -> Decision {
        println!("  [ReasoningEngine]: Making decision...");
        let (best_action, best_score) = action_scores.into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((Action::NoAction, 0.0));

        let rationale = format!("Chosen {} with score {:.2} based on contextual analysis and goal alignment.", best_action, best_score);
        let predicted_outcome = format!("Expected to improve system state and move towards goals. (Confidence: {:.2})", best_score);

        Decision {
            chosen_action: best_action,
            rationale,
            predicted_outcome,
            confidence: best_score,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::mock::StepRng;

    #[test]
    fn test_analyze_context() {
        let engine = ReasoningEngine::new();
        let context = Context {
            id: "c1".to_string(),
            environment_state: HashMap::from([("temperature".to_string(), "30C".to_string())]),
            internal_state: HashMap::from([("cpu_load".to_string(), "85%".to_string())]),
            active_threats: vec!["phishing".to_string()],
            user_sentiment: "neutral".to_string(),
        };
        let features = engine.analyze_context(&context);
        assert!(features.contains_key("cpu_status"));
        assert!(features.contains_key("security_threat"));
    }

    #[test]
    fn test_infer_implications() {
        let engine = ReasoningEngine::new();
        let mut features = HashMap::new();
        features.insert("cpu_status".to_string(), "95%".to_string());
        let implications = engine.infer_implications(&features);
        assert!(!implications.is_empty());
        assert!(implications.iter().any(|i| i.contains("overload")));
    }

    #[test]
    fn test_make_decision() {
        let engine = ReasoningEngine::new();
        let mut action_scores = HashMap::new();
        action_scores.insert(Action::ScaleService { service_id: "test".to_string(), instances: 2 }, 0.8);
        action_scores.insert(Action::NoAction, 0.2);
        let mut rng = StepRng::new(1, 1);
        let decision = engine.make_decision(action_scores, &mut rng);
        assert!(matches!(decision.chosen_action, Action::ScaleService { .. }));
        assert!(decision.confidence > 0.7);
    }
}

fn main() {
    println!("\n--- Context-Aware Reasoning & Adaptive Decision-Making Demonstration ---");

    let mut reasoning_engine = ReasoningEngine::new();
    let mut rng = rand::thread_rng();

    // Define current goals
    let primary_goal = reasoning_engine.known_goals.get("g1").unwrap(); // System stability
    let secondary_goal = reasoning_engine.known_goals.get("g2").unwrap(); // Cost optimization
    let current_goals = vec![primary_goal, secondary_goal];

    // Scenario 1: High CPU load, no immediate threats
    println!("\n--- Scenario 1: High CPU Load ---");
    let context1 = Context {
        id: "ctx_001".to_string(),
        environment_state: HashMap::from([("network_status".to_string(), "normal".to_string())]),
        internal_state: HashMap::from([("cpu_load".to_string(), "92%".to_string()), ("memory_usage".to_string(), "70%".to_string())]),
        active_threats: vec![],
        user_sentiment: "neutral".to_string(),
    };
    let features1 = reasoning_engine.analyze_context(&context1);
    let implications1 = reasoning_engine.infer_implications(&features1);
    let action_scores1 = reasoning_engine.evaluate_actions(&context1, &current_goals, &implications1);
    let decision1 = reasoning_engine.make_decision(action_scores1, &mut rng);
    println!("  Decision 1: {:?}\n", decision1);

    // Scenario 2: Moderate CPU, active security threat, negative user sentiment
    println!("\n--- Scenario 2: Threat & User Dissatisfaction ---");
    let context2 = Context {
        id: "ctx_002".to_string(),
        environment_state: HashMap::from([("network_status".to_string(), "degraded".to_string())]),
        internal_state: HashMap::from([("cpu_load".to_string(), "60%".to_string()), ("memory_usage".to_string(), "80%".to_string())]),
        active_threats: vec!["SQL_Injection_Attempt".to_string(), "DDoS_Attack".to_string()],
        user_sentiment: "negative".to_string(),
    };
    let features2 = reasoning_engine.analyze_context(&context2);
    let implications2 = reasoning_engine.infer_implications(&features2);
    let action_scores2 = reasoning_engine.evaluate_actions(&context2, &current_goals, &implications2);
    let decision2 = reasoning_engine.make_decision(action_scores2, &mut rng);
    println!("  Decision 2: {:?}\n", decision2);

    // Scenario 3: Low activity, focus on cost optimization
    println!("\n--- Scenario 3: Cost Optimization ---");
    let context3 = Context {
        id: "ctx_003".to_string(),
        environment_state: HashMap::from([("network_status".to_string(), "optimal".to_string())]),
        internal_state: HashMap::from([("cpu_load".to_string(), "20%".to_string()), ("memory_usage".to_string(), "40%".to_string())]),
        active_threats: vec![],
        user_sentiment: "positive".to_string(),
    };
    let features3 = reasoning_engine.analyze_context(&context3);
    let implications3 = reasoning_engine.infer_implications(&features3);
    let action_scores3 = reasoning_engine.evaluate_actions(&context3, &current_goals, &implications3);
    let decision3 = reasoning_engine.make_decision(action_scores3, &mut rng);
    println!("  Decision 3: {:?}\n", decision3);

    println!("Context-Aware Reasoning & Adaptive Decision-Making Demonstration Complete.");
}
