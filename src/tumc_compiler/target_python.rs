//! TUMC Python Bytecode Generation
//!
//! Code generation for Python bytecode (.pyc files).

use std::collections::HashMap;

/// Python bytecode instruction
#[derive(Debug, Clone)]
pub enum PycInst {
    // Stack operations
    POP_TOP,
    ROT_TWO,
    ROT_THREE,
    DUP_TOP,
    DUP_TOP_TWO,

    // Binary operations
    BINARY_ADD,
    BINARY_SUBTRACT,
    BINARY_MULTIPLY,
    BINARY_MATRIX_MULTIPLY,
    BINARY_TRUE_DIVIDE,
    BINARY_FLOOR_DIVIDE,
    BINARY_MODULO,
    BINARY_POWER,
    BINARY_LSHIFT,
    BINARY_RSHIFT,
    BINARY_AND,
    BINARY_OR,
    BINARY_XOR,

    // In-place operations
    INPLACE_ADD,
    INPLACE_SUBTRACT,
    INPLACE_MULTIPLY,
    INPLACE_MATRIX_MULTIPLY,
    INPLACE_TRUE_DIVIDE,
    INPLACE_FLOOR_DIVIDE,
    INPLACE_MODULO,
    INPLACE_POWER,
    INPLACE_LSHIFT,
    INPLACE_RSHIFT,
    INPLACE_AND,
    INPLACE_OR,
    INPLACE_XOR,

    // Subscript and slicing
    SUBSCR,
    STORE_SUBSCR,
    DELETE_SUBSCR,

    // Load constants
    LOAD_CONST(usize),

    // Load names
    LOAD_FAST(usize),
    LOAD_GLOBAL(usize),

    // Store names
    STORE_FAST(usize),
    STORE_GLOBAL(usize),

    // Delete names
    DELETE_FAST(usize),
    DELETE_GLOBAL(usize),
    DELETE_NAME(usize),

    // Unary operations
    UNARY_POSITIVE,
    UNARY_NEGATIVE,
    UNARY_NOT,
    UNARY_INVERT,

    // Comparisons
    COMPARE_OP(usize),

    // Jumps
    JUMP_FORWARD(i32),
    JUMP_ABSOLUTE(usize),
    POP_JUMP_IF_TRUE(usize),
    POP_JUMP_IF_FALSE(usize),
    JUMP_IF_TRUE_OR_POP(usize),
    JUMP_IF_FALSE_OR_POP(usize),

    // Function calls
    CALL_FUNCTION(usize),
    CALL_FUNCTION_KW(usize),
    CALL_FUNCTION_EX(usize),
    CALL_METHOD(usize),
    LOAD_METHOD(usize),
    CALL_METHOD_KW(usize),

    // Returns
    RETURN_VALUE,
    RETURN_CONST,

    // Imports
    IMPORT_NAME(usize),
    IMPORT_FROM(usize),
    IMPORT_STAR,
    LOAD_CONSTANT(usize),

    // Exception handling
    SETUP_FINALLY(i32),
    END_FINALLY,
    CLEANUP_SUPER,
    WITH_CLEANUP_START,
    WITH_CLEANUP_FINISH,
    POP_FINALLY(i32),
    END_FINALLY_YIELD,

    // Loops
    GET_ITER,
    FOR_ITER(i32),
    BREAK_LOOP,
    CONTINUE_LOOP(i32),

    // Setup
    SETUP_WITH(i32),
    SETUP_ASYNC_WITH(i32),
    BEFORE_WITH,
    YIELD_VALUE,
    GET_AWAITABLE,
    BEFORE_ASYNC_WITH,
    GET_YIELD_FROM_ITER,
    YIELD_FROM,

    // Other
    NOP,
    UNPACK_SEQUENCE(usize),
    UNPACK_EX(usize, usize),
    BUILD_STRING(usize),
    BUILD_TUPLE(usize),
    BUILD_LIST(usize),
    BUILD_SET(usize),
    BUILD_MAP(usize),
    BUILD_CONST_KEY_MAP(usize),
    BUILD_TUPLE_UNPACK(usize),
    BUILD_LIST_UNPACK(usize),
    BUILD_SET_UNPACK(usize),
    BUILD_MAP_UNPACK(usize),
    BUILD_MAP_UNPACK_WITH_CALL(usize),
    BUILD_TUPLE_UNPACK_WITH_CALL(usize),
    DICT_MERGE(usize),
    DICT_UPDATE(usize),
    LIST_EXTEND(usize),
    SET_UPDATE(usize),
    LIST_TO_TUPLE,
    SETUP_LOOP(i32),
    EXTENDED_ARG(usize),
}

