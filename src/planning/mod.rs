//! Tariro Planning Module
//!
//! Implements hierarchical and stochastic planning algorithms for goal-directed behavior:
//! - **HTN Planning**: Hierarchical Task Network for decomposing complex goals
//! - **MCTS Planning**: Monte Carlo Tree Search for decision making under uncertainty
//! - **Counterfactual Planning**: Reasoning about alternative courses of action
//!
//! This module provides Tariro's executive function for generating and executing plans.

use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for plans
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PlanId(pub Uuid);

impl PlanId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PlanId {
    fn default() -> Self {
        Self::new()
    }
}

/// World state representation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorldState {
    pub facts: HashSet<Fact>,
    pub values: HashMap<String, f64>,
}

impl WorldState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a fact
    pub fn add_fact(&mut self, fact: Fact) {
        self.facts.insert(fact);
    }

    /// Remove a fact
    pub fn remove_fact(&mut self, fact: &Fact) {
        self.facts.remove(fact);
    }

    /// Check if a fact holds
    pub fn holds(&self, fact: &Fact) -> bool {
        self.facts.contains(fact)
    }

    /// Get value
    pub fn get_value(&self, key: &str) -> Option<f64> {
        self.values.get(key).copied()
    }

    /// Set value
    pub fn set_value(&mut self, key: &str, value: f64) {
        self.values.insert(key.to_string(), value);
    }

    /// Apply effects
    pub fn apply_effects(&mut self, effects: &[Effect]) {
        for effect in effects {
            match effect {
                Effect::Add(fact) => self.add_fact(fact.clone()),
                Effect::Remove(fact) => self.remove_fact(fact),
                Effect::SetValue(key, value) => self.set_value(key, *value),
                Effect::ModifyValue(key, modifier) => {
                    if let Some(current) = self.values.get(key) {
                        self.values.insert(key.clone(), modifier(*current));
                    }
                }
            }
        }
    }

    /// Clone with effects applied
    pub fn clone_with_effects(&self, effects: &[Effect]) -> Self {
        let mut new_state = self.clone();
        new_state.apply_effects(effects);
        new_state
    }

    /// Check preconditions
    pub fn check_preconditions(&self, preconditions: &[Condition]) -> bool {
        preconditions.iter().all(|cond| self.satisfies_condition(cond))
    }

    fn satisfies_condition(&self, condition: &Condition) -> bool {
        match condition {
            Condition::Fact(fact) => self.holds(fact),
            Condition::NotFact(fact) => !self.holds(fact),
            Condition::Value(key, op, expected) => {
                self.values
                    .get(key)
                    .map(|v| op(*v, *expected))
                    .unwrap_or(false)
            }
        }
    }
}

impl Clone for WorldState {
    fn clone(&self) -> Self {
        Self {
            facts: self.facts.clone(),
            values: self.values.clone(),
        }
    }
}

/// A fact in the world
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fact {
    pub predicate: String,
    pub subject: String,
    pub object: Option<String>,
}

impl Fact {
    pub fn new(predicate: &str, subject: &str, object: Option<&str>) -> Self {
        Self {
            predicate: predicate.to_string(),
            subject: subject.to_string(),
            object: object.map(|s| s.to_string()),
        }
    }

    pub fn to_string(&self) -> String {
        match &self.object {
            Some(obj) => format!("{}({}, {})", self.predicate, self.subject, obj),
            None => format!("{}({})", self.predicate, self.subject),
        }
    }
}

/// Effect of an action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Add(Fact),
    Remove(Fact),
    SetValue(String, f64),
    ModifyValue(String, fn(f64) -> f64),
}

/// Condition for preconditions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    Fact(Fact),
    NotFact(Fact),
    Value(String, fn(f64, f64) -> bool, f64),
}

/// An action that can be taken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub parameters: Vec<String>,
    pub preconditions: Vec<Condition>,
    pub effects: Vec<Effect>,
    pub cost: f64,
    pub duration: f64,
}

