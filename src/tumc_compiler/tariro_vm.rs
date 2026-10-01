//! Tariro Bytecode Virtual Machine
//! A register-based VM for executing TUMC bytecode

use crate::prelude::*;
use std::collections::{HashMap, VecDeque};

/// Maximum number of registers
pub const MAX_REGISTERS: usize = 256;

/// Maximum stack depth
pub const MAX_STACK_DEPTH: usize = 1024;

/// Maximum call frame depth
pub const MAX_CALL_DEPTH: usize = 256;

/// Bytecode opcode definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    // === Control Flow ===
    /// Halt execution
    Halt = 0x00,
    /// No operation
    Nop = 0x01,
    /// Unconditional jump
    Jmp = 0x02,
    /// Jump if false
    JmpFalse = 0x03,
    /// Jump if true
    JmpTrue = 0x04,
    /// Jump if null
    JmpNull = 0x05,
    /// Jump if not null
    JmpNotNull = 0x06,
    /// Conditional jump (switch)
    Switch = 0x07,

    // === Function Calls ===
    /// Call function
    Call = 0x10,
    /// Return from function
    Ret = 0x11,
    /// Tail call optimization
    TailCall = 0x12,
    /// Call with arguments on stack
    CallVariadic = 0x13,

    // === Register Operations ===
    /// Move value between registers
    Mov = 0x20,
    /// Copy value
    Cpy = 0x21,
    /// Swap two registers
    Swap = 0x22,

    // === Stack Operations ===
    /// Push to stack
    Push = 0x30,
    /// Pop from stack
    Pop = 0x31,
    /// Push immediate
    PushImm = 0x32,
    /// Load from stack offset
    LoadSp = 0x33,
    /// Store to stack offset
    StoreSp = 0x34,

    // === Constant Loading ===
    /// Load integer constant
    LdImm32 = 0x40,
    /// Load 64-bit integer
    LdImm64 = 0x41,
    /// Load float constant
    LdF32 = 0x42,
    /// Load double constant
    LdF64 = 0x43,
    /// Load from constant pool
    LdConst = 0x44,
    /// Load string from pool
    LdStr = 0x45,
    /// Load zero (null)
    LdZero = 0x46,
    /// Load global
    LdGlobal = 0x47,
    /// Store global
    StGlobal = 0x48,

    // === Memory Operations ===
    /// Allocate on heap
    Alloc = 0x50,
    /// Allocate array
    AllocArray = 0x51,
    /// Deallocate
    Dealloc = 0x52,
    /// Load from memory
    Load8 = 0x53,
    Load16 = 0x54,
    Load32 = 0x55,
    Load64 = 0x56,
    /// Store to memory
    Store8 = 0x57,
    Store16 = 0x58,
    Store32 = 0x59,
    Store64 = 0x5A,

    // === Integer Arithmetic ===
    /// Add integers
    AddI32 = 0x60,
    /// Subtract integers
    SubI32 = 0x61,
    /// Multiply integers
    MulI32 = 0x62,
    /// Divide integers (signed)
    DivI32 = 0x63,
    /// Modulo
    ModI32 = 0x64,
    /// Negate integer
    NegI32 = 0x65,
    /// Add 64-bit integers
    AddI64 = 0x66,
    /// Subtract 64-bit integers
    SubI64 = 0x67,
    /// Multiply 64-bit integers
    MulI64 = 0x68,
    /// Divide 64-bit integers
    DivI64 = 0x69,
    /// Modulo 64-bit
    ModI64 = 0x6A,
    /// Negate 64-bit integer
    NegI64 = 0x6B,

    // === Floating Point Arithmetic ===
    /// Add floats
    AddF32 = 0x70,
    /// Subtract floats
    SubF32 = 0x71,
    /// Multiply floats
    MulF32 = 0x72,
    /// Divide floats
    DivF32 = 0x73,
    /// Modulo floats
    ModF32 = 0x74,
    /// Negate float
    NegF32 = 0x75,
    /// Add doubles
    AddF64 = 0x76,
    /// Subtract doubles
    SubF64 = 0x77,
    /// Multiply doubles
    MulF64 = 0x78,
    /// Divide doubles
    DivF64 = 0x79,
    /// Negate double
    NegF64 = 0x7A,

    // === Bitwise Operations ===
    /// Bitwise AND
    And = 0x80,
    /// Bitwise OR
    Or = 0x81,
    /// Bitwise XOR
    Xor = 0x82,
    /// Logical NOT
    Not = 0x83,
    /// Shift left
    Shl = 0x84,
    /// Shift right (logical)
    Shr = 0x85,
    /// Shift right (arithmetic)
    Sar = 0x86,
    /// Rotate left
    Rotl = 0x87,
    /// Rotate right
    Rotr = 0x88,

    // === Comparison ===
    /// Compare equal
    CmpEq = 0x90,
    /// Compare not equal
    CmpNe = 0x91,
    /// Compare less than (signed)
    CmpLt = 0x92,
    /// Compare less than or equal (signed)
    CmpLe = 0x93,
    /// Compare greater than (signed)
    CmpGt = 0x94,
    /// Compare greater than or equal (signed)
    CmpGe = 0x95,
    /// Compare less than (unsigned)
    CmpLtU = 0x96,
    /// Compare less than or equal (unsigned)
    CmpLeU = 0x97,
    /// Compare greater than (unsigned)
    CmpGtU = 0x98,
    /// Compare greater than or equal (unsigned)
    CmpGeU = 0x99,
    /// Test if zero
    TestZ = 0x9A,
    /// Test if null
    TestNull = 0x9B,

    // === Conversion ===
    /// Extend i32 to i64
    SExt32_64 = 0xA0,
    /// Truncate i64 to i32
    Trunc64_32 = 0xA1,
    /// Convert i32 to f32
    IToF32 = 0xA2,
    /// Convert i32 to f64
    IToF64 = 0xA3,
    /// Convert f32 to i32
    FToI32 = 0xA4,
    /// Convert f64 to i32
    F64ToI32 = 0xA5,
    /// Promote f32 to f64
    F32ToF64 = 0xA6,
    /// Demote f64 to f32
    F64ToF32 = 0xA7,
    /// Bitcast i32 to f32
    BitcastI32F32 = 0xA8,
    /// Bitcast f32 to i32
    BitcastF32I32 = 0xA9,

    // === Object/Struct Operations ===
    /// Create object
    NewObject = 0xB0,
    /// Get field
    GetField = 0xB1,
    /// Set field
    SetField = 0xB2,
    /// Get method
    GetMethod = 0xB3,
    /// Get array element
    GetElem = 0xB4,
    /// Set array element
    SetElem = 0xB5,
    /// Get array length
    ArrayLen = 0xB6,
    /// Get string length
    StrLen = 0xB7,
    /// String concatenation
    StrConcat = 0xB8,

    // === Exception Handling ===
    /// Throw exception
    Throw = 0xC0,
    /// Rethrow exception
    Rethrow = 0xC1,
    /// Begin try block
    TryBegin = 0xC2,
    /// End try block
    TryEnd = 0xC3,
    /// Exception handler
    Catch = 0xC4,

    // === Type Operations ===
    /// Type check
    TypeCheck = 0xD0,
    /// Type cast
    Cast = 0xD1,
    /// Get type of value
    TypeOf = 0xD2,
    /// Is instance of
    IsInstance = 0xD3,

    // === Closure Operations ===
    /// Create closure
    MakeClosure = 0xE0,
    /// Get captured variable
    GetUpvar = 0xE1,
    /// Set captured variable
    SetUpvar = 0xE2,

    // === Debug Operations ===
    /// Breakpoint
    Breakpoint = 0xF0,
    /// Debug print
    DebugPrint = 0xF1,
    /// Log message
    Log = 0xF2,
}

