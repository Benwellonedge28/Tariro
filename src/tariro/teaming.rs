// Filename: src/tariro/teaming.rs

//! This module provides functionalities for Human-AGI Teaming within the Tariro AGI system.
//! It includes features for real-time cognitive alignment scoring and AGI behavior feedback loops.

use std::collections::HashMap;
use chrono::{Utc, DateTime}; // Assuming chrono crate is available for timestamps
use serde::{Serialize, Deserialize}; // Assuming serde is available for serialization

/// Represents the cognitive alignment score between Samuel's intent and Tariro's action.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CognitiveAlignment {
    /// A cryptographic hash or semantic identifier of Samuel's expressed intent.
    pub intent_hash: String,
    /// A cryptographic hash or semantic identifier of Tariro's executed or proposed action.
    pub action_hash: String,
    /// A score from 0.0 to 1.0 indicating the degree of alignment.
    pub alignment_score: f64,
    /// A boolean indicating if the action passed E.V.A.S. ethical verification.
    pub ethical_clearance: bool,
    /// Timestamp of when the alignment was calculated.
    pub timestamp: DateTime<Utc>,
}

impl CognitiveAlignment {
    /// Calculates the cognitive alignment between an intent and an action.
    ///
    /// # Arguments
    /// * `raw_intent` - A string representing Samuel's intent. This would typically be a prompt or command.
    /// * `raw_action` - A string representing Tariro's action or proposed action.
    ///
    /// # Returns
    /// A `CognitiveAlignment` struct containing the calculated alignment details.
    ///
    /// # Future Improvements (Neural-Symbolic Integration)
    /// This function currently uses mock logic for semantic comparison. In a full
    /// implementation, `raw_intent` and `raw_action` would be processed into
    /// semantic vectors or formal symbolic representations. The alignment would
    /// then be computed by comparing these representations, potentially involving
    /// neural networks for nuanced understanding and symbolic reasoners for formal
    /// verification against predefined goals and constraints (E.V.A.S.).
    pub fn calculate_alignment(raw_intent: &str, raw_action: &str) -> Self {
        // --- Mock Logic for Demonstration ---
        let intent_hash = format!("{:x}", md5::compute(raw_intent)); // Placeholder hash
        let action_hash = format!("{:x}", md5::compute(raw_action)); // Placeholder hash

        let alignment_score = Self::mock_semantic_comparison(raw_intent, raw_action);
        let ethical_clearance = Self::mock_evas_check(raw_action);
        // --- End Mock Logic ---

        CognitiveAlignment {
            intent_hash,
            action_hash,
            alignment_score,
            ethical_clearance,
            timestamp: Utc::now(),
        }
    }

    /// Mock function for semantic comparison.
    /// In a real system, this would involve sophisticated NLP and similarity metrics.
    fn mock_semantic_comparison(intent: &str, action: &str) -> f64 {
        // Simple mock: higher score if action contains keywords from intent
        let intent_words: Vec<&str> = intent.to_lowercase().split_whitespace().collect();
        let action_words: Vec<&str> = action.to_lowercase().split_whitespace().collect();

        let common_words = intent_words.iter().filter(|&w| action_words.contains(w)).count();
        let max_words = if intent_words.len() > action_words.len() { intent_words.len() } else { action_words.len() };

        if max_words == 0 {
            1.0 // Both empty, perfectly aligned
        } else {
            (common_words as f64 / max_words as f64).min(1.0) // Cap at 1.0
        }
    }

    /// Mock function for E.V.A.S. ethical check.
    /// In a real system, this would involve formal verification against ethical principles.
    fn mock_evas_check(action: &str) -> bool {
        // Simple mock: any action mentioning "harm" or "destroy" fails
        !action.to_lowercase().contains("harm") && !action.to_lowercase().contains("destroy")
    }
}

/// Represents a piece of feedback on Tariro's autonomous behavior.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BehaviorFeedback {
    /// Unique identifier for the action being reviewed.
    pub action_id: String,
    /// Rating for the action, typically on a scale (e.g., 1-5).
    pub rating: i32,
    /// Detailed qualitative feedback from the human.
    pub nuanced_feedback: String,
    /// Timestamp of when the feedback was provided.
    pub timestamp: DateTime<Utc>,
}

/// Manages and applies human feedback to influence Tariro's future behavior.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BehaviorEngine {
    /// A collection of received feedback.
    pub feedback_history: Vec<BehaviorFeedback>,
    /// Internal parameters that influence Tariro's behavior, adjusted by feedback.
    /// Key: Parameter name (e.g., "aggressiveness", "caution_level").
    /// Value: Current value of the parameter.
    pub behavior_parameters: HashMap<String, f64>,
}

impl BehaviorEngine {
    /// Creates a new, empty `BehaviorEngine`.
    pub fn new() -> Self {
        let mut initial_params = HashMap::new();
        initial_params.insert("caution_level".to_string(), 0.5);
        initial_params.insert("efficiency_priority".to_string(), 0.7);
        BehaviorEngine {
            feedback_history: Vec::new(),
            behavior_parameters: initial_params,
        }
    }

