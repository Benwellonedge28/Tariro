//! TUMC WASM Engine and Runtime
//!
//! WebAssembly execution engine, runtime support, and WASM-specific
//! optimizations for the TUMC compiler.

use std::collections::HashMap;

/// WASM value types
#[derive(Debug, Clone, PartialEq)]
pub enum WasmValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128(i128),
    FuncRef(u32),
    ExternRef(u32),
}

impl WasmValue {
    /// Get i32 value
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            WasmValue::I32(v) => Some(*v),
            _ => None,
        }
    }

    /// Get i64 value
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            WasmValue::I64(v) => Some(*v),
            _ => None,
        }
    }

    /// Get f32 value
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            WasmValue::F32(v) => Some(*v),
            _ => None,
        }
    }

    /// Get f64 value
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            WasmValue::F64(v) => Some(*v),
            _ => None,
        }
    }
}

/// WASM function instance
#[derive(Debug, Clone)]
pub struct WasmFunction {
    /// Function type
    pub type_index: u32,
    /// Code (bytecode)
    pub code: Vec<u8>,
    /// Local variables
    pub locals: Vec<(u32, WasmValType)>,
    /// Exported name (if exported)
    pub export_name: Option<String>,
}

/// WASM export
#[derive(Debug, Clone)]
pub enum WasmExport {
    Function(u32),
    Table(u32),
    Memory(u32),
    Global(u32),
}

/// WASM module
#[derive(Debug, Clone, Default)]
pub struct WasmModule {
    /// Function types
    pub types: Vec<WasmFuncType>,
    /// Functions
    pub functions: Vec<WasmFunction>,
    /// Tables
    pub tables: Vec<WasmTable>,
    /// Memories
    pub memories: Vec<WasmMemory>,
    /// Globals
    pub globals: Vec<WasmGlobal>,
    /// Exports
    pub exports: HashMap<String, WasmExport>,
    /// Start function index
    pub start: Option<u32>,
}

/// WASM function type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WasmFuncType {
    pub params: Vec<WasmValType>,
    pub results: Vec<WasmValType>,
}

/// WASM value type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WasmValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}

/// WASM table
#[derive(Debug, Clone)]
pub struct WasmTable {
    pub element_type: WasmValType,
    pub initial: u32,
    pub maximum: Option<u32>,
    pub elements: Vec<u32>,
}

/// WASM memory
#[derive(Debug, Clone)]
pub struct WasmMemory {
    pub initial: u32,
    pub maximum: Option<u32>,
    pub data: Vec<u8>,
}

impl WasmMemory {
    /// Create new memory
    pub fn new(initial_pages: u32, max_pages: Option<u32>) -> Self {
        let page_size = 64 * 1024;
        Self {
            initial: initial_pages,
            maximum: max_pages,
            data: vec![0; (initial_pages as usize) * page_size],
        }
    }

    /// Read i32 from memory
    pub fn read_i32(&self, addr: u32) -> Option<i32> {
        if addr + 4 > self.data.len() as u32 {
            return None;
        }
        let bytes: [u8; 4] = self.data[addr as usize..addr as usize + 4].try_into().ok()?;
        Some(i32::from_le_bytes(bytes))
    }

    /// Write i32 to memory
    pub fn write_i32(&mut self, addr: u32, value: i32) -> bool {
        if addr + 4 > self.data.len() as u32 {
            return false;
        }
        self.data[addr as usize..addr as usize + 4].copy_from_slice(&value.to_le_bytes());
        true
    }

    /// Read i64 from memory
    pub fn read_i64(&self, addr: u32) -> Option<i64> {
        if addr + 8 > self.data.len() as u32 {
            return None;
        }
        let bytes: [u8; 8] = self.data[addr as usize..addr as usize + 8].try_into().ok()?;
        Some(i64::from_le_bytes(bytes))
    }

    /// Write i64 to memory
    pub fn write_i64(&mut self, addr: u32, value: i64) -> bool {
        if addr + 8 > self.data.len() as u32 {
            return false;
        }
        self.data[addr as usize..addr as usize + 8].copy_from_slice(&value.to_le_bytes());
        true
    }