impl PycInst {
    /// Get opcode number
    pub fn opcode(&self) -> u8 {
        match self {
            PycInst::POP_TOP => 1,
            PycInst::ROT_TWO => 2,
            PycInst::ROT_THREE => 3,
            PycInst::DUP_TOP => 4,
            PycInst::DUP_TOP_TWO => 5,
            PycInst::BINARY_ADD => 23,
            PycInst::BINARY_SUBTRACT => 24,
            PycInst::BINARY_MULTIPLY => 20,
            PycInst::BINARY_TRUE_DIVIDE => 27,
            PycInst::BINARY_FLOOR_DIVIDE => 26,
            PycInst::BINARY_MODULO => 22,
            PycInst::BINARY_POWER => 25,
            PycInst::BINARY_LSHIFT => 62,
            PycInst::BINARY_RSHIFT => 63,
            PycInst::BINARY_AND => 64,
            PycInst::BINARY_OR => 66,
            PycInst::BINARY_XOR => 65,
            PycInst::INPLACE_ADD => 55,
            PycInst::INPLACE_SUBTRACT => 56,
            PycInst::INPLACE_MULTIPLY => 57,
            PycInst::INPLACE_TRUE_DIVIDE => 29,
            PycInst::INPLACE_FLOOR_DIVIDE => 28,
            PycInst::INPLACE_MODULO => 24,
            PycInst::INPLACE_POWER => 67,
            PycInst::INPLACE_LSHIFT => 75,
            PycInst::INPLACE_RSHIFT => 76,
            PycInst::INPLACE_AND => 77,
            PycInst::INPLACE_OR => 78,
            PycInst::INPLACE_XOR => 79,
            PycInst::SUBSCR => 60,
            PycInst::STORE_SUBSCR => 61,
            PycInst::DELETE_SUBSCR => 59,
            PycInst::LOAD_CONST(_) => 100,
            PycInst::LOAD_FAST(_) => 125,
            PycInst::LOAD_GLOBAL(_) => 116,
            PycInst::STORE_FAST(_) => 125,
            PycInst::STORE_GLOBAL(_) => 116,
            PycInst::DELETE_FAST(_) => 126,
            PycInst::DELETE_GLOBAL(_) => 117,
            PycInst::DELETE_NAME(_) => 119,
            PycInst::UNARY_POSITIVE => 10,
            PycInst::UNARY_NEGATIVE => 11,
            PycInst::UNARY_NOT => 12,
            PycInst::UNARY_INVERT => 15,
            PycInst::COMPARE_OP(_) => 107,
            PycInst::JUMP_FORWARD(_) => 110,
            PycInst::JUMP_ABSOLUTE(_) => 113,
            PycInst::POP_JUMP_IF_TRUE(_) => 115,
            PycInst::POP_JUMP_IF_FALSE(_) => 114,
            PycInst::JUMP_IF_TRUE_OR_POP(_) => 112,
            PycInst::JUMP_IF_FALSE_OR_POP(_) => 111,
            PycInst::CALL_FUNCTION(_) => 131,
            PycInst::CALL_FUNCTION_KW(_) => 141,
            PycInst::CALL_FUNCTION_EX(_) => 142,
            PycInst::CALL_METHOD(_) => 161,
            PycInst::LOAD_METHOD(_) => 160,
            PycInst::CALL_METHOD_KW(_) => 162,
            PycInst::RETURN_VALUE => 83,
            PycInst::RETURN_CONST => 135,
            PycInst::IMPORT_NAME(_) => 108,
            PycInst::IMPORT_FROM(_) => 109,
            PycInst::IMPORT_STAR => 84,
            PycInst::GET_ITER => 68,
            PycInst::FOR_ITER(_) => 93,
            PycInst::BREAK_LOOP => 80,
            PycInst::CONTINUE_LOOP(_) => 119,
            PycInst::SETUP_FINALLY(_) => 85,
            PycInst::END_FINALLY => 88,
            PycInst::YIELD_VALUE => 72,
            PycInst::GET_AWAITABLE => 73,
            PycInst::YIELD_FROM => 88,
            PycInst::NOP => 9,
            PycInst::UNPACK_SEQUENCE(_) => 92,
            PycInst::BUILD_STRING(_) => 157,
            PycInst::BUILD_TUPLE(_) => 102,
            PycInst::BUILD_LIST(_) => 103,
            PycInst::BUILD_SET(_) => 104,
            PycInst::BUILD_MAP(_) => 105,
            PycInst::BUILD_CONST_KEY_MAP(_) => 156,
            PycInst::SETUP_LOOP(_) => 120,
            _ => 0,
        }
    }

