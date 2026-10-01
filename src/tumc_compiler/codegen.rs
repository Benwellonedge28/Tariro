//! WASM code generation for TUMC compiler
//! Generates WebAssembly bytecode from parsed and type-checked AST

use crate::prelude::*;
use super::ast::{self, *};
use super::type_checker::{Type, TypeEnv};
use super::optimizer::OptimizationLevel;
use std::collections::{HashMap, HashSet};

/// WASM value types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmType {
    I32,
    I64,
    F32,
    F64,
    V128,
    Funcref,
    Externref,
    Empty,
}

impl WasmType {
    /// Convert from TUMC type to WASM type
    pub fn from_tumc(type_: &Type) -> Option<WasmType> {
        match type_ {
            Type::Int => Some(WasmType::I32),
            Type::Float => Some(WasmType::F64), // Use F64 for precision
            Type::Bool | Type::Char => Some(WasmType::I32),
            Type::Unit => Some(WasmType::Empty),
            _ => None,
        }
    }

    /// Get WASM type name
    pub fn name(&self) -> &'static str {
        match self {
            WasmType::I32 => "i32",
            WasmType::I64 => "i64",
            WasmType::F32 => "f32",
            WasmType::F64 => "f64",
            WasmType::V128 => "v128",
            WasmType::Funcref => "funcref",
            WasmType::Externref => "externref",
            WasmType::Empty => "",
        }
    }
}

/// WASM instruction
#[derive(Debug, Clone)]
pub enum WasmInstruction {
    // Control flow
    Unreachable,
    Nop,
    Block(WasmType, Vec<WasmInstruction>),
    Loop(WasmType, Vec<WasmInstruction>),
    If(WasmType, Vec<WasmInstruction>, Vec<WasmInstruction>),
    Br(u32),
    BrIf(u32),
    BrTable(Vec<u32>, u32),
    Return,
    Call(u32),
    CallIndirect(u32, u32),

    // Parametric
    Drop,
    Select,

    // Variable access
    LocalGet(u32),
    LocalSet(u32),
    LocalTee(u32),
    GlobalGet(u32),
    GlobalSet(u32),

    // Memory instructions
    I32Load(i32, i32),
    I64Load(i32, i32),
    F32Load(i32, i32),
    F64Load(i32, i32),
    I32Store(i32, i32),
    I64Store(i32, i32),
    F32Store(i32, i32),
    F64Store(i32, i32),
    MemorySize,
    MemoryGrow,

    // Numeric constants
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),

    // Numeric operations - i32
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

    // Numeric operations - i64
    I64Eqz,
    I64Eq,
    I64Ne,
    I64LtS,
    I64LtU,
    I64GtS,
    I64GtU,
    I64LeS,
    I64LeU,
    I64GeS,
    I64GeU,
    I64Add,
    I64Sub,
    I64Mul,
    I64DivS,
    I64DivU,
    I64RemS,
    I64RemU,
    I64And,
    I64Or,
    I64Xor,
    I64Shl,
    I64ShrS,
    I64ShrU,

    // Numeric operations - f32
    F32Eq,
    F32Ne,
    F32Lt,
    F32Gt,
    F32Le,
    F32Ge,
    F32Add,
    F32Sub,
    F32Mul,
    F32Div,
    F32Neg,
    F32Abs,
    F32Ceil,
    F32Floor,
    F32Trunc,
    F32Nearest,
    F32Sqrt,

    // Numeric operations - f64
    F64Eq,
    F64Ne,
    F64Lt,
    F64Gt,
    F64Le,
    F64Ge,
    F64Add,
    F64Sub,
    F64Mul,
    F64Div,
    F64Neg,
    F64Abs,
    F64Ceil,
    F64Floor,
    F64Trunc,
    F64Nearest,
    F64Sqrt,

    // Conversion
    I32WrapI64,
    I64ExtendI32S,
    I64ExtendI32U,
    I32TruncF32S,
    I32TruncF32U,
    I32TruncF64S,
    I32TruncF64U,
    I64TruncF32S,
    I64TruncF32U,
    I64TruncF64S,
    I64TruncF64U,
    F32DemoteF64,
    F64PromoteF32,
    F32ConvertI32S,
    F32ConvertI32U,
    F32ConvertI64S,
    F32ConvertI64U,
    F64ConvertI32S,
    F64ConvertI32U,
    F64ConvertI64S,
    F64ConvertI64U,
}

/// WASM function type
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmFuncType {
    pub params: Vec<WasmType>,
    pub results: Vec<WasmType>,
}

impl WasmFuncType {
    pub fn new(params: Vec<WasmType>, results: Vec<WasmType>) -> Self {
        WasmFuncType { params, results }
    }

    /// Get type signature bytes for WAT output
    pub fn to_wat(&self) -> String {
        let params = if self.params.is_empty() {
            "".to_string()
        } else {
            format!(" (param{})", self.params.iter()
                .map(|t| format!(" {}", t.name()))
                .collect::<String>())
        };

        let results = if self.results.is_empty() {
            "".to_string()
        } else {
            format!(" (result{})", self.results.iter()
                .map(|t| format!(" {}", t.name()))
                .collect::<String>())
        };

        format!("(func (type {}){} {})", 0, params, results)
    }
}

/// WASM function definition
#[derive(Debug, Clone)]
pub struct WasmFunction {
    pub name: String,
    pub type_idx: u32,
    pub locals: Vec<WasmType>,
    pub body: Vec<WasmInstruction>,
    pub export: bool,
}

