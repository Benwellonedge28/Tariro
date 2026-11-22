// Local Optimizations

use super::*;

/// Constant folding optimization
pub struct ConstantFolding;

impl OptimizationPass for ConstantFolding {
    fn name(&self) -> &'static str {
        "const-fold"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = false;

        for func in &mut module.functions {
            for block in &mut func.body {
                let mut i = 0;
                while i < block.instructions.len() {
                    let inst = &block.instructions[i];
                    let folded = self.fold_instruction(inst);

                    if let Some((value, ty)) = folded {
                        // Replace instruction with constant
                        let new_inst =
                            Instruction::new(Operation::Const(Box::new(value), ty.clone()), ty);
                        block.instructions[i] = new_inst;
                        changed = true;
                    }

                    i += 1;
                }
            }
        }

        changed
    }

    fn description(&self) -> &'static str {
        "Fold constant expressions at compile time"
    }
}

impl ConstantFolding {
    fn fold_instruction(&self, inst: &Instruction) -> Option<(Const, IrType)> {
        match &inst.op {
            Operation::Binary(_op, _lhs, _rhs) => {
                // Would evaluate binary operations with constant operands
                None
            }
            Operation::Unary(_op, _operand) => {
                // Would evaluate unary operations
                None
            }
            Operation::ICmp(_cmp, _lhs, _rhs) => {
                // Would fold comparisons
                None
            }
            _ => None,
        }
    }
}

/// Algebraic simplification
pub struct AlgebraicSimplification;

impl OptimizationPass for AlgebraicSimplification {
    fn name(&self) -> &'static str {
        "algebraic-simp"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = false;

        for func in &mut module.functions {
            for block in &mut func.body {
                let mut i = 0;
                while i < block.instructions.len() {
                    let inst = &block.instructions[i].clone();

                    if let Some(simplified) = self.simplify(&inst.op) {
                        block.instructions[i].op = simplified;
                        changed = true;
                    }

                    i += 1;
                }
            }
        }

        changed
    }

    fn description(&self) -> &'static str {
        "Apply algebraic identities to simplify expressions"
    }
}

impl AlgebraicSimplification {
    fn simplify(&self, op: &Operation) -> Option<Operation> {
        match op {
            // x + 0 = x
            Operation::Binary(BinOp::Add, lhs, rhs) => {
                if *rhs == ValueId(0) {
                    return Some(Operation::Cast(*lhs, IrType::Int32, CastKind::Bitcast));
                }
                None
            }
            // x * 1 = x
            Operation::Binary(BinOp::Mul, lhs, rhs) => {
                if *rhs == ValueId(1) {
                    return Some(Operation::Cast(*lhs, IrType::Int32, CastKind::Bitcast));
                }
                None
            }
            // x * 0 = 0
            Operation::Binary(BinOp::Mul, _, rhs) => {
                if *rhs == ValueId(0) {
                    return Some(Operation::Const(
                        Box::new(Const::Int(0)),
                        IrType::Int32,
                    ));
                }
                None
            }
            _ => None,
        }
    }
}

/// Dead code elimination
pub struct DeadCodeElimination;

impl OptimizationPass for DeadCodeElimination {
    fn name(&self) -> &'static str {
        "dce"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = true;
        let mut total_removed = 0;

        // Iterate until no more changes
        while changed {
            changed = false;

            for func in &mut module.functions {
                // The values the surviving instructions still need.
                let mut needed: HashSet<ValueId> = HashSet::new();
                // The instructions that have to stay.
                let mut live: HashSet<InstId> = HashSet::new();

                // The values the block terminators read are always needed.
                for block in &func.body {
                    for value in terminator_operands(&block.terminator) {
                        needed.insert(value);
                    }
                }

                // An instruction stays when it has a side effect or when its
                // result is still needed.
                loop {
                    let mut changed = false;

                    for block in &func.body {
                        for inst in &block.instructions {
                            if live.contains(&inst.id) {
                                continue;
                            }

                            let result = inst.result_value();
                            if !is_pure_instruction(&inst.op) || needed.contains(&result) {
                                live.insert(inst.id);
                                changed = true;

                                for operand in operation_operands(&inst.op) {
                                    if needed.insert(operand) {
                                        changed = true;
                                    }
                                }
                            }
                        }
                    }

                    if !changed {
                        break;
                    }
                }

                // Remove dead instructions
                for block in &mut func.body {
                    let original_len = block.instructions.len();
                    block.instructions.retain(|inst| {
                        live.contains(&inst.id) || !is_pure_instruction(&inst.op)
                    });

                    if block.instructions.len() < original_len {
                        changed = true;
                        total_removed += original_len - block.instructions.len();
                    }
                }
            }
        }

        total_removed > 0
    }

    fn description(&self) -> &'static str {
        "Remove instructions with no side effects and unused results"
    }
}

/// Common subexpression elimination
pub struct CommonSubexpressionElimination;

impl OptimizationPass for CommonSubexpressionElimination {
    fn name(&self) -> &'static str {
        "cse"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = false;
        let mut available_exprs: HashMap<ExpressionHash, ValueId> = HashMap::new();

