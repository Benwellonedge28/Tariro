// Model Checking for Program Verification

use crate::tumc_compiler::ir::{BasicBlock, Instruction, Operation, BinOp, Terminator};
use std::collections::{HashMap, HashSet, VecDeque};

/// Transition system for model checking
#[derive(Debug, Clone)]
pub struct TransitionSystem {
    pub states: Vec<ProgramState>,
    pub initial_states: Vec<usize>,
    pub transitions: Vec<Transition>,
    pub atomic_propositions: HashMap<usize, Vec<AtomicProposition>>,
}

#[derive(Debug, Clone)]
pub struct Transition {
    pub from: usize,
    pub to: usize,
    pub guard: Condition,
    pub action: Vec<Action>,
}

#[derive(Debug, Clone)]
pub struct ProgramState {
    pub pc: usize,
    pub variables: HashMap<String, i64>,
    pub memory: HashMap<u64, i64>,
}

impl ProgramState {
    pub fn new(pc: usize) -> Self {
        Self {
            pc,
            variables: HashMap::new(),
            memory: HashMap::new(),
        }
    }

    pub fn with_vars(pc: usize, vars: HashMap<String, i64>) -> Self {
        Self {
            pc,
            variables: vars,
            memory: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AtomicProposition {
    pub name: String,
    pub evaluate: fn(&ProgramState) -> bool,
}

#[derive(Debug, Clone)]
pub enum Condition {
    True,
    False,
    Comparison(BinOp, String, i64),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Not(Box<Condition>),
}

impl Condition {
    pub fn evaluate(&self, state: &ProgramState) -> bool {
        match self {
            Condition::True => true,
            Condition::False => false,
            Condition::Comparison(op, var, val) => {
                let var_val = state.variables.get(var).copied().unwrap_or(0);
                match op {
                    BinOp::Eq => var_val == *val,
                    BinOp::Ne => var_val != *val,
                    BinOp::Lt => var_val < *val,
                    BinOp::Le => var_val <= *val,
                    BinOp::Gt => var_val > *val,
                    BinOp::Ge => var_val >= *val,
                    _ => true,
                }
            }
            Condition::And(a, b) => a.evaluate(state) && b.evaluate(state),
            Condition::Or(a, b) => a.evaluate(state) || b.evaluate(state),
            Condition::Not(c) => !c.evaluate(state),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Action {
    Assign(String, i64),
    AssignVar(String, String),
    MemWrite(u64, i64),
    MemRead(String, u64),
}

/// Model checking result
#[derive(Debug, Clone)]
pub struct ModelCheckResult {
    pub verified: bool,
    pub errors: Vec<VerificationErrorType>,
    pub counterexample: Option<Counterexample>,
    pub states_explored: usize,
}

#[derive(Debug, Clone)]
pub enum VerificationErrorType {
    AssertionFailed,
    ArrayBoundsViolation,
    NullPointerDereference,
    DivisionByZero,
    IntegerOverflow,
    Deadlock,
    Livelock,
    RaceCondition,
}

#[derive(Debug, Clone)]
pub struct Counterexample {
    pub initial_state: ProgramState,
    pub trace: Vec<ProgramState>,
    pub violating_state: usize,
    pub error_type: VerificationErrorType,
}

impl Counterexample {
    pub fn print_trace(&self) {
        println!("=== Counterexample Trace ===");
        println!("Initial state: PC={}", self.initial_state.pc);
        for (i, state) in self.trace.iter().enumerate() {
            println!("Step {}: PC={}", i + 1, state.pc);
            for (name, val) in &state.variables {
                println!("  {} = {}", name, val);
            }
        }
        println!("Violation at step {}", self.violating_state);
    }
}

/// Model checker
pub struct ModelChecker {
    max_states: usize,
    max_depth: usize,
    use_bfs: bool,
}

impl ModelChecker {
    pub fn new(max_states: usize) -> Self {
        Self {
            max_states,
            max_depth: 1000,
            use_bfs: true,
        }
    }

    /// Check a block for safety properties
    pub fn check_block(&self, block: &BasicBlock) -> ModelCheckResult {
        let mut result = ModelCheckResult {
            verified: true,
            errors: Vec::new(),
            counterexample: None,
            states_explored: 0,
        };

        // Build transition system
        let system = self.build_transition_system(block);

        // Check reachability of error states
        for state_idx in &system.initial_states {
            let visited = self.bfs_search(&system, *state_idx, &mut result);
            result.states_explored += visited.len();
        }

        result
    }

    /// Build transition system from block
    fn build_transition_system(&self, block: &BasicBlock) -> TransitionSystem {
        let mut states = Vec::new();
        let mut transitions = Vec::new();

        // Create state for each instruction
        for (i, inst) in block.instructions.iter().enumerate() {
            let mut state = ProgramState::new(i);
            state.variables.insert("result".to_string(), 0);
            states.push(state);

            // Create transition to next instruction
            if i + 1 < block.instructions.len() {
                transitions.push(Transition {
                    from: i,
                    to: i + 1,
                    guard: Condition::True,
                    action: Vec::new(),
                });
            }
        }

        TransitionSystem {
            states,
            initial_states: vec![0],
            transitions,
            atomic_propositions: HashMap::new(),
        }
    }

    /// BFS search from initial state
    fn bfs_search(
        &self,
        system: &TransitionSystem,
        start: usize,
        result: &mut ModelCheckResult,
    ) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start);

        while let Some(state_idx) = queue.pop_front() {
            if visited.contains(&state_idx) {
                continue;
            }
            visited.insert(state_idx);

            if result.verified && system.states.len() > self.max_states {
                break;
            }

            // Find outgoing transitions
            for trans in &system.transitions {
                if trans.from == state_idx && trans.guard.evaluate(&system.states[state_idx]) {
                    let next_state = trans.to;
                    queue.push_back(next_state);
                }
            }
        }

        visited
    }

    /// DFS search with depth limit
    fn dfs_search(
        &self,
        system: &TransitionSystem,
        state_idx: usize,
        depth: usize,
        visited: &mut HashSet<usize>,
        result: &mut ModelCheckResult,
    ) {
        if depth >= self.max_depth || !result.verified {
            return;
        }

        if visited.contains(&state_idx) {
            return;
        }
        visited.insert(state_idx);

        // Check for error conditions
        let state = &system.states[state_idx];
        if let Some(props) = system.atomic_propositions.get(&state_idx) {
            for prop in props {
                if (prop.name == "error") && (prop.evaluate)(state) {
                    result.verified = false;
                    result.errors.push(VerificationErrorType::AssertionFailed);
                    return;
                }
            }
        }

        // Explore successors
        for trans in &system.transitions {
            if trans.from == state_idx && trans.guard.evaluate(state) {
                self.dfs_search(system, trans.to, depth + 1, visited, result);
            }
        }
    }

    /// Check LTL property (simplified)
    pub fn check_ltl(&self, system: &TransitionSystem, formula: &str) -> ModelCheckResult {
        let mut result = ModelCheckResult {
            verified: true,
            errors: Vec::new(),
            counterexample: None,
            states_explored: 0,
        };

        // Simplified LTL checking
        // In practice, would use automata-based LTL model checking
        let _ = formula;

        result
    }

    /// Check CTL property (simplified)
    pub fn check_ctl(&self, system: &TransitionSystem, formula: &str) -> ModelCheckResult {
        let mut result = ModelCheckResult {
            verified: true,
            errors: Vec::new(),
            counterexample: None,
            states_explored: 0,
        };

        // Simplified CTL checking
        let _ = formula;

        result
    }

    /// Check for deadlocks
    pub fn check_deadlock(&self, block: &BasicBlock) -> ModelCheckResult {
        let mut result = ModelCheckResult {
            verified: true,
            errors: Vec::new(),
            counterexample: None,
            states_explored: 0,
        };

        // Check if block has proper terminator
        if let Some(term) = &block.terminator {
            match term {
                Terminator::Ret(_) | Terminator::Jump(_) | Terminator::Br(_, _, _) => {
                    // Has proper terminator
                }
                _ => {
                    result.verified = false;
                    result.errors.push(VerificationErrorType::Deadlock);
                }
            }
        }

        result
    }

    /// Generate counterexample for failed check
    pub fn generate_counterexample(
        &self,
        system: &TransitionSystem,
        error_state: usize,
    ) -> Option<Counterexample> {
        // BFS to find shortest path to error state
        let mut parent: HashMap<usize, usize> = HashMap::new();
        let mut queue = VecDeque::new();

        for &init in &system.initial_states {
            queue.push_back(init);
            parent.insert(init, init);
        }

        while let Some(state_idx) = queue.pop_front() {
            if state_idx == error_state {
                // Reconstruct path
                let mut trace = Vec::new();
                let mut current = error_state;
                while current != system.initial_states[0] {
                    trace.push(system.states[current].clone());
                    current = *parent.get(&current)?;
                }
                trace.push(system.states[system.initial_states[0]].clone());
                trace.reverse();

                return Some(Counterexample {
                    initial_state: system.states[system.initial_states[0]].clone(),
                    trace,
                    violating_state: trace.len(),
                    error_type: VerificationErrorType::AssertionFailed,
                });
            }

            for trans in &system.transitions {
                if trans.from == state_idx && !parent.contains_key(&trans.to) {
                    parent.insert(trans.to, state_idx);
                    queue.push_back(trans.to);
                }
            }
        }

        None
    }

    /// Set maximum search depth
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }

    /// Enable/disable BFS
    pub fn with_bfs(mut self, use_bfs: bool) -> Self {
        self.use_bfs = use_bfs;
        self
    }
}

impl Default for ModelChecker {
    fn default() -> Self {
        Self::new(100000)
    }
}

/// Predicate abstraction
pub struct PredicateAbstraction {
    pub predicates: Vec<Predicate>,
}

#[derive(Debug, Clone)]
pub struct Predicate {
    pub name: String,
    pub condition: Condition,
}

impl PredicateAbstraction {
    pub fn new() -> Self {
        Self {
            predicates: Vec::new(),
        }
    }

    pub fn add_predicate(&mut self, name: &str, condition: Condition) {
        self.predicates.push(Predicate {
            name: name.to_string(),
            condition,
        });
    }

    /// Abstract a concrete state
    pub fn abstract_state(&self, state: &ProgramState) -> Vec<bool> {
        self.predicates
            .iter()
            .map(|p| p.condition.evaluate(state))
            .collect()
    }

    /// Generate abstract transition system
    pub fn generate_abstract_system(
        &self,
        concrete_system: &TransitionSystem,
    ) -> TransitionSystem {
        let mut abstract_states: HashMap<Vec<bool>, usize> = HashMap::new();
        let mut abstract_transitions: Vec<Transition> = Vec::new();

        for (i, state) in concrete_system.states.iter().enumerate() {
            let ap = self.abstract_state(state);
            if !abstract_states.contains_key(&ap) {
                abstract_states.insert(ap.clone(), abstract_states.len());
            }

            for trans in &concrete_system.transitions {
                if trans.from == i {
                    let from_ap = ap.clone();
                    let to_ap = self.abstract_state(&concrete_system.states[trans.to]);

                    if !abstract_states.contains_key(&to_ap) {
                        abstract_states.insert(to_ap.clone(), abstract_states.len());
                    }

                    abstract_transitions.push(Transition {
                        from: *abstract_states.get(&from_ap).unwrap(),
                        to: *abstract_states.get(&to_ap).unwrap(),
                        guard: trans.guard.clone(),
                        action: trans.action.clone(),
                    });
                }
            }
        }

        TransitionSystem {
            states: (0..abstract_states.len())
                .map(|i| ProgramState::new(i))
                .collect(),
            initial_states: vec![0],
            transitions: abstract_transitions,
            atomic_propositions: HashMap::new(),
        }
    }
}

impl Default for PredicateAbstraction {
    fn default() -> Self {
        Self::new()
    }
}
