// Escape Analysis

use super::*;

/// Escape analysis implementation
pub struct EscapeAnalysis {
    pub context_sensitive: bool,
}

impl Default for EscapeAnalysis {
    fn default() -> Self {
        Self {
            context_sensitive: false,
        }
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
        let call_graph = CallGraph::new();
        call_graph.build_from_module(module);

        // Compute escape information
        for block in &func.body {
            for inst in &block.instructions {
                self.analyze_instruction(inst, &mut result, &cfg, &call_graph);
            }
        }

        // Propagate through CFG
        self.propagate(&mut result, &cfg);

        result
    }

    fn analyze_instruction(
        &self,
        inst: &Instruction,
        result: &mut FunctionEscapeResult,
        cfg: &ControlFlowGraph,
        call_graph: &CallGraph,
    ) {
        match &inst.op {
            Operation::Alloca(ty, _) => {
                // Initially not escaping
                result.escape_level.insert(inst.id, EscapeLevel::Local);
            }
            Operation::Store(_, ptr, val, _) => {
                // Check if stored value escapes
                if let Some(val_escape) = result.escape_level.get(&val) {
                    if matches!(val_escape, EscapeLevel::Escaped) {
                        // Value stored to escaping location
                        result.escape_level.insert(val, EscapeLevel::Escaped);
                    }
                }
            }
            Operation::Call(func_id, args) => {
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

    fn propagate(&self, result: &mut FunctionEscapeResult, cfg: &ControlFlowGraph) {
        // Propagate escape information backward through CFG
        let mut changed = true;
        while changed {
            changed = false;

            for block in cfg.nodes.keys() {
                for &succ in cfg.successors(*block) {
                    // Merge succ's escape info into block
                    if let Some(succ_info) = result.escape_level.get(&succ.0.into()) {
                        for (val, escape) in succ_info {
                            let current = result.escape_level.entry(*val).or_insert(EscapeLevel::Local);
                            if self.is_more_escaped(escape, current) {
                                *current = escape.clone();
                                changed = true;
                            }
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
        Self {
            function_results: HashMap::new(),
        }
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
            let node = CallGraphNode {
                function: func.id,
                calls: Vec::new(),
                called_by: Vec::new(),
            };
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
        self.nodes.get(&func).map(|n| n.calls.clone()).unwrap_or_default()
    }

    pub fn callers(&self, func: FunctionId) -> Vec<FunctionId> {
        self.nodes.get(&func).map(|n| n.called_by.clone()).unwrap_or_default()
    }
}

use std::collections::HashMap;