        for func in &mut module.functions {
            available_exprs.clear();

            for block in &func.body {
                for inst in &block.instructions {
                    if let Some(hash) = self.hash_instruction(inst) {
                        if let Some(_existing) = available_exprs.get(&hash) {
                            // Replace with existing value
                            changed = true;
                        } else {
                            available_exprs.insert(hash, inst.result_value());
                        }
                    }
                }
            }
        }

        changed
    }

    fn description(&self) -> &'static str {
        "Eliminate redundant computations of identical expressions"
    }
}

impl CommonSubexpressionElimination {
    fn hash_instruction(&self, inst: &Instruction) -> Option<ExpressionHash> {
        match &inst.op {
            Operation::Binary(op, lhs, rhs) => {
                Some(ExpressionHash::Binary(*op, *lhs, *rhs))
            }
            Operation::Unary(op, operand) => {
                Some(ExpressionHash::Unary(*op, *operand))
            }
            _ => None,
        }
    }
}

/// Expression hash for CSE
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExpressionHash {
    Binary(BinOp, ValueId, ValueId),
    Unary(UnOp, ValueId),
    Load(ValueId),
    ICmp(IntCmpOp, ValueId, ValueId),
}

/// Local value numbering
pub struct LocalValueNumbering;

impl OptimizationPass for LocalValueNumbering {
    fn name(&self) -> &'static str {
        "lvn"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = false;

        for func in &mut module.functions {
            for block in &mut func.body {
                let mut value_table: HashMap<ValueNumber, ValueId> = HashMap::new();
                let mut next_vn: usize = 1;

                for inst in block.instructions.iter_mut() {
                    let vn = self.compute_value_number(inst, &value_table, &mut next_vn);
                    if let Some(_existing) = value_table.get(&vn) {
                        // Replace with equivalent value
                        changed = true;
                    }
                    value_table.insert(vn, inst.result_value());
                }
            }
        }

        changed
    }

    fn description(&self) -> &'static str {
        "Assign value numbers to expressions within basic blocks"
    }
}

impl LocalValueNumbering {
    fn compute_value_number(
        &self,
        _inst: &Instruction,
        _table: &HashMap<ValueNumber, ValueId>,
        next_vn: &mut usize,
    ) -> ValueNumber {
        ValueNumber(*next_vn)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueNumber(usize);

/// Instruction combining
pub struct InstructionCombining;

impl OptimizationPass for InstructionCombining {
    fn name(&self) -> &'static str {
        "instcombine"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = false;

        for func in &mut module.functions {
            for block in &mut func.body {
                let mut i = 0;
                while i < block.instructions.len().saturating_sub(1) {
                    let inst1 = &block.instructions[i];
                    let inst2 = &block.instructions[i + 1];

                    if let Some(combined) = self.try_combine(inst1, inst2) {
                        block.instructions[i] = combined;
                        block.instructions.remove(i + 1);
                        changed = true;
                    } else {
                        i += 1;
                    }
                }
            }
        }

        changed
    }

    fn description(&self) -> &'static str {
        "Combine multiple instructions into single, more efficient instructions"
    }
}

impl InstructionCombining {
    fn try_combine(&self, _inst1: &Instruction, _inst2: &Instruction) -> Option<Instruction> {
        // Combine (x + y) + z -> x + (y + z) etc.
        None
    }
}

/// Reassociation
pub struct Reassociation;

impl OptimizationPass for Reassociation {
    fn name(&self) -> &'static str {
        "reassociate"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = false;

        for func in &mut module.functions {
            for block in &mut func.body {
                // Reassociate chains of associative operations
                changed = self.reassociate_block(block) || changed;
            }
        }

        changed
    }

    fn description(&self) -> &'static str {
        "Reorder operations to enable further optimizations"
    }
}

impl Reassociation {
    fn reassociate_block(&self, _block: &mut BasicBlock) -> bool {
        false
    }
}

/// Code sinking
pub struct CodeSinking;

impl OptimizationPass for CodeSinking {
    fn name(&self) -> &'static str {
        "sinking"
    }

    fn run(&self, module: &mut Module) -> bool {
        let mut changed = false;

        for func in &mut module.functions {
            let cfg = ControlFlowGraph::from_function(func);

            for block in &mut func.body {
                // Sink computations to their latest use point
                changed = self.sink_block(block, &cfg) || changed;
            }
        }

        changed
    }

    fn description(&self) -> &'static str {
        "Move instructions closer to their uses to reduce register pressure"
    }
}

impl CodeSinking {
    fn sink_block(&self, _block: &mut BasicBlock, _cfg: &ControlFlowGraph) -> bool {
        false
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

/// The values a terminator reads.
fn terminator_operands(terminator: &Terminator) -> Vec<ValueId> {
    match terminator {
        Terminator::BrCond(value, _, _) => vec![*value],
        Terminator::Switch(value, _, _) => vec![*value],
        Terminator::IndirectBr(value, _) => vec![*value],
        Terminator::Ret(Some(value)) => vec![*value],
        _ => Vec::new(),
    }
}
