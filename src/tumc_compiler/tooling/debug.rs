// Debugger Implementation with DWARF support

use crate::tumc_compiler::ir::{Module, Function, BasicBlock, Instruction};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DWARF debugging information
pub mod dwarf {
    use super::*;

    #[derive(Debug, Clone)]
    pub struct DwarfInfo {
        pub compilation_units: Vec<CompilationUnit>,
        pub line_program: LineProgram,
    }

    #[derive(Debug, Clone)]
    pub struct CompilationUnit {
        pub offset: u64,
        pub length: u16,
        pub version: u16,
        pub abbrev_offset: u64,
        pub address_size: u8,
        pub die_tree: Vec<DebugInfoEntry>,
    }

    #[derive(Debug, Clone)]
    pub struct DebugInfoEntry {
        pub tag: u64,
        pub has_children: bool,
        pub attributes: Vec<Attribute>,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Attribute {
        pub name: u64,
        pub form: u64,
        pub value: AttributeValue,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum AttributeValue {
        Unsigned(u64),
        Signed(i64),
        String(String),
        Address(u64),
        Reference(u64),
    }

    #[derive(Debug, Clone)]
    pub struct LineProgram {
        pub header: LineNumberHeader,
        pub statements: Vec<LineStatement>,
    }

    #[derive(Debug, Clone)]
    pub struct LineNumberHeader {
        pub total_length: u32,
        pub version: u16,
        pub header_length: u32,
        pub minimum_instruction_length: u8,
        pub maximum_operations_per_instruction: u8,
        pub default_is_stmt: bool,
        pub line_base: i8,
        pub line_range: u8,
        pub opcode_base: u8,
    }

    #[derive(Debug, Clone)]
    pub struct LineStatement {
        pub address: u64,
        pub file: u32,
        pub line: u32,
        pub column: u32,
        pub is_stmt: bool,
        pub basic_block: bool,
        pub end_sequence: bool,
    }
}

/// Debugger state
#[derive(Debug, Clone)]
pub struct DebuggerState {
    pub running: bool,
    pub stopped: bool,
    pub current_function: Option<String>,
    pub current_block: Option<String>,
    pub current_instruction: usize,
    pub call_stack: Vec<StackFrame>,
    pub registers: HashMap<String, u64>,
    pub memory: HashMap<u64, u8>,
}

impl Default for DebuggerState {
    fn default() -> Self {
        let mut registers = HashMap::new();
        registers.insert("rax".to_string(), 0);
        registers.insert("rbx".to_string(), 0);
        registers.insert("rcx".to_string(), 0);
        registers.insert("rdx".to_string(), 0);
        registers.insert("rsp".to_string(), 0x7fff0000);
        registers.insert("rbp".to_string(), 0x7fff0000);
        registers.insert("rsi".to_string(), 0);
        registers.insert("rdi".to_string(), 0);
        registers.insert("r8".to_string(), 0);
        registers.insert("r9".to_string(), 0);
        registers.insert("r10".to_string(), 0);
        registers.insert("r11".to_string(), 0);
        registers.insert("r12".to_string(), 0);
        registers.insert("r13".to_string(), 0);
        registers.insert("r14".to_string(), 0);
        registers.insert("r15".to_string(), 0);

        Self {
            running: false,
            stopped: true,
            current_function: None,
            current_block: None,
            current_instruction: 0,
            call_stack: Vec::new(),
            registers,
            memory: HashMap::new(),
        }
    }
}

/// Stack frame
#[derive(Debug, Clone)]
pub struct StackFrame {
    pub function: String,
    pub return_address: u64,
    pub locals: HashMap<String, u64>,
    pub parameters: Vec<u64>,
}

/// Breakpoint
#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: u64,
    pub location: BreakpointLocation,
    pub enabled: bool,
    pub condition: Option<String>,
    pub hit_count: u64,
    pub ignore_count: u64,
}

#[derive(Debug, Clone)]
pub enum BreakpointLocation {
    Address(u64),
    Function(String),
    Source { file: String, line: u32 },
}

impl Breakpoint {
    pub fn new_function(name: &str) -> Self {
        Self {
            id: 0,
            location: BreakpointLocation::Function(name.to_string()),
            enabled: true,
            condition: None,
            hit_count: 0,
            ignore_count: 0,
        }
    }

