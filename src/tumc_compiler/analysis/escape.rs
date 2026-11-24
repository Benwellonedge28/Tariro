// Escape Analysis

use super::*;
use crate::tumc_compiler::ir::ValueId;

/// Escape analysis implementation
pub struct EscapeAnalysis {
    pub context_sensitive: bool,
}

impl Default for EscapeAnalysis {
    fn default() -> Self {
        Self { context_sensitive: false }
    }
}

impl EscapeAnalysis {
    pub fn new(context_sensitive: bool) -> Self {
        Self { context_sensitive }
    }

    pub fn analyze(&self, module: &Module) -> EscapeResult {
        let mut result = EscapeResult::new();

        for func in &module.functions {
            let func_result = self.analyze_function(func, module);
            result.function_results.insert(func.id, func_result);
        }

        result
    }

    fn analyze_function(&self, func: &Function, module: &Module) -> FunctionEscapeResult {
        let mut result = FunctionEscapeResult::default();
        let cfg = ControlFlowGraph::from_function(func);

        // Build call graph for interprocedural analysis
        let mut call_graph = CallGraph::new();
        call_graph.build_from_module(module);

        // Compute escape information
        for block in &func.body {
            for inst in &block.instructions {
                self.analyze_instruction(inst, &mut result, &cfg, &call_graph);
            }
        }

        // Propagate through CFG
        self.propagate(func, &mut result, &cfg);

        result
    }

