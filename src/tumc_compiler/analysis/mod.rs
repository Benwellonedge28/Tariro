// Analysis Module - Dataflow, Points-to, Escape Analysis

pub mod dataflow;
pub mod points_to;
pub mod escape;
pub mod alias;
pub mod call_graph;

pub use dataflow::*;
pub use points_to::*;
pub use escape::*;
pub use alias::*;
pub use call_graph::*;

// ============================================================================
// Analysis Infrastructure
// ============================================================================

use crate::tumc_compiler::ir::*;

/// Analysis result trait
pub trait AnalysisResult {
    fn print(&self);
}

/// Base analysis trait
pub trait Analysis {
    fn name(&self) -> &'static str;
    fn run(&self, function: &Function) -> Box<dyn AnalysisResult>;
}

// ============================================================================
// Reaching Definitions
// ============================================================================

/// Reaching definitions analysis
pub struct ReachingDefinitions;

impl Analysis for ReachingDefinitions {
    fn name(&self) -> &'static str {
        "reaching-defs"
    }

    fn run(&self, function: &Function) -> Box<dyn AnalysisResult> {
        let cfg = ControlFlowGraph::from_function(function);
        Box::new(self.analyze(function, &cfg))
    }
}

impl ReachingDefinitions {
    fn analyze(&self, function: &Function, cfg: &ControlFlowGraph) -> ReachingDefinitionsResult {
        let mut result = ReachingDefinitionsResult {
            in_sets: HashMap::new(),
            out_sets: HashMap::new(),
        };

        // Initialize
        for block in &function.body {
            result.in_sets.insert(block.id, HashSet::new());
            result.out_sets.insert(block.id, HashSet::new());
        }

        // Iterate until fixed point
        let mut changed = true;
        while changed {
            changed = false;

            for block in &function.body {
                // IN[B] = Union of OUT[P] for all predecessors P
                let mut new_in = HashSet::new();
                for &pred in cfg.predecessors(block.id) {
                    if let Some(out) = result.out_sets.get(&pred) {
                        new_in.extend(out);
                    }
                }

                if result.in_sets.get(&block.id) != Some(&new_in) {
                    result.in_sets.insert(block.id, new_in);
                    changed = true;
                }

                // OUT[B] = GEN[B] U (IN[B] - KILL[B])
                let mut new_out = self.gen_set(block);
                let killed = self.kill_set(block);
                if let Some(in_set) = result.in_sets.get(&block.id) {
                    let in_minus_kill: HashSet<_> = in_set.difference(&killed).cloned().collect();
                    new_out.extend(in_minus_kill);
                }

                if result.out_sets.get(&block.id) != Some(&new_out) {
                    result.out_sets.insert(block.id, new_out);
                    changed = true;
                }
            }
        }

        result
    }

    fn gen_set(&self, block: &BasicBlock) -> HashSet<DefId> {
        block.instructions.iter()
            .enumerate()
            .map(|(i, _)| DefId { block: block.id, idx: i })
            .collect()
    }

    fn kill_set(&self, block: &BasicBlock) -> HashSet<DefId> {
        // Would track which variables are killed
        HashSet::new()
    }
}

#[derive(Debug, Clone)]
pub struct ReachingDefinitionsResult {
    pub in_sets: HashMap<BasicBlockId, HashSet<DefId>>,
    pub out_sets: HashMap<BasicBlockId, HashSet<DefId>>,
}