    /// Read f32 from memory
    pub fn read_f32(&self, addr: u32) -> Option<f32> {
        self.read_i32(addr).map(f32::from_bits)
    }

    /// Write f32 to memory
    pub fn write_f32(&mut self, addr: u32, value: f32) -> bool {
        self.write_i32(addr, value.to_bits())
    }

    /// Read f64 from memory
    pub fn read_f64(&self, addr: u32) -> Option<f64> {
        self.read_i64(addr).map(f64::from_bits)
    }

    /// Write f64 to memory
    pub fn write_f64(&mut self, addr: u32, value: f64) -> bool {
        self.write_i64(addr, value.to_bits())
    }

    /// Grow memory
    pub fn grow(&mut self, pages: u32) -> bool {
        let new_pages = self.data.len() as u32 / (64 * 1024) + pages;
        if let Some(max) = self.maximum {
            if new_pages > max {
                return false;
            }
        }

        let additional_size = (pages as usize) * 64 * 1024;
        self.data.resize(self.data.len() + additional_size, 0);
        true
    }

    /// Get memory size in pages
    pub fn size(&self) -> u32 {
        (self.data.len() / (64 * 1024)) as u32
    }
}

/// WASM global
#[derive(Debug, Clone)]
pub struct WasmGlobal {
    pub value_type: WasmValType,
    pub mutable: bool,
    pub value: WasmValue,
}

/// WASM instruction
#[derive(Debug, Clone)]
pub enum WasmInstruction {
    // Control flow
    Unreachable,
    Nop,
    Block(WasmBlockType),
    Loop(WasmBlockType),
    If(WasmBlockType),
    Else,
    End,
    Br(u32),
    BrIf(u32),
    BrTable(Vec<u32>, u32),
    Return,
    Call(u32),
    CallIndirect(u32, u32),

    // Numeric
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
    I32Add,
    I32Sub,
    I32Mul,
    I32DivS,
    I32DivU,
    I32RemS,
    I32RemU,
    I32And,
    I32Or,
    I32Xor,
    I32Shl,
    I32ShrS,
    I32ShrU,
    I32Rotl,
    I32Rotr,
    I32Clz,
    I32Ctz,
    I32Popcnt,
    I32Eqz,
    I32Eq,
    I32Ne,
    I32LtS,
    I32LtU,
    I32GtS,
    I32GtU,
    I32LeS,
    I32LeU,
    I32GeS,
    I32GeU,
    // ... similar for I64, F32, F64

    // Memory
    I32Load(MemArg),
    I64Load(MemArg),
    F32Load(MemArg),
    F64Load(MemArg),
    I32Store(MemArg),
    I64Store(MemArg),
    F32Store(MemArg),
    F64Store(MemArg),
    MemorySize,
    MemoryGrow,

    // Variable
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

    // Table
    TableGet(u32),
    TableSet(u32),
    TableInit(u32, u32),
    ElemDrop(u32),
    TableCopy(u32, u32),
    TableGrow(u32),
    TableSize(u32),
    TableFill(u32),

    // Reference types
    RefNull,
    RefIsNull,
    RefFunc(u32),

    // SIMD
    V128Load(MemArg),
    V128Store(MemArg),
    V128Const(i128),
    I8x16Splat,
    I8x16ExtractLane(u8),
    I8x16ReplaceLane(u8),
    // ... more SIMD instructions

    // Atomic (memory.grow.a, memory.atomic.*)
    MemoryAtomicNotify(MemArg),
    MemoryAtomicWait32(MemArg),
    MemoryAtomicWait64(MemArg),
    AtomicFence,
}

/// WASM block type
#[derive(Debug, Clone)]
pub enum WasmBlockType {
    Empty,
    Value(WasmValType),
    MultiValue(Vec<WasmValType>),
}

/// Memory argument
#[derive(Debug, Clone)]
pub struct MemArg {
    pub align: u32,
    pub offset: u32,
}

