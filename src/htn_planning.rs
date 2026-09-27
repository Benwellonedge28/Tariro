// Tariro AGI - Hierarchical Task Network (HTN) Planning Module
// Decomposition-based planning for complex goals

use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// HTN Planning Types
// ============================================================================

/// HTN Domain definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTNDomain {
    pub domain_id: String,
    pub name: String,
    pub requirements: Vec<String>,
    pub types: Vec<TypeDef>,
    pub predicates: Vec<Predicate>,
    pub methods: Vec<Method>,
    pub operators: Vec<Operator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDef {
    pub name: String,
    pub parent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Predicate {
    pub name: String,
    pub parameters: Vec<TypedParameter>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedParameter {
    pub name: String,
    pub param_type: String,
}

/// Planning method for task decomposition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub method_id: String,
    pub name: String,
    pub task: TaskTemplate,
    pub preconditions: Vec<Precondition>,
    pub subtasks: Vec<TaskNetwork>,
    pub constraints: Vec<Constraint>,
    pub cost: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplate {
    pub task_name: String,
    pub parameters: Vec<TypedParameter>,
}

/// Compound task network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskNetwork {
    pub tasks: Vec<NetworkTask>,
    pub ordering: Vec<OrderingConstraint>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTask {
    pub task_id: String,
    pub task_type: TaskType,
    pub parameters: Vec<String>,
    pub method_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Primitive,
    Compound(String),  // Method name
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderingConstraint {
    pub before: String,
    pub after: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub params: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    Preference,
    Equality,
    Inequality,
    Implication,
}

/// Primitive action/operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operator {
    pub operator_id: String,
    pub name: String,
    pub parameters: Vec<TypedParameter>,
    pub preconditions: Vec<Precondition>,
    pub effects: Vec<Effect>,  // [positive, negative]
    pub duration: Duration,
    pub cost: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Precondition {
    pub predicate: String,
    pub arguments: Vec<String>,
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Effect {
    pub predicate: String,
    pub arguments: Vec<String>,
    pub add: bool,  // true = add, false = delete
}

// ============================================================================
// Planning Problem
// ============================================================================

/// HTN Planning problem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningProblem {
    pub problem_id: String,
    pub domain_id: String,
    pub objects: Vec<ObjectInstance>,
    pub initial_state: WorldState,
    pub tasks: Vec<NetworkTask>,  // Top-level tasks
    pub goal_state: Option<WorldState>,
    pub constraints: Vec<Constraint>,
    pub optimization: OptimizationGoal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInstance {
    pub name: String,
    pub obj_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub facts: HashSet<GroundFact>,
    pub numeric_facts: HashMap<String, f32>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            facts: HashSet::new(),
            numeric_facts: HashMap::new(),
        }
    }

    pub fn with_facts(mut self, facts: Vec<String>) -> Self {
        for fact in facts {
            self.facts.insert(GroundFact { predicate: fact });
        }
        self
    }

    pub fn add_fact(&mut self, fact: &str) {
        self.facts.insert(GroundFact { predicate: fact.to_string() });
    }

    pub fn remove_fact(&mut self, fact: &str) {
        self.facts.remove(&GroundFact { predicate: fact.to_string() });
    }

    pub fn contains(&self, fact: &str) -> bool {
        self.facts.contains(&GroundFact { predicate: fact.to_string() })
    }
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct GroundFact {
    pub predicate: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OptimizationGoal {
    Makespan,     // Minimize total time
    TotalCost,    // Minimize total cost
    Both,         // Balance
}

// ============================================================================
// Planning Solution
// ============================================================================

/// HTN Planning solution/plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTNPlan {
    pub plan_id: String,
    pub problem_id: String,
    pub primitive_actions: Vec<PlanAction>,
    pub decomposition_tree: DecompositionNode,
    pub metrics: PlanMetrics,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanAction {
    pub action_id: String,
    pub operator_id: String,
    pub parameters: Vec<String>,
    pub start_time: f32,
    pub duration: f32,
    pub cost: f32,
    pub preconditions_met: Vec<String>,
    pub effects_applied: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecompositionNode {
    pub node_id: String,
    pub node_type: NodeType,
    pub method_id: Option<String>,
    pub children: Vec<DecompositionNode>,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Task(String),
    Method(String),
    Action(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanMetrics {
    pub total_cost: f32,
    pub total_duration: f32,
    pub makespan: f32,
    pub actions_count: usize,
    pub methods_count: usize,
    pub decomposition_depth: usize,
}

// ============================================================================
// HTN Planner
// ============================================================================

/// Hierarchical Task Network Planner
pub struct HTNPlanner {
    domain: Option<HTNDomain>,
    cache: PlanCache,
    config: HTNConfig,
    stats: PlannerStats,
}

#[derive(Debug, Clone, Default)]
pub struct PlanCache {
    plans: HashMap<String, Vec<HTNPlan>>,
    state_cache: HashMap<String, WorldState>,
}

#[derive(Debug, Clone)]
pub struct HTNConfig {
    pub max_depth: usize,
    pub max_search_nodes: usize,
    pub time_limit: Duration,
    pub enable_plan_reuse: bool,
    pub enable_partial_order: bool,
    pub heuristic: HeuristicType,
}

impl Default for HTNConfig {
    fn default() -> Self {
        Self {
            max_depth: 100,
            max_search_nodes: 100000,
            time_limit: Duration::from_secs(30),
            enable_plan_reuse: true,
            enable_partial_order: true,
            heuristic: HeuristicType::LevelSum,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HeuristicType {
    Zero,
    LevelSum,
    MaxLevel,
    SetLevel,
}

#[derive(Debug, Clone, Default)]
pub struct PlannerStats {
    pub nodes_expanded: usize,
    pub plans_found: usize,
    pub cache_hits: usize,
    pub planning_time_ms: u64,
    pub iterations: usize,
}

// ============================================================================
// Planning State
// ============================================================================

#[derive(Clone)]
struct PlanningState {
    state: WorldState,
    tasks: VecDeque<NetworkTask>,
    plan: Vec<PlanAction>,
    current_time: f32,
    total_cost: f32,
    decomposition: Vec<DecompositionNode>,
}

impl PlanningState {
    fn apply_action(&mut self, action: PlanAction) {
        self.plan.push(action.clone());
        self.total_cost += action.cost;
        self.current_time += action.duration;
    }
}

impl HTNPlanner {
    pub fn new() -> Self {
        Self {
            domain: None,
            cache: PlanCache::default(),
            config: HTNConfig::default(),
            stats: PlannerStats::default(),
        }
    }

    /// Load domain
    pub fn load_domain(&mut self, domain: HTNDomain) {
        self.domain = Some(domain);
    }

    /// Solve planning problem
    pub fn solve(&mut self, problem: &PlanningProblem) -> Result<HTNPlan, PlanningError> {
        let start_time = Utc::now();
        self.stats = PlannerStats::default();

        let domain = self.domain.as_ref()
            .ok_or(PlanningError::NoDomainLoaded)?;

        // Initialize planning state
        let initial_state = PlanningState {
            state: problem.initial_state.clone(),
            tasks: VecDeque::from(problem.tasks.clone()),
            plan: Vec::new(),
            current_time: 0.0,
            total_cost: 0.0,
            decomposition: Vec::new(),
        };

        // Plan search
        let result = self.plan_search(initial_state, domain, problem);

        self.stats.planning_time_ms = Utc::now().signed_duration_since(start_time).to_std().unwrap_or_default().as_millis() as u64;

        result
    }

    /// Main planning search
    fn plan_search(
        &mut self,
        mut state: PlanningState,
        domain: &HTNDomain,
        problem: &PlanningProblem,
    ) -> Result<HTNPlan, PlanningError> {
        let mut iterations = 0;

        while !state.tasks.is_empty() && iterations < self.config.max_search_nodes {
            iterations += 1;
            self.stats.nodes_expanded += 1;

            // Check time limit
            if self.stats.planning_time_ms > self.config.time_limit.as_millis() as u64 {
                return Err(PlanningError::TimeLimitExceeded);
            }

            // Get next task
            let task = state.tasks.pop_front()
                .ok_or(PlanningError::NoTasksRemaining)?;

            match task.task_type {
                TaskType::Primitive => {
                    // Find and apply operator
                    let action = self.apply_primitive(&task, &state.state, domain)?;
                    state.apply_action(action);
                }
                TaskType::Compound(ref method_name) => {
                    // Find applicable methods
                    let methods = self.find_applicable_methods(&task, &state.state, domain)?;

                    if methods.is_empty() {
                        return Err(PlanningError::NoMethodApplicable);
                    }

                    // Select best method (using heuristic)
                    let method = self.select_method(&methods, &state)?;

                    // Decompose task
                    let subtasks = self.decompose(&task, &method, domain)?;
                    state.tasks.extend(subtasks);
                }
            }
        }

        self.stats.iterations = iterations;

        // Build plan
        self.build_plan(&state, problem)
    }

    /// Apply primitive action
    fn apply_primitive(
        &self,
        task: &NetworkTask,
        state: &WorldState,
        domain: &HTNDomain,
    ) -> Result<PlanAction, PlanningError> {
        // Find operator
        let operator = domain.operators.iter()
            .find(|op| op.name == task.task_id)
            .ok_or(PlanningError::OperatorNotFound)?;

        // Check preconditions
        for pre in &operator.preconditions {
            let holds = pre.negated != state.contains(&self.ground_predicate(&pre.predicate, &task.parameters));
            if !holds && !pre.negated {
                return Err(PlanningError::PreconditionNotMet);
            }
        }

        Ok(PlanAction {
            action_id: generate_id(),
            operator_id: operator.operator_id.clone(),
            parameters: task.parameters.clone(),
            start_time: 0.0,
            duration: operator.duration.as_secs_f32(),
            cost: operator.cost,
            preconditions_met: vec![],
            effects_applied: vec![],
        })
    }

    /// Find applicable methods for task
    fn find_applicable_methods(
        &self,
        task: &NetworkTask,
        state: &WorldState,
        domain: &HTNDomain,
    ) -> Result<Vec<&Method>, PlanningError> {
        let applicable: Vec<_> = domain.methods.iter()
            .filter(|m| m.task.task_name == task.task_id)
            .filter(|m| self.check_preconditions(&m.preconditions, state, task))
            .collect();

        if applicable.is_empty() {
            Err(PlanningError::NoMethodApplicable)
        } else {
            Ok(applicable)
        }
    }

    /// Check if preconditions are satisfied
    fn check_preconditions(
        &self,
        preconditions: &[Precondition],
        state: &WorldState,
        task: &NetworkTask,
    ) -> bool {
        for pre in preconditions {
            let grounded = self.ground_predicate(&pre.predicate, &task.parameters);
            let holds = state.contains(&grounded);

            if pre.negated && holds {
                return false;
            }
            if !pre.negated && !holds {
                return false;
            }
        }
        true
    }

    /// Ground predicate with parameters
    fn ground_predicate(&self, predicate: &str, params: &[String]) -> String {
        let mut result = predicate.to_string();
        for (i, param) in params.iter().enumerate() {
            result = result.replace(&format!("?x{}", i + 1), param);
        }
        result
    }

    /// Select best method using heuristic
    fn select_method(&self, methods: &[&Method], state: &PlanningState) -> Result<&Method, PlanningError> {
        match self.config.heuristic {
            HeuristicType::LevelSum => {
                // Select method with lowest cost estimate
                Ok(*methods.iter()
                    .min_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap_or(std::cmp::Ordering::Equal))
                    .unwrap())
            }
            HeuristicType::MaxLevel => {
                // Select method with fewest subtasks
                Ok(*methods.iter()
                    .min_by_key(|m| m.subtasks.iter().map(|s| s.tasks.len()).sum::<usize>())
                    .unwrap())
            }
            _ => Ok(methods[0]),
        }
    }

    /// Decompose task using method
    fn decompose(
        &self,
        task: &NetworkTask,
        method: &Method,
        domain: &HTNDomain,
    ) -> Result<Vec<NetworkTask>, PlanningError> {
        let mut subtasks = Vec::new();

        for network in &method.subtasks {
            for sub_task in &network.tasks {
                // Bind parameters
                let mut bound_params = task.parameters.clone();
                // Parameter binding logic would go here
                bound_params.extend(sub_task.parameters.clone());

                subtasks.push(NetworkTask {
                    task_id: sub_task.task_id.clone(),
                    task_type: sub_task.task_type.clone(),
                    parameters: bound_params,
                    method_id: None,
                });
            }
        }

        Ok(subtasks)
    }

    /// Apply action to state
    fn apply_action(&mut self, action: PlanAction) {
        // This would apply effects to state
        // Implementation depends on domain specifics
    }

    /// Build final plan
    fn build_plan(
        &self,
        state: &PlanningState,
        problem: &PlanningProblem,
    ) -> Result<HTNPlan, PlanningError> {
        let total_cost = state.plan.iter().map(|a| a.cost).sum::<f32>();
        let makespan = state.plan.iter()
            .map(|a| a.start_time + a.duration)
            .fold(0.0f32, |a, b| a.max(b));

        Ok(HTNPlan {
            plan_id: generate_id(),
            problem_id: problem.problem_id.clone(),
            primitive_actions: state.plan.clone(),
            decomposition_tree: DecompositionNode {
                node_id: generate_id(),
                node_type: NodeType::Task("root".to_string()),
                method_id: None,
                children: vec![],
                actions: state.plan.iter().map(|a| a.operator_id.clone()).collect(),
            },
            metrics: PlanMetrics {
                total_cost,
                total_duration: makespan,
                makespan,
                actions_count: state.plan.len(),
                methods_count: 0,
                decomposition_depth: 1,
            },
            valid: true,
        })
    }

    /// Get planning statistics
    pub fn get_stats(&self) -> &PlannerStats {
        &self.stats
    }
}

/// Planning errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanningError {
    NoDomainLoaded,
    TimeLimitExceeded,
    NoTasksRemaining,
    NoMethodApplicable,
    PreconditionNotMet,
    OperatorNotFound,
    MaxNodesExceeded,
}

impl std::fmt::Display for PlanningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlanningError::NoDomainLoaded => write!(f, "No domain loaded"),
            PlanningError::TimeLimitExceeded => write!(f, "Planning time limit exceeded"),
            PlanningError::NoTasksRemaining => write!(f, "No tasks remaining"),
            PlanningError::NoMethodApplicable => write!(f, "No applicable method found"),
            PlanningError::PreconditionNotMet => write!(f, "Precondition not met"),
            PlanningError::OperatorNotFound => write!(f, "Operator not found"),
            PlanningError::MaxNodesExceeded => write!(f, "Maximum search nodes exceeded"),
        }
    }
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("plan_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state() {
        let state = WorldState::new()
            .with_facts(vec!["at(robot, loc1)".to_string(), "clear(loc2)".to_string()]);

        assert!(state.contains("at(robot, loc1)"));
        assert!(!state.contains("at(robot, loc2)"));
    }

    #[test]
    fn test_planner_creation() {
        let planner = HTNPlanner::new();
        assert!(planner.domain.is_none());
    }
}
