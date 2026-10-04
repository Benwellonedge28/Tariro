// Optimization Module - Local, Loop, and Global Optimizations

pub mod local;
pub mod loop_opt;
pub mod global;
pub mod vectorize;
pub mod simd;

pub use local::*;
pub use loop_opt::*;
pub use global::*;
pub use vectorize::*;
pub use simd::*;

// ============================================================================
// Optimization Pass Infrastructure
// ============================================================================

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptLevel {
    O0,  // No optimization
    O1,  // Basic optimization
    O2,  // Standard optimization
    O3,  // Aggressive optimization
    Os,  // Size optimization
    Oz,  // Aggressive size optimization
}

impl OptLevel {
    pub fn numeric(&self) -> u32 {
        match self {
            OptLevel::O0 => 0,
            OptLevel::O1 => 1,
            OptLevel::O2 => 2,
            OptLevel::O3 => 3,
            OptLevel::Os => 1,
            OptLevel::Oz => 2,
        }
    }
}

/// Pass manager for optimizations
pub struct OptimizationManager {
    passes: Vec<Box<dyn OptimizationPass>>,
    level: OptLevel,
}

pub trait OptimizationPass {
    /// Pass name
    fn name(&self) -> &'static str;

    /// Pass description
    fn description(&self) -> &'static str {
        ""
    }

    /// Run the pass
    fn run(&self, module: &mut Module) -> bool;

    /// Check if pass should run at this optimization level
    fn should_run(&self, level: OptLevel) -> bool {
        true
    }
}

impl OptimizationManager {
    pub fn new(level: OptLevel) -> Self {
        let mut manager = Self {
            passes: Vec::new(),
            level,
        };

        // Add default passes based on optimization level
        manager.add_default_passes(level);

        manager
    }

    fn add_default_passes(&mut self, level: OptLevel) {
        // Always run these
        self.passes.push(Box::new(ConstantFolding));
        self.passes.push(Box::new(AlgebraicSimplification));
        self.passes.push(Box::new(DeadCodeElimination));

        if level.numeric() >= 1 {
            self.passes.push(Box::new(LocalValueNumbering));
            self.passes.push(Box::new(CommonSubexpressionElimination));
        }

        if level.numeric() >= 2 {
            self.passes.push(Box::new(LoopInvariantCodeMotion));
            self.passes.push(Box::new(LoopUnrolling));
            self.passes.push(Box::new(GlobalValueNumbering));
            self.passes.push(Box::new(CopyPropagation));
            self.passes.push(Box::new(DeadStoreElimination));
        }

        if level.numeric() >= 3 {
            self.passes.push(Box::new(LoopVectorization));
            self.passes.push(Box::new(AggressiveDeadCodeElimination));
            self.passes.push(Box::new(InstructionCombining));
            self.passes.push(Box::new(Reassociation));
        }

        if matches!(level, OptLevel::Os | OptLevel::Oz) {
            self.passes.push(Box::new(CodeSinking));
            self.passes.push(Box::new(TailCallOptimization));
        }
    }

    pub fn add_pass<P: OptimizationPass + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    pub fn run(&self, module: &mut Module) -> usize {
        let mut total_changes = 0;

        for pass in &self.passes {
            if pass.should_run(self.level) {
                let changes = if pass.run(module) { 1 } else { 0 };
                total_changes += changes;
            }
        }

        total_changes
    }

    pub fn run_until_stable(&self, module: &mut Module, max_iterations: usize) -> usize {
        let mut total_changes = 0;

        for _ in 0..max_iterations {
            let changes = self.run(module);
            total_changes += changes;
            if changes == 0 {
                break;
            }
        }

        total_changes
    }
}

// ============================================================================
// Optimization Statistics
// ============================================================================

/// Statistics collected during optimization
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    pub passes_run: usize,
    pub total_changes: usize,
    pub instructions_removed: usize,
    pub instructions_added: usize,
    pub loops_unrolled: usize,
    pub functions_inlined: usize,
    pub vectorized_loops: usize,
    pub spills_inserted: usize,
}

impl OptimizationStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn merge(&mut self, other: &Self) {
        self.passes_run += other.passes_run;
        self.total_changes += other.total_changes;
        self.instructions_removed += other.instructions_removed;
        self.instructions_added += other.instructions_added;
        self.loops_unrolled += other.loops_unrolled;
        self.functions_inlined += other.functions_inlined;
        self.vectorized_loops += other.vectorized_loops;
        self.spills_inserted += other.spills_inserted;
    }

    pub fn report(&self) -> String {
        format!(
            "Optimization Stats:\n\
             Passes run: {}\n\
             Total changes: {}\n\
             Instructions removed: {}\n\
             Instructions added: {}\n\
             Loops unrolled: {}\n\
             Functions inlined: {}\n\
             Loops vectorized: {}\n\
             Register spills: {}",
            self.passes_run,
            self.total_changes,
            self.instructions_removed,
            self.instructions_added,
            self.loops_unrolled,
            self.functions_inlined,
            self.vectorized_loops,
            self.spills_inserted
        )
    }
}

/// Pass result wrapper
pub struct PassResult {
    pub changed: bool,
    pub stats: OptimizationStats,
}

impl PassResult {
    pub fn new(changed: bool) -> Self {
        Self {
            changed,
            stats: OptimizationStats::default(),
        }
    }

    pub fn with_stats(mut self, stats: OptimizationStats) -> Self {
        self.stats = stats;
        self
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Check if a value is a constant
pub fn is_constant(module: &Module, value: ValueId) -> bool {
    // Implementation would check value definitions
    false
}

/// Get constant integer value
pub fn get_const_int(module: &Module, value: ValueId) -> Option<i64> {
    None
}

/// Check if instruction is pure (no side effects)
pub fn is_pure_instruction(op: &Operation) -> bool {
    match op {
        Operation::Binary(_, _, _) => true,
        Operation::Unary(_, _) => true,
        Operation::ICmp(_, _, _) => true,
        Operation::FCmp(_, _, _) => true,
        Operation::Cast(_, _, _) => true,
        Operation::Bitcast(_, _) => true,
        Operation::ExtractElement(_, _) => true,
        Operation::InsertElement(_, _, _) => true,
        _ => false,
    }
}

/// Check if a basic block is reachable
pub fn is_reachable(function: &Function, block: BasicBlockId, cfg: &ControlFlowGraph) -> bool {
    let mut visited = HashSet::new();
    let mut queue = vec![function.entry_block()];

    while let Some(current) = queue.pop() {
        if current == block {
            return true;
        }

        if visited.contains(&current) {
            continue;
        }
        visited.insert(current);

        for &succ in cfg.successors(current) {
            queue.push(succ);
        }
    }

    false
}

/// Clone instruction with new ID
pub fn clone_instruction(inst: &Instruction, new_id: InstId) -> Instruction {
    Instruction {
        id: new_id,
        op: inst.op.clone(),
        result_type: inst.result_type.clone(),
        comments: inst.comments.clone(),
    }
}

use std::collections::{HashMap, HashSet};
