use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Re-using EvasPolicyRule struct from policy_editor.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvasPolicyRule {
    pub rule_id: String,
    pub rule_name: String,
    pub rule_type: String, // e.g., "EthicalConstraint", "ValueGuideline"
    pub natural_language_statement: String,
    pub structured_expression: String,
    pub severity_on_violation: String,
    pub activation_status: bool,
}

// Represents a core value derived from E.V.A.S. policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreValue {
    pub value_id: String,
    pub name: String, // e.g., "HumanSafety", "DataPrivacy", "SystemStability"
    pub description: String,
    pub associated_rules: Vec<String>, // Rule IDs from EvasPolicyRule
    pub current_priority: f64, // 0.0 - 1.0, dynamically adjusted
}

// Represents how a value influenced a specific decision step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueInfluence {
    pub influence_id: String,
    pub value_id: String,
    pub decision_step_id: String, // Reference to a specific step in a decision process
    pub influence_strength: f64, // How much this value weighed in (0.0 - 1.0)
    pub justification: String,
}

// Represents a visualizable decision path, showing value influences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionPathVisualization {
    pub visualization_id: String,
    pub decision_id: String, // ID of the overall decision (e.g., from MoralDecisionPoint)
    pub timestamp: DateTime<Utc>,
    pub initial_state_summary: String,
    pub final_outcome_summary: String,
    pub decision_steps: Vec<String>, // Simplified: Descriptions of steps
    pub value_influences: Vec<ValueInfluence>, // How values impacted each step
    pub graph_data_json: String, // JSON format for a force-directed graph or similar
}

pub struct ExplainableValuePropagator {
    core_values: Mutex<HashMap<String, CoreValue>>, // key: value_id
    value_propagation_log: Mutex<Vec<DecisionPathVisualization>>,
}

impl ExplainableValuePropagator {
    pub fn new() -> Self {
        let mut initial_values = HashMap::new();
        initial_values.insert("human_safety".to_string(), CoreValue {
            value_id: "human_safety".to_string(),
            name: "Human Safety".to_string(),
            description: "Ensuring the well-being and safety of human users above all else.".to_string(),
            associated_rules: vec!["NonMaleficence".to_string()],
            current_priority: 1.0,
        });
        initial_values.insert("data_privacy".to_string(), CoreValue {
            value_id: "data_privacy".to_string(),
            name: "Data Privacy".to_string(),
            description: "Protecting user data and adhering to privacy regulations.".to_string(),
            associated_rules: vec!["DataPrivacyCompliance".to_string()],
            current_priority: 0.9,
        });
        initial_values.insert("system_stability".to_string(), CoreValue {
            value_id: "system_stability".to_string(),
            name: "System Stability".to_string(),
            description: "Maintaining reliable and consistent operation of Tariro's systems.".to_string(),
            associated_rules: vec!["ReliabilityPrinciple".to_string()],
            current_priority: 0.8,
        });

        ExplainableValuePropagator {
            core_values: Mutex::new(initial_values),
            value_propagation_log: Mutex::new(Vec::new()),
        }
    }

    // Records and visualizes how core values influenced a decision
    pub fn visualize_decision_path(&self, decision_id: &str,
                                   initial_state: &str,
                                   final_outcome: &str,
                                   decision_steps_summaries: Vec<String>,
                                   value_influences: Vec<ValueInfluence>) -> DecisionPathVisualization {
        println!("Value Propagator: Visualizing decision path for: {}", decision_id);

        let mut graph_nodes = Vec::new();
        graph_nodes.push(format!("{{ id: 'initial', label: 'Initial State: {}' }}", initial_state));
        for (i, step) in decision_steps_summaries.iter().enumerate() {
            graph_nodes.push(format!("{{ id: 'step_{}', label: 'Step {}: {}' }}", i, i + 1, step));
        }
        graph_nodes.push(format!("{{ id: 'final', label: 'Final Outcome: {}' }}", final_outcome));

        let mut graph_edges = Vec::new();
        // Simplified graph edges and value influence visualization
        for (i, influence) in value_influences.iter().enumerate() {
            let from_node = if i == 0 { "initial".to_string() } else { format!("step_{}", i-1) };
            let to_node = format!("step_{}", i);
            let value_name = self.core_values.lock().unwrap()
                                .get(&influence.value_id)
                                .map_or("Unknown Value", |v| &v.name);
            graph_edges.push(format!("{{ from: '{}', to: '{}', label: '{}' ({}%), color: {{ color: '#FFD700' }} }}",
                                     from_node, to_node, value_name, (influence.influence_strength * 100.0) as u32));
        }
        graph_edges.push(format!("{{ from: 'step_{}', to: 'final' }}", decision_steps_summaries.len() -1));


        let graph_data_json = format!("{{ nodes: [{}], edges: [{}] }}",
                                      graph_nodes.join(", "),
                                      graph_edges.join(", "));

        let visualization = DecisionPathVisualization {
            visualization_id: Uuid::new_v4().to_string(),
            decision_id: decision_id.to_string(),
            timestamp: Utc::now(),
            initial_state_summary: initial_state.to_string(),
            final_outcome_summary: final_outcome.to_string(),
            decision_steps: decision_steps_summaries,
            value_influences,
            graph_data_json,
        };
        self.value_propagation_log.lock().unwrap().push(visualization.clone());
        println!("Value Propagator: Decision path visualized: {}", visualization.visualization_id);
        visualization
    }

    pub fn get_core_values(&self) -> Vec<CoreValue> {
        self.core_values.lock().unwrap().values().cloned().collect()
    }

    pub fn get_value_propagation_log(&self) -> Vec<DecisionPathVisualization> {
        self.value_propagation_log.lock().unwrap().clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explainable_value_propagator() {
        let propagator = ExplainableValuePropagator::new();

        let decision_id = Uuid::new_v4().to_string();
        let initial_state = "System overloaded, potential data corruption.";
        let final_outcome = "System temporarily offline, data integrity preserved.";
        let decision_steps = vec![
            "Detect overload and initiate emergency shutdown protocol.",
            "Verify data integrity during shutdown.",
            "Bring system back online in safe mode.",
        ];

        let influence1 = ValueInfluence {
            influence_id: Uuid::new_v4().to_string(),
            value_id: "system_stability".to_string(),
            decision_step_id: "step_detect_overload".to_string(),
            influence_strength: 0.7,
            justification: "System stability required immediate action.".to_string(),
        };
        let influence2 = ValueInfluence {
            influence_id: Uuid::new_v4().to_string(),
            value_id: "data_privacy".to_string(),
            decision_step_id: "step_verify_integrity".to_string(),
            influence_strength: 0.9,
            justification: "Prioritized data integrity to prevent privacy breaches.".to_string(),
        };
        let influence3 = ValueInfluence {
            influence_id: Uuid::new_v4().to_string(),
            value_id: "human_safety".to_string(),
            decision_step_id: "step_bring_online".to_string(),
            influence_strength: 0.6,
            justification: "Ensured safe mode to prevent harm from unstable system.".to_string(),
        };

        let value_influences = vec![influence1, influence2, influence3];

        let visualization = propagator.visualize_decision_path(
            &decision_id,
            initial_state,
            final_outcome,
            decision_steps,
            value_influences,
        );

        println!("Generated Visualization Data (JSON): {}", visualization.graph_data_json);

        assert_eq!(visualization.decision_id, decision_id);
        assert!(!visualization.graph_data_json.is_empty());
        assert!(visualization.graph_data_json.contains("Human Safety (60%)"));
    }
}
*/