    /// Get argument bytes
    pub fn arg_bytes(&self) -> Vec<u8> {
        match self {
            PycInst::LOAD_CONST(idx) |
            PycInst::LOAD_FAST(idx) |
            PycInst::LOAD_GLOBAL(idx) |
            PycInst::STORE_FAST(idx) |
            PycInst::STORE_GLOBAL(idx) |
            PycInst::DELETE_FAST(idx) |
            PycInst::DELETE_GLOBAL(idx) |
            PycInst::DELETE_NAME(idx) |
            PycInst::BUILD_TUPLE(idx) |
            PycInst::BUILD_LIST(idx) |
            PycInst::BUILD_SET(idx) |
            PycInst::BUILD_MAP(idx) |
            PycInst::UNPACK_SEQUENCE(idx) |
            PycInst::JUMP_ABSOLUTE(idx) |
            PycInst::POP_JUMP_IF_TRUE(idx) |
            PycInst::POP_JUMP_IF_FALSE(idx) |
            PycInst::JUMP_IF_TRUE_OR_POP(idx) |
            PycInst::JUMP_IF_FALSE_OR_POP(idx) |
            PycInst::LOAD_METHOD(idx) |
            PycInst::CALL_FUNCTION(idx) |
            PycInst::CALL_FUNCTION_KW(idx) |
            PycInst::CALL_FUNCTION_EX(idx) |
            PycInst::CALL_METHOD(idx) |
            PycInst::CALL_METHOD_KW(idx) => idx.to_le_bytes().to_vec(),
            PycInst::JUMP_FORWARD(offset) |
            PycInst::FOR_ITER(offset) |
            PycInst::SETUP_FINALLY(offset) |
            PycInst::CONTINUE_LOOP(offset) |
            PycInst::SETUP_LOOP(offset) |
            PycInst::SETUP_WITH(offset) |
            PycInst::SETUP_ASYNC_WITH(offset) => (offset / 2).to_le_bytes().to_vec(),
            PycInst::COMPARE_OP(op) => (*op as usize).to_le_bytes().to_vec(),
            PycInst::EXTENDED_ARG(arg) => (*arg as usize).to_le_bytes().to_vec(),
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
            PycInst::POP_TOP => self.stack_size.saturating_sub(1),
            PycInst::ROT_TWO | PycInst::ROT_THREE => self.stack_size,
            PycInst::DUP_TOP => self.stack_size + 1,
            PycInst::BINARY_ADD | PycInst::BINARY_SUBTRACT | PycInst::BINARY_MULTIPLY |
            PycInst::BINARY_TRUE_DIVIDE | PycInst::BINARY_FLOOR_DIVIDE |
            PycInst::BINARY_MODULO | PycInst::BINARY_POWER |
            PycInst::BINARY_LSHIFT | PycInst::BINARY_RSHIFT |
            PycInst::BINARY_AND | PycInst::BINARY_OR | PycInst::BINARY_XOR => {
                self.stack_size.saturating_sub(1)
            }
            PycInst::LOAD_CONST(_) | PycInst::LOAD_FAST(_) | PycInst::LOAD_GLOBAL(_) => {
                self.stack_size + 1
            }
            PycInst::STORE_FAST(_) | PycInst::STORE_GLOBAL(_) => {
                self.stack_size.saturating_sub(1)
            }
            PycInst::RETURN_VALUE => 0,
            PycInst::JUMP_FORWARD(_) | PycInst::JUMP_ABSOLUTE(_) => self.stack_size,
            PycInst::POP_JUMP_IF_TRUE(_) | PycInst::POP_JUMP_IF_FALSE(_) => {
                self.stack_size.saturating_sub(1)
            }
            PycInst::CALL_FUNCTION(n) => self.stack_size.saturating_sub(*n),
            PycInst::BUILD_TUPLE(n) => self.stack_size.saturating_sub(*n),
            PycInst::BUILD_LIST(n) => self.stack_size.saturating_sub(*n),
            PycInst::BUILD_MAP(n) => self.stack_size.saturating_sub(*n * 2),
            _ => self.stack_size,
        }
    }