/// WASM global
#[derive(Debug, Clone)]
pub struct WasmGlobal {
    pub name: String,
    pub type_: WasmType,
    pub mutable: bool,
    pub init: Vec<WasmInstruction>,
}

/// WASM memory
#[derive(Debug, Clone)]
pub struct WasmMemory {
    pub min_pages: u32,
    pub max_pages: Option<u32>,
    pub data: Vec<u8>,
}

/// WASM module
#[derive(Debug, Clone, Default)]
pub struct WasmModule {
    pub types: Vec<WasmFuncType>,
    pub functions: Vec<WasmFunction>,
    pub globals: Vec<WasmGlobal>,
    pub memory: Option<WasmMemory>,
    pub exports: Vec<(String, WasmExportKind, u32)>,
    pub imports: Vec<WasmImport>,
}

#[derive(Debug, Clone)]
pub enum WasmExportKind {
    Function,
    Table,
    Memory,
    Global,
}

#[derive(Debug, Clone)]
pub struct WasmImport {
    pub module: String,
    pub name: String,
    pub kind: WasmExportKind,
    pub type_idx: u32,
}

/// Code generator context
pub struct CodeGen {
    /// WASM module being built
    module: WasmModule,
    /// Function index counter
    next_func_idx: u32,
    /// Type index counter
    next_type_idx: u32,
    /// Label counter for branching
    next_label: u32,
    /// Current function context
    current_func: Option<WasmFunction>,
    /// Local variable mapping
    locals: HashMap<String, u32>,
    /// Next local index
    next_local: u32,
    /// Optimization level
    opt_level: OptimizationLevel,
    /// Debug info
    emit_debug: bool,
}

impl CodeGen {
    /// Create new code generator
    pub fn new(opt_level: OptimizationLevel) -> Self {
        CodeGen {
            module: WasmModule::default(),
            next_func_idx: 0,
            next_type_idx: 0,
            next_label: 0,
            current_func: None,
            locals: HashMap::new(),
            next_local: 0,
            opt_level,
            emit_debug: false,
        }
    }

    /// Enable debug info
    pub fn with_debug(mut self, enable: bool) -> Self {
        self.emit_debug = enable;
        self
    }

    /// Get WASM module
    pub fn module(&self) -> &WasmModule {
        &self.module
    }

    /// Generate code from program
    pub fn generate(&mut self, program: &Program) -> Result<(), String> {
        println!("TUMC: CodeGen - Generating WASM code for program with {} items", program.items.len());

        // Reserve type index 0 for the main function type
        self.module.types.push(WasmFuncType::new(vec![], vec![]));
        self.next_type_idx = 1;

        // First pass: collect type signatures
        for item in &program.items {
            if let Item::Function(func) = item {
                self.collect_type_signature(func);
            }
        }

        // Second pass: generate code for each item
        for item in &program.items {
            self.generate_item(item)?;
        }

        // Add memory if needed
        if self.module.functions.iter().any(|f| matches!(
            f.body.iter().find(|i| matches!(i, WasmInstruction::MemorySize)),
            Some(_)
        )) {
            self.module.memory = Some(WasmMemory {
                min_pages: 1,
                max_pages: Some(256),
                data: vec![],
            });
        }

        Ok(())
    }

    /// Collect type signature
    fn collect_type_signature(&mut self, func: &Function) {
        let params: Vec<WasmType> = func.params.iter()
            .filter_map(|p| WasmType::from_tumc(&Type::Int)) // Simplified
            .collect();

        let result: Vec<WasmType> = match &func.return_type {
            AnnType::Unit => vec![],
            AnnType::Int => vec![WasmType::I32],
            AnnType::Float => vec![WasmType::F64],
            _ => vec![WasmType::I32], // Default to i32
        };

        let func_type = WasmFuncType::new(params, result);

        // Check if this type already exists
        if !self.module.types.contains(&func_type) {
            self.module.types.push(func_type);
            self.next_type_idx += 1;
        }
    }

    /// Generate code for an item
    fn generate_item(&mut self, item: &Item) -> Result<(), String> {
        match item {
            Item::Function(func) => self.generate_function(func),
            Item::Struct(def) => {
                // Generate memory layout for struct
                self.generate_struct_layout(def)
            }
            Item::Constant(def) => {
                // Generate global constant
                self.generate_constant(def)
            }
            _ => Ok(()), // Skip other items for now
        }
    }

    /// Generate function
    fn generate_function(&mut self, func: &Function) -> Result<(), String> {
        println!("TUMC: CodeGen - Generating function '{}'", func.name);

        // Find or create type index
        let type_idx = self.find_or_create_type(func);

        // Create function context
        let mut wasm_func = WasmFunction {
            name: func.name.clone(),
            type_idx,
            locals: vec![],
            body: vec![],
            export: func.name == "main" || func.name == "_start",
        };

        // Setup locals mapping
        self.locals.clear();
        self.next_local = 0;

        // Add parameters as locals
        for param in &func.params {
            self.locals.insert(param.name.clone(), self.next_local);
            wasm_func.locals.push(WasmType::I32); // Simplified
            self.next_local += 1;
        }

        // Generate function body
        let prev_func = self.current_func.replace(wasm_func);

        for stmt in &func.body {
            self.generate_statement(stmt)?;
        }

        // If no explicit return, add implicit return
        if let Some(mut func) = self.current_func.take() {
            // Check if last instruction is already a control flow instruction
            if let Some(WasmInstruction::Return) = func.body.last() {
                // Already has return
            } else {
                func.body.push(WasmInstruction::End);
            }

            let func_idx = self.next_func_idx;
            self.next_func_idx += 1;

            // Add to module
            self.module.functions.push(func);

            // Add export if needed
            if func.name == "main" || func.name == "_start" {
                self.module.exports.push((
                    func.name.clone(),
                    WasmExportKind::Function,
                    func_idx,
                ));
            }
        }

        self.current_func = prev_func;
        Ok(())
    }