    fn analyze_instruction(
        &self,
        inst: &Instruction,
        result: &mut FunctionEscapeResult,
        _cfg: &ControlFlowGraph,
        _call_graph: &CallGraph,
    ) {
        match &inst.op {
            Operation::Alloca(_ty, _) => {
                // Initially not escaping
                result
                    .escape_level
                    .insert(inst.result_value(), EscapeLevel::Local);
            }
            Operation::Store(_, _ptr, val, _) => {
                // Check if stored value escapes
                if let Some(val_escape) = result.escape_level.get(&val) {
                    if matches!(val_escape, EscapeLevel::Escaped) {
                        // Value stored to escaping location
                        result.escape_level.insert(*val, EscapeLevel::Escaped);
                    }
                }
            }
            Operation::Call(_func_id, args) => {
                // Check if any arguments escape through call
                for arg in args {
                    if let Some(arg_escape) = result.escape_level.get(arg) {
                        if !matches!(arg_escape, EscapeLevel::Local) {
                            // Argument may escape through function call
                            result.escape_level.insert(*arg, EscapeLevel::Escaped);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn propagate(
        &self,
        func: &Function,
        result: &mut FunctionEscapeResult,
        cfg: &ControlFlowGraph,
    ) {
        // The values every block defines and the values it reads.
        let mut defined: HashMap<BasicBlockId, Vec<ValueId>> = HashMap::new();
        let mut used: HashMap<BasicBlockId, Vec<ValueId>> = HashMap::new();

        for block in &func.body {
            for inst in &block.instructions {
                defined
                    .entry(block.id)
                    .or_default()
                    .push(inst.result_value());
                for operand in operation_operands(&inst.op) {
                    used.entry(block.id).or_default().push(operand);
                }
            }
        }

        // A value that escapes in a successor escapes in its predecessor too.
        let mut changed = true;
        while changed {
            changed = false;

            for node in cfg.nodes.values() {
                let successors = &node.successors;
                if successors.is_empty() {
                    continue;
                }

                // The strongest escape level of the values the successors read.
                let successor_level = successors
                    .iter()
                    .filter_map(|successor| used.get(successor))
                    .flat_map(|values| values.iter())
                    .filter_map(|value| result.escape_level.get(value))
                    .fold(
                        None,
                        |strongest: Option<&EscapeLevel>, level| match strongest {
                            Some(current) if !self.is_more_escaped(level, current) => Some(current),
                            _ => Some(level),
                        },
                    )
                    .cloned();

                if let Some(level) = successor_level {
                    for value in defined.get(&node.block).cloned().unwrap_or_default() {
                        let current = result
                            .escape_level
                            .entry(value)
                            .or_insert(EscapeLevel::Local);
                        if self.is_more_escaped(&level, current) {
                            *current = level.clone();
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    fn is_more_escaped(&self, a: &EscapeLevel, b: &EscapeLevel) -> bool {
        match (a, b) {
            (EscapeLevel::Escaped, _) => true,
            (EscapeLevel::Global, EscapeLevel::Local) => true,
            (EscapeLevel::Global, EscapeLevel::Arg) => true,
            (EscapeLevel::Arg, EscapeLevel::Local) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EscapeResult {
    pub function_results: HashMap<FunctionId, FunctionEscapeResult>,
}

impl EscapeResult {
    pub fn new() -> Self {
        Self { function_results: HashMap::new() }
    }

    pub fn can_stack_allocate(&self, value: ValueId) -> bool {
        for result in self.function_results.values() {
            if let Some(level) = result.escape_level.get(&value) {
                return matches!(level, EscapeLevel::Local);
            }
        }
        false
    }

    pub fn escape_level(&self, func: FunctionId, value: ValueId) -> EscapeLevel {
        self.function_results
            .get(&func)
            .and_then(|r| r.escape_level.get(&value).cloned())
            .unwrap_or(EscapeLevel::Local)
    }
}

#[derive(Debug, Clone, Default)]
pub struct FunctionEscapeResult {
    pub escape_level: HashMap<ValueId, EscapeLevel>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeLevel {
    /// Does not escape the function
    Local,
    /// Passed as argument but not stored globally
    Arg,
    /// Stored in global or returned
    Global,
    /// Escapes to unknown location
    Escaped,
}

impl Default for EscapeLevel {
    fn default() -> Self {
        EscapeLevel::Local
    }
}

/// Call graph for interprocedural analysis
#[derive(Debug, Clone, Default)]
pub struct CallGraph {
    pub nodes: HashMap<FunctionId, CallGraphNode>,
    pub edges: Vec<(FunctionId, FunctionId)>,
}

#[derive(Debug, Clone)]
pub struct CallGraphNode {
    pub function: FunctionId,
    pub calls: Vec<FunctionId>,
    pub called_by: Vec<FunctionId>,
}

impl CallGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_from_module(&mut self, module: &Module) {
        for func in &module.functions {
            let node =
                CallGraphNode { function: func.id, calls: Vec::new(), called_by: Vec::new() };
            self.nodes.insert(func.id, node);
        }

        // Build edges
        for func in &module.functions {
            for block in &func.body {
                for inst in &block.instructions {
                    if let Operation::Call(callee, _) = &inst.op {
                        self.edges.push((func.id, *callee));
                        if let Some(node) = self.nodes.get_mut(&func.id) {
                            node.calls.push(*callee);
                        }
                        if let Some(node) = self.nodes.get_mut(callee) {
                            node.called_by.push(func.id);
                        }
                    }
                }
            }
        }
    }

    pub fn callees(&self, func: FunctionId) -> Vec<FunctionId> {
        self.nodes
            .get(&func)
            .map(|n| n.calls.clone())
            .unwrap_or_default()
    }

    pub fn callers(&self, func: FunctionId) -> Vec<FunctionId> {
        self.nodes
            .get(&func)
            .map(|n| n.called_by.clone())
            .unwrap_or_default()
    }
}

use std::collections::HashMap;

/// The values an operation reads.
fn operation_operands(op: &Operation) -> Vec<ValueId> {
    match op {
        Operation::Binary(_, lhs, rhs) => vec![*lhs, *rhs],
        Operation::Unary(_, operand) => vec![*operand],
        Operation::Load(_, ptr, _, _) => vec![*ptr],
        Operation::Store(_, value, ptr, _) => vec![*value, *ptr],
        Operation::LoadElementPtr(ptr, indices) => {
            let mut values = vec![*ptr];
            values.extend(indices.iter().copied());
            values
        }
        Operation::LoadFieldPtr(ptr, _) => vec![*ptr],
        Operation::Call(_, arguments) => arguments.clone(),
        Operation::CallIndirect(callee, arguments) => {
            let mut values = vec![*callee];
            values.extend(arguments.iter().copied());
            values
        }
        Operation::Cast(value, _, _) => vec![*value],
        Operation::Bitcast(value, _) => vec![*value],
        Operation::IntToPtr(value) => vec![*value],
        Operation::PtrToInt(value) => vec![*value],
        _ => Vec::new(),
    }
}
