// Interpreter

use super::*;

/// Bytecode interpreter
pub struct Interpreter {
    /// Stack for evaluation
    pub stack: Vec<Value>,
    /// Frames for function calls
    frames: Vec<CallFrame>,
    /// Global variables
    globals: HashMap<String, Value>,
    /// Built-in functions
    builtins: HashMap<String, BuiltinFunc>,
}

#[derive(Debug, Clone)]
pub struct CallFrame {
    pub function: FunctionId,
    pub return_address: usize,
    pub locals: Vec<Value>,
}

pub type BuiltinFunc = fn(&[Value]) -> Value;

impl Interpreter {
    pub fn new() -> Self {
        let mut interpreter = Self {
            stack: Vec::new(),
            frames: Vec::new(),
            globals: HashMap::new(),
            builtins: HashMap::new(),
        };
        interpreter.init_builtins();
        interpreter
    }

    fn init_builtins(&mut self) {
        self.builtins.insert("print".to_string(), |args| {
            for arg in args {
                print!("{:?}", arg);
            }
            Value::Unit
        });

        self.builtins.insert("println".to_string(), |args| {
            for arg in args {
                println!("{:?}", arg);
            }
            Value::Unit
        });

        self.builtins.insert("len".to_string(), |args| {
            if let Some(Value::Tuple(v)) = args.first() {
                Value::Int(v.len() as i64)
            } else {
                Value::Int(0)
            }
        });
    }

    /// Execute a function
    pub fn execute(&mut self, function: &Function, args: &[Value]) -> Result<Value, RuntimeError> {
        // Create initial frame
        let mut frame = CallFrame {
            function: function.id,
            return_address: 0,
            locals: args.to_vec(),
        };

        // Pad with default values for missing args
        while frame.locals.len() < function.params.len() {
            frame.locals.push(Value::Unit);
        }

        self.frames.push(frame);

        // Execute each basic block
        for block in &function.body {
            if let Err(e) = self.execute_block(block) {
                self.frames.pop();
                return Err(e);
            }
        }

        // Get return value
        let result = self.stack.pop().unwrap_or(Value::Unit);
        self.frames.pop();
        Ok(result)
    }

    fn execute_block(&mut self, block: &BasicBlock) -> Result<(), RuntimeError> {
        for inst in &block.instructions {
            self.execute_instruction(inst)?;
        }

        // Execute terminator
        match &block.terminator {
            Terminator::Ret(val) => {
                if let Some(v) = val {
                    let result = self.get_value(*v)?;
                    self.stack.push(result);
                }
            }
            Terminator::Br(target) => {
                // Would jump to target block
                let _ = target;
            }
            Terminator::BrCond(cond, then, else_) => {
                let cond_val = self.get_value(*cond)?;
                if matches!(cond_val, Value::Bool(true)) {
                    let _ = else_;
                    // Jump to then
                } else {
                    let _ = then;
                    // Jump to else
                }
            }
            Terminator::Unreachable => {
                return Err(RuntimeError::TypeError("unreachable".to_string()));
            }
            _ => {}
        }

        Ok(())
    }

    fn execute_instruction(&mut self, inst: &Instruction) -> Result<(), RuntimeError> {
        match &inst.op {
            Operation::Binary(op, lhs, rhs) => {
                let lhs_val = self.get_value(*lhs)?;
                let rhs_val = self.get_value(*rhs)?;
                let result = self.apply_binary_op(op, lhs_val, rhs_val)?;
                self.stack.push(result);
            }
            Operation::Unary(op, operand) => {
                let val = self.get_value(*operand)?;
                let result = self.apply_unary_op(op, val)?;
                self.stack.push(result);
            }
            Operation::Const => {
                // Would extract constant value from metadata
                self.stack.push(Value::Int(0));
            }
            Operation::Alloca(_, _) => {
                // Allocate on interpreter heap
                self.stack.push(Value::Pointer(std::ptr::null_mut()));
            }
            Operation::Load(_, ptr, _, _) => {
                let ptr_val = self.get_value(*ptr)?;
                if let Value::Pointer(ptr) = ptr_val {
                    let val = self.load_memory(ptr);
                    self.stack.push(val);
                }
            }
            Operation::Store(_, ptr, val, _) => {
                let ptr_val = self.get_value(*ptr)?;
                let val_val = self.get_value(*val)?;
                if let Value::Pointer(ptr) = ptr_val {
                    self.store_memory(ptr, val_val);
                }
            }
            Operation::Call(func_id, args) => {
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.get_value(*arg)?);
                }

                // Check if it's a builtin
                let name = format!("func_{:?}", func_id);
                if let Some(builtin) = self.builtins.get(&name) {
                    let result = builtin(&arg_values);
                    self.stack.push(result);
                }
            }
            Operation::Cast(val, target_ty, _) => {
                let val = self.get_value(*val)?;
                let casted = self.cast_value(val, target_ty)?;
                self.stack.push(casted);
            }
            _ => {
                // Unknown instruction - push dummy value
                self.stack.push(Value::Unit);
            }
        }

        Ok(())
    }

    fn get_value(&self, value: ValueId) -> Result<Value, RuntimeError> {
        // Would look up value from stack/registers
        Ok(Value::Int(0))
    }

    fn apply_binary_op(&self, op: &BinOp, lhs: Value, rhs: Value) -> Result<Value, RuntimeError> {
        match (op, lhs, rhs) {
            (BinOp::Add, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a + b)),
            (BinOp::Sub, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a - b)),
            (BinOp::Mul, Value::Int(a), Value::Int(b)) => Ok(Value::Int(a * b)),
            (BinOp::Div, Value::Int(a), Value::Int(b)) => {
                if b == 0 {
                    Err(RuntimeError::DivisionByZero)
                } else {
                    Ok(Value::Int(a / b))
                }
            }
            (BinOp::Eq, lhs, rhs) => Ok(Value::Bool(lhs == rhs)),
            (BinOp::Lt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            _ => Err(RuntimeError::TypeError(format!("invalid binary op"))),
        }
    }

    fn apply_unary_op(&self, op: &UnOp, val: Value) -> Result<Value, RuntimeError> {
        match (op, val) {
            (UnOp::Neg, Value::Int(a)) => Ok(Value::Int(-a)),
            (UnOp::Not, Value::Bool(a)) => Ok(Value::Bool(!a)),
            _ => Err(RuntimeError::TypeError(format!("invalid unary op"))),
        }
    }

    fn cast_value(&self, val: Value, _ty: &IrType) -> Result<Value, RuntimeError> {
        Ok(val)
    }

    fn load_memory(&self, _ptr: *mut u8) -> Value {
        Value::Int(0)
    }

    fn store_memory(&mut self, _ptr: *mut u8, _val: Value) {}
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple stack machine for bytecode
pub struct StackMachine {
    pub stack: Vec<StackValue>,
}

#[derive(Debug, Clone)]
pub enum StackValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Ref(usize),
    Unit,
}

impl StackMachine {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }

    pub fn push(&mut self, val: StackValue) {
        self.stack.push(val);
    }

    pub fn pop(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }

    pub fn peek(&self) -> Option<&StackValue> {
        self.stack.last()
    }
}

impl Default for StackMachine {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;