    /// Find or create type for function
    fn find_or_create_type(&mut self, func: &Function) -> u32 {
        let params: Vec<WasmType> = func.params.iter()
            .map(|_| WasmType::I32)
            .collect();

        let result: Vec<WasmType> = match &func.return_type {
            AnnType::Unit => vec![],
            AnnType::Int => vec![WasmType::I32],
            AnnType::Float => vec![WasmType::F64],
            _ => vec![WasmType::I32],
        };

        let func_type = WasmFuncType::new(params, result);

        for (idx, existing) in self.module.types.iter().enumerate() {
            if existing == &func_type {
                return idx as u32;
            }
        }

        let idx = self.module.types.len() as u32;
        self.module.types.push(func_type);
        idx
    }

    /// Generate struct layout
    fn generate_struct_layout(&mut self, struct_: &Struct) -> Result<(), String> {
        let mut offset = 0i32;
        let mut field_types = Vec::new();

        for field in &struct_.fields {
            // Assume i32 for now
            field_types.push((field.name.clone(), offset, WasmType::I32));
            offset += 4;
        }

        println!("TUMC: CodeGen - Generated layout for struct '{}': {} bytes", struct_.name, offset);
        Ok(())
    }

    /// Generate constant as global
    fn generate_constant(&mut self, const_: &Constant) -> Result<(), String> {
        let global_idx = self.module.globals.len() as u32;

        // Generate initial value
        let mut init = vec![];
        self.generate_expression(&const_.value, &mut init)?;

        self.module.globals.push(WasmGlobal {
            name: const_.name.clone(),
            type_: WasmType::I32,
            mutable: false,
            init,
        });

        self.module.exports.push((
            const_.name.clone(),
            WasmExportKind::Global,
            global_idx,
        ));

        Ok(())
    }

    /// Generate statement
    fn generate_statement(&mut self, stmt: &Stmt) -> Result<(), String> {
        let func = self.current_func.as_mut().ok_or("No current function")?;

        match &stmt.kind {
            StmtKind::Expr(expr) => {
                self.generate_expression(expr, &mut func.body)?;
                func.body.push(WasmInstruction::Drop); // Discard result
            }

            StmtKind::Let(pattern, _, value, body) => {
                if let Some(val) = value {
                    self.generate_expression(val, &mut func.body)?;

                    // Store in local
                    let local_idx = self.next_local;
                    self.next_local += 1;
                    func.locals.push(WasmType::I32);

                    if let ExprKind::Identifier(name) = &pattern.kind {
                        self.locals.insert(name.clone(), local_idx);
                    }

                    func.body.push(WasmInstruction::LocalSet(local_idx));
                }

                for s in body {
                    self.generate_statement(s)?;
                }
            }

            StmtKind::If(condition, then_block, else_block) => {
                // Generate condition
                self.generate_expression(condition, &mut func.body)?;

                // Create if-else block
                let mut then_instrs = vec![];
                let mut else_instrs = vec![];

                for s in then_block {
                    self.generate_statement_to(&s, &mut then_instrs)?;
                }

                if let Some((_, else_stmts)) = else_block {
                    for s in else_stmts {
                        self.generate_statement_to(&s, &mut else_instrs)?;
                    }
                }

                func.body.push(WasmInstruction::If(
                    WasmType::Empty,
                    then_instrs,
                    else_instrs,
                ));
            }

            StmtKind::While(condition, body) => {
                let loop_label = self.next_label;
                self.next_label += 1;

                // Generate loop header
                func.body.push(WasmInstruction::Block(WasmType::Empty, vec![
                    // Check condition
                    self.generate_expression_to(condition, &mut func.body.clone())?,
                    WasmInstruction::BrIf(loop_label), // Exit if false
                ]));

                // Generate body
                for s in body {
                    self.generate_statement(s)?;
                }

                // Jump back to condition
                func.body.push(WasmInstruction::Br(0));
                func.body.push(WasmInstruction::End);
            }

            StmtKind::For(_, _, _) => {
                // For loop: implement as while loop
                // Simplified - full implementation would handle iterators
            }

            StmtKind::Loop(body) => {
                let loop_label = self.next_label;
                self.next_label += 1;

                func.body.push(WasmInstruction::Loop(WasmType::Empty, vec![]));

                for s in body {
                    self.generate_statement(s)?;
                }

                func.body.push(WasmInstruction::Br(0));
                func.body.push(WasmInstruction::End);
            }

            StmtKind::Break => {
                func.body.push(WasmInstruction::Br(1));
            }

            StmtKind::Continue => {
                func.body.push(WasmInstruction::Br(0));
            }

            StmtKind::Return(expr) => {
                if let Some(e) = expr {
                    self.generate_expression(e, &mut func.body)?;
                }
                func.body.push(WasmInstruction::Return);
            }

            StmtKind::Assignment(lhs, rhs) => {
                // Generate RHS first
                self.generate_expression(rhs, &mut func.body)?;

                // Store to LHS
                if let ExprKind::Identifier(name) = &lhs.kind {
                    if let Some(local_idx) = self.locals.get(name) {
                        func.body.push(WasmInstruction::LocalSet(*local_idx));
                    }
                }
            }

            StmtKind::Assert(expr) => {
                self.generate_expression(expr, &mut func.body)?;
                // Could add runtime assertion handling here
            }

            _ => {}
        }

        Ok(())
    }

