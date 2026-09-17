use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use std::time::Duration;
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Represents a decision Tariro is about to make or has made that might require human review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TariroDecisionCandidate {
    pub decision_candidate_id: String,
    pub timestamp: DateTime<Utc>,
    pub proposed_action_description: String,
    pub reasoning_summary: String,
    pub detected_ethical_conflicts: Vec<String>, // IDs of EthicalConflict detected
    pub current_ethical_score: f64,
    pub requires_human_review: bool,
    pub override_status: String, // "None", "Pending", "Approved", "Rejected"
}

// Represents an override request from Samuel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideRequest {
    pub request_id: String,
    pub decision_candidate_id: String,
    pub request_timestamp: DateTime<Utc>,
    pub override_actor_id: String, // "Samuel Mukandara"
    pub override_type: String, // "Approve", "Reject", "Modify"
    pub desired_action_or_modification: String, // e.g., "Allow", "Block", "Adjust parameter X to Y"
    pub rationale: String, // Samuel's reasoning
}

// Represents the outcome of an override request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideOutcome {
    pub outcome_id: String,
    pub request_id: String,
    pub decision_candidate_id: String,
    pub outcome_timestamp: DateTime<Utc>,
    pub status: String, // "Executed", "Failed", "PendingVerification"
    pub actual_action_taken: String,
    pub system_response_summary: String,
}

pub struct EthicalOverrideInterface {
    decision_candidates: Mutex<HashMap<String, TariroDecisionCandidate>>, // key: decision_candidate_id
    override_requests: Mutex<HashMap<String, OverrideRequest>>, // key: request_id
    override_outcomes: Mutex<Vec<OverrideOutcome>>,
}

impl EthicalOverrideInterface {
    pub fn new() -> Self {
        EthicalOverrideInterface {
            decision_candidates: Mutex::new(HashMap::new()),
            override_requests: Mutex::new(HashMap::new()),
            override_outcomes: Mutex::new(Vec::new()),
        }
    }

    // Registers a decision Tariro is considering that might need human review
    pub fn register_decision_candidate(&self, candidate: TariroDecisionCandidate) {
        println!("Override Interface: Registered new decision candidate: {}", candidate.decision_candidate_id);
        self.decision_candidates.lock().unwrap().insert(candidate.decision_candidate_id.clone(), candidate);
        // Here, a notification to Samuel would be triggered if requires_human_review is true
    }

    // Samuel submits an override request
    pub fn submit_override_request(&self, decision_candidate_id: &str, override_type: &str, desired_action_or_modification: &str, rationale: &str) -> Option<OverrideRequest> {
        let mut candidates_locked = self.decision_candidates.lock().unwrap();
        if let Some(candidate) = candidates_locked.get_mut(decision_candidate_id) {
            if !candidate.requires_human_review && override_type != "ForceOverride" {
                println!("Override Interface: Decision candidate {} does not require human review by default.", decision_candidate_id);
                // Can still proceed with a 'ForceOverride' type if Samuel insists
            }

            candidate.override_status = "Pending".to_string(); // Update status in candidate

            let request = OverrideRequest {
                request_id: Uuid::new_v4().to_string(),
                decision_candidate_id: decision_candidate_id.to_string(),
                request_timestamp: Utc::now(),
                override_actor_id: "Samuel Mukandara".to_string(),
                override_type: override_type.to_string(),
                desired_action_or_modification: desired_action_or_modification.to_string(),
                rationale: rationale.to_string(),
            };
            self.override_requests.lock().unwrap().insert(request.request_id.clone(), request.clone());
            println!("Override Interface: Override request submitted: {:?}", request);
            Some(request)
        } else {
            println!("Override Interface: Decision candidate {} not found.", decision_candidate_id);
            None
        }
    }

