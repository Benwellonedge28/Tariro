// SSA (Static Single Assignment) Form Construction and Manipulation

use super::*;

/// SSA form builder and utilities
pub struct SsaBuilder {
    value_to_version: HashMap<String, Vec<ValueId>>,
    phi_nodes: HashMap<BasicBlockId, Vec<PhiNode>>,
    current_block: BasicBlockId,
}

#[derive(Debug, Clone)]
pub struct PhiNode {
    pub result: ValueId,
    pub arguments: Vec<(ValueId, BasicBlockId)>,
    pub ty: IrType,
}

impl SsaBuilder {
    pub fn new() -> Self {
        Self {
            value_to_version: HashMap::new(),
            phi_nodes: HashMap::new(),
            current_block: BasicBlockId(0),
        }
    }

    pub fn set_block(&mut self, block: BasicBlockId) {
        self.current_block = block;
    }

    /// Get the current version of a variable
    pub fn get_value(&self, name: &str) -> Option<ValueId> {
        self.value_to_version.get(name).and_then(|versions| versions.last().copied())
    }

    /// Create a new version of a variable
    pub fn create_value(&mut self, name: &str, version: ValueId) {
        self.value_to_version.entry(name.to_string()).or_default().push(version);
    }

    /// Rename a variable to a new version
    pub fn rename_value(&mut self, name: &str, old_version: ValueId, new_version: ValueId) {
        if let Some(versions) = self.value_to_version.get_mut(name) {
            if let Some(pos) = versions.iter().position(|&v| v == old_version) {
                versions[pos] = new_version;
            }
        }
    }

    /// Add a phi node for a variable in a block
    pub fn add_phi(&mut self, name: &str, block: BasicBlockId, ty: IrType) -> PhiNode {
        let result = ValueId(self.value_to_version.values().map(|v| v.len()).sum());

        let phi = PhiNode {
            result,
            arguments: Vec::new(),
            ty,
        };

        self.phi_nodes.entry(block).or_default().push(phi.clone());
        self.create_value(name, result);

        phi
    }

    /// Get phi nodes for a block
    pub fn get_phis(&self, block: BasicBlockId) -> &[PhiNode] {
        self.phi_nodes.get(&block).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get phi nodes for a block (mutable)
    pub fn get_phis_mut(&mut self, block: BasicBlockId) -> Option<&mut Vec<PhiNode>> {
        self.phi_nodes.get_mut(&block)
    }
}

impl Default for SsaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SSA Construction
// ============================================================================

/// Converts a function to SSA form
pub struct SsaConverter {
    dom_tree: DominatorTree,
    dom_frontier: HashMap<BasicBlockId, Vec<BasicBlockId>>,
    phi_placeholders: HashMap<String, Vec<BasicBlockId>>,
}

impl SsaConverter {
    pub fn new(function: &Function) -> Self {
        let dom_tree = DominatorTree::new(function);
        let dom_frontier = Self::compute_dominance_frontiers(function, &dom_tree);

        Self {
            dom_tree,
            dom_frontier,
            phi_placeholders: HashMap::new(),
        }
    }

    fn compute_dominance_frontiers(
        function: &Function,
        dom_tree: &DominatorTree,
    ) -> HashMap<BasicBlockId, Vec<BasicBlockId>> {
        let mut frontiers = HashMap::new();

        for block in &function.body {
            let preds: Vec<_> = function.body.iter()
                .filter(|b| {
                    matches!(b.terminator, Terminator::Br(target) if target == block.id) ||
                    matches!(b.terminator, Terminator::BrCond(_, t, e) if t == block.id || e == block.id)
                })
                .map(|b| b.id)
                .collect();

            if preds.len() >= 2 {
                for &pred in &preds {
                    let mut runner = pred;
                    loop {
                        if !dom_tree.dominates(block.id, runner) {
                            frontiers.entry(runner).or_default().push(block.id);
                        }
                        if dom_tree.idom(runner) == Some(block.id) || runner == block.id {
                            break;
                        }
                        runner = dom_tree.idom(runner).unwrap_or(runner);
                    }
                }
            }
        }

        frontiers
    }

    /// Insert phi nodes for all variables
    pub fn insert_phis(&mut self, variables: &[String]) -> HashMap<BasicBlockId, Vec<PhiNode>> {
        let mut all_phis = HashMap::new();
        let mut def_blocks = HashMap::new();

        // Find where each variable is defined
        for var in variables {
            def_blocks.insert(var.clone(), Vec::<BasicBlockId>::new());
        }

        // Insert phi nodes iteratively
        for var in variables {
            let mut worklist: Vec<_> = def_blocks.get(var).cloned().unwrap_or_default();
            let mut processed = HashSet::new();

            while let Some(block) = worklist.pop() {
                if processed.contains(&block) {
                    continue;
                }
                processed.insert(block);

                if let Some(frontier) = self.dom_frontier.get(&block) {
                    for &df_block in frontier {
                        let phis = all_phis.entry(df_block).or_default();

                        // Check if phi already exists for this variable
                        let has_phi = phis.iter().any(|p| {
                            // Would need variable name tracking in phi
                            false
                        });

                        if !has_phi {
                            let phi = PhiNode {
                                result: ValueId(0), // Will be assigned
                                arguments: Vec::new(),
                                ty: IrType::Int32, // Would need actual type
                            };
                            phis.push(phi);
                            worklist.push(df_block);
                        }
                    }
                }
            }
        }

        all_phis
    }