    /// Generate statement to specific instruction list
    fn generate_statement_to(&mut self, stmt: &Stmt, instrs: &mut Vec<WasmInstruction>) -> Result<(), String> {
        match &stmt.kind {
            StmtKind::Expr(expr) => {
                self.generate_expression(expr, instrs)?;
                instrs.push(WasmInstruction::Drop);
            }
            StmtKind::Return(Some(expr)) => {
                self.generate_expression(expr, instrs)?;
                instrs.push(WasmInstruction::Return);
            }
            StmtKind::Return(None) => {
                instrs.push(WasmInstruction::Return);
            }
            StmtKind::Break => {
                instrs.push(WasmInstruction::Br(1));
            }
            StmtKind::Continue => {
                instrs.push(WasmInstruction::Br(0));
            }
            _ => {
                self.generate_statement(stmt)?;
            }
        }
        Ok(())
    }

    /// Generate expression to specific instruction list
    fn generate_expression_to(&self, expr: &Expr, instrs: &mut Vec<WasmInstruction>) -> Result<WasmInstruction, String> {
        // This is a helper - actual generation happens in generate_expression
        Ok(WasmInstruction::Nop)
    }

    /// Generate expression
    fn generate_expression(&self, expr: &Expr, instrs: &mut Vec<WasmInstruction>) -> Result<(), String> {
        match &expr.kind {
            ExprKind::Literal(lit) => {
                self.generate_literal(lit, instrs);
            }

            ExprKind::Identifier(name) => {
                if let Some(local_idx) = self.locals.get(name) {
                    instrs.push(WasmInstruction::LocalGet(*local_idx));
                } else {
                    // Check for global
                    instrs.push(WasmInstruction::GlobalGet(0)); // Simplified
                }
            }

            ExprKind::Binary { op, lhs, rhs } => {
                self.generate_expression(lhs, instrs)?;
                self.generate_expression(rhs, instrs)?;
                self.generate_binary_op(op, instrs);
            }

            ExprKind::Unary { op, operand } => {
                self.generate_expression(operand, instrs)?;
                self.generate_unary_op(op, instrs);
            }

            ExprKind::Call { function, arguments } => {
                // Generate arguments in reverse order for stack
                for arg in arguments.iter().rev() {
                    self.generate_expression(arg, instrs)?;
                }

                // Call function
                if let ExprKind::Identifier(name) = &function.kind {
                    let func_idx = self.resolve_function(name)?;
                    instrs.push(WasmInstruction::Call(func_idx));
                }
            }

            ExprKind::If { condition, then_block, else_expr } => {
                self.generate_expression(condition, instrs)?;

                let mut then_instrs = vec![];
                let mut else_instrs = vec![];

                for (pat, body) in then_block {
                    for stmt in body {
                        self.generate_statement(stmt)?;
                    }
                }

                if let Some((pat, else_body)) = else_expr {
                    for stmt in else_body {
                        self.generate_statement(stmt)?;
                    }
                }

                instrs.push(WasmInstruction::If(WasmType::Empty, then_instrs, else_instrs));
            }

            ExprKind::Array(elements) => {
                // Allocate array in linear memory
                let size = elements.len() as i32;
                instrs.push(WasmInstruction::I32Const(size));
                // Call allocation function (would be imported or defined)
                instrs.push(WasmInstruction::Call(self.resolve_function("alloc")?));

                // Store elements
                for (i, elem) in elements.iter().enumerate() {
                    instrs.push(WasmInstruction::I32Const(i as i32 * 4));
                    self.generate_expression(elem, instrs)?;
                    instrs.push(WasmInstruction::I32Store(0, 0));
                }
            }

            ExprKind::Block(statements) => {
                for stmt in statements {
                    self.generate_statement(stmt)?;
                }
            }

            ExprKind::Parenthesis(inner) => {
                self.generate_expression(inner, instrs)?;
            }

            _ => {
                // Default: push zero
                instrs.push(WasmInstruction::I32Const(0));
            }
        }

        Ok(())
    }

    /// Generate literal
    fn generate_literal(&self, lit: &Literal, instrs: &mut Vec<WasmInstruction>) {
        match lit {
            Literal::Int(n) => instrs.push(WasmInstruction::I32Const(*n as i32)),
            Literal::Float(f) => instrs.push(WasmInstruction::F64Const(*f)),
            Literal::Bool(b) => instrs.push(WasmInstruction::I32Const(if *b { 1 } else { 0 })),
            Literal::Char(c) => instrs.push(WasmInstruction::I32Const(*c as i32)),
            Literal::String(s) => {
                // For strings, we'd typically allocate and copy
                // Simplified: push address of string constant
                instrs.push(WasmInstruction::I32Const(0));
            }
            Literal::Unit => {
                // No-op for unit type
            }
            _ => {}
        }
    }