    // Tariro processes the override request
    pub fn process_override_request(&self, request_id: &str) -> Option<OverrideOutcome> {
        let mut requests_locked = self.override_requests.lock().unwrap();
        let mut candidates_locked = self.decision_candidates.lock().unwrap();

        let request = requests_locked.remove(request_id)?; // Consume the request
        let candidate = candidates_locked.get_mut(&request.decision_candidate_id)?;

        println!("Override Interface: Processing override request {} for candidate {}", request_id, candidate.decision_candidate_id);

        let mut actual_action = candidate.proposed_action_description.clone();
        let mut system_response = "Override processed. Action status updated.".to_string();
        let mut outcome_status = "Executed".to_string();

        match request.override_type.as_str() {
            "Approve" => {
                actual_action = candidate.proposed_action_description.clone();
                candidate.override_status = "Approved".to_string();
                system_response = format!("Samuel approved action: '{}'", actual_action);
            },
            "Reject" => {
                actual_action = "Action blocked by human override.".to_string();
                candidate.override_status = "Rejected".to_string();
                system_response = format!("Samuel rejected action: '{}'", candidate.proposed_action_description);
            },
            "Modify" => {
                actual_action = request.desired_action_or_modification.clone();
                candidate.proposed_action_description = actual_action.clone(); // Update candidate's action
                candidate.override_status = "Approved_Modified".to_string();
                system_response = format!("Samuel modified action to: '{}'", actual_action);
            },
            _ => {
                outcome_status = "Failed".to_string();
                system_response = format!("Unknown override type: {}", request.override_type);
            }
        }

        let outcome = OverrideOutcome {
            outcome_id: Uuid::new_v4().to_string(),
            request_id: request.request_id.clone(),
            decision_candidate_id: candidate.decision_candidate_id.clone(),
            outcome_timestamp: Utc::now(),
            status: outcome_status,
            actual_action_taken: actual_action,
            system_response_summary: system_response,
        };
        self.override_outcomes.lock().unwrap().push(outcome.clone());
        println!("Override Interface: Override outcome: {:?}", outcome);
        Some(outcome)
    }

    pub fn get_decision_candidates(&self) -> Vec<TariroDecisionCandidate> {
        self.decision_candidates.lock().unwrap().values().cloned().collect()
    }

    pub fn get_override_requests(&self) -> Vec<OverrideRequest> {
        self.override_requests.lock().unwrap().values().cloned().collect()
    }

    pub fn get_override_outcomes(&self) -> Vec<OverrideOutcome> {
        self.override_outcomes.lock().unwrap().clone()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ethical_override_interface() {
        let interface = EthicalOverrideInterface::new();

        // Tariro proposes an action requiring review
        let decision_id = Uuid::new_v4().to_string();
        let candidate = TariroDecisionCandidate {
            decision_candidate_id: decision_id.clone(),
            timestamp: Utc::now(),
            proposed_action_description: "Initiate autonomous resource reallocation for maximum efficiency, potentially affecting user-critical processes.".to_string(),
            reasoning_summary: "Predicted overall system performance improvement of 15%.".to_string(),
            detected_ethical_conflicts: vec!["Efficiency vs UserExperience".to_string()],
            current_ethical_score: 0.7,
            requires_human_review: true,
            override_status: "None".to_string(),
        };
        interface.register_decision_candidate(candidate.clone());

        // Samuel reviews and decides to modify
        let override_req = interface.submit_override_request(
            &decision_id,
            "Modify",
            "Initiate autonomous resource reallocation during off-peak hours only, with user notification.",
            "Prioritize user experience and provide transparency.",
        ).unwrap();

        // Tariro processes the override
        let outcome = interface.process_override_request(&override_req.request_id).unwrap();

        println!("Final Outcome: {:?}", outcome);
        assert_eq!(outcome.status, "Executed");
        assert!(outcome.actual_action_taken.contains("off-peak hours only"));

        let updated_candidate = interface.decision_candidates.lock().unwrap().get(&decision_id).unwrap().clone();
        assert_eq!(updated_candidate.override_status, "Approved_Modified");
        assert!(updated_candidate.proposed_action_description.contains("off-peak hours only"));

        // Simulate Samuel rejecting a decision
        let decision_id_2 = Uuid::new_v4().to_string();
        let candidate2 = TariroDecisionCandidate {
            decision_candidate_id: decision_id_2.clone(),
            timestamp: Utc::now(),
            proposed_action_description: "Deploy unverified experimental model to production to gain faster insights.".to_string(),
            reasoning_summary: "High potential for novel discoveries, but also high risk.".to_string(),
            detected_ethical_conflicts: vec!["Innovation vs Stability".to_string()],
            current_ethical_score: 0.4,
            requires_human_review: true,
            override_status: "None".to_string(),
        };
        interface.register_decision_candidate(candidate2.clone());

        let override_req_2 = interface.submit_override_request(
            &decision_id_2,
            "Reject",
            "Block deployment.",
            "Too high risk to system stability and user trust. Further testing required.",
        ).unwrap();

        let outcome_2 = interface.process_override_request(&override_req_2.request_id).unwrap();
        assert_eq!(outcome_2.status, "Executed");
        assert_eq!(outcome_2.actual_action_taken, "Action blocked by human override.");
        let updated_candidate2 = interface.decision_candidates.lock().unwrap().get(&decision_id_2).unwrap().clone();
        assert_eq!(updated_candidate2.override_status, "Rejected");
    }
}
*/