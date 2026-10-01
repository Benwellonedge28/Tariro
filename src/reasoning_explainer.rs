/// Module for Reasoning Path Visualization and Explanation in Rust.
///
/// This module aims to make AGI decisions transparent by capturing its reasoning
/// steps and generating human-understandable explanations and visual representations.

use crate::prelude::*;
use std::collections::VecDeque;
use std::fmt;

/// Represents a distinct point or outcome in the AGI's decision-making process.
#[derive(Debug, Clone, PartialEq)]
pub struct Decision {
    pub id: String,
    pub description: String,
    pub outcome: String,
}

/// Details a single step in the AGI's reasoning process.
#[derive(Debug, Clone, PartialEq)]
pub enum ReasoningStep {
    DataFetch { source: String, query: String, result_summary: String },
    RuleApplication { rule_name: String, condition_met: String, action_taken: String },
    ModelInference { model_id: String, inputs: String, output_prediction: String, confidence: f32 },
    HumanInput { user_id: String, input_value: String },
    InternalComputation { operation: String, result: String },
}

impl fmt::Display for ReasoningStep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReasoningStep::DataFetch { source, query, result_summary } => write!(f, "Fetched data from '{}' with query '{}'. Summary: '{}'.", source, query, result_summary),
            ReasoningStep::RuleApplication { rule_name, condition_met, action_taken } => write!(f, "Applied rule '{}' because '{}'. Action: '{}'.", rule_name, condition_met, action_taken),
            ReasoningStep::ModelInference { model_id, inputs, output_prediction, confidence } => write!(f, "Inferred using model '{}' with inputs '{}'. Prediction: '{}' (Confidence: {:.2}).", model_id, inputs, output_prediction, confidence),
            ReasoningStep::HumanInput { user_id, input_value } => write!(f, "Received human input from '{}': '{}'.", user_id, input_value),
            ReasoningStep::InternalComputation { operation, result } => write!(f, "Performed internal computation '{}'. Result: '{}'.", operation, result),
        }
    }
}

/// Stores a sequence of reasoning steps leading to a specific decision.
pub struct ReasoningPath {
    pub decision: Decision,
    pub steps: VecDeque<ReasoningStep>,
}

impl ReasoningPath {
    pub fn new(decision: Decision) -> Self {
        ReasoningPath { decision, steps: VecDeque::new() }
    }

    pub fn add_step(&mut self, step: ReasoningStep) {
        self.steps.push_back(step);
    }
}

/// The Reasoning Explainer module.
pub struct ReasoningExplainer;

impl ReasoningExplainer {
    pub fn new() -> Self {
        ReasoningExplainer {}
    }