    /// Generate binary operation
    fn generate_binary_op(&self, op: &BinOp, instrs: &mut Vec<WasmInstruction>) {
        match op {
            // Integer arithmetic
            BinOp::Add => instrs.push(WasmInstruction::I32Add),
            BinOp::Sub => instrs.push(WasmInstruction::I32Sub),
            BinOp::Mul => instrs.push(WasmInstruction::I32Mul),
            BinOp::Div => instrs.push(WasmInstruction::I32DivS),
            BinOp::Mod => instrs.push(WasmInstruction::I32RemS),

            // Integer comparisons
            BinOp::Eq => instrs.push(WasmInstruction::I32Eq),
            BinOp::Ne => instrs.push(WasmInstruction::I32Ne),
            BinOp::Lt => instrs.push(WasmInstruction::I32LtS),
            BinOp::Le => instrs.push(WasmInstruction::I32LeS),
            BinOp::Gt => instrs.push(WasmInstruction::I32GtS),
            BinOp::Ge => instrs.push(WasmInstruction::I32GeS),

            // Boolean operations
            BinOp::And => instrs.push(WasmInstruction::I32And),
            BinOp::Or => instrs.push(WasmInstruction::I32Or),

            // Shift operations
            BinOp::Shl => instrs.push(WasmInstruction::I32Shl),
            BinOp::Shr => instrs.push(WasmInstruction::I32ShrS),

            _ => {}
        }
    }

    /// Generate unary operation
    fn generate_unary_op(&self, op: &UnOp, instrs: &mut Vec<WasmInstruction>) {
        match op {
            UnOp::Neg => instrs.push(WasmInstruction::I32Const(0)),
            UnOp::Not => instrs.push(WasmInstruction::I32Eqz),
            UnOp::BitNot => instrs.push(WasmInstruction::I32Const(0)), // Simplified
            UnOp::Deref => {} // Handled by pointer type
        }
    }

    /// Resolve function to index
    fn resolve_function(&self, name: &str) -> Result<u32, String> {
        // Find function by name in module
        for (idx, func) in self.module.functions.iter().enumerate() {
            if func.name == name {
                return Ok(idx as u32);
            }
        }
        Err(format!("Undefined function: {}", name))
    }

    /// Emit WAT (WebAssembly Text) format
    pub fn emit_wat(&self) -> String {
        let mut output = String::new();

        // Module header
        output.push_str("(module\n");

        // Type definitions
        for (idx, ty) in self.module.types.iter().enumerate() {
            output.push_str(&format!("  (type (${}) (func", idx));
            if !ty.params.is_empty() {
                output.push_str(&ty.params.iter()
                    .map(|t| format!(" (param {})", t.name()))
                    .collect::<String>());
            }
            if !ty.results.is_empty() {
                output.push_str(&ty.results.iter()
                    .map(|t| format!(" (result {})", t.name()))
                    .collect::<String>());
            }
            output.push_str("))\n");
        }

        // Functions
        for func in &self.module.functions {
            output.push_str(&format!("  (func (${})", func.name));

            // Type reference
            output.push_str(&format!(" (type {})\n", func.type_idx));

            // Locals
            for (i, local) in func.locals.iter().enumerate() {
                if i >= func.type_idx as usize {
                    output.push_str(&format!("    (local ${} {})\n", i, local.name()));
                }
            }

            // Body
            output.push_str("    ");
            for instr in &func.body {
                output.push_str(&self.format_instruction(instr, 0));
            }
            output.push_str("\n  )\n");

            // Export
            if func.export {
                output.push_str(&format!("  (export \"{}\" (func ${}))\n", func.name, func.name));
            }
        }

        // Memory
        if let Some(ref mem) = self.module.memory {
            output.push_str(&format!("  (memory (export \"memory\") {}",
                if let Some(max) = mem.max_pages {
                    format!("{} {}", mem.min_pages, max)
                } else {
                    mem.min_pages.to_string()
                }
            ));
            if !mem.data.is_empty() {
                // Data sections would go here
            }
            output.push_str(")\n");
        }

        // Close module
        output.push_str(")\n");

        output
    }