/// Value type in the VM
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Unit type
    Unit,
    /// Boolean
    Bool(bool),
    /// 32-bit integer
    I32(i32),
    /// 64-bit integer
    I64(i64),
    /// 32-bit float
    F32(f32),
    /// 64-bit float
    F64(f64),
    /// String
    String(String),
    /// Object reference
    Object(GcPtr<Object>),
    /// Array reference
    Array(GcPtr<Array>),
    /// Closure reference
    Closure(GcPtr<Closure>),
    /// Function reference
    Function(GcPtr<Function>),
    /// Type reference
    Type(GcPtr<Type>),
    /// Null
    Null,
    /// Uninitialized
    Uninitialized,
}

impl Value {
    /// Get the type name
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Unit => "Unit",
            Value::Bool(_) => "Bool",
            Value::I32(_) | Value::I64(_) => "Int",
            Value::F32(_) | Value::F64(_) => "Float",
            Value::String(_) => "String",
            Value::Object(_) => "Object",
            Value::Array(_) => "Array",
            Value::Closure(_) => "Closure",
            Value::Function(_) => "Function",
            Value::Type(_) => "Type",
            Value::Null => "Null",
            Value::Uninitialized => "Uninitialized",
        }
    }

    /// Check if truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::I32(i) => *i != 0,
            Value::I64(i) => *i != 0,
            Value::F32(f) => *f != 0.0,
            Value::F64(f) => *f != 0.0,
            Value::Null | Value::Uninitialized => false,
            _ => true,
        }
    }

    /// Try to get i32
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Value::I32(i) => Some(*i),
            Value::Bool(b) => Some(if *b { 1 } else { 0 }),
            _ => None,
        }
    }

    /// Try to get i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Value::I64(i) => Some(*i),
            Value::I32(i) => Some(*i as i64),
            _ => None,
        }
    }

    /// Try to get f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Value::F64(f) => Some(*f),
            Value::F32(f) => Some(*f as f64),
            Value::I32(i) => Some(*i as f64),
            Value::I64(i) => Some(*i as f64),
            _ => None,
        }
    }

    /// Get string representation
    pub fn to_string(&self) -> String {
        match self {
            Value::Unit => "()".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::I32(i) => i.to_string(),
            Value::I64(i) => i.to_string(),
            Value::F32(f) => f.to_string(),
            Value::F64(f) => f.to_string(),
            Value::String(s) => s.clone(),
            Value::Null => "null".to_string(),
            Value::Uninitialized => "uninitialized".to_string(),
            Value::Object(ptr) => format!("Object@{:p}", ptr),
            Value::Array(ptr) => format!("Array@{:p}", ptr),
            Value::Closure(ptr) => format!("Closure@{:p}", ptr),
            Value::Function(ptr) => format!("Function@{:p}", ptr),
            Value::Type(ptr) => format!("Type@{:p}", ptr),
        }
    }
}