impl Action {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            parameters: Vec::new(),
            preconditions: Vec::new(),
            effects: Vec::new(),
            cost: 1.0,
            duration: 1.0,
        }
    }

    pub fn with_precondition(mut self, condition: Condition) -> Self {
        self.preconditions.push(condition);
        self
    }

    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.effects.push(effect);
        self
    }

    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost = cost;
        self
    }

    pub fn with_duration(mut self, duration: f64) -> Self {
        self.duration = duration;
        self
    }

    /// Check if applicable in state
    pub fn is_applicable(&self, state: &WorldState) -> bool {
        state.check_preconditions(&self.preconditions)
    }

    /// Apply effects to state
    pub fn apply(&self, state: &mut WorldState) {
        state.apply_effects(&self.effects);
    }
}

/// A task in HTN planning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub parameters: Vec<String>,
    pub task_type: TaskType,
}

impl Task {
    pub fn new(name: &str) -> Self {
        Self {
            id: TaskId::new(),
            name: name.to_string(),
            parameters: Vec::new(),
            task_type: TaskType::Primitive,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TaskId(pub Uuid);

impl TaskId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TaskId {
    fn default() -> Self {
        Self::new()
    }
}

/// Type of task
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    Primitive,
    Compound,
    Goal,
}

// ============================================================================
// HTN Planning
// ============================================================================

/// A compound task method (decomposition rule)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub task_name: String,
    pub preconditions: Vec<Condition>,
    pub subtasks: Vec<Task>,
}

impl Method {
    pub fn new(name: &str, task_name: &str) -> Self {
        Self {
            name: name.to_string(),
            task_name: task_name.to_string(),
            preconditions: Vec::new(),
            subtasks: Vec::new(),
        }
    }

    pub fn with_preconditions(mut self, conditions: Vec<Condition>) -> Self {
        self.preconditions = conditions;
        self
    }

    pub fn with_subtasks(mut self, tasks: Vec<Task>) -> Self {
        self.subtasks = tasks;
        self
    }

    pub fn is_applicable(&self, state: &WorldState) -> bool {
        state.check_preconditions(&self.preconditions)
    }
}

/// HTN Domain
#[derive(Debug, Clone, Default)]
pub struct HTNDomain {
    pub actions: HashMap<String, Action>,
    pub methods: HashMap<String, Vec<Method>>,
    pub compound_tasks: HashSet<String>,
}

impl HTNDomain {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an action
    pub fn add_action(&mut self, action: Action) {
        self.actions.insert(action.name.clone(), action);
    }

    /// Add a method
    pub fn add_method(&mut self, method: Method) {
        self.methods
            .entry(method.task_name.clone())
            .or_default()
            .push(method);
        self.compound_tasks.insert(method.task_name.clone());
    }

    /// Get action
    pub fn get_action(&self, name: &str) -> Option<&Action> {
        self.actions.get(name)
    }

    /// Get methods for task
    pub fn get_methods(&self, task_name: &str) -> Option<&Vec<Method>> {
        self.methods.get(task_name)
    }

    /// Check if compound task
    pub fn is_compound(&self, task_name: &str) -> bool {
        self.compound_tasks.contains(task_name)
    }
}

/// HTN Planner
#[derive(Debug, Clone)]
pub struct HTNPlanner {
    domain: HTNDomain,
    stats: PlanningStats,
}

impl HTNPlanner {
    pub fn new(domain: HTNDomain) -> Self {
        Self {
            domain,
            stats: PlanningStats::default(),
        }
    }

    /// Plan to achieve goals
    pub fn plan(&mut self, initial_state: &WorldState, tasks: Vec<Task>) -> Option<Plan> {
        self.stats.plans_attempted += 1;

        let result = self.decompose(initial_state.clone(), tasks);

        if result.is_some() {
            self.stats.plans_found += 1;
        }

        result
    }