impl AnalysisResult for ReachingDefinitionsResult {
    fn print(&self) {
        println!("Reaching Definitions:");
        for (block, defs) in &self.in_sets {
            println!("  Block {:?}: IN = {:?}", block, defs);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DefId {
    pub block: BasicBlockId,
    pub idx: usize,
}

// ============================================================================
// Live Variable Analysis
// ============================================================================

/// Live variable analysis
pub struct LiveVariables;

impl Analysis for LiveVariables {
    fn name(&self) -> &'static str {
        "live-vars"
    }

    fn run(&self, function: &Function) -> Box<dyn AnalysisResult> {
        let cfg = ControlFlowGraph::from_function(function);
        Box::new(self.analyze(function, &cfg))
    }
}

impl LiveVariables {
    fn analyze(&self, function: &Function, cfg: &ControlFlowGraph) -> LiveVariablesResult {
        let mut result = LiveVariablesResult {
            in_sets: HashMap::new(),
            out_sets: HashMap::new(),
        };

        // Initialize
        for block in &function.body {
            result.in_sets.insert(block.id, HashSet::new());
            result.out_sets.insert(block.id, HashSet::new());
        }

        // Iterate until fixed point (reverse order)
        let mut changed = true;
        while changed {
            changed = false;

            for block in function.body.iter().rev() {
                // OUT[B] = Union of IN[S] for all successors S
                let mut new_out = HashSet::new();
                for &succ in cfg.successors(block.id) {
                    if let Some(in_set) = result.in_sets.get(&succ) {
                        new_out.extend(in_set);
                    }
                }

                if result.out_sets.get(&block.id) != Some(&new_out) {
                    result.out_sets.insert(block.id, new_out);
                    changed = true;
                }

                // IN[B] = USE[B] U (OUT[B] - DEF[B])
                let mut new_in = self.use_set(block);
                if let Some(out_set) = result.out_sets.get(&block.id) {
                    let def_set = self.def_set(block);
                    for var in out_set.difference(&def_set) {
                        new_in.insert(var.clone());
                    }
                }

                if result.in_sets.get(&block.id) != Some(&new_in) {
                    result.in_sets.insert(block.id, new_in);
                    changed = true;
                }
            }
        }

        result
    }

    fn use_set(&self, block: &BasicBlock) -> HashSet<String> {
        // Extract used variables
        HashSet::new()
    }

    fn def_set(&self, block: &BasicBlock) -> HashSet<String> {
        // Extract defined variables
        HashSet::new()
    }
}

#[derive(Debug, Clone)]
pub struct LiveVariablesResult {
    pub in_sets: HashMap<BasicBlockId, HashSet<String>>,
    pub out_sets: HashMap<BasicBlockId, HashSet<String>>,
}

impl AnalysisResult for LiveVariablesResult {
    fn print(&self) {
        println!("Live Variables:");
        for (block, vars) in &self.in_sets {
            println!("  Block {:?}: IN = {:?}", block, vars);
        }
    }
}

// ============================================================================
// Available Expressions
// ============================================================================

/// Available expressions analysis
pub struct AvailableExpressions;

impl Analysis for AvailableExpressions {
    fn name(&self) -> &'static str {
        "avail-exprs"
    }

    fn run(&self, function: &Function) -> Box<dyn AnalysisResult> {
        let cfg = ControlFlowGraph::from_function(function);
        Box::new(self.analyze(function, &cfg))
    }
}

impl AvailableExpressions {
    fn analyze(&self, function: &Function, cfg: &ControlFlowGraph) -> AvailableExpressionsResult {
        let mut result = AvailableExpressionsResult {
            in_sets: HashMap::new(),
            out_sets: HashMap::new(),
        };

        // Initialize entry with empty, others with universal
        for block in &function.body {
            if block.id == function.entry_block() {
                result.in_sets.insert(block.id, HashSet::new());
                result.out_sets.insert(block.id, HashSet::new());
            } else {
                result.in_sets.insert(block.id, HashSet::new());
                result.out_sets.insert(block.id, HashSet::new());
            }
        }

        // Iterate until fixed point
        let mut changed = true;
        while changed {
            changed = false;

            for block in &function.body {
                // IN[B] = Intersection of OUT[P] for all predecessors P
                let mut new_in: Option<HashSet<ExprHash>> = None;
                for &pred in cfg.predecessors(block.id) {
                    if let Some(out) = result.out_sets.get(&pred) {
                        if let Some(ref mut intersect) = new_in {
                            *intersect = intersect.intersection(out).cloned().collect();
                        } else {
                            new_in = Some(out.clone());
                        }
                    }
                }

                let new_in = new_in.unwrap_or_default();
                if result.in_sets.get(&block.id) != Some(&new_in) {
                    result.in_sets.insert(block.id, new_in);
                    changed = true;
                }

                // OUT[B] = GEN[B] U (IN[B] - KILL[B])
                let mut new_out = self.gen_set(block);
                let killed = self.kill_set(block);
                if let Some(in_set) = result.in_sets.get(&block.id) {
                    for expr in in_set.difference(&killed) {
                        new_out.insert(expr.clone());
                    }
                }

                if result.out_sets.get(&block.id) != Some(&new_out) {
                    result.out_sets.insert(block.id, new_out);
                    changed = true;
                }
            }
        }

        result
    }