/// WASM execution engine
pub struct WasmEngine {
    /// Module
    module: WasmModule,
    /// Operand stack
    operand_stack: Vec<WasmValue>,
    /// Control stack
    control_stack: Vec<ControlFrame>,
    /// Globals
    globals: Vec<WasmValue>,
    /// Tables
    tables: Vec<WasmTable>,
    /// Memory
    memory: WasmMemory,
    /// Call stack limit
    call_stack_limit: usize,
}

#[derive(Debug, Clone)]
struct ControlFrame {
    pub start: usize,
    pub end: usize,
    pub block_type: WasmBlockType,
    pub locals: Vec<WasmValue>,
}

impl WasmEngine {
    /// Create new engine
    pub fn new(module: WasmModule) -> Self {
        let globals = module
            .globals
            .iter()
            .map(|g| g.value.clone())
            .collect();

        let tables = module.tables.clone();
        let memory = module
            .memories
            .first()
            .cloned()
            .unwrap_or_else(|| WasmMemory::new(0, None));

        Self {
            module,
            operand_stack: Vec::new(),
            control_stack: Vec::new(),
            globals,
            tables,
            memory,
            call_stack_limit: 10000,
        }
    }

    /// Execute a function
    pub fn execute(&mut self, func_index: u32, args: &[WasmValue]) -> Result<Vec<WasmValue>, WasmError> {
        if self.control_stack.len() >= self.call_stack_limit {
            return Err(WasmError::StackOverflow);
        }

        let func = self
            .module
            .functions
            .get(func_index as usize)
            .ok_or(WasmError::UnknownFunction)?;

        // Initialize locals
        let mut locals = args.to_vec();
        for (count, ty) in &func.locals {
            for _ in 0..*count {
                locals.push(match ty {
                    WasmValType::I32 => WasmValue::I32(0),
                    WasmValType::I64 => WasmValue::I64(0),
                    WasmValType::F32 => WasmValue::F32(0.0),
                    WasmValType::F64 => WasmValue::F64(0.0),
                    WasmValType::V128 => WasmValue::V128(0),
                    _ => WasmValue::FuncRef(0),
                });
            }
        }

        // Create initial control frame
        self.control_stack.push(ControlFrame {
            start: 0,
            end: 0,
            block_type: WasmBlockType::Empty,
            locals,
        });

        // Decode and execute
        let mut pc = 0usize;
        loop {
            if pc >= func.code.len() {
                break;
            }

            let opcode = func.code[pc];
            pc += 1;

            match self.execute_instruction(opcode, &func.code, &mut pc)? {
                ControlFlow::Continue => {}
                ControlFlow::Break(level) => {
                    // Unwind to end of block
                    while let Some(frame) = self.control_stack.last() {
                        if self.operand_stack.len() > frame.start {
                            self.operand_stack.truncate(frame.start);
                        }
                        self.control_stack.pop();
                        if level == 0 {
                            break;
                        }
                        let _ = level - 1;
                    }
                }
                ControlFlow::Return => {
                    break;
                }
                ControlFlow::Trap(msg) => {
                    return Err(WasmError::Trap(msg));
                }
            }
        }

        // Get results
        let results = self.operand_stack.drain(..).collect();
        Ok(results)
    }