    pub fn new_source(file: &str, line: u32) -> Self {
        Self {
            id: 0,
            location: BreakpointLocation::Source {
                file: file.to_string(),
                line,
            },
            enabled: true,
            condition: None,
            hit_count: 0,
            ignore_count: 0,
        }
    }

    pub fn new_address(addr: u64) -> Self {
        Self {
            id: 0,
            location: BreakpointLocation::Address(addr),
            enabled: true,
            condition: None,
            hit_count: 0,
            ignore_count: 0,
        }
    }
}

/// Watchpoint
#[derive(Debug, Clone)]
pub struct Watchpoint {
    pub id: u64,
    pub expression: String,
    pub watch_type: WatchType,
    pub enabled: bool,
    pub hit_count: u64,
}

#[derive(Debug, Clone)]
pub enum WatchType {
    Read,
    Write,
    ReadWrite,
}

/// Debugger
pub struct Debugger {
    port: u16,
    breakpoints: HashMap<u64, Breakpoint>,
    watchpoints: Vec<Watchpoint>,
    state: DebuggerState,
    dwarf_info: Option<dwarf::DwarfInfo>,
    source_map: HashMap<String, SourceFile>,
}

#[derive(Debug, Clone)]
pub struct SourceFile {
    pub path: String,
    pub lines: Vec<String>,
    pub compiled_at: Option<u64>,
}

/// Debug command
#[derive(Debug, Clone)]
pub enum DebugCommand {
    Continue,
    Step,
    Next,
    StepInstruction,
    Finish,
    Breakpoint(Breakpoint),
    DeleteBreakpoint(u64),
    Watchpoint(Watchpoint),
    Print(String),
    Backtrace,
    InfoRegisters,
    InfoBreakpoints,
    InfoThreads,
    SwitchThread(u64),
    SetVariable(String, String),
    Condition(u64, Option<String>),
}

/// Debug event
#[derive(Debug, Clone)]
pub enum DebugEvent {
    BreakpointHit(Breakpoint),
    WatchpointTriggered(Watchpoint, u64),
    FunctionEntry(String),
    FunctionExit(String),
    SingleStep(Instruction),
    Exception(String),
    ThreadCreated(u64),
    ThreadExited(u64),
}

impl Debugger {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            breakpoints: HashMap::new(),
            watchpoints: Vec::new(),
            state: DebuggerState::default(),
            dwarf_info: None,
            source_map: HashMap::new(),
        }
    }

    /// Set breakpoint at function
    pub fn set_breakpoint(&mut self, name: &str) -> u64 {
        let id = self.breakpoints.len() as u64 + 1;
        let mut bp = Breakpoint::new_function(name);
        bp.id = id;
        self.breakpoints.insert(id, bp);
        id
    }

    /// Set breakpoint at source location
    pub fn set_source_breakpoint(&mut self, file: &str, line: u32) -> u64 {
        let id = self.breakpoints.len() as u64 + 1;
        let mut bp = Breakpoint::new_source(file, line);
        bp.id = id;
        self.breakpoints.insert(id, bp);
        id
    }

    /// Set breakpoint at address
    pub fn set_address_breakpoint(&mut self, addr: u64) -> u64 {
        let id = self.breakpoints.len() as u64 + 1;
        let mut bp = Breakpoint::new_address(addr);
        bp.id = id;
        self.breakpoints.insert(id, bp);
        id
    }

    /// Set watchpoint
    pub fn set_watchpoint(&mut self, expr: &str, watch_type: WatchType) -> u64 {
        let id = self.watchpoints.len() as u64 + 1;
        self.watchpoints.push(Watchpoint {
            id,
            expression: expr.to_string(),
            watch_type,
            enabled: true,
            hit_count: 0,
        });
        id
    }

    /// Delete breakpoint
    pub fn delete_breakpoint(&mut self, id: u64) -> bool {
        self.breakpoints.remove(&id).is_some()
    }

    /// Enable/disable breakpoint
    pub fn toggle_breakpoint(&mut self, id: u64, enabled: bool) -> bool {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Set condition on breakpoint
    pub fn set_breakpoint_condition(&mut self, id: u64, condition: Option<String>) -> bool {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.condition = condition;
            true
        } else {
            false
        }
    }

    /// Execute debug command
    pub fn execute(&mut self, command: DebugCommand) -> Result<DebugEvent, DebugError> {
        match command {
            DebugCommand::Continue => {
                self.state.stopped = false;
                Ok(DebugEvent::BreakpointHit(self.breakpoints.values().next().cloned().unwrap()))
            }
            DebugCommand::Step => {
                self.step();
                Ok(DebugEvent::SingleStep(Instruction::default()))
            }
            DebugCommand::Next => {
                self.next();
                Ok(DebugEvent::SingleStep(Instruction::default()))
            }
            DebugCommand::StepInstruction => {
                self.step_instruction();
                Ok(DebugEvent::SingleStep(Instruction::default()))
            }
            DebugCommand::Finish => {
                self.finish();
                Ok(DebugEvent::FunctionExit(self.state.current_function.clone().unwrap_or_default()))
            }
            DebugCommand::Breakpoint(bp) => {
                let id = self.breakpoints.len() as u64 + 1;
                let mut bp = bp;
                bp.id = id;
                self.breakpoints.insert(id, bp);
                Ok(DebugEvent::BreakpointHit(self.breakpoints.get(&id).cloned().unwrap()))
            }
            DebugCommand::DeleteBreakpoint(id) => {
                self.breakpoints.remove(&id);
                Ok(DebugEvent::SingleStep(Instruction::default()))
            }
            DebugCommand::Watchpoint(wp) => {
                self.watchpoints.push(wp);
                Ok(DebugEvent::WatchpointTriggered(
                    self.watchpoints.last().cloned().unwrap(),
                    0,
                ))
            }
            DebugCommand::Print(expr) => {
                let value = self.evaluate_expression(&expr);
                println!("{} = {}", expr, value);
                Ok(DebugEvent::SingleStep(Instruction::default()))
            }
            DebugCommand::Backtrace => {
                self.print_backtrace();
                Ok(DebugEvent::SingleStep(Instruction::default()))
            }
            DebugCommand::InfoRegisters => {
                self.print_registers();
                Ok(DebugEvent::SingleStep(Instruction::default()))
            }
            DebugCommand::InfoBreakpoints => {
                self.print_breakpoints();
                Ok(DebugEvent::SingleStep(Instruction::default()))
            }
            DebugCommand::InfoThreads => {
                self.print_threads();
                Ok(DebugEvent::SingleStep(Instruction::default()))
            }
            _ => Ok(DebugEvent::SingleStep(Instruction::default())),
        }
    }

    /// Step to next instruction
    fn step(&mut self) {
        self.state.current_instruction += 1;
    }

    /// Step to next line
    fn next(&mut self) {
        self.state.current_instruction += 1;
    }

    /// Step single instruction
    fn step_instruction(&mut self) {
        self.state.current_instruction += 1;
    }

    /// Finish current function
    fn finish(&mut self) {
        if let Some(frame) = self.state.call_stack.pop() {
            self.state.current_function = Some(frame.function);
        }
    }

    /// Evaluate expression
    fn evaluate_expression(&self, expr: &str) -> i64 {
        // Simplified expression evaluation
        if let Some(value) = self.state.registers.get(expr) {
            return *value as i64;
        }
        0
    }

    /// Print backtrace
    fn print_backtrace(&self) {
        for (i, frame) in self.state.call_stack.iter().enumerate() {
            println!("#{} {} at 0x{:016x}", i, frame.function, frame.return_address);
        }
    }

    /// Print registers
    fn print_registers(&self) {
        for (name, value) in &self.state.registers {
            println!("{} = 0x{:016x} ({})", name, value, value);
        }
    }

    /// Print breakpoints
    fn print_breakpoints(&self) {
        for bp in self.breakpoints.values() {
            println!(
                "{}: {} [{}] hit={} ignore={}",
                bp.id,
                match &bp.location {
                    BreakpointLocation::Address(addr) => format!("0x{:x}", addr),
                    BreakpointLocation::Function(name) => name.clone(),
                    BreakpointLocation::Source { file, line } =>
                        format!("{}:{}", file, line),
                },
                if bp.enabled { "enabled" } else { "disabled" },
                bp.hit_count,
                bp.ignore_count
            );
        }
    }

    /// Print threads
    fn print_threads(&self) {
        println!("Current thread: 1");
        println!("  => 1 Thread 0x{:x} (running)", 1);
    }

    /// Check breakpoint hit
    pub fn check_breakpoint(&self, location: &BreakpointLocation) -> Option<&Breakpoint> {
        for bp in self.breakpoints.values() {
            if !bp.enabled {
                continue;
            }
            if bp.ignore_count > 0 {
                continue;
            }
            match (&bp.location, location) {
                (BreakpointLocation::Function(name), BreakpointLocation::Function(n)) if name == n => {
                    return Some(bp);
                }
                (BreakpointLocation::Address(addr), BreakpointLocation::Address(a)) if addr == a => {
                    return Some(bp);
                }
                _ => {}
            }
        }
        None
    }

    /// Generate DWARF info for module
    pub fn generate_dwarf_info(&mut self, module: &Module) {
        let mut units = Vec::new();

        for func in &module.functions {
            let mut die_tree = Vec::new();

            // DW_TAG_compile_unit
            die_tree.push(dwarf::DebugInfoEntry {
                tag: 0x11, // DW_TAG_compile_unit
                has_children: true,
                attributes: vec![
                    dwarf::Attribute {
                        name: 0x03, // DW_AT_name
                        form: 0x0e, // DW_FORM_string
                        value: dwarf::AttributeValue::String("source.tum".to_string()),
                    },
                ],
            });

            // DW_TAG_subprogram for function
            die_tree.push(dwarf::DebugInfoEntry {
                tag: 0x2e, // DW_TAG_subprogram
                has_children: false,
                attributes: vec![
                    dwarf::Attribute {
                        name: 0x47, // DW_AT_name
                        form: 0x0e, // DW_FORM_string
                        value: dwarf::AttributeValue::String(func.name.clone()),
                    },
                ],
            });

            units.push(dwarf::CompilationUnit {
                offset: 0,
                length: 0,
                version: 5,
                abbrev_offset: 0,
                address_size: 8,
                die_tree,
            });
        }

        self.dwarf_info = Some(dwarf::DwarfInfo {
            compilation_units: units,
            line_program: dwarf::LineProgram {
                header: dwarf::LineNumberHeader {
                    total_length: 0,
                    version: 5,
                    header_length: 0,
                    minimum_instruction_length: 1,
                    maximum_operations_per_instruction: 1,
                    default_is_stmt: true,
                    line_base: 1,
                    line_range: 14,
                    opcode_base: 13,
                },
                statements: Vec::new(),
            },
        });
    }

    /// Get current source location
    pub fn get_source_location(&self) -> Option<(String, u32, u32)> {
        if let Some(func) = &self.state.current_function {
            Some((format!("{}.tum", func), 1, 0))
        } else {
            None
        }
    }

    /// Read memory
    pub fn read_memory(&self, addr: u64, len: usize) -> Vec<u8> {
        (0..len)
            .map(|i| *self.state.memory.get(&(addr + i as u64)).unwrap_or(&0))
            .collect()
    }

    /// Write memory
    pub fn write_memory(&mut self, addr: u64, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.state.memory.insert(addr + i as u64, byte);
        }
    }

    /// Get state
    pub fn get_state(&self) -> &DebuggerState {
        &self.state
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new(9508)
    }
}

/// Debug error
#[derive(Debug, Clone)]
pub enum DebugError {
    NotStopped,
    NoSuchBreakpoint(u64),
    InvalidAddress(u64),
    EvaluationError(String),
}

impl std::fmt::Display for DebugError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebugError::NotStopped => write!(f, "Debugger is not stopped"),
            DebugError::NoSuchBreakpoint(id) => write!(f, "No such breakpoint: {}", id),
            DebugError::InvalidAddress(addr) => write!(f, "Invalid address: 0x{:x}", addr),
            DebugError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
        }
    }
}

impl std::error::Error for DebugError {}
