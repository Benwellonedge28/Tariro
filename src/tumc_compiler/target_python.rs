//! TUMC Python Bytecode Generation
//!
//! Code generation for Python bytecode (.pyc files).

use std::collections::HashMap;

/// Python bytecode instruction
#[derive(Debug, Clone)]
pub enum PycInst {
    // Stack operations
    PopTop,
    RotTwo,
    RotThree,
    DupTop,
    DupTopTwo,

    // Binary operations
    BinaryAdd,
    BinarySubtract,
    BinaryMultiply,
    BinaryMatrixMultiply,
    BinaryTrueDivide,
    BinaryFloorDivide,
    BinaryModulo,
    BinaryPower,
    BinaryLshift,
    BinaryRshift,
    BinaryAnd,
    BinaryOr,
    BinaryXor,

    // In-place operations
    InplaceAdd,
    InplaceSubtract,
    InplaceMultiply,
    InplaceMatrixMultiply,
    InplaceTrueDivide,
    InplaceFloorDivide,
    InplaceModulo,
    InplacePower,
    InplaceLshift,
    InplaceRshift,
    InplaceAnd,
    InplaceOr,
    InplaceXor,

    // Subscript and slicing
    SUBSCR,
    StoreSubscr,
    DeleteSubscr,

    // Load constants
    LoadConst(usize),

    // Load names
    LoadFast(usize),
    LoadGlobal(usize),

    // Store names
    StoreFast(usize),
    StoreGlobal(usize),

    // Delete names
    DeleteFast(usize),
    DeleteGlobal(usize),
    DeleteName(usize),

    // Unary operations
    UnaryPositive,
    UnaryNegative,
    UnaryNot,
    UnaryInvert,

    // Comparisons
    CompareOp(usize),

    // Jumps
    JumpForward(i32),
    JumpAbsolute(usize),
    PopJumpIfTrue(usize),
    PopJumpIfFalse(usize),
    JumpIfTrueOrPop(usize),
    JumpIfFalseOrPop(usize),

    // Function calls
    CallFunction(usize),
    CallFunctionKw(usize),
    CallFunctionEx(usize),
    CallMethod(usize),
    LoadMethod(usize),
    CallMethodKw(usize),

    // Returns
    ReturnValue,
    ReturnConst,

    // Imports
    ImportName(usize),
    ImportFrom(usize),
    ImportStar,
    LoadConstant(usize),

    // Exception handling
    SetupFinally(i32),
    EndFinally,
    CleanupSuper,
    WithCleanupStart,
    WithCleanupFinish,
    PopFinally(i32),
    EndFinallyYield,

    // Loops
    GetIter,
    ForIter(i32),
    BreakLoop,
    ContinueLoop(i32),

    // Setup
    SetupWith(i32),
    SetupAsyncWith(i32),
    BeforeWith,
    YieldValue,
    GetAwaitable,
    BeforeAsyncWith,
    GetYieldFromIter,
    YieldFrom,

    // Other
    NOP,
    UnpackSequence(usize),
    UnpackEx(usize, usize),
    BuildString(usize),
    BuildTuple(usize),
    BuildList(usize),
    BuildSet(usize),
    BuildMap(usize),
    BuildConstKeyMap(usize),
    BuildTupleUnpack(usize),
    BuildListUnpack(usize),
    BuildSetUnpack(usize),
    BuildMapUnpack(usize),
    BuildMapUnpackWithCall(usize),
    BuildTupleUnpackWithCall(usize),
    DictMerge(usize),
    DictUpdate(usize),
    ListExtend(usize),
    SetUpdate(usize),
    ListToTuple,
    SetupLoop(i32),
    ExtendedArg(usize),
}