    /// Execute single instruction
    fn execute_instruction(
        &mut self,
        opcode: u8,
        code: &[u8],
        pc: &mut usize,
    ) -> Result<ControlFlow, WasmError> {
        match opcode {
            // Constants
            0x41 => {
                // i32.const
                let val = i32::from_le_bytes(code[*pc..*pc + 4].try_into().unwrap());
                *pc += 4;
                self.operand_stack.push(WasmValue::I32(val));
            }
            0x42 => {
                // i64.const
                let val = i64::from_le_bytes(code[*pc..*pc + 8].try_into().unwrap());
                *pc += 8;
                self.operand_stack.push(WasmValue::I64(val));
            }
            0x43 => {
                // f32.const
                let val = f32::from_le_bytes(code[*pc..*pc + 4].try_into().unwrap());
                *pc += 4;
                self.operand_stack.push(WasmValue::F32(val));
            }
            0x44 => {
                // f64.const
                let val = f64::from_le_bytes(code[*pc..*pc + 8].try_into().unwrap());
                *pc += 8;
                self.operand_stack.push(WasmValue::F64(val));
            }

            // Memory operations
            0x28 => {
                // i32.load
                let align = code[*pc];
                let offset = u32::from_le_bytes(code[*pc + 1..*pc + 5].try_into().unwrap());
                *pc += 5;
                let addr = self.pop_i32()?;
                let effective_addr = addr + offset;
                if let Some(val) = self.memory.read_i32(effective_addr) {
                    self.operand_stack.push(WasmValue::I32(val));
                } else {
                    return Err(WasmError::MemoryAccessViolation);
                }
            }
            0x36 => {
                // i32.store
                let align = code[*pc];
                let offset = u32::from_le_bytes(code[*pc + 1..*pc + 5].try_into().unwrap());
                *pc += 5;
                let val = self.pop_i32()?;
                let addr = self.pop_i32()?;
                if !self.memory.write_i32(addr + offset, val) {
                    return Err(WasmError::MemoryAccessViolation);
                }
            }
            0x3F => {
                // memory.size
                self.operand_stack.push(WasmValue::I32(self.memory.size() as i32));
            }
            0x40 => {
                // memory.grow
                let pages = self.pop_i32()?;
                let old_size = self.memory.size();
                if self.memory.grow(pages as u32) {
                    self.operand_stack.push(WasmValue::I32(old_size as i32));
                } else {
                    self.operand_stack.push(WasmValue::I32(-1));
                }
            }

            // Numeric operations
            0x6A => {
                // i32.add
                let rhs = self.pop_i32()?;
                let lhs = self.pop_i32()?;
                self.operand_stack.push(WasmValue::I32(lhs.wrapping_add(rhs)));
            }
            0x6B => {
                // i32.sub
                let rhs = self.pop_i32()?;
                let lhs = self.pop_i32()?;
                self.operand_stack.push(WasmValue::I32(lhs.wrapping_sub(rhs)));
            }
            0x6C => {
                // i32.mul
                let rhs = self.pop_i32()?;
                let lhs = self.pop_i32()?;
                self.operand_stack.push(WasmValue::I32(lhs.wrapping_mul(rhs)));
            }
            0x71 => {
                // i32.eqz
                let val = self.pop_i32()?;
                self.operand_stack.push(WasmValue::I32(if val == 0 { 1 } else { 0 }));
            }
            0x46 => {
                // i32.eq
                let rhs = self.pop_i32()?;
                let lhs = self.pop_i32()?;
                self.operand_stack.push(WasmValue::I32(if lhs == rhs { 1 } else { 0 }));
            }
            0x47 => {
                // i32.ne
                let rhs = self.pop_i32()?;
                let lhs = self.pop_i32()?;
                self.operand_stack.push(WasmValue::I32(if lhs != rhs { 1 } else { 0 }));
            }

            // Control flow
            0x0B => {
                // end
                // Return from current block/function
            }
            0x0F => {
                // return
                return Ok(ControlFlow::Return);
            }
            0x10 => {
                // call
                let func_index = code[*pc] as u32;
                *pc += 1;
                let args: Vec<_> = (0..self.module.types[self.module.functions[func_index as usize].type_index as usize].params.len())
                    .map(|_| self.operand_stack.pop().unwrap())
                    .collect();
                let args: Vec<_> = args.into_iter().rev().collect();

                let result = self.execute(func_index, &args)?;
                for val in result {
                    self.operand_stack.push(val);
                }
            }
            0x1A => {
                // drop
                self.operand_stack.pop();
            }

            // Local operations
            0x20 => {
                // local.get
                let idx = code[*pc] as usize;
                *pc += 1;
                if let Some(frame) = self.control_stack.last() {
                    if idx < frame.locals.len() {
                        self.operand_stack.push(frame.locals[idx].clone());
                    }
                }
            }
            0x21 => {
                // local.set
                let idx = code[*pc] as usize;
                *pc += 1;
                if let Some(frame) = self.control_stack.last_mut() {
                    if idx < frame.locals.len() {
                        if let Some(val) = self.operand_stack.pop() {
                            frame.locals[idx] = val;
                        }
                    }
                }
            }

            // Global operations
            0x23 => {
                // global.get
                let idx = code[*pc] as usize;
                *pc += 1;
                if idx < self.globals.len() {
                    self.operand_stack.push(self.globals[idx].clone());
                }
            }
            0x24 => {
                // global.set
                let idx = code[*pc] as usize;
                *pc += 1;
                if idx < self.globals.len() {
                    if let Some(val) = self.operand_stack.pop() {
                        self.globals[idx] = val;
                    }
                }
            }

            _ => {
                // Unknown opcode
            }
        }

        Ok(ControlFlow::Continue)
    }

