/// Module for Interactive Explanation Query Interface (Rust/WebAssembly).
///
/// This module conceptually outlines a Rust-based service that allows users
/// to query and explore the AGI's reasoning paths, providing interactive
/// explanations for its decisions. While a full WebAssembly frontend is beyond
/// this conceptual implementation, the Rust code demonstrates the backend logic.

use crate::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::io::{self, Write};

/// Reusing Decision and ReasoningStep structures from `reasoning_explainer.rs`.
#[derive(Debug, Clone, PartialEq)]
pub struct Decision {
    pub id: String,
    pub description: String,
    pub outcome: String,
}

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
    pub steps: Vec<ReasoningStep>, // Changed to Vec for simpler indexing
}

impl ReasoningPath {
    pub fn new(decision: Decision, steps: Vec<ReasoningStep>) -> Self {
        ReasoningPath { decision, steps }
    }
}

/// The Explanation Service manages and provides queries for reasoning paths.
pub struct ExplanationService {
    reasoning_paths: HashMap<String, ReasoningPath>,
}

impl ExplanationService {
    pub fn new() -> Self {
        ExplanationService { reasoning_paths: HashMap::new() }
    }

    /// Stores a reasoning path for later querying.
    pub fn store_reasoning_path(&mut self, path: ReasoningPath) {
        self.reasoning_paths.insert(path.decision.id.clone(), path);
        println!("  [ExplanationService]: Stored reasoning path for decision {}.", path.decision.id);
    }

    /// Processes a user query and attempts to provide an explanation.
    pub fn query_explanation(&self, query: &str) -> String {
        println!("  [ExplanationService]: Processing query: \"{}\"", query);

        if query.to_lowercase().contains("why") {
            // Example: "Why was loan_001 approved?"
            if let Some(decision_id) = self.extract_decision_id(query) {
                if let Some(path) = self.reasoning_paths.get(decision_id) {
                    return self.generate_detailed_explanation(path);
                }
                return format!("Could not find reasoning path for decision ID: {}.", decision_id);
            }
            return "Please specify a decision ID in your 'why' query (e.g., \"Why was loan_001 approved?\").".to_string();
        }

        if query.to_lowercase().contains("how") {
            // Example: "How did AGI determine risk for loan_001?"
            if let Some(decision_id) = self.extract_decision_id(query) {
                if let Some(path) = self.reasoning_paths.get(decision_id) {
                    return self.summarize_process(path);
                }
                return format!("Could not find reasoning path for decision ID: {}.", decision_id);
            }
            return "Please specify a decision ID in your 'how' query (e.g., \"How did AGI determine risk for loan_001?\").".to_string();
        }

        "I can explain 'why' a decision was made or 'how' a process was executed. Please rephrase your query.".to_string()
    }

    /// Generates a detailed step-by-step explanation.
    fn generate_detailed_explanation(&self, path: &ReasoningPath) -> String {
        let mut explanation = format!("Decision: \"{}\- ({})\" was made because:\n", path.decision.description, path.decision.outcome, path.decision.id);
        for (i, step) in path.steps.iter().enumerate() {
            explanation.push_str(&format!("  {}. {}\n", i + 1, step));
        }
        explanation.push_str(&format!("\nThis led to the final outcome: {}.", path.decision.outcome));
        explanation
    }

    /// Provides a summary of the execution process.
    fn summarize_process(&self, path: &ReasoningPath) -> String {
        let mut summary = format!("To reach the decision \"{}\- ({})\", the AGI performed the following actions:\n", path.decision.description, path.decision.outcome, path.decision.id);
        for step in path.steps.iter() {
            match step {
                ReasoningStep::DataFetch { source, .. } => summary.push_str(&format!("  - Fetched data from {}\n", source)),
                ReasoningStep::RuleApplication { rule_name, .. } => summary.push_str(&format!("  - Applied rule {}\n", rule_name)),
                ReasoningStep::ModelInference { model_id, .. } => summary.push_str(&format!("  - Ran inference on model {}\n", model_id)),
                _ => summary.push_str(&format!("  - Performed an internal operation.\n")), // Generalize other steps
            }
        }
        summary.push_str(&format!("Ultimately resulting in: {}.", path.decision.outcome));
        summary
    }

    /// Extracts a decision ID from a query string.
    fn extract_decision_id(&self, query: &str) -> Option<&str> {
        // Simple regex-like extraction for conceptual demo
        let parts: Vec<&str> = query.split_whitespace().collect();
        for part in parts {
            if part.contains("_") && part.chars().any(|c| c.is_ascii_digit()) && part.chars().any(|c| c.is_ascii_alphabetic()) {
                // Heuristic: looks like an ID (e.g., loan_001, decision_xyz)
                return Some(part.trim_matches(|c: char| !c.is_alphanumeric() && c != '_'));
            }
        }
        None
    }