impl PycInst {
    /// Get opcode number
    pub fn opcode(&self) -> u8 {
        match self {
            PycInst::PopTop => 1,
            PycInst::RotTwo => 2,
            PycInst::RotThree => 3,
            PycInst::DupTop => 4,
            PycInst::DupTopTwo => 5,
            PycInst::BinaryAdd => 23,
            PycInst::BinarySubtract => 24,
            PycInst::BinaryMultiply => 20,
            PycInst::BinaryTrueDivide => 27,
            PycInst::BinaryFloorDivide => 26,
            PycInst::BinaryModulo => 22,
            PycInst::BinaryPower => 25,
            PycInst::BinaryLshift => 62,
            PycInst::BinaryRshift => 63,
            PycInst::BinaryAnd => 64,
            PycInst::BinaryOr => 66,
            PycInst::BinaryXor => 65,
            PycInst::InplaceAdd => 55,
            PycInst::InplaceSubtract => 56,
            PycInst::InplaceMultiply => 57,
            PycInst::InplaceTrueDivide => 29,
            PycInst::InplaceFloorDivide => 28,
            PycInst::InplaceModulo => 24,
            PycInst::InplacePower => 67,
            PycInst::InplaceLshift => 75,
            PycInst::InplaceRshift => 76,
            PycInst::InplaceAnd => 77,
            PycInst::InplaceOr => 78,
            PycInst::InplaceXor => 79,
            PycInst::SUBSCR => 60,
            PycInst::StoreSubscr => 61,
            PycInst::DeleteSubscr => 59,
            PycInst::LoadConst(_) => 100,
            PycInst::LoadFast(_) => 125,
            PycInst::LoadGlobal(_) => 116,
            PycInst::StoreFast(_) => 125,
            PycInst::StoreGlobal(_) => 116,
            PycInst::DeleteFast(_) => 126,
            PycInst::DeleteGlobal(_) => 117,
            PycInst::DeleteName(_) => 119,
            PycInst::UnaryPositive => 10,
            PycInst::UnaryNegative => 11,
            PycInst::UnaryNot => 12,
            PycInst::UnaryInvert => 15,
            PycInst::CompareOp(_) => 107,
            PycInst::JumpForward(_) => 110,
            PycInst::JumpAbsolute(_) => 113,
            PycInst::PopJumpIfTrue(_) => 115,
            PycInst::PopJumpIfFalse(_) => 114,
            PycInst::JumpIfTrueOrPop(_) => 112,
            PycInst::JumpIfFalseOrPop(_) => 111,
            PycInst::CallFunction(_) => 131,
            PycInst::CallFunctionKw(_) => 141,
            PycInst::CallFunctionEx(_) => 142,
            PycInst::CallMethod(_) => 161,
            PycInst::LoadMethod(_) => 160,
            PycInst::CallMethodKw(_) => 162,
            PycInst::ReturnValue => 83,
            PycInst::ReturnConst => 135,
            PycInst::ImportName(_) => 108,
            PycInst::ImportFrom(_) => 109,
            PycInst::ImportStar => 84,
            PycInst::GetIter => 68,
            PycInst::ForIter(_) => 93,
            PycInst::BreakLoop => 80,
            PycInst::ContinueLoop(_) => 119,
            PycInst::SetupFinally(_) => 85,
            PycInst::EndFinally => 88,
            PycInst::YieldValue => 72,
            PycInst::GetAwaitable => 73,
            PycInst::YieldFrom => 88,
            PycInst::NOP => 9,
            PycInst::UnpackSequence(_) => 92,
            PycInst::BuildString(_) => 157,
            PycInst::BuildTuple(_) => 102,
            PycInst::BuildList(_) => 103,
            PycInst::BuildSet(_) => 104,
            PycInst::BuildMap(_) => 105,
            PycInst::BuildConstKeyMap(_) => 156,
            PycInst::SetupLoop(_) => 120,
            _ => 0,
        }
    }

