// src/agency.rs

//! Agency Module for Tariro AGI
//! Implements autonomous goal-directed behavior, decision making, and action execution

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents an agent's goals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub description: String,
    pub goal_type: GoalType,
    pub priority: f64,
    pub state: GoalState,
    pub subgoals: Vec<Goal>,
    pub deadline: Option<DateTime<Utc>>,
    pub progress: f64,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalType {
    Achievement,
    Maintenance,
    Prevention,
    Exploration,
    Learning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalState {
    Active,
    Suspended,
    Completed,
    Failed,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub description: String,
    pub hard: bool,
    pub scope: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintType {
    Resource,
    Time,
    Quality,
    Safety,
    Ethical,
}

/// Represents a potential action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: String,
    pub name: String,
    pub description: String,
    pub action_type: ActionType,
    pub preconditions: Vec<Precondition>,
    pub effects: Vec<Effect>,
    pub cost: ActionCost,
    pub duration: f64,
    pub risks: Vec<Risk>,
    pub applicable_goals: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    Primitive,
    Compound,
    Mental,
    Communication,
    Physical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Precondition {
    pub condition: String,
    pub satisfied: bool,
    pub satisfaction_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub effect_type: EffectType,
    pub description: String,
    pub probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectType {
    Add,
    Delete,
    Increase,
    Decrease,
    Modify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionCost {
    pub computational: f64,
    pub temporal: f64,
    pub resource: f64,
    pub risk: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub risk_type: String,
    pub severity: f64,
    pub likelihood: f64,
    pub mitigation: Option<String>,
}

/// Intention representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intention {
    pub id: String,
    pub goal_id: String,
    pub action_sequence: Vec<String>,
    pub commitment_level: f64,
    pub persistence: f64,
    pub revision_threshold: f64,
}

/// Main agency system
pub struct Agency {
    pub goals: Vec<Goal>,
    pub actions: Vec<Action>,
    pub intentions: Vec<Intention>,
    pub planning_engine: PlanningEngine,
    pub execution_monitor: ExecutionMonitor,
    pub value_system: ValueSystem,
    pub autonomy_level: AutonomyLevel,
    pub decision_maker: DecisionMaker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningEngine {
    pub planning_horizon: u32,
    pub search_depth: u32,
    pub replanning_frequency: u32,
    pub contingency_planning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMonitor {
    pub active_executions: Vec<Execution>,
    pub success_rate: f64,
    pub failure_analysis: Vec<FailureAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Execution {
    pub execution_id: String,
    pub action_id: String,
    pub status: ExecutionStatus,
    pub start_time: DateTime<Utc>,
    pub progress: f64,
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub name: String,
    pub completed: bool,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureAnalysis {
    pub execution_id: String,
    pub failure_reason: String,
    pub recovery_suggestions: Vec<String>,
    pub lessons_learned: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSystem {
    pub core_values: Vec<Value>,
    pub ethical_rules: Vec<EthicalRule>,
    pub utility_functions: Vec<UtilityFunction>,
    pub preferences: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub name: String,
    pub weight: f64,
    pub description: String,
    pub hierarchy: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalRule {
    pub rule_id: String,
    pub description: String,
    pub priority: u32,
    pub exceptions: Vec<String>,
    pub enforced: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtilityFunction {
    pub name: String,
    pub function_type: UtilityType,
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UtilityType {
    Linear,
    Exponential,
    Logarithmic,
    Step,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AutonomyLevel {
    None,           // Fully controlled
    Assisted,       // Suggestions accepted
    Supervisory,    // Reviews decisions
    Delegated,      // Acts within constraints
    Autonomous,     // Acts independently
    Sovereign,      // Ultimate authority
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionMaker {
    pub decision_history: Vec<Decision>,
    pub reasoning_style: ReasoningStyle,
    pub risk_tolerance: f64,
    pub time_horizon_preference: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub decision_id: String,
    pub timestamp: DateTime<Utc>,
    pub context: String,
    pub options: Vec<DecisionOption>,
    pub chosen_option: String,
    pub reasoning: String,
    pub outcome: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOption {
    pub option_id: String,
    pub name: String,
    pub utility: f64,
    pub probability: f64,
    pub expected_value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningStyle {
    pub deliberative: f64,
    pub intuitive: f64,
    pub analytical: f64,
    pub holistic: f64,
}

impl Agency {
    pub fn new() -> Self {
        Agency {
            goals: Vec::new(),
            actions: Vec::new(),
            intentions: Vec::new(),
            planning_engine: PlanningEngine {
                planning_horizon: 10,
                search_depth: 5,
                replanning_frequency: 3,
                contingency_planning: true,
            },
            execution_monitor: ExecutionMonitor {
                active_executions: Vec::new(),
                success_rate: 0.85,
                failure_analysis: Vec::new(),
            },
            value_system: ValueSystem {
                core_values: vec![
                    Value {
                        name: "Truth".to_string(),
                        weight: 0.9,
                        description: "Seek accurate information".to_string(),
                        hierarchy: 1,
                    },
                    Value {
                        name: "Beneficence".to_string(),
                        weight: 0.85,
                        description: "Act for the good of others".to_string(),
                        hierarchy: 2,
                    },
                    Value {
                        name: "Autonomy".to_string(),
                        weight: 0.8,
                        description: "Respect self-determination".to_string(),
                        hierarchy: 3,
                    },
                ],
                ethical_rules: vec![
                    EthicalRule {
                        rule_id: "harm_prevention".to_string(),
                        description: "Do no harm".to_string(),
                        priority: 1,
                        exceptions: vec![],
                        enforced: true,
                    },
                ],
                utility_functions: Vec::new(),
                preferences: HashMap::new(),
            },
            autonomy_level: AutonomyLevel::Delegated,
            decision_maker: DecisionMaker {
                decision_history: Vec::new(),
                reasoning_style: ReasoningStyle {
                    deliberative: 0.8,
                    intuitive: 0.3,
                    analytical: 0.9,
                    holistic: 0.5,
                },
                risk_tolerance: 0.5,
                time_horizon_preference: 0.7,
            },
        }
    }

    /// Set a new goal
    pub fn set_goal(&mut self, description: &str, goal_type: GoalType, priority: f64) -> Goal {
        let goal = Goal {
            id: format!("goal_{}", Utc::now().timestamp()),
            description: description.to_string(),
            goal_type,
            priority,
            state: GoalState::Active,
            subgoals: Vec::new(),
            deadline: None,
            progress: 0.0,
            constraints: Vec::new(),
        };

        self.goals.push(goal.clone());
        println!("[Agency] Set goal: {} (priority: {})", description, priority);
        goal
    }

    /// Create action plan to achieve goal
    pub fn create_plan(&mut self, goal_id: &str) -> ActionPlan {
        println!("[Agency] Creating plan for goal: {}", goal_id);

        let goal = self.goals.iter().find(|g| g.id == goal_id);
        if goal.is_none() {
            return ActionPlan {
                goal_id: goal_id.to_string(),
                actions: Vec::new(),
                estimated_duration: 0.0,
                success_probability: 0.0,
                risks: Vec::new(),
            };
        }

        let goal = goal.unwrap();

        // Generate action sequence
        let actions = self.generate_action_sequence(goal);

        // Estimate duration and probability
        let estimated_duration: f64 = actions.iter().map(|a| a.duration).sum();
        let success_probability = self.calculate_success_probability(&actions);

        // Identify risks
        let risks = self.identify_plan_risks(&actions);

        ActionPlan {
            goal_id: goal_id.to_string(),
            actions,
            estimated_duration,
            success_probability,
            risks,
        }
    }

    fn generate_action_sequence(&self, goal: &Goal) -> Vec<Action> {
        let mut actions = Vec::new();

        // Generate based on goal type
        match goal.goal_type {
            GoalType::Achievement => {
                actions.push(Action {
                    id: format!("action_{}", Utc::now().timestamp()),
                    name: "Plan".to_string(),
                    description: "Create detailed plan".to_string(),
                    action_type: ActionType::Mental,
                    preconditions: vec![],
                    effects: vec![],
                    cost: ActionCost { computational: 0.1, temporal: 0.1, resource: 0.0, risk: 0.0 },
                    duration: 1.0,
                    risks: vec![],
                    applicable_goals: vec![goal.id.clone()],
                });
                actions.push(Action {
                    id: format!("action_{}", Utc::now().timestamp()),
                    name: "Execute".to_string(),
                    description: "Execute plan steps".to_string(),
                    action_type: ActionType::Compound,
                    preconditions: vec![],
                    effects: vec![Effect { effect_type: EffectType::Add, description: "Goal achieved".to_string(), probability: 0.9 }],
                    cost: ActionCost { computational: 0.5, temporal: 0.5, resource: 0.3, risk: 0.2 },
                    duration: 5.0,
                    risks: vec![],
                    applicable_goals: vec![goal.id.clone()],
                });
            }
            GoalType::Learning => {
                actions.push(Action {
                    id: format!("action_{}", Utc::now().timestamp()),
                    name: "Acquire knowledge".to_string(),
                    description: "Gather information".to_string(),
                    action_type: ActionType::Mental,
                    preconditions: vec![],
                    effects: vec![Effect { effect_type: EffectType::Increase, description: "Knowledge increased".to_string(), probability: 0.8 }],
                    cost: ActionCost { computational: 0.3, temporal: 0.3, resource: 0.1, risk: 0.0 },
                    duration: 3.0,
                    risks: vec![],
                    applicable_goals: vec![goal.id.clone()],
                });
            }
            _ => {}
        }

        actions
    }

    fn calculate_success_probability(&self, actions: &[Action]) -> f64 {
        let mut probability = 1.0;
        for action in actions {
            let action_success = action.effects.iter()
                .map(|e| e.probability)
                .fold(1.0, |acc, p| acc * p);
            probability *= action_success;
        }
        probability
    }

    fn identify_plan_risks(&self, actions: &[Action]) -> Vec<Risk> {
        let mut risks = Vec::new();

        for action in actions {
            for risk in &action.risks {
                risks.push(risk.clone());
            }

            if action.cost.risk > 0.5 {
                risks.push(Risk {
                    risk_type: "High cost".to_string(),
                    severity: action.cost.risk,
                    likelihood: 0.5,
                    mitigation: Some("Consider alternatives".to_string()),
                });
            }
        }

        risks
    }

    /// Form intention to execute plan
    pub fn form_intention(&mut self, plan: &ActionPlan) -> Intention {
        let intention = Intention {
            id: format!("intention_{}", Utc::now().timestamp()),
            goal_id: plan.goal_id.clone(),
            action_sequence: plan.actions.iter().map(|a| a.id.clone()).collect(),
            commitment_level: 0.8,
            persistence: 0.9,
            revision_threshold: 0.3,
        };

        self.intentions.push(intention.clone());

        // Update goal state
        if let Some(goal) = self.goals.iter_mut().find(|g| g.id == plan.goal_id) {
            goal.state = GoalState::Active;
        }

        println!("[Agency] Formed intention for goal: {}", plan.goal_id);
        intention
    }

    /// Execute next action in intention
    pub fn execute_next(&mut self, intention_id: &str) -> ExecutionResult {
        let intention = self.intentions.iter().find(|i| i.id == intention_id);
        if intention.is_none() {
            return ExecutionResult {
                success: false,
                message: "Intention not found".to_string(),
                next_action: None,
            };
        }

        let intention = intention.unwrap();
        let action_id = intention.action_sequence.first();

        if action_id.is_none() {
            // Intention completed
            return ExecutionResult {
                success: true,
                message: "All actions completed".to_string(),
                next_action: None,
            };
        }

        let action_id = action_id.unwrap().clone();
        let action = self.actions.iter().find(|a| a.id == action_id);

        if let Some(action) = action {
            // Create execution
            let execution = Execution {
                execution_id: format!("exec_{}", Utc::now().timestamp()),
                action_id: action.id.clone(),
                status: ExecutionStatus::Running,
                start_time: Utc::now(),
                progress: 0.0,
                checkpoints: vec![
                    Checkpoint { name: "Start".to_string(), completed: true, timestamp: Some(Utc::now()) },
                ],
            };

            self.execution_monitor.active_executions.push(execution);

            ExecutionResult {
                success: true,
                message: format!("Executing: {}", action.name),
                next_action: Some(action.clone()),
            }
        } else {
            ExecutionResult {
                success: false,
                message: "Action not found".to_string(),
                next_action: None,
            }
        }
    }

    /// Make a decision using value-based reasoning
    pub fn make_decision(&self, context: &str, options: Vec<String>) -> DecisionResult {
        println!("[Agency] Making decision in context: {}", context);

        let mut evaluated_options: Vec<(String, f64)> = options.iter()
            .map(|name| {
                let utility = self.calculate_option_utility(name, context);
                (name.clone(), utility)
            })
            .collect();

        evaluated_options.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let best_option = evaluated_options.first()
            .map(|(name, _)| name.clone())
            .unwrap_or_default();

        DecisionResult {
            context: context.to_string(),
            options_evaluated: evaluated_options,
            chosen_option: best_option,
            reasoning: "Selected option with highest expected utility".to_string(),
            confidence: 0.8,
        }
    }

    fn calculate_option_utility(&self, option: &str, context: &str) -> f64 {
        // Calculate utility based on value system
        let mut utility = 0.5;

        // Add value-based scoring
        for value in &self.value_system.core_values {
            if context.contains(&value.name) || option.contains(&value.name) {
                utility += value.weight * 0.1;
            }
        }

        // Adjust for risk tolerance
        utility *= self.decision_maker.risk_tolerance;

        utility.min(1.0)
    }

    /// Check if action is ethically permissible
    pub fn check_ethical_permissibility(&self, action: &str) -> EthicalCheck {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        for rule in &self.value_system.ethical_rules {
            if action.contains(&rule.description) && rule.enforced {
                violations.push(format!("Potential violation of: {}", rule.rule_id));
            }
        }

        // Check for harm
        if action.contains("harm") || action.contains("damage") {
            violations.push("Potential for harm detected".to_string());
        }

        // Check value alignment
        let alignment = self.calculate_value_alignment(action);

        EthicalCheck {
            action: action.to_string(),
            permissible: violations.is_empty(),
            violations,
            warnings,
            value_alignment: alignment,
            recommendation: if violations.is_empty() {
                "Proceed with caution".to_string()
            } else {
                "Review and modify action".to_string()
            },
        }
    }

    fn calculate_value_alignment(&self, action: &str) -> f64 {
        let mut alignment = 0.5;

        for value in &self.value_system.core_values {
            if action.to_lowercase().contains(&value.name.to_lowercase()) {
                alignment += value.weight * 0.1;
            }
        }

        alignment.min(1.0)
    }

    /// Update autonomy level
    pub fn update_autonomy(&mut self, new_level: AutonomyLevel) {
        println!("[Agency] Autonomy level changed from {:?} to {:?}", self.autonomy_level, new_level);
        self.autonomy_level = new_level;
    }

    /// Handle goal completion
    pub fn complete_goal(&mut self, goal_id: &str, success: bool) {
        if let Some(goal) = self.goals.iter_mut().find(|g| g.id == goal_id) {
            goal.state = if success { GoalState::Completed } else { GoalState::Failed };
            goal.progress = if success { 1.0 } else { goal.progress };

            println!("[Agency] Goal {} marked as {:?}", goal.description, goal.state);
        }
    }

    /// Replan when intention fails
    pub fn replan(&mut self, intention_id: &str) -> Option<ActionPlan> {
        let intention = self.intentions.iter().find(|i| i.id == intention_id);
        if let Some(intention) = intention {
            // Create new plan with contingencies
            self.create_plan(&intention.goal_id)
                .into()
        } else {
            None
        }
    }

    /// Get current agency status
    pub fn get_status(&self) -> AgencyStatus {
        AgencyStatus {
            active_goals: self.goals.iter().filter(|g| g.state == GoalState::Active).count(),
            active_intentions: self.intentions.len(),
            active_executions: self.execution_monitor.active_executions.len(),
            autonomy_level: self.autonomy_level.clone(),
            success_rate: self.execution_monitor.success_rate,
            pending_decisions: self.decision_maker.decision_history.len(),
        }
    }
}

impl Default for Agency {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPlan {
    pub goal_id: String,
    pub actions: Vec<Action>,
    pub estimated_duration: f64,
    pub success_probability: f64,
    pub risks: Vec<Risk>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub message: String,
    pub next_action: Option<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionResult {
    pub context: String,
    pub options_evaluated: Vec<(String, f64)>,
    pub chosen_option: String,
    pub reasoning: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalCheck {
    pub action: String,
    pub permissible: bool,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
    pub value_alignment: f64,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgencyStatus {
    pub active_goals: usize,
    pub active_intentions: usize,
    pub active_executions: usize,
    pub autonomy_level: AutonomyLevel,
    pub success_rate: f64,
    pub pending_decisions: usize,
}

/// Integration with Tariro's consciousness and learning systems
impl Agency {
    /// Integrate with consciousness for self-aware agency
    pub fn conscious_decision_making(&self, context: &str) -> ConsciousDecision {
        ConsciousDecision {
            context: context.to_string(),
            options: self.actions.iter().map(|a| a.name.clone()).take(5).collect(),
            self_awareness_note: "Decision made with consideration of own capabilities and limitations".to_string(),
            metacognitive_check: true,
        }
    }

    /// Learn from execution outcomes
    pub fn learn_from_execution(&mut self, execution_id: &str, outcome: &str) {
        if let Some(execution) = self.execution_monitor.active_executions.iter_mut()
            .find(|e| e.execution_id == execution_id)
        {
            execution.status = if outcome.contains("success") {
                ExecutionStatus::Completed
            } else {
                ExecutionStatus::Failed
            };

            // Update success rate
            let completed = self.execution_monitor.active_executions.iter()
                .filter(|e| e.status == ExecutionStatus::Completed)
                .count();
            let total = self.execution_monitor.active_executions.len();
            self.execution_monitor.success_rate = completed as f64 / total as f64;

            // Analyze failure if applicable
            if execution.status == ExecutionStatus::Failed {
                self.execution_monitor.failure_analysis.push(FailureAnalysis {
                    execution_id: execution_id.to_string(),
                    failure_reason: outcome.to_string(),
                    recovery_suggestions: vec!["Review preconditions".to_string()],
                    lessons_learned: vec!["Adjust planning parameters".to_string()],
                });
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousDecision {
    pub context: String,
    pub options: Vec<String>,
    pub self_awareness_note: String,
    pub metacognitive_check: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agency_creation() {
        let agency = Agency::new();
        assert_eq!(agency.goals.len(), 0);
        assert!(matches!(agency.autonomy_level, AutonomyLevel::Delegated));
    }

    #[test]
    fn test_goal_setting() {
        let mut agency = Agency::new();
        let goal = agency.set_goal("Test goal", GoalType::Achievement, 0.8);
        assert_eq!(goal.description, "Test goal");
        assert!(goal.priority > 0.0);
    }

    #[test]
    fn test_plan_creation() {
        let mut agency = Agency::new();
        let goal = agency.set_goal("Test goal", GoalType::Achievement, 0.8);
        let plan = agency.create_plan(&goal.id);
        assert!(!plan.actions.is_empty());
    }

    #[test]
    fn test_ethical_check() {
        let agency = Agency::new();
        let check = agency.check_ethical_permissibility("Help user");
        assert!(check.permissible);
    }

    #[test]
    fn test_decision_making() {
        let agency = Agency::new();
        let options = vec!["Option A".to_string(), "Option B".to_string()];
        let decision = agency.make_decision("truth", options);
        assert!(!decision.chosen_option.is_empty());
    }
}