    /// Define label
    pub fn label(&mut self, name: &str) {
        self.labels.insert(name.to_string(), self.instructions.len());
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
        self.emit(PycInst::LOAD_CONST(idx));
    }

    /// Load local variable
    pub fn load_fast(&mut self, name: &str) {
        let idx = self.add_varname(name);
        self.emit(PycInst::LOAD_FAST(idx));
    }

    /// Load global variable
    pub fn load_global(&mut self, name: &str) {
        let idx = self.add_name(name);
        self.emit(PycInst::LOAD_GLOBAL(idx));
    }

    /// Store local variable
    pub fn store_fast(&mut self, name: &str) {
        let idx = self.add_varname(name);
        self.emit(PycInst::STORE_FAST(idx));
    }

    /// Store global variable
    pub fn store_global(&mut self, name: &str) {
        let idx = self.add_name(name);
        self.emit(PycInst::STORE_GLOBAL(idx));
    }

    /// Binary operation
    pub fn binary_op(&mut self, op: &str) {
        match op {
            "+" => self.emit(PycInst::BINARY_ADD),
            "-" => self.emit(PycInst::BINARY_SUBTRACT),
            "*" => self.emit(PycInst::BINARY_MULTIPLY),
            "/" => self.emit(PycInst::BINARY_TRUE_DIVIDE),
            "//" => self.emit(PycInst::BINARY_FLOOR_DIVIDE),
            "%" => self.emit(PycInst::BINARY_MODULO),
            "**" => self.emit(PycInst::BINARY_POWER),
            "<<" => self.emit(PycInst::BINARY_LSHIFT),
            ">>" => self.emit(PycInst::BINARY_RSHIFT),
            "&" => self.emit(PycInst::BINARY_AND),
            "|" => self.emit(PycInst::BINARY_OR),
            "^" => self.emit(PycInst::BINARY_XOR),
            _ => {}
        }
    }

    /// Unary operation
    pub fn unary_op(&mut self, op: &str) {
        match op {
            "+" => self.emit(PycInst::UNARY_POSITIVE),
            "-" => self.emit(PycInst::UNARY_NEGATIVE),
            "not" => self.emit(PycInst::UNARY_NOT),
            "~" => self.emit(PycInst::UNARY_INVERT),
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
        self.emit(PycInst::COMPARE_OP(idx));
    }

    /// Jump
    pub fn jump(&mut self, target: &str) {
        let offset = self.labels.get(target).copied().unwrap_or(0);
        self.emit(PycInst::JUMP_ABSOLUTE(offset));
    }

    /// Conditional jump
    pub fn pop_jump_if(&mut self, cond: bool, target: &str) {
        let offset = self.labels.get(target).copied().unwrap_or(0);
        if cond {
            self.emit(PycInst::POP_JUMP_IF_TRUE(offset));
        } else {
            self.emit(PycInst::POP_JUMP_IF_FALSE(offset));
        }
    }

    /// Call function
    pub fn call(&mut self, argc: usize) {
        self.emit(PycInst::CALL_FUNCTION(argc));
    }

    /// Return
    pub fn ret(&mut self) {
        self.emit(PycInst::RETURN_VALUE);
    }

    /// Build tuple
    pub fn build_tuple(&mut self, size: usize) {
        self.emit(PycInst::BUILD_TUPLE(size));
    }

    /// Build list
    pub fn build_list(&mut self, size: usize) {
        self.emit(PycInst::BUILD_LIST(size));
    }

    /// Build map
    pub fn build_map(&mut self, size: usize) {
        self.emit(PycInst::BUILD_MAP(size));
    }

    /// Build set
    pub fn build_set(&mut self, size: usize) {
        self.emit(PycInst::BUILD_SET(size));
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
        assert!(code.code.contains(&PycInst::BINARY_ADD.opcode()));
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
