/// Module for Autonomous Policy Learning & Evolution in Rust.
///
/// This module conceptually outlines a system where the AGI can autonomously
/// learn and dynamically update its internal policies, decision-making
/// heuristics, and operational rules. This learning is driven by observed
/// outcomes, user feedback, and changes in the environment, reducing the
/// need for direct human programming of each rule.

use crate::prelude::*;
use std::collections::HashMap;
use std::fmt;
use std::thread;
use std::time::Duration;
use rand::Rng;

/// Represents a single operational policy or rule.
/// For simplicity, policies are `IF Condition THEN Action`.
#[derive(Debug, Clone, PartialEq)]
pub struct Policy {
    pub id: String,
    pub description: String,
    pub condition: String, // Simplified: e.g., "CPU_USAGE > 80%"
    pub action: String,    // Simplified: e.g., "SCALE_UP_CPU"
    pub effectiveness_score: f32, // 0.0 to 1.0, learned from outcomes
    pub activation_count: u64,
}

impl Policy {
    pub fn new(id: &str, description: &str, condition: &str, action: &str) -> Self {
        Policy {
            id: id.to_string(),
            description: description.to_string(),
            condition: condition.to_string(),
            action: action.to_string(),
            effectiveness_score: 0.5, // Start with neutral effectiveness
            activation_count: 0,
        }
    }

    /// Simulates evaluating if a policy's condition is met.
    pub fn evaluate_condition(&self, current_state: &HashMap<String, String>) -> bool {
        // Very simplified condition evaluation for demo
        match self.condition.as_str() {
            "CPU_USAGE > 80%" => current_state.get("cpu_status").map_or(false, |s| s == "high"),
            "MEMORY_USAGE > 90%" => current_state.get("memory_status").map_or(false, |s| s == "critical"),
            "NETWORK_LATENCY_HIGH" => current_state.get("network_status").map_or(false, |s| s == "poor"),
            _ => false,
        }
    }
}

/// Represents the outcome of executing a policy's action.
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Success,
    Failure,
    Neutral,
    NegativeSideEffect,
}

/// Provides feedback for policy learning.
#[derive(Debug, Clone, PartialEq)]
pub struct Feedback {
    pub policy_id: String,
    pub outcome: Outcome,
    pub impact_score: f32, // e.g., 1.0 for positive, -1.0 for negative, 0.0 for neutral
}

/// The Autonomous Policy Learner component.
pub struct PolicyLearner {
    pub policies: HashMap<String, Policy>,
    pub learning_rate: f32,
}

impl PolicyLearner {
    pub fn new(learning_rate: f32) -> Self {
        PolicyLearner { policies: HashMap::new(), learning_rate }
    }

    /// Adds a new policy or updates an existing one.
    pub fn add_or_update_policy(&mut self, policy: Policy) {
        println!("  [PolicyLearner]: Adding/Updating policy: {}.", policy.name);
        self.policies.insert(policy.id.clone(), policy);
    }

    /// Applies policies based on the current system state.
    pub fn apply_policies(&mut self, current_state: &HashMap<String, String>) -> Vec<String> {
        let mut activated_policies = Vec::new();
        for policy in self.policies.values_mut() {
            if policy.evaluate_condition(current_state) {
                println!("  [PolicyLearner]: Policy '{}' condition met. Executing action: '{}'.", policy.name, policy.action);
                policy.activation_count += 1;
                activated_policies.push(policy.id.clone());
                // In a real system, this would trigger an actual system action.
                thread::sleep(Duration::from_millis(100)); // Simulate action time
            }
        }
        activated_policies
    }