    /// Find paths that contain specific keywords or concepts.
    pub fn find_related_paths(&self, keyword: &str) -> Vec<&ReasoningPath> {
        let mut results = Vec::new();
        for path in self.reasoning_paths.values() {
            if path.decision.description.contains(keyword) || path.decision.outcome.contains(keyword) {
                results.push(path);
                continue;
            }
            for step in &path.steps {
                let step_str = format!("{}", step);
                if step_str.contains(keyword) {
                    results.push(path);
                    break;
                }
            }
        }
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mock_path(decision_id: &str) -> ReasoningPath {
        let decision = Decision { id: decision_id.to_string(), description: "Approve loan".to_string(), outcome: "approved".to_string() };
        let steps = vec![
            ReasoningStep::DataFetch { source: "credit_score_db".to_string(), query: "user_id=123".to_string(), result_summary: "Score 750".to_string() },
            ReasoningStep::RuleApplication { rule_name: "HighCreditScoreRule".to_string(), condition_met: "credit_score > 700".to_string(), action_taken: "Flag as low risk".to_string() },
            ReasoningStep::ModelInference { model_id: "risk_model_v2".to_string(), inputs: "score=750".to_string(), output_prediction: "risk=low".to_string(), confidence: 0.98 },
        ];
        ReasoningPath::new(decision, steps)
    }

    #[test]
    fn test_store_and_query_explanation() {
        let mut service = ExplanationService::new();
        let path = create_mock_path("loan_001");
        service.store_reasoning_path(path);

        let explanation = service.query_explanation("Why was loan_001 approved?");
        assert!(explanation.contains("Decision: \"Approve loan\- (approved)" was made because:"));
        assert!(explanation.contains("Fetched data from 'credit_score_db'"));
    }

    #[test]
    fn test_query_summary_process() {
        let mut service = ExplanationService::new();
        let path = create_mock_path("loan_002");
        service.store_reasoning_path(path);

        let summary = service.query_explanation("How did AGI determine risk for loan_002?");
        assert!(summary.contains("To reach the decision \"Approve loan\- (approved)", `"Approve loan"` is decision.description, `"approved"` is decision.outcome, `loan_002` is decision.id));
        assert!(summary.contains("Applied rule HighCreditScoreRule"));
        assert!(summary.contains("Ran inference on model risk_model_v2"));
    }

    #[test]
    fn test_find_related_paths() {
        let mut service = ExplanationService::new();
        service.store_reasoning_path(create_mock_path("loan_003"));
        service.store_reasoning_path(create_mock_path("fraud_001"));

        let related = service.find_related_paths("loan");
        assert_eq!(related.len(), 1);
        assert_eq!(related[0].decision.id, "loan_003");
    }

    #[test]
    fn test_extract_decision_id() {
        let service = ExplanationService::new();
        assert_eq!(service.extract_decision_id("Why was loan_001 approved?"), Some("loan_001"));
        assert_eq!(service.extract_decision_id("Tell me about decision_xyz."), Some("decision_xyz"));
        assert_eq!(service.extract_decision_id("No ID here"), None);
    }
}

fn main() {
    println!("\n--- Interactive Explanation Query Interface Demonstration ---");

    let mut explanation_service = ExplanationService::new();

    // Populate with some simulated reasoning paths
    let decision1 = Decision { id: "loan_001".to_string(), description: "Approve loan application".to_string(), outcome: "approved".to_string() };
    let steps1 = vec![
        ReasoningStep::DataFetch { source: "CreditBureauAPI".to_string(), query: "credit_score".to_string(), result_summary: "Score: 780".to_string() },
        ReasoningStep::ModelInference { model_id: "risk_model_v2".to_string(), inputs: "score=780".to_string(), output_prediction: "low_risk".to_string(), confidence: 0.99 },
        ReasoningStep::RuleApplication { rule_name: "ApproveLowRiskLoans".to_string(), condition_met: "risk == low".to_string(), action_taken: "Proceed with approval".to_string() },
    ];
    explanation_service.store_reasoning_path(ReasoningPath::new(decision1, steps1));

    let decision2 = Decision { id: "fraud_alert_007".to_string(), description: "Flag transaction as suspicious".to_string(), outcome: "flagged".to_string() };
    let steps2 = vec![
        ReasoningStep::DataFetch { source: "TransactionStream".to_string(), query: "transaction_id=XYZ".to_string(), result_summary: "Amount: $10k, Location: Nigeria".to_string() },
        ReasoningStep::ModelInference { model_id: "fraud_detection_v1".to_string(), inputs: "amount=$10k, location=Nigeria".to_string(), output_prediction: "high_fraud_risk".to_string(), confidence: 0.85 },
        ReasoningStep::RuleApplication { rule_name: "FlagHighRiskTransactions".to_string(), condition_met: "fraud_risk > 0.8".to_string(), action_taken: "Flag and alert human".to_string() },
    ];
    explanation_service.store_reasoning_path(ReasoningPath::new(decision2, steps2));

    println!("\nEnter your queries below (e.g., \"Why was loan_001 approved?\", \"How did AGI flag fraud_alert_007?\", \"Find paths for loan\"). Type 'exit' to quit.");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let query = input.trim();

        if query == "exit" {
            break;
        }

        if query.to_lowercase().starts_with("find paths for") {
            let keyword = query.trim_start_matches("find paths for ").trim();
            let results = explanation_service.find_related_paths(keyword);
            if results.is_empty() {
                println!("  No related paths found for '{}'.", keyword);
            } else {
                println!("  Found {} related paths for '{}':", results.len(), keyword);
                for path in results {
                    println!("    - Decision ID: {}, Description: '{}', Outcome: '{}'", path.decision.id, path.decision.description, path.decision.outcome);
                }
            }
        } else {
            let explanation = explanation_service.query_explanation(query);
            println!("\n{}", explanation);
        }
        println!(""); // Newline for readability
    }

    println!("Interactive Explanation Query Interface Demonstration Complete.");
}