    /// Get argument bytes
    pub fn arg_bytes(&self) -> Vec<u8> {
        match self {
            PycInst::LoadConst(idx)
            | PycInst::LoadFast(idx)
            | PycInst::LoadGlobal(idx)
            | PycInst::StoreFast(idx)
            | PycInst::StoreGlobal(idx)
            | PycInst::DeleteFast(idx)
            | PycInst::DeleteGlobal(idx)
            | PycInst::DeleteName(idx)
            | PycInst::BuildTuple(idx)
            | PycInst::BuildList(idx)
            | PycInst::BuildSet(idx)
            | PycInst::BuildMap(idx)
            | PycInst::UnpackSequence(idx)
            | PycInst::JumpAbsolute(idx)
            | PycInst::PopJumpIfTrue(idx)
            | PycInst::PopJumpIfFalse(idx)
            | PycInst::JumpIfTrueOrPop(idx)
            | PycInst::JumpIfFalseOrPop(idx)
            | PycInst::LoadMethod(idx)
            | PycInst::CallFunction(idx)
            | PycInst::CallFunctionKw(idx)
            | PycInst::CallFunctionEx(idx)
            | PycInst::CallMethod(idx)
            | PycInst::CallMethodKw(idx) => idx.to_le_bytes().to_vec(),
            PycInst::JumpForward(offset)
            | PycInst::ForIter(offset)
            | PycInst::SetupFinally(offset)
            | PycInst::ContinueLoop(offset)
            | PycInst::SetupLoop(offset)
            | PycInst::SetupWith(offset)
            | PycInst::SetupAsyncWith(offset) => (offset / 2).to_le_bytes().to_vec(),
            PycInst::CompareOp(op) => (*op as usize).to_le_bytes().to_vec(),
            PycInst::ExtendedArg(arg) => (*arg as usize).to_le_bytes().to_vec(),
            _ => Vec::new(),
        }
    }
}

/// Python bytecode compiler
pub struct PythonBytecodeCompiler {
    /// Instructions
    instructions: Vec<PycInst>,
    /// Constants pool
    constants: Vec<PyConstant>,
    /// Names (variables, functions, etc.)
    names: Vec<String>,
    /// Variable names (local)
    varnames: Vec<String>,
    /// Free variables
    freevars: Vec<String>,
    /// Cell variables
    cellvars: Vec<String>,
    /// Current stack size
    stack_size: usize,
    /// Labels for jumps
    labels: HashMap<String, usize>,
}