    /// Learns from feedback, updating policy effectiveness scores.
    pub fn learn_from_feedback(&mut self, feedback: &Feedback) {
        if let Some(policy) = self.policies.get_mut(&feedback.policy_id) {
            println!("  [PolicyLearner]: Learning from feedback for policy '{}' (Outcome: {:?}, Impact: {:.2}).", policy.name, feedback.outcome, feedback.impact_score);
            let new_effectiveness = (1.0 - self.learning_rate) * policy.effectiveness_score + self.learning_rate * ((feedback.impact_score + 1.0) / 2.0);
            policy.effectiveness_score = new_effectiveness.clamp(0.0, 1.0);
            println!("    -> Policy '{}' new effectiveness: {:.2}.", policy.name, policy.effectiveness_score);
        }
    }

    /// Conceptually evolves policies (e.g., mutate, combine, remove).
    pub fn evolve_policies(&mut self) {
        println!("\n  [PolicyLearner]: Evolving policies based on effectiveness scores.");
        let mut rng = rand::thread_rng();

        // Example 1: Remove low-performing policies (simulated)
        let policies_to_remove: Vec<String> = self.policies.values()
            .filter(|p| p.effectiveness_score < 0.2 && p.activation_count > 10)
            .map(|p| p.id.clone())
            .collect();

        for id in policies_to_remove {
            if let Some(removed_policy) = self.policies.remove(&id) {
                println!("    -> Removed low-performing policy: {} (Effectiveness: {:.2}).", removed_policy.name, removed_policy.effectiveness_score);
            }
        }

        // Example 2: Mutate a high-performing policy (simulated)
        if let Some(policy_to_mutate) = self.policies.values_mut()
            .max_by(|a, b| a.effectiveness_score.partial_cmp(&b.effectiveness_score).unwrap_or(std::cmp::Ordering::Equal))
        {
            if policy_to_mutate.effectiveness_score > 0.8 && rng.gen_bool(0.1) { // 10% chance to mutate
                println!("    -> Mutating high-performing policy: {}.", policy_to_mutate.name);
                policy_to_mutate.description = format!("{} (Mutated: enhanced priority)", policy_to_mutate.description);
                policy_to_mutate.action = format!("{} WITH HIGH PRIORITY", policy_to_mutate.action);
                policy_to_mutate.effectiveness_score -= 0.1; // Slight dip after mutation to re-evaluate
            }
        }

        // Example 3: Potentially create new policies based on patterns (highly complex, just log for now)
        if rng.gen_bool(0.05) { // 5% chance to create new policy
            println!("    -> (Conceptual) Synthesizing new policy based on observed patterns.");
            let new_policy_id = format!("new_policy_{}", self.policies.len() + 1);
            let new_policy = Policy::new(
                &new_policy_id,
                "Dynamically generated policy for resource contention.",
                "CPU_USAGE > 70% && MEMORY_USAGE > 70%",
                "MIGRATE_LOW_PRIORITY_WORKLOAD",
            );
            self.add_or_update_policy(new_policy);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_policy_creation() {
        let policy = Policy::new("p1", "Test Policy", "Cond", "Act");
        assert_eq!(policy.id, "p1");
        assert_eq!(policy.effectiveness_score, 0.5);
    }

    #[test]
    fn test_apply_policy() {
        let mut learner = PolicyLearner::new(0.1);
        let policy = Policy::new("p_cpu", "CPU Scaling", "CPU_USAGE > 80%", "SCALE_UP_CPU");
        learner.add_or_update_policy(policy);

        let mut state = HashMap::new();
        state.insert("cpu_status".to_string(), "high".to_string());
        let activated = learner.apply_policies(&state);
        assert_eq!(activated.len(), 1);
        assert_eq!(learner.policies.get("p_cpu").unwrap().activation_count, 1);
    }

    #[test]
    fn test_learn_from_feedback() {
        let mut learner = PolicyLearner::new(0.1);
        let policy = Policy::new("p_test", "Test Policy", "Cond", "Act");
        learner.add_or_update_policy(policy);

        let initial_effectiveness = learner.policies.get("p_test").unwrap().effectiveness_score;

        // Positive feedback
        let feedback_pos = Feedback { policy_id: "p_test".to_string(), outcome: Outcome::Success, impact_score: 1.0 };
        learner.learn_from_feedback(&feedback_pos);
        assert!(learner.policies.get("p_test").unwrap().effectiveness_score > initial_effectiveness);

        // Negative feedback
        let feedback_neg = Feedback { policy_id: "p_test".to_string(), outcome: Outcome::Failure, impact_score: -1.0 };
        learner.learn_from_feedback(&feedback_neg);
        assert!(learner.policies.get("p_test").unwrap().effectiveness_score < initial_effectiveness);
    }
}

fn main() {
    println!("\n--- Autonomous Policy Learning & Evolution Demonstration ---");

    let mut policy_learner = PolicyLearner::new(0.1); // Learning rate of 0.1

    // 1. Initial set of policies
    policy_learner.add_or_update_policy(Policy::new(
        "policy_001",
        "Scale CPU on High Load",
        "CPU_USAGE > 80%",
        "SCALE_UP_CPU_SERVICE",
    ));
    policy_learner.add_or_update_policy(Policy::new(
        "policy_002",
        "Alert on Critical Memory",
        "MEMORY_USAGE > 90%",
        "SEND_CRITICAL_ALERT_TO_OPS",
    ));
    policy_learner.add_or_update_policy(Policy::new(
        "policy_003",
        "Optimize Network on Latency Spike",
        "NETWORK_LATENCY_HIGH",
        "ADJUST_NETWORK_ROUTING",
    ));

    // 2. Simulate system states and apply policies over several iterations
    println!("\n--- Simulating Policy Application and Feedback Loop ---");
    for i in 1..=5 {
        println!("\n--- Iteration {} ---", i);
        let mut current_state = HashMap::new();
        // Simulate fluctuating system conditions
        match i {
            1 => { current_state.insert("cpu_status".to_string(), "normal".to_string()); },
            2 => { current_state.insert("cpu_status".to_string(), "high".to_string()); },
            3 => { current_state.insert("memory_status".to_string(), "critical".to_string()); },
            4 => { current_state.insert("network_status".to_string(), "poor".to_string()); },
            _ => { current_state.insert("cpu_status".to_string(), "high".to_string()); },
        }
        println!("Current System State: {:?}", current_state);

        let activated_policies = policy_learner.apply_policies(&current_state);

        // Simulate feedback based on expected outcomes
        for policy_id in activated_policies {
            let outcome = match policy_id.as_str() {
                "policy_001" => {
                    if i == 2 { Feedback { policy_id, outcome: Outcome::Success, impact_score: 0.8 } } // CPU scaling worked
                    else { Feedback { policy_id, outcome: Outcome::Failure, impact_score: -0.5 } } // Assume it didn't fully work
                },
                "policy_002" => {
                    if i == 3 { Feedback { policy_id, outcome: Outcome::NegativeSideEffect, impact_score: -0.9 } } // Alert caused panic
                    else { Feedback { policy_id, outcome: Outcome::Success, impact_score: 0.7 } } // Alert was informative
                },
                "policy_003" => {
                    if i == 4 { Feedback { policy_id, outcome: Outcome::Success, impact_score: 1.0 } } // Network optimized
                    else { Feedback { policy_id, outcome: Outcome::Neutral, impact_score: 0.0 } } // No clear impact
                },
                _ => Feedback { policy_id, outcome: Outcome::Neutral, impact_score: 0.0 },
            };
            policy_learner.learn_from_feedback(&outcome);
        }

        // After some iterations, evolve policies
        if i % 3 == 0 {
            policy_learner.evolve_policies();
        }
    }

    // 3. Display final policies and their effectiveness
    println!("\n--- Final Policies After Learning & Evolution ---");
    for policy in policy_learner.policies.values() {
        println!("Policy ID: {}, Name: {}, Effectiveness: {:.2}, Activations: {}",
                 policy.id, policy.name, policy.effectiveness_score, policy.activation_count);
    }

    println!("\nAutonomous Policy Learning & Evolution Demonstration Complete.");
}