    /// Format instruction for WAT
    fn format_instruction(&self, instr: &WasmInstruction, indent: usize) -> String {
        let prefix = "  ".repeat(indent);

        match instr {
            WasmInstruction::Unreachable => "unreachable".to_string(),
            WasmInstruction::Nop => "nop".to_string(),
            WasmInstruction::Return => "return".to_string(),
            WasmInstruction::Drop => "drop".to_string(),
            WasmInstruction::Select => "select".to_string(),
            WasmInstruction::LocalGet(idx) => format!("local.get ${}", idx),
            WasmInstruction::LocalSet(idx) => format!("local.set ${}", idx),
            WasmInstruction::LocalTee(idx) => format!("local.tee ${}", idx),
            WasmInstruction::GlobalGet(idx) => format!("global.get ${}", idx),
            WasmInstruction::GlobalSet(idx) => format!("global.set ${}", idx),
            WasmInstruction::I32Const(val) => format!("i32.const {}", val),
            WasmInstruction::I64Const(val) => format!("i64.const {}", val),
            WasmInstruction::F32Const(val) => format!("f32.const {}", val),
            WasmInstruction::F64Const(val) => format!("f64.const {}", val),
            WasmInstruction::I32Add => "i32.add".to_string(),
            WasmInstruction::I32Sub => "i32.sub".to_string(),
            WasmInstruction::I32Mul => "i32.mul".to_string(),
            WasmInstruction::I32DivS => "i32.div_s".to_string(),
            WasmInstruction::I32RemS => "i32.rem_s".to_string(),
            WasmInstruction::I32Eq => "i32.eq".to_string(),
            WasmInstruction::I32Ne => "i32.ne".to_string(),
            WasmInstruction::I32LtS => "i32.lt_s".to_string(),
            WasmInstruction::I32LeS => "i32.le_s".to_string(),
            WasmInstruction::I32GtS => "i32.gt_s".to_string(),
            WasmInstruction::I32GeS => "i32.ge_s".to_string(),
            WasmInstruction::I32And => "i32.and".to_string(),
            WasmInstruction::I32Or => "i32.or".to_string(),
            WasmInstruction::I32Xor => "i32.xor".to_string(),
            WasmInstruction::I32Shl => "i32.shl".to_string(),
            WasmInstruction::I32ShrS => "i32.shr_s".to_string(),
            WasmInstruction::I32Eqz => "i32.eqz".to_string(),
            WasmInstruction::I64Eqz => "i64.eqz".to_string(),
            WasmInstruction::F64Add => "f64.add".to_string(),
            WasmInstruction::F64Sub => "f64.sub".to_string(),
            WasmInstruction::F64Mul => "f64.mul".to_string(),
            WasmInstruction::F64Div => "f64.div".to_string(),
            WasmInstruction::F64Neg => "f64.neg".to_string(),
            WasmInstruction::F64Sqrt => "f64.sqrt".to_string(),
            WasmInstruction::F64Eq => "f64.eq".to_string(),
            WasmInstruction::F64Ne => "f64.ne".to_string(),
            WasmInstruction::F64Lt => "f64.lt".to_string(),
            WasmInstruction::F64Le => "f64.le".to_string(),
            WasmInstruction::F64Gt => "f64.gt".to_string(),
            WasmInstruction::F64Ge => "f64.ge".to_string(),
            WasmInstruction::I32Load(align, offset) => format!("i32.load {} {}", align, offset),
            WasmInstruction::I32Store(align, offset) => format!("i32.store {} {}", align, offset),
            WasmInstruction::I32WrapI64 => "i32.wrap_i64".to_string(),
            WasmInstruction::I64ExtendI32S => "i64.extend_i32_s".to_string(),
            WasmInstruction::F64PromoteF32 => "f64.promote_f32".to_string(),
            WasmInstruction::F32DemoteF64 => "f32.demote_f64".to_string(),
            WasmInstruction::I32TruncF64S => "i32.trunc_f64_s".to_string(),
            WasmInstruction::F64ConvertI32S => "f64.convert_i32_s".to_string(),
            WasmInstruction::MemorySize => "memory.size".to_string(),
            WasmInstruction::MemoryGrow => "memory.grow".to_string(),
            WasmInstruction::Block(result_type, instrs) => {
                let inner = instrs.iter()
                    .map(|i| format!("\n{}{}", prefix, self.format_instruction(i, indent + 1)))
                    .collect::<String>();
                format!("(block {}{})", result_type.name(), inner)
            }
            WasmInstruction::Loop(result_type, instrs) => {
                let inner = instrs.iter()
                    .map(|i| format!("\n{}{}", prefix, self.format_instruction(i, indent + 1)))
                    .collect::<String>();
                format!("(loop {}{})", result_type.name(), inner)
            }
            WasmInstruction::If(result_type, then_instrs, else_instrs) => {
                let then_inner = then_instrs.iter()
                    .map(|i| format!("\n{}{}", prefix, self.format_instruction(i, indent + 1)))
                    .collect::<String>();
                let else_inner = else_instrs.iter()
                    .map(|i| format!("\n{}{}", prefix, self.format_instruction(i, indent + 1)))
                    .collect::<String>();
                format!("(if {}{}{}{}{})",
                    result_type.name(),
                    then_inner,
                    if else_instrs.is_empty() { "" } else { "(then" },
                    if else_instrs.is_empty() { "" } else { else_inner.as_str() },
                    if else_instrs.is_empty() { "" } else { ")" }
                )
            }
            WasmInstruction::Br(label) => format!("br {}", label),
            WasmInstruction::BrIf(label) => format!("br_if {}", label),
            WasmInstruction::Call(idx) => format!("call {}", idx),
            WasmInstruction::CallIndirect(type_idx, table_idx) => {
                format!("call_indirect (type {})", type_idx)
            }
            WasmInstruction::End => "end".to_string(),
            _ => format!("; {:?}", instr),
        }
    }

    /// Emit binary WASM format
    pub fn emit_binary(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // WASM magic number
        bytes.extend_from_slice(&[0x00, 0x61, 0x73, 0x6d]); // "\0asm"
        // Version
        bytes.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);

        // Type section
        bytes.push(0x01); // Section 1: Type
        let type_bytes = self.emit_type_section();
        bytes.extend_from_slice(&type_bytes.to_le_bytes()[..4]); // Size
        bytes.extend_from_slice(&type_bytes);

        // Function section
        bytes.push(0x03); // Section 3: Function
        let func_bytes = self.emit_function_section();
        bytes.extend_from_slice(&func_bytes.to_le_bytes()[..4]);
        bytes.extend_from_slice(&func_bytes);

        // Export section
        if !self.module.exports.is_empty() {
            bytes.push(0x07); // Section 7: Export
            let export_bytes = self.emit_export_section();
            bytes.extend_from_slice(&export_bytes.to_le_bytes()[..4]);
            bytes.extend_from_slice(&export_bytes);
        }

        // Code section
        bytes.push(0x0a); // Section 10: Code
        let code_bytes = self.emit_code_section();
        bytes.extend_from_slice(&code_bytes.to_le_bytes()[..4]);
        bytes.extend_from_slice(&code_bytes);

