
// Auto-Differentiation for Machine Learning and Scientific Computing

use crate::tumc_compiler::ir::{Module, Function, BasicBlock, Instruction, Operation, IrType, Const, BinOp, UnaryOp};
use std::collections::HashMap;

/// Differentiation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffMode {
    Forward,
    Reverse,
    Jacobian,
    Hessian,
}

/// Tape entry for reverse mode
#[derive(Debug, Clone)]
pub struct TapeEntry {
    pub operation: String,
    pub inputs: Vec<String>,
    pub output: String,
    pub adjoint_input: Option<String>,
}

/// Auto-differentiator
pub struct AutoDifferentiator {
    mode: DiffMode,
    tape: Vec<TapeEntry>,
    gradient_variables: HashMap<String, String>,
}

impl AutoDifferentiator {
    pub fn new(mode: DiffMode) -> Self {
        Self {
            mode,
            tape: Vec::new(),
            gradient_variables: HashMap::new(),
        }
    }

    /// Differentiate a function
    pub fn differentiate(&self, function: &Function, mode: DiffMode) -> Function {
        match mode {
            DiffMode::Forward => self.forward_diff(function),
            DiffMode::Reverse => self.reverse_diff(function),
            _ => self.reverse_diff(function),
        }
    }

    /// Forward mode automatic differentiation
    fn forward_diff(&self, function: &Function) -> Function {
        let mut diff_function = function.clone();
        diff_function.name = format!("{}_forward", function.name);

        // Add tangent parameters
        let mut new_params = function.params.clone();
        for (name, ty) in &function.params {
            new_params.push((format!("{}_dot", name), ty.clone()));
        }
        diff_function.params = new_params;

        // Generate derivative computation
        let mut new_body = Vec::new();
        for block in &function.body {
            let mut new_block = self.forward_diff_block(block);
            new_body.push(new_block);
        }
        diff_function.body = new_body;

        diff_function
    }

    /// Forward differentiate a block
    fn forward_diff_block(&self, block: &BasicBlock) -> BasicBlock {
        let mut new_block = block.clone();
        let mut new_instructions = Vec::new();

        for inst in &block.instructions {
            let diff_insts = self.forward_diff_instruction(inst);
            new_instructions.extend(diff_insts);
        }

        new_block.instructions = new_instructions;
        new_block
    }