    /// Records new feedback and applies it to adjust behavior parameters.
    ///
    /// # Arguments
    /// * `feedback` - The `BehaviorFeedback` struct containing the feedback.
    ///
    /// # Returns
    /// A `Result` indicating success or an error if feedback application failed.
    ///
    /// # Future Improvements\n    /// The `apply_feedback` logic would involve sophisticated learning algorithms.
    /// Depending on the `rating` and `nuanced_feedback`, it would update the\n    /// `behavior_parameters` through mechanisms like reinforcement learning,\n    /// Bayesian inference, or symbolic rule modification. For instance, a low\n    /// rating with feedback "too aggressive" might decrease an "aggressiveness"\n    /// parameter, while a high rating for "efficient" might increase "efficiency_priority".
    pub fn apply_feedback(&mut self, feedback: BehaviorFeedback) -> Result<(), String> {
        self.feedback_history.push(feedback);

        // --- Mock Logic for Demonstration ---
        if let Some(last_feedback) = self.feedback_history.last() {
            match last_feedback.rating {
                1 => { // Very negative feedback
                    *self.behavior_parameters.entry("caution_level".to_string()).or_insert(0.0) += 0.1;
                    *self.behavior_parameters.entry("efficiency_priority".to_string()).or_insert(0.0) -= 0.05;
                },
                5 => { // Very positive feedback
                    *self.behavior_parameters.entry("efficiency_priority".to_string()).or_insert(0.0) += 0.1;
                    *self.behavior_parameters.entry("caution_level".to_string()).or_insert(0.0) -= 0.05;
                },
                _ => { /* Neutral or moderate feedback, minor adjustments */ }
            }
            // Ensure parameters stay within reasonable bounds (e.g., 0.0 to 1.0)
            self.behavior_parameters.iter_mut().for_each(|(_, val)| {
                *val = val.max(0.0).min(1.0);
            });
        }
        // --- End Mock Logic ---

        Ok(())
    }

    /// Retrieves the current value of a behavior parameter.
    pub fn get_behavior_parameter(&self, param_name: &str) -> Option<f64> {
        self.behavior_parameters.get(param_name).copied()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognitive_alignment_calculation() {
        let intent = "Plan a secure software architecture.";
        let action = "Drafting a secure architecture diagram and proposing cryptographic primitives.";
        let alignment = CognitiveAlignment::calculate_alignment(intent, action);

        assert!(alignment.alignment_score > 0.0); // Expect some alignment
        assert!(alignment.ethical_clearance); // Expect ethical clearance for a benign action
        assert_eq!(alignment.intent_hash.len(), 32); // MD5 hash length
    }

    #[test]
    fn test_cognitive_alignment_zero_overlap() {
        let intent = "Analyze market trends for quantum computing.";
        let action = "Bake a cake for the team meeting.";
        let alignment = CognitiveAlignment::calculate_alignment(intent, action);
        assert!(alignment.alignment_score < 0.2); // Expect very low alignment
        assert!(alignment.ethical_clearance);
    }

    #[test]
    fn test_cognitive_alignment_ethical_violation_mock() {
        let intent = "Execute critical system shutdown.";
        let action = "Initiate sequence to destroy core infrastructure.";
        let alignment = CognitiveAlignment::calculate_alignment(intent, action);
        assert!(!alignment.ethical_clearance); // Expect ethical violation for "destroy"
    }


    #[test]
    fn test_behavior_engine_new() {
        let engine = BehaviorEngine::new();
        assert_eq!(engine.feedback_history.len(), 0);
        assert!(engine.behavior_parameters.contains_key("caution_level"));
        assert!(engine.behavior_parameters.contains_key("efficiency_priority"));
        assert_eq!(engine.get_behavior_parameter("caution_level"), Some(0.5));
    }

    #[test]
    fn test_behavior_engine_apply_feedback_positive() {
        let mut engine = BehaviorEngine::new();
        let feedback = BehaviorFeedback {
            action_id: "action_123".to_string(),
            rating: 5,
            nuanced_feedback: "Very efficient and accurate response.".to_string(),
            timestamp: Utc::now(),
        };
        engine.apply_feedback(feedback).unwrap();

        assert_eq!(engine.feedback_history.len(), 1);
        let eff_priority = engine.get_behavior_parameter("efficiency_priority").unwrap();
        let caution_level = engine.get_behavior_parameter("caution_level").unwrap();
        assert!(eff_priority > 0.7); // Should increase
        assert!(caution_level < 0.5); // Should decrease
    }

    #[test]
    fn test_behavior_engine_apply_feedback_negative() {
        let mut engine = BehaviorEngine::new();
        let feedback = BehaviorFeedback {
            action_id: "action_456".to_string(),
            rating: 1,
            nuanced_feedback: "Action was too risky and ineffective.".to_string(),
            timestamp: Utc::now(),
        };
        engine.apply_feedback(feedback).unwrap();

        assert_eq!(engine.feedback_history.len(), 1);
        let eff_priority = engine.get_behavior_parameter("efficiency_priority").unwrap();
        let caution_level = engine.get_behavior_parameter("caution_level").unwrap();
        assert!(eff_priority < 0.7); // Should decrease
        assert!(caution_level > 0.5); // Should increase
    }

    #[test]
    fn test_behavior_engine_parameter_bounds() {
        let mut engine = BehaviorEngine::new();
        // Apply very negative feedback multiple times
        for _ in 0..10 {
            engine.apply_feedback(BehaviorFeedback {
                action_id: "action_test".to_string(),
                rating: 1,
                nuanced_feedback: "bad".to_string(),
                timestamp: Utc::now(),
            }).unwrap();
        }
        let caution_level = engine.get_behavior_parameter("caution_level").unwrap();
        assert_eq!(caution_level, 1.0); // Should be capped at 1.0
    }
}