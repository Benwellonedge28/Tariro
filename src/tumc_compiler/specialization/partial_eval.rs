// Partial Evaluation for Program Specialization

use crate::tumc_compiler::ir::{Module, Function, BasicBlock, Instruction, Operation, IrType, Const, Terminator};
use std::collections::HashMap;

/// Static value (known at specialization time)
#[derive(Debug, Clone)]
pub enum StaticValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    FunctionRef(String),
    Undefined,
}

impl StaticValue {
    pub fn as_int(&self) -> Option<i64> {
        if let StaticValue::Int(i) = self { Some(*i) } else { None }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let StaticValue::Bool(b) = self { Some(*b) } else { None }
    }

    pub fn is_defined(&self) -> bool {
        !matches!(self, StaticValue::Undefined)
    }
}

/// Dynamic value (unknown at specialization time)
#[derive(Debug, Clone)]
pub enum DynamicValue {
    Unknown(String),
    Computed(String),
}

/// Partial evaluator
pub struct PartialEvaluator {
    static_env: HashMap<String, StaticValue>,
    dynamic_env: HashMap<String, DynamicValue>,
    inline_threshold: usize,
    max_depth: usize,
}

impl PartialEvaluator {
    pub fn new() -> Self {
        Self {
            static_env: HashMap::new(),
            dynamic_env: HashMap::new(),
            inline_threshold: 10,
            max_depth: 10,
        }
    }

    /// Specialize a function with given static arguments
    pub fn specialize_function(
        &self,
        function: &Function,
        static_args: &[StaticValue],
    ) -> Function {
        let mut specialized = function.clone();
        specialized.name = format!("{}_spec", function.name);

        // Set up static environment with arguments
        let mut env = self.static_env.clone();
        for (i, (name, _)) in function.params.iter().enumerate() {
            if let Some(val) = static_args.get(i) {
                env.insert(name.clone(), val.clone());
            }
        }

        // Specialize each block
        let mut new_body = Vec::new();
        for block in &function.body {
            let specialized_block = self.specialize_block(block, &env, 0);
            new_body.push(specialized_block);
        }
        specialized.body = new_body;

        specialized
    }

    /// Specialize a basic block
    fn specialize_block(
        &self,
        block: &BasicBlock,
        env: &HashMap<String, StaticValue>,
        depth: usize,
    ) -> BasicBlock {
        let mut specialized_block = block.clone();
        let mut new_instructions = Vec::new();

        for inst in &block.instructions {
            let specialized = self.specialize_instruction(inst, env, depth);
            new_instructions.extend(specialized);
        }

        specialized_block.instructions = new_instructions;
        specialized_block
    }

    /// Specialize a single instruction
    fn specialize_instruction(
        &self,
        inst: &Instruction,
        env: &HashMap<String, StaticValue>,
        depth: usize,
    ) -> Vec<Instruction> {
        // Check if all operands are static
        let all_static = self.operands_are_static(inst, env);

        if all_static {
            // Try to evaluate statically
            if let Some(value) = self.try_static_eval(inst, env) {
                return vec![self.create_const_instruction(inst, value)];
            }
        }

        // Return original instruction with environment update
        vec![inst.clone()]
    }

    /// Check if all operands are static
    fn operands_are_static(&self, inst: &Instruction, env: &HashMap<String, StaticValue>) -> bool {
        match &inst.op {
            Operation::Binary(_, lhs, rhs) => {
                env.contains_key(lhs) && env.contains_key(rhs)
            }
            Operation::Unary(_, operand) => {
                env.contains_key(operand)
            }
            _ => false,
        }
    }

    /// Try to evaluate instruction statically
    fn try_static_eval(
        &self,
        inst: &Instruction,
        env: &HashMap<String, StaticValue>,
    ) -> Option<StaticValue> {
        match &inst.op {
            Operation::Binary(op, lhs, rhs) => {
                let lval = env.get(lhs)?.as_int()?;
                let rval = env.get(rhs)?.as_int()?;
                let result = match op {
                    crate::tumc_compiler::ir::BinOp::Add => lval + rval,
                    crate::tumc_compiler::ir::BinOp::Sub => lval - rval,
                    crate::tumc_compiler::ir::BinOp::Mul => lval * rval,
                    crate::tumc_compiler::ir::BinOp::Div => lval / rval,
                    crate::tumc_compiler::ir::BinOp::Mod => lval % rval,
                    crate::tumc_compiler::ir::BinOp::And => lval & rval,
                    crate::tumc_compiler::ir::BinOp::Or => lval | rval,
                    crate::tumc_compiler::ir::BinOp::Xor => lval ^ rval,
                    _ => return None,
                };
                Some(StaticValue::Int(result))
            }
            Operation::Unary(op, operand) => {
                let val = env.get(operand)?.as_int()?;
                let result = match op {
                    crate::tumc_compiler::ir::UnaryOp::Neg => -val,
                    crate::tumc_compiler::ir::UnaryOp::Not => !val,
                    _ => return None,
                };
                Some(StaticValue::Int(result))
            }
            _ => None,
        }
    }

