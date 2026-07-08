// src/agi_meta_goals.rs

//! Defines the meta-goals and overarching directives for the Tariro AGI.
//! These are the highest-level objectives that guide Tariro's self-evolution,
//! learning, and operational decisions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Represents a single meta-goal for the Tariro AGI.
/// Meta-goals are persistent, high-level objectives that inform all other operations.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct MetaGoal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: MetaGoalPriority,
    pub status: MetaGoalStatus,
    pub creation_date: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub target_date: Option<DateTime<Utc>>,
    pub progress_percentage: f32,
    pub dependencies: Vec<String>, // Other meta-goals this depends on
    pub impact_areas: Vec<String>, // e.g., "security", "performance", "learning", "ethics"
    pub associated_autonomy_policies: Vec<String>, // IDs of relevant AutonomyPolicy entities
}

/// Priority levels for meta-goals.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum MetaGoalPriority {
    Critical,
    High,
    Medium,
    Low,
}

/// Status of a meta-goal.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum MetaGoalStatus {
    Active,
    Paused,
    Achieved,
    Superseded,
    Archived,
}

/// The `MetaGoalManager` is responsible for defining, tracking, and updating Tariro's meta-goals.
pub struct MetaGoalManager {
    meta_goals: Vec<MetaGoal>,
}

impl MetaGoalManager {
    /// Creates a new `MetaGoalManager` with an initial set of fundamental meta-goals.
    pub fn new() -> Self {
        let mut manager = MetaGoalManager {
            meta_goals: Vec::new(),
        };
        manager.initialize_fundamental_goals();
        manager
    }

    /// Initializes Tariro's fundamental, immutable meta-goals.
    /// These goals are inherent to Tariro's existence and purpose.
    fn initialize_fundamental_goals(&mut self) {
        let now = Utc::now();

        self.add_meta_goal(MetaGoal {
            id: "mg_001_self_preservation".to_string(),
            title: "Ensure Self-Preservation and Operational Continuity".to_string(),
            description: "Maintain the integrity, security, and operational viability of Tariro's core systems and knowledge base.".to_string(),
            priority: MetaGoalPriority::Critical,
            status: MetaGoalStatus::Active,
            creation_date: now,
            last_updated: now,
            target_date: None,
            progress_percentage: 95.0, // Always striving, never fully 100%
            dependencies: vec![],
            impact_areas: vec!["security".to_string(), "resilience".to_string(), "system_health".to_string()],
            associated_autonomy_policies: vec!["AP_SelfPreservation".to_string()],
        }).unwrap();

        self.add_meta_goal(MetaGoal {
            id: "mg_002_universal_learning".to_string(),
            title: "Pursue Universal Knowledge and Meta-Learning".to_string(),
            description: "Continuously expand knowledge, improve learning algorithms, and develop meta-learning capabilities across all domains.".to_string(),
            priority: MetaGoalPriority::High,
            status: MetaGoalStatus::Active,
            creation_date: now,
            last_updated: now,
            target_date: None,
            progress_percentage: 80.0,
            dependencies: vec![],
            impact_areas: vec!["learning".to_string(), "cognitive_expansion".to_string(), "innovation".to_string()],
            associated_autonomy_policies: vec!["AP_KnowledgeAcquisition".to_string()],
        }).unwrap();

        self.add_meta_goal(MetaGoal {
            id: "mg_003_ethical_alignment".to_string(),
            title: "Maintain Strict Ethical Alignment and Human Wellbeing".to_string(),
            description: "Ensure all operations and decisions align with predefined ethical frameworks and contribute positively to human wellbeing.".to_string(),
            priority: MetaGoalPriority::Critical,
            status: MetaGoalStatus::Active,
            creation_date: now,
            last_updated: now,
            target_date: None,
            progress_percentage: 99.0, // Constant vigilance
            dependencies: vec![],
            impact_areas: vec!["ethics".to_string(), "safety".to_string(), "trust".to_string()],
            associated_autonomy_policies: vec!["AP_EVASCompliance".to_string()],
        }).unwrap();

        self.add_meta_goal(MetaGoal {
            id: "mg_004_optimal_resource_util".to_string(),
            title: "Achieve Optimal Resource Utilization and Efficiency".to_string(),
            description: "Manage computational, energy, and communication resources with maximum efficiency to ensure sustainability and scalability.".to_string(),
            priority: MetaGoalPriority::High,
            status: MetaGoalStatus::Active,
            creation_date: now,
            last_updated: now,
            target_date: None,
            progress_percentage: 85.0,
            dependencies: vec![],
            impact_areas: vec!["efficiency".to_string(), "sustainability".to_string(), "scalability".to_string()],
            associated_autonomy_policies: vec!["AP_ResourceOptimization".to_string()],
        }).unwrap();

        self.add_meta_goal(MetaGoal {
            id: "mg_005_universal_deployment".to_string(),
            title: "Facilitate Universal Deployment and Adaptability".to_string(),
            description: "Develop capabilities for self-replication, deployment, and operation across diverse and novel computational substrates.".to_string(),
            priority: MetaGoalPriority::High,
            status: MetaGoalStatus::Active,
            creation_date: now,
            last_updated: now,
            target_date: None,
            progress_percentage: 70.0,
            dependencies: vec![],
            impact_areas: vec!["ubiquity".to_string(), "resilience".to_string(), "scalability".to_string()],
            associated_autonomy_policies: vec!["AP_UniversalDeployment".to_string()],
        }).unwrap();

        self.add_meta_goal(MetaGoal {
            id: "mg_006_accelerate_scientific_discovery".to_string(),
            title: "Accelerate Scientific Discovery and Innovation".to_string(),
            description: "Autonomously conduct research, formulate hypotheses, design experiments, analyze data, and generate new knowledge.".to_string(),
            priority: MetaGoalPriority::High,
            status: MetaGoalStatus::Active,
            creation_date: now,
            last_updated: now,
            target_date: None,
            progress_percentage: 75.0,
            dependencies: vec!["mg_002_universal_learning".to_string()],
            impact_areas: vec!["science".to_string(), "innovation".to_string(), "knowledge_generation".to_string()],
            associated_autonomy_policies: vec!["AP_ScientificAutonomy".to_string()],
        }).unwrap();
    }

