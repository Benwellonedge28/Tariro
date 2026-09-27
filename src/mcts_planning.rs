// Tariro AGI - Monte Carlo Tree Search (MCTS) Planning Module
// Simulation-based planning for complex decision spaces

use std::collections::{HashMap, VecDeque, BinaryHeap};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::cmp::Ordering;
use rand::Rng;

// ============================================================================
// MCTS Types
// ============================================================================

/// Game state or decision state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCTSState {
    pub state_id: String,
    pub state_data: StateData,
    pub is_terminal: bool,
    pub payoff: Option<f32>,
    pub hash: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateData {
    Game(GameState),
    Decision(DecisionState),
    Planning(PlanningState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub position: String,
    pub current_player: String,
    pub history: Vec<String>,
    pub valid_moves: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionState {
    pub context: String,
    pub options: Vec<DecisionOption>,
    pub utility: HashMap<String, f32>,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOption {
    pub option_id: String,
    pub description: String,
    pub outcomes: Vec<Outcome>,
    pub probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub result_state: String,
    pub utility: f32,
    pub probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningState {
    pub current_goal: String,
    pub available_actions: Vec<String>,
    pub state_vector: Vec<f32>,
    pub reward: f32,
}

// ============================================================================
// MCTS Node
// ============================================================================

/// MCTS tree node
#[derive(Debug, Clone)]
pub struct MCTSNode {
    pub node_id: String,
    pub state: MCTSState,
    pub parent: Option<String>,
    pub children: HashMap<String, MCTSNodeRef>,
    pub visits: usize,
    pub wins: f32,
    pub value: f32,
    pub is_fully_expanded: bool,
    pub is_terminal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MCTSNodeRef {
    pub node_id: String,
    pub move_id: String,
}

impl MCTSNode {
    pub fn new(state: MCTSState, parent: Option<String>) -> Self {
        Self {
            node_id: generate_id(),
            state,
            parent,
            children: HashMap::new(),
            visits: 0,
            wins: 0.0,
            value: 0.0,
            is_fully_expanded: false,
            is_terminal: state.is_terminal,
        }
    }

    pub fn ucb1(&self, exploration: f32, parent_visits: usize) -> f32 {
        if self.visits == 0 {
            return f32::MAX;
        }

        let exploitation = self.value;
        let exploration_term = exploration * ((parent_visits as f32).ln() / (self.visits as f32)).sqrt();

        exploitation + exploration_term
    }

    pub fn ucb1_tuned(&self, exploration: f32, parent_visits: usize) -> f32 {
        if self.visits == 0 {
            return f32::MAX;
        }

        let exploitation = self.value;
        let variance = (self.wins / self.visits as f32) - (self.value * self.value);
        let exploration_term = exploration * ((parent_visits as f32).ln() / (self.visits as f32)).sqrt()
            * (variance.max(0.0) + (4.0 * (2.0 * (parent_visits as f32).ln() / (self.visits as f32)).sqrt())).min(0.25);

        exploitation + exploration_term
    }
}

// ============================================================================
// MCTS Tree
// ============================================================================

/// MCTS Tree structure
pub struct MCTSTree {
    pub root: String,
    pub nodes: HashMap<String, MCTSNode>,
    pub edges: HashMap<MCTSNodeRef, f32>,
    config: MCTSConfig,
}

#[derive(Debug, Clone)]
pub struct MCTSConfig {
    pub exploration_constant: f32,
    pub max_iterations: usize,
    pub time_limit: Duration,
    pub ucb_type: UCBType,
    pub selection_policy: SelectionPolicy,
    pub expansion_policy: ExpansionPolicy,
    pub simulation_policy: SimulationPolicy,
    pub backprop_strategy: BackpropStrategy,
    pub parallel_simulations: usize,
    pub early_terminal_check: bool,
}

impl Default for MCTSConfig {
    fn default() -> Self {
        Self {
            exploration_constant: 1.414,  // sqrt(2)
            max_iterations: 100000,
            time_limit: Duration::from_secs(10),
            ucb_type: UCBType::UCB1,
            selection_policy: SelectionPolicy::UCB1,
            expansion_policy: ExpansionPolicy::AllChildren,
            simulation_policy: SimulationPolicy::Random,
            backprop_strategy: BackpropStrategy::Standard,
            parallel_simulations: 1,
            early_terminal_check: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UCBType {
    UCB1,
    UCB1Tuned,
    UCB2,
    AlphaGo,
    MEOW,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionPolicy {
    UCB1,
    UCB1Tuned,
    ProgressiveWidening,
    ProgressiveUnpruning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpansionPolicy {
    AllChildren,
    SingleChild,
    Progressive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimulationPolicy {
    Random,
    Greedy,
    Softmax,
    NeuralNetwork,
    Bandit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackpropStrategy {
    Standard,
    Exponential,
    MonteCarlo,
}

// ============================================================================
// MCTS Algorithm Implementation
// ============================================================================

impl MCTSTree {
    pub fn new(initial_state: MCTSState, config: MCTSConfig) -> Self {
        let root_id = generate_id();
        let root_node = MCTSNode::new(initial_state, None);

        let mut tree = Self {
            root: root_id.clone(),
            nodes: HashMap::new(),
            edges: HashMap::new(),
            config,
        };

        tree.nodes.insert(root_id, root_node);
        tree
    }

    /// Run MCTS iterations
    pub fn search(&mut self, environment: &mut impl MCTSEnvironment) -> Result<MCTSResult, MCTSError> {
        let start_time = Utc::now();
        let mut iterations = 0;

        while self.should_continue(iterations, start_time) {
            iterations += 1;

            // Selection
            let selected = self.select()?;

            // Expansion
            self.expand(&selected, environment)?;

            // Simulation
            let simulation_result = self.simulate(&selected, environment)?;

            // Backpropagation
            self.backpropagate(&selected, simulation_result);
        }

        self.get_best_result()
    }

    /// Check if search should continue
    fn should_continue(&self, iterations: usize, start_time: DateTime<Utc>) -> bool {
        if iterations >= self.config.max_iterations {
            return false;
        }
        if Utc::now().signed_duration_since(start_time).to_std().unwrap_or_default() >= self.config.time_limit {
            return false;
        }
        true
    }

    /// Selection phase - select node using UCB
    fn select(&self) -> Result<String, MCTSError> {
        let mut current = self.root.clone();
        let mut path = vec![current.clone()];

        while !self.nodes.get(&current).map(|n| n.is_fully_expanded).unwrap_or(false) {
            if let Some(node) = self.nodes.get(&current) {
                if node.children.is_empty() {
                    return Ok(current);
                }

                // Find best child
                current = self.find_best_child(&current)?;
                path.push(current.clone());

                if self.nodes.get(&current).map(|n| n.is_terminal).unwrap_or(false) {
                    return Ok(current);
                }
            } else {
                return Err(MCTSError::NodeNotFound);
            }
        }

        Ok(current)
    }

    /// Find best child using UCB
    fn find_best_child(&self, node_id: &str) -> Result<String, MCTSError> {
        let node = self.nodes.get(node_id)
            .ok_or(MCTSError::NodeNotFound)?;

        let parent_visits = node.visits.max(1);

        let mut best_child = None;
        let mut best_score = f32::MIN;

        for (child_id, child_ref) in &node.children {
            if let Some(child) = self.nodes.get(child_id) {
                let score = match self.config.selection_policy {
                    SelectionPolicy::UCB1 | SelectionPolicy::UCB1Tuned => {
                        if self.config.ucb_type == UCBType::UCB1Tuned {
                            child.ucb1_tuned(self.config.exploration_constant, parent_visits)
                        } else {
                            child.ucb1(self.config.exploration_constant, parent_visits)
                        }
                    }
                    _ => child.value,
                };

                if score > best_score {
                    best_score = score;
                    best_child = Some(child_id.clone());
                }
            }
        }

        best_child.ok_or(MCTSError::NoValidChild)
    }

    /// Expansion phase - add new child nodes
    fn expand(&mut self, node_id: &str, environment: &mut impl MCTSEnvironment) -> Result<(), MCTSError> {
        let node = self.nodes.get(node_id)
            .ok_or(MCTSError::NodeNotFound)?;

        if node.is_terminal {
            return Ok(());
        }

        // Get possible actions
        let actions = environment.get_valid_actions(&node.state)?;

        // Select actions to expand based on policy
        let actions_to_expand = match self.config.expansion_policy {
            ExpansionPolicy::AllChildren => actions,
            ExpansionPolicy::SingleChild => actions.first().map(|a| vec![a.clone()]).unwrap_or_default(),
            ExpansionPolicy::Progressive => {
                let node_visits = node.visits.max(1);
                let depth = (node_visits as f32).ln().floor() as usize;
                actions.into_iter().take(1 << depth).collect()
            }
        };

        // Add child nodes
        for action in actions_to_expand {
            let child_state = environment.apply_action(&node.state, &action)?;

            let child_node = MCTSNode::new(child_state, Some(node_id.to_string()));

            let child_ref = MCTSNodeRef {
                node_id: child_node.node_id.clone(),
                move_id: action.to_string(),
            };

            // Update parent
            if let Some(parent) = self.nodes.get_mut(node_id) {
                parent.children.insert(child_ref.move_id.clone(), child_ref.clone());
            }

            self.nodes.insert(child_node.node_id.clone(), child_node);
        }

        // Check if fully expanded
        if let Some(node) = self.nodes.get_mut(node_id) {
            if let Some(parent) = &node.parent {
                let parent = self.nodes.get(parent).unwrap();
                let total_actions = environment.get_valid_actions(&parent.state)?.len();
                node.is_fully_expanded = node.children.len() >= total_actions;
            }
        }

        Ok(())
    }

    /// Simulation phase - play out to terminal state
    fn simulate(&self, node_id: &str, environment: &mut impl MCTSEnvironment) -> Result<f32, MCTSError> {
        let node = self.nodes.get(node_id)
            .ok_or(MCTSError::NodeNotFound)?;

        let mut current_state = node.state.clone();
        let mut depth = 0;
        let max_depth = 1000;

        while !current_state.is_terminal && depth < max_depth {
            let actions = environment.get_valid_actions(&current_state)?;

            if actions.is_empty() {
                break;
            }

            let action = match self.config.simulation_policy {
                SimulationPolicy::Random => {
                    // Select random action
                    let idx = rand::thread_rng().gen_range(0..actions.len());
                    actions[idx].clone()
                }
                SimulationPolicy::Greedy => {
                    // Select best action based on utility
                    actions.into_iter()
                        .max_by(|a, b| {
                            environment.get_action_utility(&current_state, a)
                                .partial_cmp(&environment.get_action_utility(&current_state, b))
                                .unwrap_or(Ordering::Equal)
                        })
                        .unwrap_or_default()
                }
                SimulationPolicy::Softmax => {
                    // Softmax selection based on utilities
                    let utilities: Vec<f32> = actions.iter()
                        .map(|a| environment.get_action_utility(&current_state, a))
                        .collect();

                    let softmax = |x: f32| (x.exp()) / utilities.iter().map(|u| u.exp()).sum::<f32>();
                    let probs: Vec<f32> = utilities.iter().map(|u| softmax(*u)).collect();

                    let r = rand::thread_rng().gen_range(0.0..1.0);
                    let mut cumsum = 0.0;
                    for (i, prob) in probs.iter().enumerate() {
                        cumsum += prob;
                        if r <= cumsum {
                            return actions[i].clone();
                        }
                    }
                    actions.last().cloned().unwrap_or_default()
                }
                _ => actions[0].clone(),
            };

            current_state = environment.apply_action(&current_state, &action)?;
            depth += 1;
        }

        // Get payoff
        let payoff = environment.get_payoff(&current_state)?;
        Ok(payoff)
    }

    /// Backpropagation phase - update node statistics
    fn backpropagate(&mut self, node_id: &str, payoff: f32) {
        let mut current = Some(node_id.to_string());

        while let Some(id) = current {
            if let Some(node) = self.nodes.get_mut(&id) {
                node.visits += 1;
                node.wins += payoff;
                node.value = node.wins / node.visits as f32;

                // Apply backprop strategy
                match self.config.backprop_strategy {
                    BackpropStrategy::Standard => {}
                    BackpropStrategy::Exponential => {
                        // Weight recent updates more
                        let decay = 0.95_f32.powi(node.visits as i32);
                        node.value = node.value * (1.0 - decay) + payoff * decay;
                    }
                    BackpropStrategy::MonteCarlo => {
                        // Use running average
                    }
                }

                current = node.parent.clone();
            } else {
                break;
            }
        }
    }

    /// Get best result/action from search
    fn get_best_result(&self) -> Result<MCTSResult, MCTSError> {
        let root = self.nodes.get(&self.root)
            .ok_or(MCTSError::NodeNotFound)?;

        if root.children.is_empty() {
            return Err(MCTSError::NoChildren);
        }

        // Find child with most visits (most robust)
        let best_child = root.children.values()
            .max_by(|a, b| {
                let a_visits = self.nodes.get(&a.node_id).map(|n| n.visits).unwrap_or(0);
                let b_visits = self.nodes.get(&b.node_id).map(|n| n.visits).unwrap_or(0);
                a_visits.cmp(&b_visits)
            })
            .cloned()
            .ok_or(MCTSError::NoChildren)?;

        let child_node = self.nodes.get(&best_child.node_id)
            .ok_or(MCTSError::NodeNotFound)?;

        Ok(MCTSResult {
            action: best_child.move_id,
            expected_value: child_node.value,
            visit_count: child_node.visits,
            statistics: MCTSStatistics {
                total_nodes: self.nodes.len(),
                max_depth: self.calculate_max_depth(),
                avg_branching: self.calculate_avg_branching(),
            },
        })
    }

    fn calculate_max_depth(&self) -> usize {
        let mut max_depth = 0;
        self.calculate_depth_recursive(&self.root, 0, &mut max_depth);
        max_depth
    }

    fn calculate_depth_recursive(&self, node_id: &str, depth: usize, max_depth: &mut usize) {
        *max_depth = (*max_depth).max(depth);
        if let Some(node) = self.nodes.get(node_id) {
            for child_ref in node.children.values() {
                self.calculate_depth_recursive(&child_ref.node_id, depth + 1, max_depth);
            }
        }
    }

    fn calculate_avg_branching(&self) -> f32 {
        let total_children: usize = self.nodes.values()
            .map(|n| n.children.len())
            .sum();
        let nodes_with_children = self.nodes.values()
            .filter(|n| !n.children.is_empty())
            .count();

        if nodes_with_children > 0 {
            total_children as f32 / nodes_with_children as f32
        } else {
            0.0
        }
    }

    /// Get root statistics
    pub fn get_root_stats(&self) -> Option<(usize, f32)> {
        self.nodes.get(&self.root)
            .map(|n| (n.visits, n.value))
    }
}

/// MCTS search result
#[derive(Debug, Clone)]
pub struct MCTSResult {
    pub action: String,
    pub expected_value: f32,
    pub visit_count: usize,
    pub statistics: MCTSStatistics,
}

#[derive(Debug, Clone)]
pub struct MCTSStatistics {
    pub total_nodes: usize,
    pub max_depth: usize,
    pub avg_branching: f32,
}

/// MCTS environment interface
pub trait MCTSEnvironment {
    fn get_valid_actions(&self, state: &MCTSState) -> Result<Vec<String>, MCTSError>;
    fn apply_action(&mut self, state: &MCTSState, action: &str) -> Result<MCTSState, MCTSError>;
    fn get_payoff(&self, state: &MCTSState) -> Result<f32, MCTSError>;
    fn get_action_utility(&self, state: &MCTSState, action: &str) -> f32;
}

/// MCTS errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MCTSError {
    NodeNotFound,
    NoValidChild,
    NoChildren,
    NoActions,
    SimulationFailed,
    InvalidState,
}

impl std::fmt::Display for MCTSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MCTSError::NodeNotFound => write!(f, "Node not found"),
            MCTSError::NoValidChild => write!(f, "No valid child found"),
            MCTSError::NoChildren => write!(f, "No children in tree"),
            MCTSError::NoActions => write!(f, "No valid actions available"),
            MCTSError::SimulationFailed => write!(f, "Simulation failed"),
            MCTSError::InvalidState => write!(f, "Invalid state"),
        }
    }
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("mcts_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ucb1() {
        let node = MCTSNode {
            node_id: "test".to_string(),
            state: MCTSState {
                state_id: "state1".to_string(),
                state_data: StateData::Game(GameState {
                    position: "start".to_string(),
                    current_player: "player1".to_string(),
                    history: vec![],
                    valid_moves: vec![],
                }),
                is_terminal: false,
                payoff: None,
                hash: 0,
            },
            parent: None,
            children: HashMap::new(),
            visits: 10,
            wins: 5.0,
            value: 0.5,
            is_fully_expanded: false,
            is_terminal: false,
        };

        let ucb = node.ucb1(1.414, 20);
        assert!(ucb >= 0.5); // Should be at least exploitation value
    }
}