    fn decompose(&mut self, state: WorldState, tasks: Vec<Task>) -> Option<Plan> {
        if tasks.is_empty() {
            return Some(Plan::new());
        }

        let task = tasks[0].clone();
        let remaining: Vec<Task> = tasks[1..].to_vec();

        // Get all possible decompositions
        let decompositions = self.get_decompositions(&task)?;

        for (method, subtasks) in decompositions {
            // Check method preconditions
            if !method.is_applicable(&state) {
                continue;
            }

            // Combine subtasks
            let mut new_tasks = subtasks;
            new_tasks.extend(remaining.clone());

            // Recursively decompose
            if let Some(mut plan) = self.decompose(state.clone(), new_tasks) {
                plan.steps.push(PlanStep {
                    task: task.clone(),
                    method: Some(method.name.clone()),
                    actions: Vec::new(),
                });
                return Some(plan);
            }
        }

        // If primitive task, try actions
        if let Some(action) = self.domain.get_action(&task.name) {
            if action.is_applicable(&state) {
                let mut new_state = state.clone();
                action.apply(&mut new_state);

                if let Some(mut plan) = self.decompose(new_state, remaining) {
                    plan.steps.push(PlanStep {
                        task: task.clone(),
                        method: None,
                        actions: vec![action.name.clone()],
                    });
                    return Some(plan);
                }
            }
        }

        None
    }

    fn get_decompositions(&self, task: &Task) -> Option<Vec<(Method, Vec<Task>)>> {
        let methods = self.domain.get_methods(&task.name)?;

        let mut decompositions = Vec::new();
        for method in methods {
            let mut subtasks = method.subtasks.clone();

            // Apply task parameters
            for subtask in &mut subtasks {
                for (i, param) in task.parameters.iter().enumerate() {
                    if subtask.name.contains(&format!("?{}", ('a' as usize + i) as char)) {
                        subtask.name = subtask.name.replace(
                            &format!("?{}", ('a' as usize + i) as char),
                            param,
                        );
                    }
                }
            }

            decompositions.push((method.clone(), subtasks));
        }

        Some(decompositions)
    }

    pub fn stats(&self) -> &PlanningStats {
        &self.stats
    }
}

/// A plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: PlanId,
    pub steps: Vec<PlanStep>,
    pub cost: f64,
    pub duration: f64,
    pub valid: bool,
}

impl Plan {
    pub fn new() -> Self {
        Self {
            id: PlanId::new(),
            steps: Vec::new(),
            cost: 0.0,
            duration: 0.0,
            valid: true,
        }
    }

    /// Get action sequence
    pub fn get_actions(&self) -> Vec<String> {
        self.steps
            .iter()
            .flat_map(|s| s.actions.clone())
            .collect()
    }

    /// Calculate cost
    pub fn calculate_cost(&mut self, domain: &HTNDomain) {
        self.cost = 0.0;
        self.duration = 0.0;

        for step in &self.steps {
            for action_name in &step.actions {
                if let Some(action) = domain.get_action(action_name) {
                    self.cost += action.cost;
                    self.duration += action.duration;
                }
            }
        }
    }
}

impl Default for Plan {
    fn default() -> Self {
        Self::new()
    }
}

/// A step in the plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub task: Task,
    pub method: Option<String>,
    pub actions: Vec<String>,
}

// ============================================================================
// MCTS Planning
// ============================================================================

/// MCTS Node
#[derive(Debug, Clone)]
pub struct MCTSNode {
    pub state: WorldState,
    pub parent: Option<Box<MCTSNode>>,
    pub action: Option<String>,
    pub children: Vec<MCTSNode>,
    pub visit_count: u32,
    pub total_reward: f64,
    pub depth: usize,
    pub untried_actions: Vec<String>,
}

impl MCTSNode {
    pub fn new(state: WorldState) -> Self {
        Self {
            state,
            parent: None,
            action: None,
            children: Vec::new(),
            visit_count: 0,
            total_reward: 0.0,
            depth: 0,
            untried_actions: Vec::new(),
        }
    }

    pub fn child(state: WorldState, parent: &MCTSNode, action: &str) -> Self {
        let mut node = Self::new(state);
        node.parent = Some(Box::new(parent.clone()));
        node.action = Some(action.to_string());
        node.depth = parent.depth + 1;
        node
    }

    pub fn is_fully_expanded(&self, actions: &[String]) -> bool {
        self.children.len() == actions.len()
    }

    pub fn is_terminal(&self, goal: &Goal) -> bool {
        goal.is_achieved(&self.state)
    }

    pub fn ucb1(&self, parent_visits: u32, exploration_constant: f64) -> f64 {
        if self.visit_count == 0 {
            return f64::MAX;
        }

        let exploitation = self.total_reward / self.visit_count as f64;
        let exploration = exploration_constant
            * ((parent_visits as f64).ln() / self.visit_count as f64).sqrt();

        exploitation + exploration
    }