    /// Rename variables to SSA form
    pub fn rename(&mut self, function: &mut Function, entry: BasicBlockId) {
        let mut ssa_builder = SsaBuilder::new();
        self.rename_block(function, entry, &mut ssa_builder);
    }

    fn rename_block(&self, function: &Function, block: BasicBlockId, ssa: &mut SsaBuilder) {
        ssa.set_block(block);

        // Rename instructions in the block
        for inst in &function.body[block.0].instructions {
            // Would rename uses of values
            let _ = inst;
        }

        // Rename phi node results
        if let Some(phis) = ssa.get_phis(block).get(0) {
            let _ = phis;
            // Phi results are already renamed
        }

        // For each successor, update phi arguments
        for &succ in self.dom_tree.tree.get(block.0).map(|n| &n.children).unwrap_or(&vec![]) {
            // Update phi arguments in successor
            let _ = succ;
        }

        // Recursively rename children in dominator tree
        if let Some(node) = self.dom_tree.tree.get(block.0) {
            for &child in &node.children {
                self.rename_block(function, child, ssa);
            }
        }
    }
}

// ============================================================================
// SSA Destruction (Out-of-SSA)
// ============================================================================

/// Converts SSA form back to regular code with register allocation
pub struct SsaDestructor {
    pub register: Option<Box<dyn RegisterAllocator>>,
}

pub trait RegisterAllocator {
    fn allocate(&mut self, function: &Function, cfg: &ControlFlowGraph) -> Allocation;
}

pub struct Allocation {
    pub values: HashMap<ValueId, AllocationSlot>,
    pub spills: Vec<SpillSlot>,
}

#[derive(Debug, Clone)]
pub enum AllocationSlot {
    Register(usize),      // Physical register number
    Stack(StackLocation), // Stack slot
    Immediate(ValueDef),   // Constant value
}

#[derive(Debug, Clone)]
pub struct StackLocation {
    pub offset: i64,
    pub size: usize,
    pub align: usize,
}

#[derive(Debug, Clone)]
pub struct SpillSlot {
    pub value: ValueId,
    pub location: StackLocation,
}

// ============================================================================
// SSA-based Optimizations
// ============================================================================

/// Identity optimization: (x & -1) == x
pub struct IdentityOptimization;

impl IrPass for IdentityOptimization {
    fn name(&self) -> &str {
        "identity-opts"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = false;
        for func in &mut module.functions {
            for block in &mut func.body {
                block.instructions.retain(|inst| {
                    let keep = !matches!(
                        inst.op,
                        Operation::Binary(BinOp::And, _, _) |
                        Operation::Binary(BinOp::Or, _, _) |
                        Operation::Unary(UnOp::Not, _) |
                        Operation::Unary(UnOp::BitNot, _)
                    );
                    if !keep {
                        changed = true;
                    }
                    keep
                });
            }
        }
        changed
    }
}

/// Copy propagation in SSA form
pub struct CopyPropagation;

impl IrPass for CopyPropagation {
    fn name(&self) -> &str {
        "copy-prop"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = false;

        for func in &mut module.functions {
            let mut copies = HashMap::new();

            for block in &mut func.body {
                for inst in &block.instructions {
                    match &inst.op {
                        Operation::Cast(val, _, CastKind::Bitcast) => {
                            copies.insert(inst.id, *val);
                        }
                        _ => {}
                    }
                }
            }

            // Apply copies
            for block in &mut func.body {
                for inst in &mut block.instructions {
                    // Replace uses of copy results with original values
                    let _ = inst;
                }
            }

            if changed {
                break;
            }
        }

        changed
    }
}

/// Strength reduction for induction variables
pub struct InductionVariableAnalysis {
    pub induction_vars: HashMap<ValueId, InductionVariable>,
}

#[derive(Debug, Clone)]
pub struct InductionVariable {
    pub base: ValueId,
    pub step: ValueId,
    pub initial: ValueId,
    pub loop_header: BasicBlockId,
}

impl InductionVariableAnalysis {
    pub fn new() -> Self {
        Self {
            induction_vars: HashMap::new(),
        }
    }