/// Garbage collected object pointer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GcPtr<T: 'static>(*const T);

impl<T> GcPtr<T> {
    pub fn new(ptr: *const T) -> Self {
        GcPtr(ptr)
    }

    pub fn as_ref(&self) -> &'a T {
        unsafe { &*self.0 }
    }
}

unsafe impl<T> Send for GcPtr<T> {}
unsafe impl<T> Sync for GcPtr<T> {}

/// VM object
#[derive(Debug, Clone)]
pub struct Object {
    /// Object class/type
    pub class: String,
    /// Instance fields
    pub fields: HashMap<String, Value>,
}

impl Object {
    pub fn new(class: String) -> Self {
        Object {
            class,
            fields: HashMap::new(),
        }
    }
}

/// VM array
#[derive(Debug, Clone)]
pub struct Array {
    /// Element type
    pub element_type: String,
    /// Elements
    pub elements: Vec<Value>,
}

impl Array {
    pub fn new(size: usize) -> Self {
        Array {
            element_type: "Any".to_string(),
            elements: vec![Value::Uninitialized; size],
        }
    }

    pub fn with_elements(elements: Vec<Value>) -> Self {
        Array {
            element_type: "Any".to_string(),
            elements,
        }
    }
}

/// Closure
#[derive(Debug, Clone)]
pub struct Closure {
    /// Function being closed over
    pub function: GcPtr<Function>,
    /// Captured variables
    pub upvars: Vec<Value>,
}

impl Closure {
    pub fn new(function: GcPtr<Function>, upvars: Vec<Value>) -> Self {
        Closure {
            function,
            upvars,
        }
    }
}

/// Compiled function
#[derive(Debug, Clone)]
pub struct Function {
    /// Function name
    pub name: String,
    /// Parameter count
    pub arity: usize,
    /// Bytecode instructions
    pub code: Vec<u8>,
    /// Register count
    pub register_count: usize,
    /// Constant pool
    pub constants: Vec<Value>,
    /// Local variable names
    pub locals: Vec<String>,
    /// Source span
    pub span: Span,
}

/// Type representation
#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
    pub fields: Vec<(String, String)>,
    pub methods: Vec<String>,
}