impl PythonBytecodeCompiler {
    /// Create new compiler
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            names: Vec::new(),
            varnames: Vec::new(),
            freevars: Vec::new(),
            cellvars: Vec::new(),
            stack_size: 0,
            labels: HashMap::new(),
        }
    }

    /// Emit instruction
    pub fn emit(&mut self, inst: PycInst) {
        let new_stack = self.calc_stack(&inst);
        if new_stack > self.stack_size {
            self.stack_size = new_stack;
        }
        self.instructions.push(inst);
    }

    /// Calculate stack effect
    fn calc_stack(&self, inst: &PycInst) -> usize {
        match inst {
            PycInst::PopTop => self.stack_size.saturating_sub(1),
            PycInst::RotTwo | PycInst::RotThree => self.stack_size,
            PycInst::DupTop => self.stack_size + 1,
            PycInst::BinaryAdd
            | PycInst::BinarySubtract
            | PycInst::BinaryMultiply
            | PycInst::BinaryTrueDivide
            | PycInst::BinaryFloorDivide
            | PycInst::BinaryModulo
            | PycInst::BinaryPower
            | PycInst::BinaryLshift
            | PycInst::BinaryRshift
            | PycInst::BinaryAnd
            | PycInst::BinaryOr
            | PycInst::BinaryXor => self.stack_size.saturating_sub(1),
            PycInst::LoadConst(_) | PycInst::LoadFast(_) | PycInst::LoadGlobal(_) => {
                self.stack_size + 1
            }
            PycInst::StoreFast(_) | PycInst::StoreGlobal(_) => self.stack_size.saturating_sub(1),
            PycInst::ReturnValue => 0,
            PycInst::JumpForward(_) | PycInst::JumpAbsolute(_) => self.stack_size,
            PycInst::PopJumpIfTrue(_) | PycInst::PopJumpIfFalse(_) => {
                self.stack_size.saturating_sub(1)
            }
            PycInst::CallFunction(n) => self.stack_size.saturating_sub(*n),
            PycInst::BuildTuple(n) => self.stack_size.saturating_sub(*n),
            PycInst::BuildList(n) => self.stack_size.saturating_sub(*n),
            PycInst::BuildMap(n) => self.stack_size.saturating_sub(*n * 2),
            _ => self.stack_size,
        }
    }

    /// Define label
    pub fn label(&mut self, name: &str) {
        self.labels
            .insert(name.to_string(), self.instructions.len());
    }

    /// Add constant
    pub fn add_const(&mut self, constant: PyConstant) -> usize {
        // Check if constant already exists
        if let Some(idx) = self.constants.iter().position(|c| c == &constant) {
            return idx;
        }
        self.constants.push(constant);
        self.constants.len() - 1
    }

    /// Add name
    pub fn add_name(&mut self, name: &str) -> usize {
        if let Some(idx) = self.names.iter().position(|n| n == name) {
            return idx;
        }
        self.names.push(name.to_string());
        self.names.len() - 1
    }

    /// Add varname
    pub fn add_varname(&mut self, name: &str) -> usize {
        if let Some(idx) = self.varnames.iter().position(|n| n == name) {
            return idx;
        }
        self.varnames.push(name.to_string());
        self.varnames.len() - 1
    }

    /// Load constant
    pub fn load_const(&mut self, constant: PyConstant) {
        let idx = self.add_const(constant);
        self.emit(PycInst::LoadConst(idx));
    }

    /// Load local variable
    pub fn load_fast(&mut self, name: &str) {
        let idx = self.add_varname(name);
        self.emit(PycInst::LoadFast(idx));
    }

    /// Load global variable
    pub fn load_global(&mut self, name: &str) {
        let idx = self.add_name(name);
        self.emit(PycInst::LoadGlobal(idx));
    }

    /// Store local variable
    pub fn store_fast(&mut self, name: &str) {
        let idx = self.add_varname(name);
        self.emit(PycInst::StoreFast(idx));
    }

    /// Store global variable
    pub fn store_global(&mut self, name: &str) {
        let idx = self.add_name(name);
        self.emit(PycInst::StoreGlobal(idx));
    }

    /// Binary operation
    pub fn binary_op(&mut self, op: &str) {
        match op {
            "+" => self.emit(PycInst::BinaryAdd),
            "-" => self.emit(PycInst::BinarySubtract),
            "*" => self.emit(PycInst::BinaryMultiply),
            "/" => self.emit(PycInst::BinaryTrueDivide),
            "//" => self.emit(PycInst::BinaryFloorDivide),
            "%" => self.emit(PycInst::BinaryModulo),
            "**" => self.emit(PycInst::BinaryPower),
            "<<" => self.emit(PycInst::BinaryLshift),
            ">>" => self.emit(PycInst::BinaryRshift),
            "&" => self.emit(PycInst::BinaryAnd),
            "|" => self.emit(PycInst::BinaryOr),
            "^" => self.emit(PycInst::BinaryXor),
            _ => {}
        }
    }

    /// Unary operation
    pub fn unary_op(&mut self, op: &str) {
        match op {
            "+" => self.emit(PycInst::UnaryPositive),
            "-" => self.emit(PycInst::UnaryNegative),
            "not" => self.emit(PycInst::UnaryNot),
            "~" => self.emit(PycInst::UnaryInvert),
            _ => {}
        }
    }

    /// Comparison
    pub fn compare_op(&mut self, op: &str) {
        let idx = match op {
            "<" => 0,
            "<=" => 1,
            "==" => 2,
            "!=" => 3,
            ">" => 4,
            ">=" => 5,
            "in" => 6,
            "not in" => 7,
            "is" => 8,
            "is not" => 9,
            _ => 0,
        };
        self.emit(PycInst::CompareOp(idx));
    }

    /// Jump
    pub fn jump(&mut self, target: &str) {
        let offset = self.labels.get(target).copied().unwrap_or(0);
        self.emit(PycInst::JumpAbsolute(offset));
    }

    /// Conditional jump
    pub fn pop_jump_if(&mut self, cond: bool, target: &str) {
        let offset = self.labels.get(target).copied().unwrap_or(0);
        if cond {
            self.emit(PycInst::PopJumpIfTrue(offset));
        } else {
            self.emit(PycInst::PopJumpIfFalse(offset));
        }
    }

    /// Call function
    pub fn call(&mut self, argc: usize) {
        self.emit(PycInst::CallFunction(argc));
    }

    /// Return
    pub fn ret(&mut self) {
        self.emit(PycInst::ReturnValue);
    }

    /// Build tuple
    pub fn build_tuple(&mut self, size: usize) {
        self.emit(PycInst::BuildTuple(size));
    }

    /// Build list
    pub fn build_list(&mut self, size: usize) {
        self.emit(PycInst::BuildList(size));
    }

    /// Build map
    pub fn build_map(&mut self, size: usize) {
        self.emit(PycInst::BuildMap(size));
    }

    /// Build set
    pub fn build_set(&mut self, size: usize) {
        self.emit(PycInst::BuildSet(size));
    }

    /// Finalize bytecode
    pub fn finalize(self) -> PycCodeObject {
        let mut code = Vec::new();

        for inst in &self.instructions {
            code.push(inst.opcode());
            code.extend_from_slice(&inst.arg_bytes());
        }

        PycCodeObject {
            argcount: 0,
            posonlyargcount: 0,
            kwonlyargcount: 0,
            nlocals: self.varnames.len(),
            stacksize: self.stack_size,
            flags: 0,
            code,
            constants: self.constants,
            names: self.names,
            varnames: self.varnames,
            filename: String::new(),
            name: String::new(),
            firstlineno: 1,
            lnotab: Vec::new(),
        }
    }
}