        bytes
    }

    fn emit_type_section(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.module.types.len() as u8);

        for ty in &self.module.types {
            bytes.push(0x60); // func type
            bytes.push(ty.params.len() as u8);
            for param in &ty.params {
                bytes.push(self.wasm_type_to_byte(param));
            }
            bytes.push(ty.results.len() as u8);
            for result in &ty.results {
                bytes.push(self.wasm_type_to_byte(result));
            }
        }

        bytes
    }

    fn emit_function_section(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.module.functions.len() as u8);

        for func in &self.module.functions {
            bytes.push(func.type_idx as u8);
        }

        bytes
    }

    fn emit_export_section(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.module.exports.len() as u8);

        for (name, kind, idx) in &self.module.exports {
            bytes.push(name.len() as u8);
            bytes.extend_from_slice(name.as_bytes());
            bytes.push(match kind {
                WasmExportKind::Function => 0x00,
                WasmExportKind::Table => 0x01,
                WasmExportKind::Memory => 0x02,
                WasmExportKind::Global => 0x03,
            });
            bytes.push(*idx as u8);
        }

        bytes
    }

    fn emit_code_section(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.module.functions.len() as u8);

        for func in &self.module.functions {
            // Local declarations
            let mut local_count = 0;
            let mut local_bytes = vec![];

            for (i, local) in func.locals.iter().enumerate() {
                if i >= func.type_idx as usize {
                    local_count += 1;
                    local_bytes.push(1); // Count
                    local_bytes.push(self.wasm_type_to_byte(local));
                }
            }

            // Function body
            let mut body_bytes = vec![];
            body_bytes.push(local_count);
            body_bytes.extend_from_slice(&local_bytes);

            // Instructions
            for instr in &func.body {
                self.emit_instruction(instr, &mut body_bytes);
            }

            bytes.push(body_bytes.len() as u8);
            bytes.extend_from_slice(&body_bytes);
        }

        bytes
    }

    fn emit_instruction(&self, instr: &WasmInstruction, bytes: &mut Vec<u8>) {
        match instr {
            WasmInstruction::Unreachable => bytes.push(0x00),
            WasmInstruction::Nop => bytes.push(0x01),
            WasmInstruction::Block(_, _) => bytes.push(0x02),
            WasmInstruction::Loop(_, _) => bytes.push(0x03),
            WasmInstruction::If(_, _, _) => bytes.push(0x04),
            WasmInstruction::End => bytes.push(0x0b),
            WasmInstruction::Br(idx) => {
                bytes.push(0x0c);
                bytes.push(*idx as u8);
            }
            WasmInstruction::BrIf(idx) => {
                bytes.push(0x0d);
                bytes.push(*idx as u8);
            }
            WasmInstruction::Return => bytes.push(0x0f),
            WasmInstruction::Call(idx) => {
                bytes.push(0x10);
                bytes.push(*idx as u8);
            }
            WasmInstruction::Drop => bytes.push(0x1a),
            WasmInstruction::Select => bytes.push(0x1b),
            WasmInstruction::LocalGet(idx) => {
                bytes.push(0x20);
                bytes.push(*idx as u8);
            }
            WasmInstruction::LocalSet(idx) => {
                bytes.push(0x21);
                bytes.push(*idx as u8);
            }
            WasmInstruction::LocalTee(idx) => {
                bytes.push(0x22);
                bytes.push(*idx as u8);
            }
            WasmInstruction::GlobalGet(idx) => {
                bytes.push(0x23);
                bytes.push(*idx as u8);
            }
            WasmInstruction::GlobalSet(idx) => {
                bytes.push(0x24);
                bytes.push(*idx as u8);
            }
            WasmInstruction::I32Load(_, _) => bytes.push(0x28),
            WasmInstruction::I64Load(_, _) => bytes.push(0x29),
            WasmInstruction::F32Load(_, _) => bytes.push(0x2a),
            WasmInstruction::F64Load(_, _) => bytes.push(0x2b),
            WasmInstruction::I32Store(_, _) => bytes.push(0x36),
            WasmInstruction::I64Store(_, _) => bytes.push(0x37),
            WasmInstruction::F32Store(_, _) => bytes.push(0x38),
            WasmInstruction::F64Store(_, _) => bytes.push(0x39),
            WasmInstruction::MemorySize => bytes.push(0x3f),
            WasmInstruction::MemoryGrow => bytes.push(0x40),
            WasmInstruction::I32Const(val) => {
                bytes.push(0x41);
                bytes.extend_from_slice(&val.to_le_bytes());
            }
            WasmInstruction::I64Const(val) => {
                bytes.push(0x42);
                bytes.extend_from_slice(&val.to_le_bytes());
            }
            WasmInstruction::F32Const(val) => {
                bytes.push(0x43);
                bytes.extend_from_slice(&val.to_le_bytes());
            }
            WasmInstruction::F64Const(val) => {
                bytes.push(0x44);
                bytes.extend_from_slice(&val.to_le_bytes());
            }
            WasmInstruction::I32Eqz => bytes.push(0x45),
            WasmInstruction::I32Eq => bytes.push(0x46),
            WasmInstruction::I32Ne => bytes.push(0x47),
            WasmInstruction::I32LtS => bytes.push(0x48),
            WasmInstruction::I32LtU => bytes.push(0x49),
            WasmInstruction::I32GtS => bytes.push(0x4a),
            WasmInstruction::I32GtU => bytes.push(0x4b),
            WasmInstruction::I32LeS => bytes.push(0x4c),
            WasmInstruction::I32LeU => bytes.push(0x4d),
            WasmInstruction::I32GeS => bytes.push(0x4e),
            WasmInstruction::I32GeU => bytes.push(0x4f),
            WasmInstruction::I32Add => bytes.push(0x6a),
            WasmInstruction::I32Sub => bytes.push(0x6b),
            WasmInstruction::I32Mul => bytes.push(0x6c),
            WasmInstruction::I32DivS => bytes.push(0x6d),
            WasmInstruction::I32RemS => bytes.push(0x6f),
            WasmInstruction::I32And => bytes.push(0x71),
            WasmInstruction::I32Or => bytes.push(0x72),
            WasmInstruction::I32Xor => bytes.push(0x73),
            WasmInstruction::I32Shl => bytes.push(0x74),
            WasmInstruction::I32ShrS => bytes.push(0x75),
            WasmInstruction::I64Eqz => bytes.push(0x50),
            WasmInstruction::I64Eq => bytes.push(0x51),
            WasmInstruction::I64Ne => bytes.push(0x52),
            WasmInstruction::I64LtS => bytes.push(0x53),
            WasmInstruction::I64LtU => bytes.push(0x54),
            WasmInstruction::I64GtS => bytes.push(0x55),
            WasmInstruction::I64GtU => bytes.push(0x56),
            WasmInstruction::I64LeS => bytes.push(0x57),
            WasmInstruction::I64LeU => bytes.push(0x58),
            WasmInstruction::I64GeS => bytes.push(0x59),
            WasmInstruction::I64GeU => bytes.push(0x5a),
            WasmInstruction::I64Add => bytes.push(0x7c),
            WasmInstruction::I64Sub => bytes.push(0x7d),
            WasmInstruction::I64Mul => bytes.push(0x7e),
            WasmInstruction::I64DivS => bytes.push(0x7f),
            WasmInstruction::I64RemS => bytes.push(0x81),
            WasmInstruction::I64And => bytes.push(0x83),
            WasmInstruction::I64Or => bytes.push(0x84),
            WasmInstruction::I64Xor => bytes.push(0x85),
            WasmInstruction::I64Shl => bytes.push(0x86),
            WasmInstruction::I64ShrS => bytes.push(0x87),
            WasmInstruction::F32Eq => bytes.push(0x5b),
            WasmInstruction::F32Ne => bytes.push(0x5c),
            WasmInstruction::F32Lt => bytes.push(0x5d),
            WasmInstruction::F32Gt => bytes.push(0x5e),
            WasmInstruction::F32Le => bytes.push(0x5f),
            WasmInstruction::F32Ge => bytes.push(0x60),
            WasmInstruction::F32Add => bytes.push(0x92),
            WasmInstruction::F32Sub => bytes.push(0x93),
            WasmInstruction::F32Mul => bytes.push(0x94),
            WasmInstruction::F32Div => bytes.push(0x95),
            WasmInstruction::F32Neg => bytes.push(0x90),
            WasmInstruction::F32Abs => bytes.push(0x91),
            WasmInstruction::F32Sqrt => bytes.push(0xa1),
            WasmInstruction::F64Eq => bytes.push(0x61),
            WasmInstruction::F64Ne => bytes.push(0x62),
            WasmInstruction::F64Lt => bytes.push(0x63),
            WasmInstruction::F64Gt => bytes.push(0x64),
            WasmInstruction::F64Le => bytes.push(0x65),
            WasmInstruction::F64Ge => bytes.push(0x66),
            WasmInstruction::F64Add => bytes.push(0xa0),
            WasmInstruction::F64Sub => bytes.push(0xa1),
            WasmInstruction::F64Mul => bytes.push(0xa2),
            WasmInstruction::F64Div => bytes.push(0xa3),
            WasmInstruction::F64Neg => bytes.push(0x9e),
            WasmInstruction::F64Abs => bytes.push(0x9f),
            WasmInstruction::F64Sqrt => bytes.push(0xa9),
            WasmInstruction::I32WrapI64 => bytes.push(0xa7),
            WasmInstruction::I64ExtendI32S => bytes.push(0xac),
            WasmInstruction::F64PromoteF32 => bytes.push(0xbb),
            WasmInstruction::F32DemoteF64 => bytes.push(0xb6),
            WasmInstruction::I32TruncF64S => bytes.push(0xa8),
            WasmInstruction::F64ConvertI32S => bytes.push(0xa7),
            _ => {}
        }
    }

    fn wasm_type_to_byte(&self, ty: &WasmType) -> u8 {
        match ty {
            WasmType::I32 => 0x7f,
            WasmType::I64 => 0x7e,
            WasmType::F32 => 0x7d,
            WasmType::F64 => 0x7c,
            WasmType::V128 => 0x7b,
            WasmType::Funcref => 0x70,
            WasmType::Externref => 0x6f,
            WasmType::Empty => 0x40,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_type_conversion() {
        assert_eq!(WasmType::from_tumc(&Type::Int), Some(WasmType::I32));
        assert_eq!(WasmType::from_tumc(&Type::Float), Some(WasmType::F64));
        assert_eq!(WasmType::from_tumc(&Type::Unit), Some(WasmType::Empty));
    }

    #[test]
    fn test_codegen_simple_function() {
        let mut codegen = CodeGen::new(OptimizationLevel::O2);

        let program = Program {
            items: vec![Item::Function(Function {
                name: "add".to_string(),
                params: vec![
                    FunctionParam { name: "a".to_string(), ann_type: AnnType::Int },
                    FunctionParam { name: "b".to_string(), ann_type: AnnType::Int },
                ],
                return_type: AnnType::Int,
                body: vec![Stmt::new_return(Some(Expr::new_binary(
                    BinOp::Add,
                    Expr::new_identifier("a"),
                    Expr::new_identifier("b"),
                    Span::default(),
                )))],
                span: Span::default(),
            })],
            span: Span::default(),
        };

        codegen.generate(&program).unwrap();

        let wat = codegen.emit_wat();
        assert!(wat.contains("add"));
        assert!(wat.contains("i32.add"));
    }
}