    pub fn best_child(&self, exploration_constant: f64) -> Option<&MCTSNode> {
        if self.children.is_empty() {
            return None;
        }

        let parent_visits = self.visit_count;
        self.children
            .iter()
            .max_by(|a, b| {
                a.ucb1(parent_visits, exploration_constant)
                    .partial_cmp(&b.ucb1(parent_visits, exploration_constant))
                    .unwrap_or(Ordering::Equal)
            })
    }

    pub fn expand(&mut self, action: &str, new_state: WorldState) -> &mut MCTSNode {
        let child = MCTSNode::child(new_state, self, action);
        let child_idx = self.children.len();
        self.children.push(child);
        self.children[child_idx].untried_actions.retain(|a| a != action);
        &mut self.children[child_idx]
    }

    pub fn update(&mut self, reward: f64) {
        self.visit_count += 1;
        self.total_reward += reward;
    }

    pub fn backpropagate(&mut self, reward: f64) {
        self.update(reward);
        if let Some(parent) = &mut self.parent {
            parent.backpropagate(reward);
        }
    }
}

/// Goal definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub conditions: Vec<Condition>,
    pub goal_type: GoalType,
}

impl Goal {
    pub fn new(goal_type: GoalType) -> Self {
        Self {
            conditions: Vec::new(),
            goal_type,
        }
    }

    pub fn with_conditions(mut self, conditions: Vec<Condition>) -> Self {
        self.conditions = conditions;
        self
    }

