/// Module for Dynamic Goal Generation & Prioritization in Rust.
///
/// This module conceptually outlines a system where the AGI can autonomously
/// form, decompose, and prioritize its own short-term sub-goals and long-term
/// objectives. This is driven by its current understanding of the environment,
/// overarching directives, and internal state, including mechanisms for
/// conflict resolution between competing goals.

use std::collections::HashMap;
use std::fmt;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use rand::Rng;

/// Represents an objective for the AGI.
#[derive(Debug, Clone, PartialEq)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub priority: u8, // Higher value means higher priority (0-100)
    pub deadline: Option<u64>, // Unix timestamp
    pub value: f32, // Estimated value or utility of achieving this goal
    pub depends_on: Vec<String>, // Other goal IDs this one depends on
    pub conflicts_with: Vec<String>, // Other goal IDs this one conflicts with
}

/// Represents the current environment's state.
#[derive(Debug, Clone)]
pub struct EnvironmentState {
    pub cpu_load_pct: f32,
    pub network_latency_ms: u32,
    pub security_alert_level: u8, // 0=none, 10=critical
    pub energy_cost_per_hour: f32,
}

/// Represents the AGI's internal operational state.
#[derive(Debug, Clone)]
pub struct InternalState {
    pub self_preservation_score: f32,
    pub knowledge_acquisition_rate: f32,
    pub resource_utilization_pct: f32,
}

/// High-level, persistent commands or objectives given to the AGI.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OverarchingDirective {
    MaximizeUptime,
    MinimizeCost,
    EnsureSecurity,
    OptimizeLearning,
    UserSatisfaction,
}

/// The Dynamic Goal Manager.
pub struct GoalManager {
    pub active_goals: HashMap<String, Goal>,
    pub overarching_directives: HashSet<OverarchingDirective>,
}

impl GoalManager {
    pub fn new() -> Self {
        let mut directives = HashSet::new();
        directives.insert(OverarchingDirective::MaximizeUptime);
        directives.insert(OverarchingDirective::EnsureSecurity);
        directives.insert(OverarchingDirective::UserSatisfaction);

        GoalManager {
            active_goals: HashMap::new(),
            overarching_directives: directives,
        }
    }

    /// Generates new goals based on environment, internal state, and directives.
    pub fn generate_new_goals(&self, env: &EnvironmentState, internal: &InternalState) -> Vec<Goal> {
        println!("  [GoalManager]: Generating new goals...");
        let mut new_goals = Vec::new();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        // Goals from Environment State
        if env.cpu_load_pct > 80.0 {
            new_goals.push(Goal {
                id: format!("cpu_optimize_{}", now),
                description: "Reduce CPU load to ensure stability.".to_string(),
                priority: 85,
                deadline: Some(now + 3600), // 1 hour
                value: 0.8,
                depends_on: vec![],
                conflicts_with: vec![],
            });
        }
        if env.security_alert_level >= 5 {
            new_goals.push(Goal {
                id: format!("threat_mitigate_{}", now),
                description: "Investigate and mitigate security threat.".to_string(),
                priority: 95,
                deadline: Some(now + 1800), // 30 minutes
                value: 0.95,
                depends_on: vec![],
                conflicts_with: vec![],
            });
        }

        // Goals from Internal State (e.g., self-preservation, learning)
        if internal.self_preservation_score < 0.7 {
            new_goals.push(Goal {
                id: format!("self_heal_{}", now),
                description: "Execute self-healing protocols.".to_string(),
                priority: 90,
                deadline: Some(now + 7200),
                value: 0.9,
                depends_on: vec![],
                conflicts_with: vec![],
            });
        }

        // Goals from Overarching Directives (if not already handled by specific conditions)
        if self.overarching_directives.contains(&OverarchingDirective::MinimizeCost) {
            // Check if current costs are high and if there's no active cost-saving goal
            if env.energy_cost_per_hour > 10.0 && !self.active_goals.values().any(|g| g.description.contains("cost")) {
                new_goals.push(Goal {
                    id: format!("cost_reduce_{}", now),
                    description: "Identify and implement cost reduction measures.".to_string(),
                    priority: 60,
                    deadline: None,
                    value: 0.6,
                    depends_on: vec![],
                    conflicts_with: vec!["cpu_optimize".to_string()], // Example conflict
                });
            }
        }

        thread::sleep(Duration::from_millis(100));
        new_goals
    }

    /// Prioritizes a list of goals based on various factors.
    pub fn prioritize_goals(&self, goals: &mut Vec<Goal>) {
        println!("  [GoalManager]: Prioritizing {} goals.", goals.len());
        // Sort goals: Higher priority first, then earlier deadline, then higher value
        goals.sort_by(|a, b| {
            b.priority.cmp(&a.priority) // Descending priority
                .then_with(|| a.deadline.cmp(&b.deadline)) // Ascending deadline (earlier first)
                .then_with(|| b.value.partial_cmp(&a.value).unwrap_or(std::cmp::Ordering::Equal)) // Descending value
        });
        thread::sleep(Duration::from_millis(50));
    }