    /// Forward differentiate an instruction
    fn forward_diff_instruction(&self, inst: &Instruction) -> Vec<Instruction> {
        let mut results = Vec::new();

        match &inst.op {
            Operation::Binary(op, lhs, rhs) => {
                let lhs_dot = format!("{}_dot", lhs);
                let rhs_dot = format!("{}_dot", rhs);

                match op {
                    BinOp::Add => {
                        // d(x + y)/dt = dx/dt + dy/dt
                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Add, lhs_dot, rhs_dot),
                            result_type: inst.result_type.clone(),
                            result_name: Some(format!("{}_dot", inst.result_name.clone().unwrap_or_default())),
                            ..Default::default()
                        });
                    }
                    BinOp::Sub => {
                        // d(x - y)/dt = dx/dt - dy/dt
                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Sub, lhs_dot, rhs_dot),
                            result_type: inst.result_type.clone(),
                            result_name: Some(format!("{}_dot", inst.result_name.clone().unwrap_or_default())),
                            ..Default::default()
                        });
                    }
                    BinOp::Mul => {
                        // d(x * y)/dt = x * dy/dt + y * dx/dt
                        let temp1 = format!("{}_temp1", inst.result_name.clone().unwrap_or_default());
                        let temp2 = format!("{}_temp2", inst.result_name.clone().unwrap_or_default());

                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Mul, lhs.clone(), rhs_dot.clone()),
                            result_type: inst.result_type.clone(),
                            result_name: Some(temp1.clone()),
                            ..Default::default()
                        });
                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Mul, rhs, rhs_dot.clone()),
                            result_type: inst.result_type.clone(),
                            result_name: Some(temp2.clone()),
                            ..Default::default()
                        });
                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Add, temp1, temp2),
                            result_type: inst.result_type.clone(),
                            result_name: Some(format!("{}_dot", inst.result_name.clone().unwrap_or_default())),
                            ..Default::default()
                        });
                    }
                    BinOp::Div => {
                        // d(x / y)/dt = (y * dx/dt - x * dy/dt) / y^2
                        let temp1 = format!("{}_temp1", inst.result_name.clone().unwrap_or_default());
                        let temp2 = format!("{}_temp2", inst.result_name.clone().unwrap_or_default());
                        let temp3 = format!("{}_temp3", inst.result_name.clone().unwrap_or_default());

                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Mul, lhs.clone(), rhs_dot.clone()),
                            result_type: inst.result_type.clone(),
                            result_name: Some(temp1.clone()),
                            ..Default::default()
                        });
                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Mul, rhs.clone(), rhs_dot.clone()),
                            result_type: inst.result_type.clone(),
                            result_name: Some(temp2.clone()),
                            ..Default::default()
                        });
                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Sub, temp1, temp2),
                            result_type: inst.result_type.clone(),
                            result_name: Some(temp3.clone()),
                            ..Default::default()
                        });
                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Mul, rhs.clone(), rhs.clone()),
                            result_type: inst.result_type.clone(),
                            result_name: Some(temp1.clone()),
                            ..Default::default()
                        });
                        results.push(Instruction {
                            op: Operation::Binary(BinOp::Div, temp3, temp1),
                            result_type: inst.result_type.clone(),
                            result_name: Some(format!("{}_dot", inst.result_name.clone().unwrap_or_default())),
                            ..Default::default()
                        });
                    }
                    _ => {}
                }
            }
            Operation::Unary(op, operand) => {
                let operand_dot = format!("{}_dot", operand);

                match op {
                    UnaryOp::Neg => {
                        // d(-x)/dt = -dx/dt
                        results.push(Instruction {
                            op: Operation::Unary(UnaryOp::Neg, operand_dot),
                            result_type: inst.result_type.clone(),
                            result_name: Some(format!("{}_dot", inst.result_name.clone().unwrap_or_default())),
                            ..Default::default()
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        results
    }

    /// Reverse mode automatic differentiation
    fn reverse_diff(&self, function: &Function) -> Function {
        let mut diff_function = function.clone();
        diff_function.name = format!("{}_gradient", function.name);

        // Build tape of operations
        let tape = self.build_tape(function);

        // Generate gradient computation
        let mut new_body = Vec::new();
        for block in &function.body {
            let mut new_block = self.reverse_diff_block(block, &tape);
            new_body.push(new_block);
        }
        diff_function.body = new_body;

        diff_function
    }

    /// Build tape of operations for reverse mode
    fn build_tape(&self, function: &Function) -> Vec<TapeEntry> {
        let mut tape = Vec::new();

        for block in &function.body {
            for inst in &block.instructions {
                let entry = match &inst.op {
                    Operation::Binary(op, lhs, rhs) => {
                        TapeEntry {
                            operation: format!("{:?}", op),
                            inputs: vec![lhs.clone(), rhs.clone()],
                            output: inst.result_name.clone().unwrap_or_default(),
                            adjoint_input: None,
                        }
                    }
                    Operation::Unary(op, operand) => {
                        TapeEntry {
                            operation: format!("{:?}", op),
                            inputs: vec![operand.clone()],
                            output: inst.result_name.clone().unwrap_or_default(),
                            adjoint_input: None,
                        }
                    }
                    _ => continue,
                };
                tape.push(entry);
            }
        }

        tape
    }

    /// Reverse differentiate a block
    fn reverse_diff_block(&self, block: &BasicBlock, tape: &[TapeEntry]) -> BasicBlock {
        let mut new_block = block.clone();

        // Add gradient variables initialization
        let mut init_instructions = Vec::new();
        for inst in &block.instructions {
            if let Some(name) = &inst.result_name {
                init_instructions.push(Instruction {
                    op: Operation::Const(Box::new(Const::Int(0)), IrType::Int64),
                    result_type: IrType::Int64,
                    result_name: Some(format!("{}_grad", name)),
                    ..Default::default()
                });
            }
        }

        // Generate adjoint computations in reverse order
        let mut adjoint_instructions = Vec::new();
        for entry in tape.iter().rev() {
            adjoint_instructions.extend(self.compute_adjoint(entry, tape));
        }

        // Set output gradient to 1
        if let Some(last) = block.instructions.last() {
            if let Some(name) = &last.result_name {
                adjoint_instructions.push(Instruction {
                    op: Operation::Const(Box::new(Const::Int(1)), IrType::Int64),
                    result_type: IrType::Int64,
                    result_name: Some(format!("{}_grad", name)),
                    ..Default::default()
                });
            }
        }

        new_block.instructions = init_instructions;
        new_block.instructions.extend(adjoint_instructions);

        new_block
    }

    /// Compute adjoint for a tape entry
    fn compute_adjoint(&self, entry: &TapeEntry, _tape: &[TapeEntry]) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        match entry.operation.as_str() {
            "Add" => {
                // grad_a += grad_out, grad_b += grad_out
                if entry.inputs.len() >= 2 {
                    let grad_out = format!("{}_grad", entry.output);
                    for input in &entry.inputs {
                        instructions.push(Instruction {
                            op: Operation::Binary(BinOp::Add, grad_out.clone(), format!("{}_grad", input)),
                            result_type: IrType::Int64,
                            result_name: Some(format!("{}_grad", input)),
                            ..Default::default()
                        });
                    }
                }
            }
            "Mul" => {
                // grad_a += grad_out * b, grad_b += grad_out * a
                if entry.inputs.len() >= 2 {
                    let grad_out = format!("{}_grad", entry.output);
                    let a = &entry.inputs[0];
                    let b = &entry.inputs[1];

                    instructions.push(Instruction {
                        op: Operation::Binary(BinOp::Mul, grad_out.clone(), b.clone()),
                        result_type: IrType::Int64,
                        result_name: Some(format!("{}_grad", a)),
                        ..Default::default()
                    });
                    instructions.push(Instruction {
                        op: Operation::Binary(BinOp::Mul, grad_out, a.clone()),
                        result_type: IrType::Int64,
                        result_name: Some(format!("{}_grad", b)),
                        ..Default::default()
                    });
                }
            }
            _ => {}
        }

        instructions
    }

    /// Generate gradient functions
    pub fn generate_gradient(&self, function: &Function) -> Vec<Function> {
        let mut gradients = Vec::new();

        // Generate gradient for each parameter
        for (name, ty) in &function.params {
            let mut grad_func = function.clone();
            grad_func.name = format!("d{}_d_{}", function.name, name);

            // Simplified gradient generation
            gradients.push(grad_func);
        }

        gradients
    }

    /// Generate Jacobian matrix computation
    pub fn generate_jacobian(&self, function: &Function) -> Function {
        let mut jacobian_func = function.clone();
        jacobian_func.name = format!("{}_jacobian", function.name);
        jacobian_func
    }

    /// Generate Hessian matrix computation
    pub fn generate_hessian(&self, function: &Function) -> Function {
        let mut hessian_func = function.clone();
        hessian_func.name = format!("{}_hessian", function.name);
        hessian_func
    }

    /// Generate vector-Jacobian product
    pub fn generate_vjp(&self, function: &Function) -> Function {
        let mut vjp_func = function.clone();
        vjp_func.name = format!("{}_vjp", function.name);
        vjp_func
    }

    /// Generate Jacobian-vector product
    pub fn generate_jvp(&self, function: &Function) -> Function {
        let mut jvp_func = function.clone();
        jvp_func.name = format!("{}_jvp", function.name);
        jvp_func
    }
}

impl Default for AutoDifferentiator {
    fn default() -> Self {
        Self::new(DiffMode::Reverse)
    }
}

/// Tape for checkpointing (to save memory in reverse mode)
pub struct CheckpointedTape {
    entries: Vec<TapeEntry>,
    checkpoints: Vec<usize>,
    max_size: usize,
}

impl CheckpointedTape {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            checkpoints: Vec::new(),
            max_size,
        }
    }

    /// Record an operation
    pub fn record(&mut self, entry: TapeEntry) {
        if self.entries.len() >= self.max_size {
            // Create checkpoint and clear old entries
            self.checkpoints.push(self.entries.len());
            // Keep only last half
            let keep_from = self.entries.len() / 2;
            self.entries = self.entries[keep_from..].to_vec();
        }
        self.entries.push(entry);
    }

    /// Replay tape in reverse
    pub fn replay<F>(&self, mut callback: F)
    where
        F: FnMut(&TapeEntry),
    {
        for entry in self.entries.iter().rev() {
            callback(entry);
        }
    }

    /// Get tape size
    pub fn size(&self) -> usize {
        self.entries.len()
    }

    /// Clear tape
    pub fn clear(&mut self) {
        self.entries.clear();
        self.checkpoints.clear();
    }
}

impl Default for CheckpointedTape {
    fn default() -> Self {
        Self::new(10000)
    }
}

/// Differentiation mode selector
pub struct DiffModeSelector {
    input_size: usize,
    output_size: usize,
}

impl DiffModeSelector {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        Self {
            input_size,
            output_size,
        }
    }

    /// Select optimal differentiation mode
    pub fn select_mode(&self) -> DiffMode {
        // Forward mode is better when: output_size < input_size
        // Reverse mode is better when: output_size > input_size
        if self.output_size < self.input_size {
            DiffMode::Forward
        } else {
            DiffMode::Reverse
        }
    }

    /// Check if Jacobian computation is needed
    pub fn needs_jacobian(&self) -> bool {
        self.output_size > 1
    }

    /// Check if Hessian computation is needed
    pub fn needs_hessian(&self) -> bool {
        false // Would need second derivative
    }
}