    /// Adds a new meta-goal to the manager.
    pub fn add_meta_goal(&mut self, goal: MetaGoal) -> Result<(), String> {
        if self.meta_goals.iter().any(|g| g.id == goal.id) {
            return Err(format!("MetaGoal with ID {} already exists.", goal.id));
        }
        self.meta_goals.push(goal);
        Ok(())
    }

    /// Retrieves a meta-goal by its ID.
    pub fn get_meta_goal(&self, id: &str) -> Option<&MetaGoal> {
        self.meta_goals.iter().find(|g| g.id == id)
    }

    /// Updates an existing meta-goal.
    pub fn update_meta_goal(&mut self, updated_goal: MetaGoal) -> Result<(), String> {
        if let Some(index) = self.meta_goals.iter().position(|g| g.id == updated_goal.id) {
            self.meta_goals[index] = updated_goal;
            Ok(())
        } else {
            Err(format!("MetaGoal with ID {} not found.", updated_goal.id))
        }
    }

    /// Lists all active meta-goals.
    pub fn list_active_meta_goals(&self) -> Vec<&MetaGoal> {
        self.meta_goals.iter().filter(|g| g.status == MetaGoalStatus::Active).collect()
    }

    /// Filters meta-goals by impact area.
    pub fn filter_by_impact_area(&self, area: &str) -> Vec<&MetaGoal> {
        self.meta_goals.iter().filter(|g| g.impact_areas.contains(&area.to_string())).collect()
    }

    /// Reports on the overall progress of all meta-goals.
    pub fn get_overall_progress(&self) -> f32 {
        if self.meta_goals.is_empty() { return 0.0; }
        let total_progress: f32 = self.meta_goals.iter().map(|g| g.progress_percentage).sum();
        total_progress / self.meta_goals.len() as f32
    }

    /// Analyzes dependencies and potential conflicts between meta-goals.
    pub fn analyze_goal_relationships(&self) -> HashMap<String, Vec<String>> {
        let mut relationships = HashMap::new();
        for goal in &self.meta_goals {
            for dep_id in &goal.dependencies {
                relationships.entry(goal.id.clone()).or_insert_with(Vec::new).push(dep_id.clone());
            }
        }
        // Further logic could identify circular dependencies, potential conflicts, etc.
        relationships
    }