    fn gen_set(&self, _block: &BasicBlock) -> HashSet<ExprHash> {
        HashSet::new()
    }

    fn kill_set(&self, _block: &BasicBlock) -> HashSet<ExprHash> {
        HashSet::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprHash {
    pub op: BinOp,
    pub lhs: ValueId,
    pub rhs: ValueId,
}

#[derive(Debug, Clone)]
pub struct AvailableExpressionsResult {
    pub in_sets: HashMap<BasicBlockId, HashSet<ExprHash>>,
    pub out_sets: HashMap<BasicBlockId, HashSet<ExprHash>>,
}

impl AnalysisResult for AvailableExpressionsResult {
    fn print(&self) {
        println!("Available Expressions:");
        for (block, exprs) in &self.in_sets {
            println!("  Block {:?}: IN = {:?}", block, exprs);
        }
    }
}

// ============================================================================
// Very Busy Expressions
// ============================================================================

/// Very busy expressions analysis
pub struct VeryBusyExpressions;

impl Analysis for VeryBusyExpressions {
    fn name(&self) -> &'static str {
        "very-busy"
    }

    fn run(&self, function: &Function) -> Box<dyn AnalysisResult> {
        let cfg = ControlFlowGraph::from_function(function);
        Box::new(self.analyze(function, &cfg))
    }
}

impl VeryBusyExpressions {
    fn analyze(&self, function: &Function, cfg: &ControlFlowGraph) -> VeryBusyExpressionsResult {
        let mut result = VeryBusyExpressionsResult {
            in_sets: HashMap::new(),
            out_sets: HashMap::new(),
        };

        // Initialize with empty
        for block in &function.body {
            result.in_sets.insert(block.id, HashSet::new());
            result.out_sets.insert(block.id, HashSet::new());
        }

        // Iterate until fixed point (reverse postorder)
        let mut changed = true;
        while changed {
            changed = false;

            for block in function.body.iter().rev() {
                // OUT[B] = Intersection of IN[S] for all successors S
                let mut new_out: Option<HashSet<ExprHash>> = None;
                for &succ in cfg.successors(block.id) {
                    if let Some(in_set) = result.in_sets.get(&succ) {
                        if let Some(ref mut intersect) = new_out {
                            *intersect = intersect.intersection(in_set).cloned().collect();
                        } else {
                            new_out = Some(in_set.clone());
                        }
                    }
                }

                let new_out = new_out.unwrap_or_default();
                if result.out_sets.get(&block.id) != Some(&new_out) {
                    result.out_sets.insert(block.id, new_out);
                    changed = true;
                }

                // IN[B] = EXPR[B] U (OUT[B] - KILL[B])
                let mut new_in = self.expr_set(block);
                if let Some(out_set) = result.out_sets.get(&block.id) {
                    let killed = self.kill_set(block);
                    for expr in out_set.difference(&killed) {
                        new_in.insert(expr.clone());
                    }
                }

                if result.in_sets.get(&block.id) != Some(&new_in) {
                    result.in_sets.insert(block.id, new_in);
                    changed = true;
                }
            }
        }

        result
    }

    fn expr_set(&self, _block: &BasicBlock) -> HashSet<ExprHash> {
        HashSet::new()
    }

    fn kill_set(&self, _block: &BasicBlock) -> HashSet<ExprHash> {
        HashSet::new()
    }
}

#[derive(Debug, Clone)]
pub struct VeryBusyExpressionsResult {
    pub in_sets: HashMap<BasicBlockId, HashSet<ExprHash>>,
    pub out_sets: HashMap<BasicBlockId, HashSet<ExprHash>>,
}

impl AnalysisResult for VeryBusyExpressionsResult {
    fn print(&self) {
        println!("Very Busy Expressions:");
        for (block, exprs) in &self.in_sets {
            println!("  Block {:?}: IN = {:?}", block, exprs);
        }
    }
}

use std::collections::HashSet;