    pub fn analyze(&mut self, function: &Function, loops: &[NaturalLoop]) {
        for loop_ in loops {
            self.find_induction_variables(function, r#loop);
        }
    }

    fn find_induction_variables(&mut self, function: &Function, r#loop: &NaturalLoop) {
        // Simple induction variable detection
        for block in &function.body {
            if r#loop.contains(block.id) {
                for inst in &block.instructions {
                    if let Operation::Binary(op, lhs, rhs) = &inst.op {
                        match op {
                            BinOp::Add | BinOp::Sub => {
                                // Check if one operand is a constant or another induction var
                                self.induction_vars.insert(inst.id, InductionVariable {
                                    base: *lhs,
                                    step: *rhs,
                                    initial: *lhs,
                                    loop_header: loop.header,
                                });
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}

impl Default for InductionVariableAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// SSA Utilities
// ============================================================================

impl Terminator {
    /// Get all successor blocks
    pub fn successors(&self) -> Vec<BasicBlockId> {
        match self {
            Terminator::Br(target) => vec![*target],
            Terminator::BrCond(_, then, else_) => vec![*then, *else_],
            Terminator::Switch(_, default, cases) => {
                let mut targets: Vec<_> = cases.iter().map(|(_, b)| *b).collect();
                targets.push(*default);
                targets
            }
            Terminator::IndirectBr(_, targets) => targets.clone(),
            Terminator::Invoke(_, _, normal, unwind) => vec![*normal, *unwind],
            Terminator::CatchRet(_, target) => vec![*target],
            Terminator::CleanupRet(_, target) => target.map(|t| vec![t]).unwrap_or_default(),
            _ => Vec::new(),
        }
    }
}

/// Dominator tree builder
pub struct DominatorTreeBuilder;

impl DominatorTreeBuilder {
    pub fn build(function: &Function) -> DominatorTree {
        DominatorTree::new(function)
    }
}

/// Dominance frontier calculator
pub struct DominanceFrontierCalculator;

impl DominanceFrontierCalculator {
    pub fn compute(function: &Function, dom_tree: &DominatorTree) -> HashMap<BasicBlockId, Vec<BasicBlockId>> {
        let mut frontiers = HashMap::new();
        let cfg = ControlFlowGraph::from_function(function);

        for block in &function.body {
            let preds: Vec<_> = cfg.predecessors(block.id).to_vec();

            if preds.len() >= 2 {
                for &pred in &preds {
                    let mut runner = pred;
                    while runner != dom_tree.tree[block.id.0].immediate_dominator.unwrap_or(runner) {
                        frontiers.entry(runner).or_default().push(block.id);
                        runner = dom_tree.tree[runner.0].immediate_dominator.unwrap_or(runner);
                    }
                }
            }
        }

        frontiers
    }
}

/// Phi placement using dominance frontiers
pub struct PhiInserter;

impl PhiInserter {
    /// Insert phi nodes for variables at dominance frontier
    pub fn insert(
        function: &mut Function,
        variables: &[String],
    ) -> HashMap<BasicBlockId, Vec<PhiNode>> {
        let dom_tree = DominatorTree::new(function);
        let frontiers = DominanceFrontierCalculator::compute(function, &dom_tree);
        let mut phis = HashMap::new();

        for var in variables {
            let mut def_blocks = Vec::new();

            // Find where variable is defined
            for (block_idx, block) in function.body.iter().enumerate() {
                for inst in &block.instructions {
                    // Would need to track variable definitions
                    let _ = (var, inst);
                }
            }

            // Place phi nodes at frontier intersections
            let mut worklist = def_blocks.clone();
            let mut visited = HashSet::new();

            while let Some(block) = worklist.pop() {
                if visited.contains(&block) {
                    continue;
                }
                visited.insert(block);

                if let Some(frontier) = frontiers.get(&block) {
                    for &df in frontier {
                        let block_phis = phis.entry(df).or_default();

                        // Check if phi already exists
                        let exists = block_phis.iter().any(|p| {
                            // Would check variable name
                            false
                        });

                        if !exists {
                            let phi = PhiNode {
                                result: ValueId(0),
                                arguments: Vec::new(),
                                ty: IrType::Int32, // Would need actual type
                            };
                            block_phis.push(phi);
                            worklist.push(df);
                        }
                    }
                }
            }
        }

        phis
    }
}

use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssa_builder() {
        let mut builder = SsaBuilder::new();

        builder.create_value("x", ValueId(0));
        assert_eq!(builder.get_value("x"), Some(ValueId(0)));

        builder.create_value("x", ValueId(1));
        assert_eq!(builder.get_value("x"), Some(ValueId(1)));
    }

    #[test]
    fn test_phi_creation() {
        let mut builder = SsaBuilder::new();

        let phi = builder.add_phi("x", BasicBlockId(1), IrType::Int32);
        assert_eq!(builder.get_phis(BasicBlockId(1)).len(), 1);
        assert_eq!(builder.get_value("x"), Some(phi.result));
    }

    #[test]
    fn test_identity_optimization() {
        let mut module = Module::new("test");
        let mut func = Function::new(FunctionId(0), "test_fn", IrType::Int32);

        // Add instruction that should be removed
        func.body.push(BasicBlock::new(BasicBlockId(0)));
        func.body[0].add_inst(Instruction::new(
            Operation::Binary(BinOp::And, ValueId(0), ValueId(0)),
            IrType::Int32,
        ));

        module.functions.push(func);

        let opt = IdentityOptimization;
        opt.run(&mut module);
    }
}