    /// Generates a human-readable explanation for a given reasoning path.
    pub fn generate_explanation(&self, path: &ReasoningPath) -> String {
        let mut explanation = format!("AGI decided \"{}\- ({})": {}\n", path.decision.description, path.decision.outcome, path.decision.id);
        explanation.push_str("Here's how: \n");

        for (i, step) in path.steps.iter().enumerate() {
            explanation.push_str(&format!("  {}. {}\n", i + 1, step));
        }
        explanation
    }

    /// Conceptually generates a graph string (e.g., Mermaid syntax) for visualization.
    pub fn visualize_path(&self, path: &ReasoningPath) -> String {
        let mut mermaid_graph = String::from("graph TD\n");
        let mut previous_node_id = format!("Start\n");

        for (i, step) in path.steps.iter().enumerate() {
            let current_node_id = format!("S{}([{}. {}])", i, i + 1, step.to_string().replace("\"", "\'"));
            mermaid_graph.push_str(&format!("    {} --> {}\n", previous_node_id, current_node_id));
            previous_node_id = current_node_id;
        }

        mermaid_graph.push_str(&format!("    {} --> D((\"{}\"))\n", previous_node_id, path.decision.description.replace("\"", "\'")));
        mermaid_graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reasoning_path_creation() {
        let decision = Decision { id: "D1".to_string(), description: "Approve loan".to_string(), outcome: "approved".to_string() };
        let mut path = ReasoningPath::new(decision);
        path.add_step(ReasoningStep::DataFetch { source: "credit_score_db".to_string(), query: "user_id=123".to_string(), result_summary: "Score 750".to_string() });
        assert_eq!(path.steps.len(), 1);
    }

    #[test]
    fn test_generate_explanation() {
        let decision = Decision { id: "D1".to_string(), description: "Approve loan".to_string(), outcome: "approved".to_string() };
        let mut path = ReasoningPath::new(decision);
        path.add_step(ReasoningStep::DataFetch { source: "credit_score_db".to_string(), query: "user_id=123".to_string(), result_summary: "Score 750".to_string() });
        path.add_step(ReasoningStep::RuleApplication { rule_name: "HighCreditScoreRule".to_string(), condition_met: "credit_score > 700".to_string(), action_taken: "Flag as low risk".to_string() });

        let explainer = ReasoningExplainer::new();
        let explanation = explainer.generate_explanation(&path);
        assert!(explanation.contains("AGI decided \"Approve loan\- (approved)": D1\n"));
        assert!(explanation.contains("1. Fetched data from 'credit_score_db'"));
        assert!(explanation.contains("2. Applied rule 'HighCreditScoreRule'"));
    }

    #[test]
    fn test_visualize_path() {
        let decision = Decision { id: "D1".to_string(), description: "Approve loan".to_string(), outcome: "approved".to_string() };
        let mut path = ReasoningPath::new(decision);
        path.add_step(ReasoningStep::DataFetch { source: "credit_score_db".to_string(), query: "user_id=123".to_string(), result_summary: "Score 750".to_string() });
        path.add_step(ReasoningStep::ModelInference { model_id: "risk_model_v2".to_string(), inputs: "score=750".to_string(), output_prediction: "risk=low".to_string(), confidence: 0.98 });

        let explainer = ReasoningExplainer::new();
        let mermaid = explainer.visualize_path(&path);
        println!("{}", mermaid);
        assert!(mermaid.contains("graph TD"));
        assert!(mermaid.contains("S0([1. Fetched data from 'credit_score_db'"));
        assert!(mermaid.contains("S1([2. Inferred using model 'risk_model_v2'"));
        assert!(mermaid.contains("S1 --> D((\"Approve loan\"))"));
    }
}

fn main() {
    println!("\n--- Reasoning Path Visualization and Explanation Demonstration ---");

    let explainer = ReasoningExplainer::new();

    // Simulate an AGI decision-making process
    let final_decision = Decision {
        id: "LOAN-APPROVE-789".to_string(),
        description: "Approve a loan application".to_string(),
        outcome: "Approved with preferred rates".to_string(),
    };

    let mut loan_approval_path = ReasoningPath::new(final_decision.clone());

    loan_approval_path.add_step(ReasoningStep::DataFetch {
        source: "CreditBureauAPI".to_string(),
        query: "Fetch credit score for applicant X".to_string(),
        result_summary: "Credit score: 780, Payment history: Excellent".to_string(),
    });

    loan_approval_path.add_step(ReasoningStep::DataFetch {
        source: "BankInternalDB".to_string(),
        query: "Retrieve existing account history for applicant X".to_string(),
        result_summary: "Account balance: Consistent, No overdrafts".to_string(),
    });

    loan_approval_path.add_step(ReasoningStep::ModelInference {
        model_id: "LoanEligibilityModel_v3.1".to_string(),
        inputs: "CreditScore=780, AccountHistory=Good, Income=High".to_string(),
        output_prediction: "Eligibility: High, Risk: Low".to_string(),
        confidence: 0.99,
    });

    loan_approval_path.add_step(ReasoningStep::RuleApplication {
        rule_name: "PreferredRateEligibilityRule".to_string(),
        condition_met: "Eligibility is High AND Risk is Low".to_string(),
        action_taken: "Apply preferred interest rate calculation.".to_string(),
    });

    loan_approval_path.add_step(ReasoningStep::InternalComputation {
        operation: "Calculate APR".to_string(),
        result: "APR: 3.5%".to_string(),
    });

    // Generate and print human-readable explanation
    println!("\n--- Human-Readable Explanation ---");
    let explanation = explainer.generate_explanation(&loan_approval_path);
    println!("{}", explanation);

    // Generate and print Mermaid graph for visualization
    println!("\n--- Mermaid Graph for Visualization (can be rendered by Mermaid.js) ---");
    let mermaid_graph = explainer.visualize_path(&loan_approval_path);
    println!("{}", mermaid_graph);

    println!("\nReasoning Path Visualization and Explanation Demonstration Complete.");
}