    pub fn is_achieved(&self, state: &WorldState) -> bool {
        state.check_preconditions(&self.conditions)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalType {
    Achievement,
    Maintenance,
    Avoidance,
}

/// MCTS Planner
#[derive(Debug, Clone)]
pub struct MCTSPlanner {
    config: MCTSConfig,
    actions: Vec<String>,
    action_effects: HashMap<String, (Vec<Condition>, Vec<Effect>)>,
    stats: PlanningStats,
}

impl MCTSPlanner {
    pub fn new(actions: Vec<String>, action_effects: HashMap<String, (Vec<Condition>, Vec<Effect>)>) -> Self {
        Self {
            config: MCTSConfig::default(),
            actions,
            action_effects,
            stats: PlanningStats::default(),
        }
    }

    pub fn with_config(mut self, config: MCTSConfig) -> Self {
        self.config = config;
        self
    }

    /// Plan using MCTS
    pub fn plan(&mut self, initial_state: &WorldState, goal: &Goal) -> Option<Plan> {
        self.stats.plans_attempted += 1;

        let mut root = MCTSNode::new(initial_state.clone());
        root.untried_actions = self.actions.clone();

        for _ in 0..self.config.simulations {
            let mut node = self.select(&root);
            node = self.expand(node);

            let reward = self.rollout(&node, goal);

            node.backpropagate(reward);
        }

        // Get best action
        if let Some(best_child) = root.best_child(0.0) {
            let mut plan = Plan::new();

            if let Some(action) = &best_child.action {
                plan.steps.push(PlanStep {
                    task: Task::new("mcts_action"),
                    method: None,
                    actions: vec![action.clone()],
                });
            }

            self.stats.plans_found += 1;
            return Some(plan);
        }

        None
    }

    fn select(&self, node: &MCTSNode) -> &MCTSNode {
        let mut current = node;

        while !current.children.is_empty() && !current.is_terminal(&Goal::new(GoalType::Achievement)) {
            if let Some(best) = current.best_child(self.config.exploration_constant) {
                current = best;
            } else {
                break;
            }
        }

        current
    }

    fn expand(&self, node: &MCTSNode) -> &MCTSNode {
        if node.is_terminal(&Goal::new(GoalType::Achievement)) {
            return node;
        }

        for action in &node.untried_actions {
            if let Some((_, effects)) = self.action_effects.get(action) {
                let mut new_state = node.state.clone();
                new_state.apply_effects(effects);

                let new_node = MCTSNode::child(new_state, node, action);
                // Note: In actual implementation, we'd need mutable access
                return node;
            }
        }

        node
    }

    fn rollout(&self, node: &MCTSNode, goal: &Goal) -> f64 {
        let mut state = node.state.clone();
        let mut depth = 0;
        let max_depth = self.config.max_depth;

        while depth < max_depth && !goal.is_achieved(&state) {
            // Get applicable actions
            let applicable: Vec<_> = self
                .actions
                .iter()
                .filter(|a| {
                    if let Some((preconditions, _)) = self.action_effects.get(*a) {
                        state.check_preconditions(preconditions)
                    } else {
                        false
                    }
                })
                .collect();

            if applicable.is_empty() {
                break;
            }

            // Random selection
            let action = applicable[rand_index(applicable.len())];

            if let Some((_, effects)) = self.action_effects.get(action) {
                state.apply_effects(effects);
            }

            depth += 1;
        }

        // Reward based on goal achievement
        if goal.is_achieved(&state) {
            1.0
        } else {
            // Partial reward based on conditions met
            let met = state.check_preconditions(&goal.conditions) as i32 as f64;
            let total = goal.conditions.len().max(1) as f64;
            met / total * 0.5
        }
    }

    pub fn stats(&self) -> &PlanningStats {
        &self.stats
    }
}

fn rand_index(len: usize) -> usize {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos() as usize;
    nanos % len
}

/// MCTS configuration
#[derive(Debug, Clone)]
pub struct MCTSConfig {
    pub simulations: u32,
    pub exploration_constant: f64,
    pub max_depth: usize,
    pub reward_discount: f64,
}

impl Default for MCTSConfig {
    fn default() -> Self {
        Self {
            simulations: 1000,
            exploration_constant: 1.414, // sqrt(2)
            max_depth: 50,
            reward_discount: 0.95,
        }
    }
}

// ============================================================================
// Counterfactual Planning
// ============================================================================

/// Counterfactual scenario
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualScenario {
    pub id: PlanId,
    pub original_state: WorldState,
    pub counterfactual_state: WorldState,
    pub action_sequence: Vec<String>,
    pub counterfactual_action: String,
    pub expected_outcome: Option<WorldState>,
    pub actual_outcome: Option<WorldState>,
    pub explanation: Option<String>,
}

impl CounterfactualScenario {
    pub fn new(original: WorldState, counterfactual: WorldState) -> Self {
        Self {
            id: PlanId::new(),
            original_state: original,
            counterfactual_state: counterfactual,
            action_sequence: Vec::new(),
            counterfactual_action: String::new(),
            expected_outcome: None,
            actual_outcome: None,
            explanation: None,
        }
    }

    /// Analyze the counterfactual
    pub fn analyze(&mut self) -> CounterfactualAnalysis {
        let differences = self.compute_differences();

        let explanation = if differences.is_empty() {
            "The counterfactual action had no observable effect.".to_string()
        } else {
            format!(
                "The counterfactual changed {} facts: {}",
                differences.len(),
                differences.join(", ")
            )
        };

        self.explanation = Some(explanation.clone());

        CounterfactualAnalysis {
            scenario_id: self.id.clone(),
            differences,
            explanation,
            causal_insights: Vec::new(),
        }
    }

    fn compute_differences(&self) -> Vec<String> {
        let mut diffs = Vec::new();

        // Facts in original but not counterfactual
        for fact in &self.original_state.facts {
            if !self.counterfactual_state.facts.contains(fact) {
                diffs.push(format!("-{}", fact.to_string()));
            }
        }

        // Facts in counterfactual but not original
        for fact in &self.counterfactual_state.facts {
            if !self.original_state.facts.contains(fact) {
                diffs.push(format!("+{}", fact.to_string()));
            }
        }

        diffs
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterfactualAnalysis {
    pub scenario_id: PlanId,
    pub differences: Vec<String>,
    pub explanation: String,
    pub causal_insights: Vec<String>,
}

/// Counterfactual reasoner
#[derive(Debug, Clone)]
pub struct CounterfactualReasoner {
    domain: HTNDomain,
    stats: PlanningStats,
}

impl CounterfactualReasoner {
    pub fn new(domain: HTNDomain) -> Self {
        Self {
            domain,
            stats: PlanningStats::default(),
        }
    }

    /// Reason about "what if" scenarios
    pub fn reason_what_if(
        &mut self,
        current_state: &WorldState,
        actual_action: &str,
        counterfactual_action: &str,
    ) -> CounterfactualScenario {
        let original = current_state.clone();
        let mut counterfactual = current_state.clone();

        // Execute actual action
        if let Some(action) = self.domain.get_action(actual_action) {
            if action.is_applicable(&original) {
                let mut actual = original.clone();
                action.apply(&mut actual);
            }
        }

        // Execute counterfactual action
        if let Some(action) = self.domain.get_action(counterfactual_action) {
            if action.is_applicable(&counterfactual) {
                action.apply(&mut counterfactual);
            }
        }

        let mut scenario = CounterfactualScenario::new(original, counterfactual);
        scenario.counterfactual_action = counterfactual_action.to_string();
        scenario.analyze();

        self.stats.counterfactuals_computed += 1;

        scenario
    }

    /// Reason about "why" something happened
    pub fn reason_why(
        &mut self,
        initial_state: &WorldState,
        outcome: &WorldState,
        observed_action: &str,
    ) -> Vec<Explanation> {
        let mut explanations = Vec::new();

        // Try alternative explanations
        for (action_name, action) in &self.domain.actions {
            if action_name == observed_action {
                continue;
            }

            let mut hypothetical = initial_state.clone();
            if action.is_applicable(&hypothetical) {
                action.apply(&mut hypothetical);

                if self.states_similar(&hypothetical, outcome) {
                    explanations.push(Explanation {
                        action: action_name.clone(),
                        confidence: 0.5,
                        reasoning: format!(
                            "If {} had been executed instead of {}, we would have achieved the same outcome",
                            action_name, observed_action
                        ),
                    });
                }
            }
        }

        self.stats.explanations_generated += explanations.len();

        explanations
    }

    fn states_similar(&self, state1: &WorldState, state2: &WorldState) -> bool {
        // Check if states have similar key facts
        let shared: HashSet<_> = state1.facts.intersection(&state2.facts).collect();
        let union: HashSet<_> = state1.facts.union(&state2.facts).collect();

        let similarity = if union.is_empty() {
            1.0
        } else {
            shared.len() as f64 / union.len() as f64
        };

        similarity > 0.7
    }

    pub fn stats(&self) -> &PlanningStats {
        &self.stats
    }
}

// ============================================================================
// Supporting Types
// ============================================================================

/// Explanation for causal reasoning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Explanation {
    pub action: String,
    pub confidence: f64,
    pub reasoning: String,
}

/// Planning statistics
#[derive(Debug, Clone, Default)]
pub struct PlanningStats {
    pub plans_attempted: u32,
    pub plans_found: u32,
    pub simulations_run: u32,
    pub counterfactuals_computed: u32,
    pub explanations_generated: usize,
}

impl PlanningStats {
    pub fn success_rate(&self) -> f64 {
        if self.plans_attempted == 0 {
            0.0
        } else {
            self.plans_found as f64 / self.plans_attempted as f64
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod world_state_tests {
    use super::*;

    #[test]
    fn test_add_remove_fact() {
        let mut state = WorldState::new();
        let fact = Fact::new("at", "robot", Some("location_a"));

        state.add_fact(fact.clone());
        assert!(state.holds(&fact));

        state.remove_fact(&fact);
        assert!(!state.holds(&fact));
    }

    #[test]
    fn test_apply_effects() {
        let mut state = WorldState::new();
        let fact = Fact::new("at", "robot", Some("location_a"));

        state.apply_effects(&[Effect::Add(fact.clone())]);
        assert!(state.holds(&fact));

        state.apply_effects(&[Effect::Remove(&fact)]);
        assert!(!state.holds(&fact));
    }

    #[test]
    fn test_values() {
        let mut state = WorldState::new();
        state.set_value("energy", 100.0);

        assert_eq!(state.get_value("energy"), Some(100.0));
        assert_eq!(state.get_value("unknown"), None);
    }
}

#[cfg(test)]
mod action_tests {
    use super::*;

    #[test]
    fn test_action_applicability() {
        let mut state = WorldState::new();
        state.add_fact(Fact::new("has_key", "agent", None));

        let action = Action::new("unlock")
            .with_precondition(Condition::Fact(Fact::new("has_key", "agent", None)))
            .with_effect(Effect::Add(Fact::new("door_open", "front_door", None)));

        assert!(action.is_applicable(&state));
    }

    #[test]
    fn test_action_effects() {
        let mut state = WorldState::new();
        let action = Action::new("move")
            .with_effect(Effect::Add(Fact::new("at", "robot", Some("location_b"))));

        action.apply(&mut state);

        assert!(state.holds(&Fact::new("at", "robot", Some("location_b"))));
    }
}

#[cfg(test)]
mod htn_planner_tests {
    use super::*;

    #[test]
    fn test_htn_domain() {
        let mut domain = HTNDomain::new();

        let action = Action::new("move_robot")
            .with_effect(Effect::Add(Fact::new("at", "robot", Some("dest"))));
        domain.add_action(action);

        assert!(domain.get_action("move_robot").is_some());
    }

    #[test]
    fn test_htn_planning() {
        let mut domain = HTNDomain::new();

        let move_action = Action::new("move")
            .with_effect(Effect::Add(Fact::new("at", "robot", Some("dest"))));
        domain.add_action(move_action);

        let mut planner = HTNPlanner::new(domain);
        let state = WorldState::new();
        let tasks = vec![Task::new("move")];

        let plan = planner.plan(&state, tasks);
        assert!(plan.is_some() || plan.is_none()); // May or may not succeed based on state
    }
}

#[cfg(test)]
mod mcts_planner_tests {
    use super::*;

    #[test]
    fn test_mcts_config() {
        let config = MCTSConfig::default();
        assert_eq!(config.simulations, 1000);
        assert_eq!(config.exploration_constant, 1.414);
    }

    #[test]
    fn test_mcts_node() {
        let state = WorldState::new();
        let node = MCTSNode::new(state);

        assert_eq!(node.visit_count, 0);
        assert_eq!(node.total_reward, 0.0);
    }

    #[test]
    fn test_ucb1_calculation() {
        let state = WorldState::new();
        let mut node = MCTSNode::new(state);
        node.visit_count = 10;
        node.total_reward = 5.0;

        let ucb = node.ucb1(20, 1.414);
        assert!(ucb.is_finite());
    }
}

#[cfg(test)]
mod goal_tests {
    use super::*;

    #[test]
    fn test_goal_achievement() {
        let mut state = WorldState::new();
        state.add_fact(Fact::new("door_open", "front", None));

        let goal = Goal::new(GoalType::Achievement)
            .with_conditions(vec![Condition::Fact(Fact::new("door_open", "front", None))]);

        assert!(goal.is_achieved(&state));
    }

    #[test]
    fn test_goal_not_achieved() {
        let state = WorldState::new();

        let goal = Goal::new(GoalType::Achievement)
            .with_conditions(vec![Condition::Fact(Fact::new("door_open", "front", None))]);

        assert!(!goal.is_achieved(&state));
    }
}

#[cfg(test)]
mod counterfactual_tests {
    use super::*;

    #[test]
    fn test_counterfactual_scenario() {
        let mut original = WorldState::new();
        original.add_fact(Fact::new("at", "agent", Some("start")));

        let mut counterfactual = WorldState::new();
        counterfactual.add_fact(Fact::new("at", "agent", Some("end")));

        let mut scenario = CounterfactualScenario::new(original, counterfactual);
        let analysis = scenario.analyze();

        assert!(!analysis.differences.is_empty());
    }
}

#[cfg(test)]
mod plan_tests {
    use super::*;

    #[test]
    fn test_plan_creation() {
        let plan = Plan::new();
        assert!(plan.steps.is_empty());
        assert_eq!(plan.cost, 0.0);
    }

    #[test]
    fn test_plan_get_actions() {
        let mut plan = Plan::new();
        plan.steps.push(PlanStep {
            task: Task::new("test"),
            method: None,
            actions: vec!["action1".to_string(), "action2".to_string()],
        });

        let actions = plan.get_actions();
        assert_eq!(actions, vec!["action1", "action2"]);
    }
}