    /// Pop i32 from stack
    fn pop_i32(&mut self) -> Result<i32, WasmError> {
        self.operand_stack
            .pop()
            .and_then(|v| v.as_i32())
            .ok_or(WasmError::StackUnderflow)
    }

    /// Pop i64 from stack
    fn pop_i64(&mut self) -> Result<i64, WasmError> {
        self.operand_stack
            .pop()
            .and_then(|v| v.as_i64())
            .ok_or(WasmError::StackUnderflow)
    }
}

/// WASM control flow
#[derive(Debug)]
pub enum ControlFlow {
    Continue,
    Break(u32),
    Return,
    Trap(String),
}

/// WASM error
#[derive(Debug)]
pub enum WasmError {
    StackOverflow,
    StackUnderflow,
    UnknownFunction,
    MemoryAccessViolation,
    TypeMismatch,
    Trap(String),
    InvalidOpcode,
}

/// WASM validator
pub struct WasmValidator;

impl WasmValidator {
    /// Validate module
    pub fn validate(module: &WasmModule) -> Result<(), ValidationError> {
        // Check function types exist
        for func in &module.functions {
            if func.type_index as usize >= module.types.len() {
                return Err(ValidationError::InvalidTypeIndex);
            }
        }

        // Check exports exist
        for (name, export) in &module.exports {
            match export {
                WasmExport::Function(idx) if *idx as usize >= module.functions.len() => {
                    return Err(ValidationError::InvalidExport(name.clone()));
                }
                _ => {}
            }
        }

        // Check start function exists
        if let Some(start) = module.start {
            if start as usize >= module.functions.len() {
                return Err(ValidationError::InvalidStart);
            }
        }

        Ok(())
    }
}

/// Validation error
#[derive(Debug)]
pub enum ValidationError {
    InvalidTypeIndex,
    InvalidExport(String),
    InvalidStart,
    TypeMismatch,
}

/// WASM linker
pub struct WasmLinker;

impl WasmLinker {
    /// Link module with imports
    pub fn link(module: &mut WasmModule, imports: HashMap<String, WasmExport>) -> Result<(), LinkError> {
        for (name, export) in imports {
            module.exports.insert(name, export);
        }
        Ok(())
    }
}

/// Link error
#[derive(Debug)]
pub enum LinkError {
    MissingImport(String),
    TypeMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_memory() {
        let mut mem = WasmMemory::new(1, None);
        assert!(mem.write_i32(0, 42));
        assert_eq!(mem.read_i32(0), Some(42));
    }

    #[test]
    fn test_wasm_engine_execution() {
        // Simple function: i32.const 42
        let module = WasmModule {
            types: vec![WasmFuncType {
                params: vec![],
                results: vec![WasmValType::I32],
            }],
            functions: vec![WasmFunction {
                type_index: 0,
                code: vec![0x41, 0x2A, 0x00, 0x00], // i32.const 42
                locals: vec![],
                export_name: Some("test".to_string()),
            }],
            ..Default::default()
        };

        let mut engine = WasmEngine::new(module);
        let result = engine.execute(0, &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_wasm_validator() {
        let module = WasmModule::default();
        assert!(WasmValidator::validate(&module).is_ok());
    }
}