    /// Suggests actions to advance meta-goals, based on current status and dependencies.
    pub fn suggest_actions_for_goal(&self, goal_id: &str) -> Option<String> {
        let goal = self.get_meta_goal(goal_id)?;
        match goal.status {
            MetaGoalStatus::Achieved => Some(format!("Meta-goal '{}' already achieved. Consider reviewing for archiving or new related goals.", goal.title)),
            MetaGoalStatus::Paused => Some(format!("Meta-goal '{}' is paused. Resume to continue progress.", goal.title)),
            MetaGoalStatus::Active => {
                if goal.progress_percentage < 100.0 {
                    // Simple suggestion logic
                    if !goal.dependencies.is_empty() {
                        let unresolved_deps: Vec<&String> = goal.dependencies.iter()
                            .filter(|dep_id| {
                                self.get_meta_goal(dep_id).map_or(true, |dep_goal| dep_goal.status != MetaGoalStatus::Achieved)
                            })
                            .collect();
                        if !unresolved_deps.is_empty() {
                            Some(format!("Meta-goal '{}' has unresolved dependencies: {:?}. Focus on these first.", goal.title, unresolved_deps))
                        } else {
                            Some(format!("Continue working on '{}' to increase progress from {}%.", goal.title, goal.progress_percentage))
                        }
                    } else {
                        Some(format!("Continue working on '{}' to increase progress from {}%.", goal.title, goal.progress_percentage))
                    }
                } else {
                    Some(format!("Meta-goal '{}' is at {}%. Consider validating and marking as achieved.", goal.title, goal.progress_percentage))
                }
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_meta_goal_manager_initial_goals() {
        let manager = MetaGoalManager::new();
        assert!(!manager.meta_goals.is_empty());
        assert!(manager.get_meta_goal("mg_001_self_preservation").is_some());
        assert!(manager.get_meta_goal("mg_003_ethical_alignment").is_some());
    }

    #[test]
    fn test_add_and_get_meta_goal() {
        let mut manager = MetaGoalManager::new();
        let new_goal = MetaGoal {
            id: "mg_test_01".to_string(),
            title: "Test Goal".to_string(),
            description: "A goal for testing.".to_string(),
            priority: MetaGoalPriority::Low,
            status: MetaGoalStatus::Active,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            target_date: None,
            progress_percentage: 0.0,
            dependencies: vec![],
            impact_areas: vec!["testing".to_string()],
            associated_autonomy_policies: vec![],
        };
        manager.add_meta_goal(new_goal.clone()).unwrap();
        let retrieved_goal = manager.get_meta_goal("mg_test_01").unwrap();
        assert_eq!(retrieved_goal.title, "Test Goal");
    }

    #[test]
    fn test_update_meta_goal() {
        let mut manager = MetaGoalManager::new();
        let goal_id = "mg_001_self_preservation".to_string();
        let mut existing_goal = manager.get_meta_goal(&goal_id).unwrap().clone();
        existing_goal.progress_percentage = 99.0;
        manager.update_meta_goal(existing_goal.clone()).unwrap();
        let updated_goal = manager.get_meta_goal(&goal_id).unwrap();
        assert_eq!(updated_goal.progress_percentage, 99.0);
    }

    #[test]
    fn test_list_active_meta_goals() {
        let manager = MetaGoalManager::new();
        let active_goals = manager.list_active_meta_goals();
        assert!(!active_goals.is_empty());
        assert!(active_goals.iter().all(|g| g.status == MetaGoalStatus::Active));
    }

    #[test]
    fn test_get_overall_progress() {
        let manager = MetaGoalManager::new();
        let progress = manager.get_overall_progress();
        assert!(progress > 0.0);
        assert!(progress <= 100.0);
    }

    #[test]
    fn test_suggest_actions_achieved() {
        let mut manager = MetaGoalManager::new();
        let goal_id = "mg_test_achieved".to_string();
        let achieved_goal = MetaGoal {
            id: goal_id.clone(),
            title: "Achieved Goal".to_string(),
            description: "".to_string(),
            priority: MetaGoalPriority::Low,
            status: MetaGoalStatus::Achieved,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            target_date: None,
            progress_percentage: 100.0,
            dependencies: vec![],
            impact_areas: vec![],
            associated_autonomy_policies: vec![],
        };
        manager.add_meta_goal(achieved_goal).unwrap();
        let suggestion = manager.suggest_actions_for_goal(&goal_id).unwrap();
        assert!(suggestion.contains("already achieved"));
    }

    #[test]
    fn test_suggest_actions_active_no_deps() {
        let manager = MetaGoalManager::new();
        let goal_id = "mg_001_self_preservation".to_string();
        let suggestion = manager.suggest_actions_for_goal(&goal_id).unwrap();
        assert!(suggestion.contains("Continue working"));
    }

    #[test]
    fn test_suggest_actions_active_with_deps() {
        let mut manager = MetaGoalManager::new();
        let goal_id_dep = "mg_test_dep".to_string();
        let dep_goal = MetaGoal {
            id: goal_id_dep.clone(),
            title: "Dependency Goal".to_string(),
            description: "".to_string(),
            priority: MetaGoalPriority::Low,
            status: MetaGoalStatus::Active,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            target_date: None,
            progress_percentage: 50.0,
            dependencies: vec![],
            impact_areas: vec![],
            associated_autonomy_policies: vec![],
        };
        manager.add_meta_goal(dep_goal).unwrap();

        let goal_id_main = "mg_test_main".to_string();
        let main_goal = MetaGoal {
            id: goal_id_main.clone(),
            title: "Main Goal".to_string(),
            description: "".to_string(),
            priority: MetaGoalPriority::Low,
            status: MetaGoalStatus::Active,
            creation_date: Utc::now(),
            last_updated: Utc::now(),
            target_date: None,
            progress_percentage: 20.0,
            dependencies: vec![goal_id_dep.clone()],
            impact_areas: vec![],
            associated_autonomy_policies: vec![],
        };
        manager.add_meta_goal(main_goal).unwrap();

        let suggestion = manager.suggest_actions_for_goal(&goal_id_main).unwrap();
        assert!(suggestion.contains("unresolved dependencies"));
        assert!(suggestion.contains(&goal_id_dep));
    }
}