    /// Create a constant instruction
    fn create_const_instruction(&self, inst: &Instruction, value: StaticValue) -> Instruction {
        let const_val = match value {
            StaticValue::Int(i) => Const::Int(i),
            StaticValue::Bool(b) => Const::Int(if b { 1 } else { 0 }),
            _ => Const::Int(0),
        };

        Instruction {
            op: Operation::Const(Box::new(const_val), IrType::Int64),
            result_type: IrType::Int64,
            result_name: inst.result_name.clone(),
            ..Default::default()
        }
    }

    /// Specialize a conditional branch
    pub fn specialize_branch(
        &self,
        condition: &str,
        then_block: &BasicBlock,
        else_block: &BasicBlock,
        env: &HashMap<String, StaticValue>,
    ) -> Vec<BasicBlock> {
        if let Some(val) = env.get(condition) {
            if let Some(true) = val.as_bool() {
                // Only then block is reachable
                return vec![then_block.clone()];
            } else if let Some(false) = val.as_bool() {
                // Only else block is reachable
                return vec![else_block.clone()];
            }
        }

        // Both branches may be reachable
        vec![then_block.clone(), else_block.clone()]
    }

    /// Specialize a loop
    pub fn specialize_loop(
        &self,
        header: &BasicBlock,
        body: &[BasicBlock],
        env: &HashMap<String, StaticValue>,
    ) -> Vec<BasicBlock> {
        // Check if loop bound is static
        let mut new_env = env.clone();

        // Unroll small loops
        let loop_body = body.to_vec();
        let mut unrolled = Vec::new();

        // Simplified: just return original loop structure
        unrolled.push(header.clone());
        unrolled.extend(loop_body);

        unrolled
    }

    /// Inline a function call
    pub fn inline_call(
        &self,
        callee: &Function,
        caller: &Function,
        args: &[String],
        call_site: usize,
    ) -> Function {
        let mut inlined = caller.clone();
        let mut inlined_body = Vec::new();

        // Substitute arguments
        let mut subst = HashMap::new();
        for (i, (param, _)) in callee.params.iter().enumerate() {
            if let Some(arg) = args.get(i) {
                subst.insert(param.clone(), arg.clone());
            }
        }

        // Inline function body
        for (i, block) in callee.body.iter().enumerate() {
            let mut new_block = block.clone();
            for inst in &block.instructions {
                let mut new_inst = inst.clone();
                // Substitute variables
                new_inst.op = self.substitute_operation(&inst.op, &subst);
                new_block.instructions.push(new_inst);
            }
            inlined_body.push(new_block);
        }

        inlined.body = inlined_body;
        inlined
    }

    /// Substitute variables in an operation
    fn substitute_operation(
        &self,
        op: &Operation,
        subst: &HashMap<String, String>,
    ) -> Operation {
        match op {
            Operation::Binary(binop, lhs, rhs) => {
                let new_lhs = subst.get(lhs).cloned().unwrap_or_else(|| lhs.clone());
                let new_rhs = subst.get(rhs).cloned().unwrap_or_else(|| rhs.clone());
                Operation::Binary(*binop, new_lhs, new_rhs)
            }
            _ => op.clone(),
        }
    }

    /// Get binding time analysis info
    pub fn get_binding_time(&self, function: &Function) -> HashMap<String, BindingTime> {
        let mut binding_times = HashMap::new();

        for (name, _) in &function.params {
            binding_times.insert(name.clone(), BindingTime::Static);
        }

        for block in &function.body {
            for inst in &block.instructions {
                if let Some(name) = &inst.result_name {
                    if !binding_times.contains_key(name) {
                        binding_times.insert(name.clone(), BindingTime::Dynamic);
                    }
                }
            }
        }

        binding_times
    }
}

impl Default for PartialEvaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Binding time (when value is known)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingTime {
    Static,
    Dynamic,
    CompileTime,
    Runtime,
}

/// Residual code generator
pub struct ResidualGenerator {
    pub preserve_types: bool,
    pub generate_debug_info: bool,
}

impl ResidualGenerator {
    pub fn new() -> Self {
        Self {
            preserve_types: true,
            generate_debug_info: false,
        }
    }

    /// Generate residual function with dynamic values
    pub fn generate_residual(
        &self,
        original: &Function,
        static_values: &HashMap<String, StaticValue>,
    ) -> Function {
        let mut residual = original.clone();
        residual.name = format!("{}_residual", original.name);

        // Generate residual instructions
        let mut new_body = Vec::new();
        for block in &original.body {
            let mut new_block = block.clone();
            new_block.instructions = block
                .instructions
                .iter()
                .filter(|inst| !self.is_static_computable(inst, static_values))
                .cloned()
                .collect();
            new_body.push(new_block);
        }

        residual.body = new_body;
        residual
    }

    /// Check if instruction can be computed statically
    fn is_static_computable(
        &self,
        inst: &Instruction,
        static_values: &HashMap<String, StaticValue>,
    ) -> bool {
        match &inst.op {
            Operation::Const(_, _) => true,
            Operation::Binary(_, lhs, rhs) => {
                static_values.contains_key(lhs) && static_values.contains_key(rhs)
            }
            _ => false,
        }
    }
}

impl Default for ResidualGenerator {
    fn default() -> Self {
        Self::new()
    }
}