/// Call frame
#[derive(Debug, Clone)]
pub struct Frame {
    /// Function being executed
    pub function: GcPtr<Function>,
    /// Instruction pointer
    pub ip: usize,
    /// Stack pointer
    pub sp: usize,
    /// Base pointer
    pub bp: usize,
}

impl Frame {
    pub fn new(function: GcPtr<Function>) -> Self {
        Frame {
            function,
            ip: 0,
            sp: 0,
            bp: 0,
        }
    }
}

/// VM state
pub struct VM {
    /// Registers
    registers: Vec<Value>,
    /// Value stack
    stack: Vec<Value>,
    /// Call stack
    call_stack: Vec<Frame>,
    /// Current frame
    current_frame: Option<Frame>,
    /// Global variables
    globals: HashMap<String, Value>,
    /// Loaded modules
    modules: HashMap<String, Module>,
    /// Heap for GC
    heap: Vec<Value>,
    /// GC threshold
    gc_threshold: usize,
    /// Instructions executed
    instructions_executed: u64,
    /// Error state
    error: Option<VMError>,
    /// Trace execution
    trace: bool,
}

impl VM {
    /// Create new VM
    pub fn new() -> Self {
        VM {
            registers: vec![Value::Uninitialized; MAX_REGISTERS],
            stack: Vec::with_capacity(MAX_STACK_DEPTH),
            call_stack: Vec::with_capacity(MAX_CALL_DEPTH),
            current_frame: None,
            globals: HashMap::new(),
            modules: HashMap::new(),
            heap: Vec::new(),
            gc_threshold: 1024 * 1024, // 1MB
            instructions_executed: 0,
            error: None,
            trace: false,
        }
    }

    /// Load a module
    pub fn load_module(&mut self, module: Module) {
        self.modules.insert(module.name.clone(), module);
    }

    /// Set a global variable
    pub fn set_global(&mut self, name: String, value: Value) {
        self.globals.insert(name, value);
    }

    /// Get a global variable
    pub fn get_global(&self, name: &str) -> Option<&Value> {
        self.globits.get(name)
    }

    /// Execute bytecode
    pub fn execute(&mut self, mut code: Vec<u8>, entry: &str) -> Result<Value, VMError> {
        // Find entry function
        let entry_func = self.find_function(entry)?;

        // Create initial frame
        let frame = Frame::new(entry_func);
        self.call_stack.push(frame);
        self.current_frame = self.call_stack.last().cloned();

        // Main execution loop
        loop {
            // Check for error
            if let Some(ref err) = self.error {
                return Err(err.clone());
            }

            // Fetch next instruction
            let ip = self.current_frame.as_ref().map(|f| f.ip).unwrap_or(0);
            if ip >= code.len() {
                break;
            }

            let opcode = Opcode::try_from(code[ip])
                .map_err(|_| VMError::InvalidOpcode(code[ip]))?;

            if self.trace {
                println!("IP: {} | Opcode: {:?}", ip, opcode);
            }

            // Execute instruction
            self.execute_instruction(opcode, &mut code)?;

            self.instructions_executed += 1;

            // Check for halt
            if matches!(opcode, Opcode::Halt) {
                break;
            }
        }

        // Get return value
        Ok(self.registers[0].clone())
    }

