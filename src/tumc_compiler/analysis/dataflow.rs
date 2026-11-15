//! Dataflow Analysis Module for TUMC
//! Implements dataflow analysis frameworks and common dataflow analyses.

use crate::tumc_compiler::ir::*;
use std::collections::{HashMap, HashSet};

// ============================================================================
// Dataflow Framework
// ============================================================================

/// Direction of dataflow analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataflowDirection {
    Forward,
    Backward,
}

/// Lattice element for dataflow analysis
pub trait LatticeElement: Clone + Default + PartialEq {
    /// Compute the meet (join) of two elements
    fn meet(&self, other: &Self) -> Self;

    /// Check if this is the top element
    fn is_top(&self) -> bool {
        false
    }

    /// Check if this is the bottom element
    fn is_bottom(&self) -> bool {
        false
    }
}

/// Transfer function for dataflow analysis
pub trait TransferFunction {
    type Domain: LatticeElement;

    /// Apply the transfer function to a basic block
    fn transfer(&self, block: &BasicBlock, input: &Self::Domain) -> Self::Domain;
}

// ============================================================================
// Constant Propagation
// ============================================================================

/// Constant value representation
#[derive(Debug, Clone, PartialEq)]
pub enum ConstValue {
    /// Undefined
    Undefined,
    /// Constant integer
    Int(i64),
    /// Constant float
    Float(f64),
    /// Constant string
    String(String),
    /// Non-constant (cannot be determined)
    NonConstant,
    /// Bottom element
    Bottom,
}

impl Default for ConstValue {
    fn default() -> Self {
        ConstValue::Bottom
    }
}

impl LatticeElement for ConstValue {
    fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (ConstValue::Bottom, _) => other.clone(),
            (_, ConstValue::Bottom) => self.clone(),
            (a, b) if a == b => a.clone(),
            _ => ConstValue::NonConstant,
        }
    }

    fn is_bottom(&self) -> bool {
        matches!(self, ConstValue::Bottom)
    }
}

/// Constant propagation analysis result
pub struct ConstPropResult {
    /// Map from instruction id to constant value
    pub values: HashMap<ValueId, ConstValue>,
}

impl ConstPropResult {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

// ============================================================================
// Reaching Definitions
// ============================================================================

/// Definition site for reaching definitions analysis
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Definition {
    /// Variable name
    pub var: String,
    /// Basic block where definition occurs
    pub block: BasicBlockId,
    /// Instruction index within the block
    pub idx: usize,
}

/// Reaching definitions transfer function
pub struct ReachingDefsTransfer;

impl LatticeElement for HashSet<Definition> {
    /// Meet of two sets of reaching definitions is their union
    fn meet(&self, other: &Self) -> Self {
        let mut result = self.clone();
        result.extend(other.iter().cloned());
        result
    }
}

impl TransferFunction for ReachingDefsTransfer {
    type Domain = HashSet<Definition>;

    fn transfer(&self, block: &BasicBlock, input: &HashSet<Definition>) -> HashSet<Definition> {
        let mut result = input.clone();

        // Add definitions from this block
        for (idx, inst) in block.instructions.iter().enumerate() {
            // Extract defined variables from instruction
            if let Some(dest) = &inst.result_name {
                result.insert(Definition {
                    var: dest.clone(),
                    block: block.id,
                    idx,
                });
            }
        }

        result
    }
}

// ============================================================================
// Live Variables
// ============================================================================

/// Live variable analysis result
pub struct LiveVarResult {
    /// Variables live at entry to each block
    pub in_vars: HashMap<BasicBlockId, HashSet<String>>,
    /// Variables live at exit from each block
    pub out_vars: HashMap<BasicBlockId, HashSet<String>>,
}

impl LiveVarResult {
    pub fn new() -> Self {
        Self {
            in_vars: HashMap::new(),
            out_vars: HashMap::new(),
        }
    }
}

// ============================================================================
// Bit Vector Dataflow
// ============================================================================

/// Generic bit vector lattice element
#[derive(Debug, Clone, Default)]
pub struct BitVector(pub HashSet<usize>);

impl PartialEq for BitVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for BitVector {}

impl LatticeElement for BitVector {
    fn meet(&self, other: &Self) -> Self {
        BitVector(self.0.intersection(&other.0).cloned().collect())
    }
}

/// Generic dataflow solver
pub struct DataflowSolver<D> {
    pub direction: D,
}

impl<D> DataflowSolver<D> {
    pub fn new(direction: D) -> Self {
        Self { direction }
    }
}

// ============================================================================
// Worklist Algorithm
// ============================================================================

/// Worklist-based dataflow analysis
pub struct WorklistAlgorithm {
    /// Worklist of blocks to process
    worklist: Vec<BasicBlockId>,
    /// Set of blocks in worklist
    in_worklist: HashSet<BasicBlockId>,
}

impl WorklistAlgorithm {
    pub fn new() -> Self {
        Self {
            worklist: Vec::new(),
            in_worklist: HashSet::new(),
        }
    }

    /// Add block to worklist
    pub fn add_to_worklist(&mut self, block: BasicBlockId) {
        if self.in_worklist.insert(block) {
            self.worklist.push(block);
        }
    }

    /// Get next block from worklist
    pub fn pop(&mut self) -> Option<BasicBlockId> {
        self.worklist.pop()
    }

    /// Check if worklist is empty
    pub fn is_empty(&self) -> bool {
        self.worklist.is_empty()
    }
}

impl Default for WorklistAlgorithm {
    fn default() -> Self {
        Self::new()
    }
}

// Re-export commonly used types
pub use super::points_to::PointsToResult;
pub use super::escape::EscapeResult;
pub use super::alias::AliasResult;
pub use super::call_graph::CallGraph;