impl Default for PythonBytecodeCompiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Python constant
#[derive(Debug, Clone, PartialEq)]
pub enum PyConstant {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    Complex(f64, f64),
    Str(String),
    Bytes(Vec<u8>),
    Tuple(Vec<PyConstant>),
    Frozenset(Vec<PyConstant>),
    Ellipsis,
}

/// Python code object
#[derive(Debug, Clone)]
pub struct PycCodeObject {
    pub argcount: usize,
    pub posonlyargcount: usize,
    pub kwonlyargcount: usize,
    pub nlocals: usize,
    pub stacksize: usize,
    pub flags: usize,
    pub code: Vec<u8>,
    pub constants: Vec<PyConstant>,
    pub names: Vec<String>,
    pub varnames: Vec<String>,
    pub filename: String,
    pub name: String,
    pub firstlineno: usize,
    pub lnotab: Vec<u8>,
}

/// Python marshal format writer
pub struct PycMarshal {
    data: Vec<u8>,
}

impl PycMarshal {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn write_type(&mut self, t: u8) {
        self.data.push(t);
    }

    pub fn write_none(&mut self) {
        self.write_type(b'N');
    }

    pub fn write_bool(&mut self, b: bool) {
        self.write_type(if b { b'T' } else { b'F' });
    }

    pub fn write_int(&mut self, i: i64) {
        if i >= -1 && i <= 10 {
            self.write_type((b'0' as i8 + i as i8) as u8);
        } else {
            self.write_type(b'i');
            self.write_i32(i as i32);
        }
    }

    pub fn write_i32(&mut self, i: i32) {
        self.data.extend_from_slice(&i.to_le_bytes());
    }

    pub fn write_i64(&mut self, i: i64) {
        self.data.extend_from_slice(&i.to_le_bytes());
    }