    /// Execute a single instruction
    fn execute_instruction(&mut self, opcode: Opcode, code: &mut Vec<u8>) -> Result<(), VMError> {
        let ip = self.current_frame.as_mut().ok_or(VMError::NoFrame)?.ip;

        match opcode {
            Opcode::Halt => {
                // Handled in main loop
            }

            Opcode::Nop => {
                self.current_frame.as_mut().unwrap().ip += 1;
            }

            Opcode::Jmp => {
                let target = self.read_u16(code, ip + 1) as usize;
                self.current_frame.as_mut().unwrap().ip = target;
            }

            Opcode::JmpFalse => {
                let target = self.read_u16(code, ip + 1) as usize;
                let cond = self.registers[code[ip + 3] as usize].clone();
                self.current_frame.as_mut().unwrap().ip = if !cond.is_truthy() {
                    target
                } else {
                    ip + 4
                };
            }

            Opcode::JmpTrue => {
                let target = self.read_u16(code, ip + 1) as usize;
                let cond = self.registers[code[ip + 3] as usize].clone();
                self.current_frame.as_mut().unwrap().ip = if cond.is_truthy() {
                    target
                } else {
                    ip + 4
                };
            }

            Opcode::Mov => {
                let dest = code[ip + 1] as usize;
                let src = code[ip + 2] as usize;
                self.registers[dest] = self.registers[src].clone();
                self.current_frame.as_mut().unwrap().ip = ip + 3;
            }

            Opcode::LdImm32 => {
                let dest = code[ip + 1] as usize;
                let val = self.read_i32(code, ip + 2);
                self.registers[dest] = Value::I32(val);
                self.current_frame.as_mut().unwrap().ip = ip + 6;
            }

            Opcode::LdImm64 => {
                let dest = code[ip + 1] as usize;
                let val = self.read_i64(code, ip + 2);
                self.registers[dest] = Value::I64(val);
                self.current_frame.as_mut().unwrap().ip = ip + 10;
            }

            Opcode::LdF64 => {
                let dest = code[ip + 1] as usize;
                let val = self.read_f64(code, ip + 2);
                self.registers[dest] = Value::F64(val);
                self.current_frame.as_mut().unwrap().ip = ip + 10;
            }

            Opcode::LdStr => {
                let dest = code[ip + 1] as usize;
                let idx = self.read_u16(code, ip + 2) as usize;
                if let Some(frame) = self.current_frame.as_ref() {
                    let func = frame.function.as_ref();
                    if idx < func.constants.len() {
                        self.registers[dest] = func.constants[idx].clone();
                    } else {
                        return Err(VMError::OutOfBounds(idx));
                    }
                }
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::LdZero => {
                let dest = code[ip + 1] as usize;
                self.registers[dest] = Value::I32(0);
                self.current_frame.as_mut().unwrap().ip = ip + 2;
            }

            Opcode::AddI32 => {
                let dest = code[ip + 1] as usize;
                let a = self.registers[code[ip + 2] as usize].as_i32().unwrap_or(0);
                let b = self.registers[code[ip + 3] as usize].as_i32().unwrap_or(0);
                self.registers[dest] = Value::I32(a.wrapping_add(b));
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::SubI32 => {
                let dest = code[ip + 1] as usize;
                let a = self.registers[code[ip + 2] as usize].as_i32().unwrap_or(0);
                let b = self.registers[code[ip + 3] as usize].as_i32().unwrap_or(0);
                self.registers[dest] = Value::I32(a.wrapping_sub(b));
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::MulI32 => {
                let dest = code[ip + 1] as usize;
                let a = self.registers[code[ip + 2] as usize].as_i32().unwrap_or(0);
                let b = self.registers[code[ip + 3] as usize].as_i32().unwrap_or(0);
                self.registers[dest] = Value::I32(a.wrapping_mul(b));
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::DivI32 => {
                let dest = code[ip + 1] as usize;
                let a = self.registers[code[ip + 2] as usize].as_i32()?;
                let b = self.registers[code[ip + 3] as usize].as_i32()?;
                if b == 0 {
                    return Err(VMError::DivisionByZero);
                }
                self.registers[dest] = Value::I32(a / b);
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::ModI32 => {
                let dest = code[ip + 1] as usize;
                let a = self.registers[code[ip + 2] as usize].as_i32()?;
                let b = self.registers[code[ip + 3] as usize].as_i32()?;
                if b == 0 {
                    return Err(VMError::DivisionByZero);
                }
                self.registers[dest] = Value::I32(a % b);
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::AddF64 => {
                let dest = code[ip + 1] as usize;
                let a = self.registers[code[ip + 2] as usize].as_f64().unwrap_or(0.0);
                let b = self.registers[code[ip + 3] as usize].as_f64().unwrap_or(0.0);
                self.registers[dest] = Value::F64(a + b);
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::CmpEq => {
                let dest = code[ip + 1] as usize;
                let a = self.registers[code[ip + 2] as usize].clone();
                let b = self.registers[code[ip + 3] as usize].clone();
                self.registers[dest] = Value::Bool(a == b);
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::CmpNe => {
                let dest = code[ip + 1] as usize;
                let a = self.registers[code[ip + 2] as usize].clone();
                let b = self.registers[code[ip + 3] as usize].clone();
                self.registers[dest] = Value::Bool(a != b);
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::CmpLt => {
                let dest = code[ip + 1] as usize;
                let a = self.registers[code[ip + 2] as usize].as_i32().unwrap_or(0);
                let b = self.registers[code[ip + 3] as usize].as_i32().unwrap_or(0);
                self.registers[dest] = Value::Bool(a < b);
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::Not => {
                let dest = code[ip + 1] as usize;
                let val = self.registers[code[ip + 2] as usize].clone();
                self.registers[dest] = Value::Bool(!val.is_truthy());
                self.current_frame.as_mut().unwrap().ip = ip + 3;
            }

            Opcode::Call => {
                let func_idx = self.read_u16(code, ip + 1) as usize;
                // Would look up function and create new frame
                self.current_frame.as_mut().unwrap().ip = ip + 3;
            }

            Opcode::Ret => {
                // Pop frame and return
                if self.call_stack.len() > 1 {
                    self.call_stack.pop();
                    self.current_frame = self.call_stack.last().cloned();
                } else {
                    // Return from entry function
                    self.current_frame.as_mut().unwrap().ip = code.len();
                }
            }

            Opcode::Alloc => {
                let dest = code[ip + 1] as usize;
                let size = self.registers[code[ip + 2] as usize].as_i32().unwrap_or(0) as usize;
                let obj = Value::Array(GcPtr::new(std::ptr::null()));
                self.heap.push(obj.clone());
                self.registers[dest] = obj;
                self.current_frame.as_mut().unwrap().ip = ip + 3;
            }

            Opcode::GetElem => {
                let dest = code[ip + 1] as usize;
                let arr = self.registers[code[ip + 2] as usize].clone();
                let idx = self.registers[code[ip + 3] as usize].as_i32().unwrap_or(0) as usize;

                if let Value::Array(ptr) = arr {
                    if idx < ptr.as_ref().elements.len() {
                        self.registers[dest] = ptr.as_ref().elements[idx].clone();
                    } else {
                        return Err(VMError::OutOfBounds(idx));
                    }
                } else {
                    return Err(VMError::TypeError {
                        expected: "Array".to_string(),
                        found: arr.type_name().to_string(),
                    });
                }
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::SetElem => {
                let arr = self.registers[code[ip + 1] as usize].clone();
                let idx = self.registers[code[ip + 2] as usize].as_i32().unwrap_or(0) as usize;
                let val = self.registers[code[ip + 3] as usize].clone();

                if let Value::Array(ptr) = arr {
                    if idx < ptr.as_ref().elements.len() {
                        // Would need mutable access
                    } else {
                        return Err(VMError::OutOfBounds(idx));
                    }
                } else {
                    return Err(VMError::TypeError {
                        expected: "Array".to_string(),
                        found: arr.type_name().to_string(),
                    });
                }
                self.current_frame.as_mut().unwrap().ip = ip + 4;
            }

            Opcode::Push => {
                let val = self.registers[code[ip + 1] as usize].clone();
                self.stack.push(val);
                self.current_frame.as_mut().unwrap().ip = ip + 2;
            }

            Opcode::Pop => {
                let dest = code[ip + 1] as usize;
                if let Some(val) = self.stack.pop() {
                    self.registers[dest] = val;
                } else {
                    return Err(VMError::StackUnderflow);
                }
                self.current_frame.as_mut().unwrap().ip = ip + 2;
            }

            Opcode::Throw => {
                let msg = self.registers[code[ip + 1] as usize].to_string();
                self.error = Some(VMError::Exception(msg));
                self.current_frame.as_mut().unwrap().ip += 2;
            }

            Opcode::Breakpoint => {
                println!("Breakpoint hit at IP: {}", ip);
                self.current_frame.as_mut().unwrap().ip += 1;
            }

            Opcode::DebugPrint => {
                let val = self.registers[code[ip + 1] as usize].clone();
                println!("DEBUG: {:?}", val);
                self.current_frame.as_mut().unwrap().ip = ip + 2;
            }

            _ => {
                return Err(VMError::UnimplementedOpcode(opcode));
            }
        }

        Ok(())
    }

    fn read_u16(&self, code: &[u8], offset: usize) -> u16 {
        u16::from_le_bytes([code[offset], code[offset + 1]])
    }

    fn read_i32(&self, code: &[u8], offset: usize) -> i32 {
        i32::from_le_bytes([code[offset], code[offset + 1], code[offset + 2], code[offset + 3]])
    }

    fn read_i64(&self, code: &[u8], offset: usize) -> i64 {
        i64::from_le_bytes([
            code[offset], code[offset + 1], code[offset + 2], code[offset + 3],
            code[offset + 4], code[offset + 5], code[offset + 6], code[offset + 7],
        ])
    }

    fn read_f64(&self, code: &[u8], offset: usize) -> f64 {
        f64::from_le_bytes([
            code[offset], code[offset + 1], code[offset + 2], code[offset + 3],
            code[offset + 4], code[offset + 5], code[offset + 6], code[offset + 7],
        ])
    }

    fn find_function(&self, name: &str) -> Result<GcPtr<Function>, VMError> {
        for module in self.modules.values() {
            if let Some(func) = module.functions.get(name) {
                return Ok(GcPtr::new(std::ptr::addr_of!(*func)));
            }
        }
        Err(VMError::FunctionNotFound(name.to_string()))
    }
}

/// Loaded module
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub functions: HashMap<String, Function>,
    pub globals: HashMap<String, Value>,
    pub constants: Vec<Value>,
}

/// VM error types
#[derive(Debug, Clone)]
pub enum VMError {
    /// Invalid opcode
    InvalidOpcode(u8),
    /// Unimplemented opcode
    UnimplementedOpcode(Opcode),
    /// Stack underflow
    StackUnderflow,
    /// Stack overflow
    StackOverflow,
    /// Out of bounds access
    OutOfBounds(usize),
    /// Division by zero
    DivisionByZero,
    /// Type error
    TypeError {
        expected: String,
        found: String,
    },
    /// Null pointer
    NullPointer,
    /// Function not found
    FunctionNotFound(String),
    /// No current frame
    NoFrame,
    /// Exception thrown
    Exception(String),
    /// Illegal instruction
    IllegalInstruction,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::InvalidOpcode(op) => write!(f, "Invalid opcode: 0x{:02x}", op),
            VMError::UnimplementedOpcode(op) => write!(f, "Unimplemented opcode: {:?}", op),
            VMError::StackUnderflow => write!(f, "Stack underflow"),
            VMError::StackOverflow => write!(f, "Stack overflow"),
            VMError::OutOfBounds(idx) => write!(f, "Out of bounds access at index {}", idx),
            VMError::DivisionByZero => write!(f, "Division by zero"),
            VMError::TypeError { expected, found } => {
                write!(f, "Type error: expected {}, found {}", expected, found)
            }
            VMError::NullPointer => write!(f, "Null pointer"),
            VMError::FunctionNotFound(name) => write!(f, "Function not found: {}", name),
            VMError::NoFrame => write!(f, "No current frame"),
            VMError::Exception(msg) => write!(f, "Exception: {}", msg),
            VMError::IllegalInstruction => write!(f, "Illegal instruction"),
        }
    }
}

impl std::error::Error for VMError {}

/// JIT compilation context
pub struct JITContext {
    /// Compiled functions
    compiled: HashMap<String, *const Function>,
    /// Code cache
    code_cache: Vec<Vec<u8>>,
}

impl JITContext {
    pub fn new() -> Self {
        JITContext {
            compiled: HashMap::new(),
            code_cache: Vec::new(),
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_operations() {
        let a = Value::I32(10);
        let b = Value::I32(20);

        assert_eq!(a.as_i32(), Some(10));
        assert_eq!(b.as_i32(), Some(20));
        assert_eq!(a.as_i64(), Some(10));
    }

    #[test]
    fn test_vm_creation() {
        let mut vm = VM::new();
        vm.set_global("test".to_string(), Value::I32(42));

        assert_eq!(vm.get_global("test"), Some(&Value::I32(42)));
    }

    #[test]
    fn test_bytecode_execution() {
        let mut vm = VM::new();

        // Simple bytecode: 10 + 20 = 30
        let mut code = vec![
            Opcode::LdImm32 as u8, 0, // dest = r0
            10, 0, 0, 0, // value = 10
            Opcode::LdImm32 as u8, 1, // dest = r1
            20, 0, 0, 0, // value = 20
            Opcode::AddI32 as u8, 2, 0, 1, // r2 = r0 + r1
            Opcode::Halt as u8,
        ];

        let result = vm.execute(code, "main");
        assert!(result.is_ok());
    }
}