    /// Decomposes a high-level goal into smaller, actionable sub-goals.
    pub fn decompose_goal(&self, goal: &Goal) -> Vec<Goal> {
        println!("  [GoalManager]: Decomposing goal: {}.", goal.description);
        let mut sub_goals = Vec::new();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        if goal.description.contains("Reduce CPU load") {
            sub_goals.push(Goal {
                id: format!("{}_sub1", goal.id),
                description: "Identify high-CPU processes.".to_string(),
                priority: goal.priority + 5,
                deadline: Some(now + 900), // 15 mins
                value: goal.value * 0.3,
                depends_on: vec![], conflicts_with: vec![],
            });
            sub_goals.push(Goal {
                id: format!("{}_sub2", goal.id),
                description: "Optimize or migrate high-CPU workloads.".to_string(),
                priority: goal.priority + 3,
                deadline: Some(now + 2700), // 45 mins
                value: goal.value * 0.7,
                depends_on: vec![sub_goals[0].id.clone()],
                conflicts_with: vec![],
            });
        }
        thread::sleep(Duration::from_millis(70));
        sub_goals
    }

    /// Resolves conflicts between goals (conceptual).
    pub fn resolve_conflicts(&self, goals: &mut Vec<Goal>) {
        println!("  [GoalManager]: Resolving conflicts among {} goals.", goals.len());
        // Simple conceptual conflict resolution: High priority wins, or compromise.
        // If "cpu_optimize" and "cost_reduce" conflict, and CPU optimize is higher priority,
        // cost reduction might be deprioritized or altered.
        let mut rng = rand::thread_rng();

        for i in 0..goals.len() {
            for j in (i + 1)..goals.len() {
                if goals[i].conflicts_with.contains(&goals[j].id) || goals[j].conflicts_with.contains(&goals[i].id) {
                    println!("    -> Detected conflict between '{}' and '{}'.", goals[i].description, goals[j].description);
                    if goals[i].priority > goals[j].priority {
                        println!("       ('{}' wins due to higher priority). Adjusting '{}'.", goals[i].description, goals[j].description);
                        goals[j].priority = (goals[j].priority as f32 * 0.5).round() as u8; // Deprioritize
                        goals[j].description = format!("{} (deprioritized due to conflict with higher priority goal)", goals[j].description);
                    } else if goals[j].priority > goals[i].priority {
                        println!("       ('{}' wins due to higher priority). Adjusting '{}'.", goals[j].description, goals[i].description);
                        goals[i].priority = (goals[i].priority as f32 * 0.5).round() as u8;
                        goals[i].description = format!("{} (deprioritized due to conflict with higher priority goal)", goals[i].description);
                    } else { // Equal priority, attempt compromise
                        if rng.gen_bool(0.5) {
                            println!("       (Equal priority). Attempting compromise: '{}' over '{}'.", goals[i].description, goals[j].description);
                            goals[j].description = format!("{} (compromised)", goals[j].description);
                        } else {
                            println!("       (Equal priority). Attempting compromise: '{}' over '{}'.", goals[j].description, goals[i].description);
                            goals[i].description = format!("{} (compromised)", goals[i].description);
                        }
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(80));
    }

    /// Integrates newly generated and prioritized goals into the active set.
    pub fn integrate_goals(&mut self, new_goals: Vec<Goal>) {
        println!("  [GoalManager]: Integrating {} new goals.", new_goals.len());
        for goal in new_goals {
            self.active_goals.insert(goal.id.clone(), goal);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_env() -> EnvironmentState {
        EnvironmentState {
            cpu_load_pct: 70.0,
            network_latency_ms: 50,
            security_alert_level: 0,
            energy_cost_per_hour: 5.0,
        }
    }
    fn create_sample_internal() -> InternalState {
        InternalState {
            self_preservation_score: 0.9,
            knowledge_acquisition_rate: 0.7,
            resource_utilization_pct: 0.6,
        }
    }

    #[test]
    fn test_generate_goals_cpu_load() {
        let manager = GoalManager::new();
        let mut env = create_sample_env();
        env.cpu_load_pct = 85.0;
        let internal = create_sample_internal();
        let goals = manager.generate_new_goals(&env, &internal);
        assert!(!goals.is_empty());
        assert!(goals.iter().any(|g| g.description.contains("Reduce CPU load")));
    }

    #[test]
    fn test_prioritize_goals() {
        let manager = GoalManager::new();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut goals = vec![
            Goal { id: "gA".to_string(), description: "A".to_string(), priority: 50, deadline: Some(now + 100), value: 0.5, depends_on: vec![], conflicts_with: vec![] },
            Goal { id: "gB".to_string(), description: "B".to_string(), priority: 80, deadline: Some(now + 200), value: 0.8, depends_on: vec![], conflicts_with: vec![] },
            Goal { id: "gC".to_string(), description: "C".to_string(), priority: 50, deadline: Some(now + 50), value: 0.9, depends_on: vec![], conflicts_with: vec![] },
        ];
        manager.prioritize_goals(&mut goals);
        assert_eq!(goals[0].id, "gB"); // Highest priority
        assert_eq!(goals[1].id, "gC"); // Same priority as A, but earlier deadline
        assert_eq!(goals[2].id, "gA");
    }

    #[test]
    fn test_decompose_goal() {
        let manager = GoalManager::new();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let cpu_goal = Goal {
            id: "main_cpu_goal".to_string(),
            description: "Reduce CPU load to ensure stability.".to_string(),
            priority: 85,
            deadline: Some(now + 3600),
            value: 0.8,
            depends_on: vec![],
            conflicts_with: vec![],
        };
        let sub_goals = manager.decompose_goal(&cpu_goal);
        assert_eq!(sub_goals.len(), 2);
        assert!(sub_goals[1].depends_on.contains(&sub_goals[0].id));
    }

    #[test]
    fn test_resolve_conflict() {
        let mut manager = GoalManager::new();
        manager.overarching_directives.insert(OverarchingDirective::MinimizeCost);
        manager.active_goals.insert("cpu_optimize".to_string(), Goal {
            id: "cpu_optimize".to_string(), description: "Reduce CPU load.".to_string(), priority: 85,
            deadline: None, value: 0.8, depends_on: vec![], conflicts_with: vec![],
        });
        let mut goals_to_resolve = vec![
            manager.active_goals.get("cpu_optimize").unwrap().clone(),
            Goal { id: "cost_reduce_X".to_string(), description: "Reduce operational costs.".to_string(), priority: 60,
                deadline: None, value: 0.6, depends_on: vec![], conflicts_with: vec!["cpu_optimize".to_string()],
            },
        ];
        manager.resolve_conflicts(&mut goals_to_resolve);
        // CPU optimize should have higher priority, so cost_reduce gets deprioritized
        assert!(goals_to_resolve[0].description.contains("Reduce CPU load"));
        assert!(goals_to_resolve[1].description.contains("deprioritized"));
        assert!(goals_to_resolve[1].priority < 60);
    }
}

fn main() {
    println!("\n--- Dynamic Goal Generation & Prioritization Demonstration ---");

    let mut goal_manager = GoalManager::new();

    // 1. Initial State: MaximizeUptime, EnsureSecurity, UserSatisfaction
    println!("\n--- Initial Directives: {:?} ---", goal_manager.overarching_directives);

    // 2. Simulate Environment and Internal State Changes
    println!("\n--- Simulating Dynamic Goal Generation ---");
    let env1 = EnvironmentState {
        cpu_load_pct: 88.0,
        network_latency_ms: 20,
        security_alert_level: 0,
        energy_cost_per_hour: 7.0,
    };
    let internal1 = InternalState {
        self_preservation_score: 0.95,
        knowledge_acquisition_rate: 0.8,
        resource_utilization_pct: 0.7,
    };

    let mut generated_goals = goal_manager.generate_new_goals(&env1, &internal1);
    println!("  Generated {} goals.", generated_goals.len());
    for goal in &generated_goals { println!("    - {}: (P:{}, V:{:.2})", goal.description, goal.priority, goal.value); }

    // 3. Prioritize Goals
    goal_manager.prioritize_goals(&mut generated_goals);
    println!("\n--- Prioritized Goals ---");
    for goal in &generated_goals { println!("    - {}: (P:{}, V:{:.2})", goal.description, goal.priority, goal.value); }

    // 4. Decompose a high-priority goal
    if let Some(high_priority_goal) = generated_goals.get(0) {
        println!("\n--- Decomposing High Priority Goal: {} ---", high_priority_goal.description);
        let sub_goals = goal_manager.decompose_goal(high_priority_goal);
        println!("  Generated {} sub-goals.", sub_goals.len());
        for goal in &sub_goals { println!("    - {}: (P:{}, V:{:.2})", goal.description, goal.priority, goal.value); }
        // Add sub-goals to generated_goals for further processing/integration
        generated_goals.extend(sub_goals);
    }

    // 5. Simulate a new directive and a conflicting goal
    goal_manager.overarching_directives.insert(OverarchingDirective::MinimizeCost);
    let env2 = EnvironmentState {
        cpu_load_pct: 70.0,
        network_latency_ms: 20,
        security_alert_level: 0,
        energy_cost_per_hour: 12.0, // High cost
    };
    let internal2 = internal1.clone();
    let mut new_goals_from_cost = goal_manager.generate_new_goals(&env2, &internal2);
    generated_goals.extend(new_goals_from_cost);

    // 6. Resolve Conflicts (and re-prioritize)
    println!("\n--- Resolving Conflicts & Re-Prioritizing All Goals ---");
    goal_manager.resolve_conflicts(&mut generated_goals);
    goal_manager.prioritize_goals(&mut generated_goals);
    for goal in &generated_goals { println!("    - {}: (P:{}, V:{:.2})", goal.description, goal.priority, goal.value); }

    // 7. Integrate goals into active set
    goal_manager.integrate_goals(generated_goals);
    println!("\n--- Final Active Goals in Manager ---");
    for goal in goal_manager.active_goals.values() {
        println!("    - {}: (P:{}, V:{:.2})", goal.description, goal.priority, goal.value);
    }

    println!("\nDynamic Goal Generation & Prioritization Demonstration Complete.");
}