    pub fn write_float(&mut self, f: f64) {
        self.write_type(b'f');
        let repr = format!("{}", f);
        let len = repr.len() as u8;
        self.data.push(len);
        self.data.extend_from_slice(repr.as_bytes());
    }

    pub fn write_bytes(&mut self, b: &[u8]) {
        self.write_type(b's');
        self.write_int(b.len() as i64);
        self.data.extend_from_slice(b);
    }

    pub fn write_str(&mut self, s: &str) {
        self.write_type(b't');
        self.write_bytes(s.as_bytes());
    }

    pub fn write_code(&mut self, code: &PycCodeObject) {
        self.write_type(b'c');
        self.write_int(code.argcount as i64);
        self.write_int(code.posonlyargcount as i64);
        self.write_int(code.kwonlyargcount as i64);
        self.write_int(code.nlocals as i64);
        self.write_int(code.stacksize as i64);
        self.write_int(code.flags as i64);
        self.write_bytes(&code.code);
        self.write_container(code.constants.len(), |s| {
            for c in &code.constants {
                s.write_constant(c);
            }
        });
        self.write_container(code.names.len(), |s| {
            for n in &code.names {
                s.write_str(n);
            }
        });
        self.write_container(code.varnames.len(), |s| {
            for v in &code.varnames {
                s.write_str(v);
            }
        });
        self.write_container(0, |_| {});
        self.write_container(0, |_| {});
        self.write_str(&code.filename);
        self.write_str(&code.name);
        self.write_int(code.firstlineno as i64);
        self.write_bytes(&code.lnotab);
    }

    fn write_container<F>(&mut self, len: usize, f: F)
    where
        F: FnOnce(&mut PycMarshal),
    {
        self.write_int(len as i64);
        f(self);
    }

    pub fn write_constant(&mut self, c: &PyConstant) {
        match c {
            PyConstant::None => self.write_none(),
            PyConstant::Bool(b) => self.write_bool(*b),
            PyConstant::Int(i) => self.write_int(*i),
            PyConstant::Float(f) => self.write_float(*f),
            PyConstant::Complex(r, i) => {
                self.write_float(*r);
                self.write_float(*i);
            }
            PyConstant::Str(s) => self.write_str(s),
            PyConstant::Bytes(b) => self.write_bytes(b),
            PyConstant::Tuple(ts) => {
                self.write_type(b'(');
                self.write_container(ts.len(), |s| {
                    for t in ts {
                        s.write_constant(t);
                    }
                });
            }
            PyConstant::Ellipsis => self.write_type(b'.'),
            PyConstant::Frozenset(_) => {}
        }
    }

    pub fn finalize(self) -> Vec<u8> {
        self.data
    }
}

impl Default for PycMarshal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compiler_basic() {
        let mut compiler = PythonBytecodeCompiler::new();
        compiler.load_const(PyConstant::Int(42));
        compiler.ret();

        let code = compiler.finalize();
        assert!(!code.code.is_empty());
    }

    #[test]
    fn test_binary_ops() {
        let mut compiler = PythonBytecodeCompiler::new();
        compiler.load_const(PyConstant::Int(1));
        compiler.load_const(PyConstant::Int(2));
        compiler.binary_op("+");
        compiler.ret();

        let code = compiler.finalize();
        assert!(code.code.contains(&PycInst::BinaryAdd.opcode()));
    }

    #[test]
    fn test_labels() {
        let mut compiler = PythonBytecodeCompiler::new();
        compiler.label("start");
        compiler.load_const(PyConstant::Int(1));
        compiler.jump("start");
        compiler.ret();

        let code = compiler.finalize();
        assert!(compiler.labels.contains_key("start"));
    }

    #[test]
    fn test_marshal() {
        let mut marshal = PycMarshal::new();
        marshal.write_int(42);
        marshal.write_str("hello");

        let data = marshal.finalize();
        assert!(!data.is_empty());
    }
}
