//! Native code generation for x86_64 and AArch64
//! Generates machine code for native execution

use crate::prelude::*;
use super::ast::*;
use std::collections::HashMap;

/// Target architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    /// x86_64 (AMD64)
    X86_64,
    /// ARM64 (AArch64)
    AArch64,
    /// ARM32 (ARMv7)
    Arm,
    /// RISC-V 64-bit
    Riscv64,
}

/// Target operating system
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OS {
    Linux,
    MacOS,
    Windows,
}

/// Target environment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Env {
    Gnu,
    Musl,
    MsVC,
}

/// Complete target specification
#[derive(Debug, Clone)]
pub struct Target {
    pub arch: Arch,
    pub os: OS,
    pub env: Env,
}

impl Target {
    /// Linux x86_64 GNU
    pub fn x86_64_linux_gnu() -> Self {
        Target {
            arch: Arch::X86_64,
            os: OS::Linux,
            env: Env::Gnu,
        }
    }

    /// Linux AArch64 GNU
    pub fn aarch64_linux_gnu() -> Self {
        Target {
            arch: Arch::AArch64,
            os: OS::Linux,
            env: Env::Gnu,
        }
    }

    /// macOS x86_64
    pub fn x86_64_macos() -> Self {
        Target {
            arch: Arch::X86_64,
            os: OS::MacOS,
            env: Env::Gnu,
        }
    }

    /// Windows x86_64 MSVC
    pub fn x86_64_windows() -> Self {
        Target {
            arch: Arch::X86_64,
            os: OS::Windows,
            env: Env::MsVC,
        }
    }
}

/// Register representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    // x86_64 general purpose
    X86_64(R8),
    // AArch64 general purpose
    AArch64(A64),
    // Virtual register
    Virtual(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R8 {
    Rax, Rcx, Rdx, Rbx, Rsp, Rbp, Rsi, Rdi,
    R8, R9, R10, R11, R12, R13, R14, R15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum A64 {
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30, XZR,
    // Aliases
    Sp, Wzr,
}

/// Memory operand
#[derive(Debug, Clone)]
pub enum Operand {
    /// Register operand
    Reg(Register),
    /// Immediate value
    Imm(i64),
    /// Memory at address
    Mem {
        base: Register,
        offset: i32,
        scale: u8,
        index: Option<Register>,
    },
    /// Label reference
    Label(String),
}

/// x86_64 instruction
#[derive(Debug, Clone)]
pub enum X86Inst {
    // Data movement
    Mov { dest: R8, src: Operand },
    Movzx { dest: R8, src: Operand },
    Movsx { dest: R8, src: Operand },
    Lea { dest: R8, src: Operand },
    Push { src: Operand },
    Pop { dest: R8 },
    Cmove { dest: R8, src: Operand },
    Cmovne { dest: R8, src: Operand },

    // Arithmetic
    Add { dest: R8, src: Operand },
    Sub { dest: R8, src: Operand },
    Imul { dest: R8, src: Operand },
    Mul { src: Operand },
    Idiv { src: Operand },
    Inc { dest: R8 },
    Dec { dest: R8 },
    Neg { dest: R8 },
    Not { dest: R8 },

    // Logical
    And { dest: R8, src: Operand },
    Or { dest: R8, src: Operand },
    Xor { dest: R8, src: Operand },
    Not { dest: R8 },
    Test { a: Operand, b: Operand },

    // Shifts
    Shl { dest: R8, src: Operand },
    Shr { dest: R8, src: Operand },
    Sar { dest: R8, src: Operand },
    Rol { dest: R8, src: Operand },
    Ror { dest: R8, src: Operand },

    // Comparison
    Cmp { a: Operand, b: Operand },
    Test { a: Operand, b: Operand },

    // Control flow
    Jmp { target: String },
    Je { target: String },
    Jne { target: String },
    Jl { target: String },
    Jle { target: String },
    Jg { target: String },
    Jge { target: String },
    Ja { target: String },
    Jae { target: String },
    Jb { target: String },
    Jbe { target: String },
    Jo { target: String },
    Jno { target: String },
    Js { target: String },
    Jns { target: String },
    Call { target: String },
    Ret,
    Int3,

    // Floating point
    Cvtsi2sd { dest: Xmm, src: Operand },
    Cvtsd2si { dest: R8, src: Operand },
    Addsd { dest: Xmm, src: Operand },
    Subsd { dest: Xmm, src: Operand },
    Mulsd { dest: Xmm, src: Operand },
    Divsd { dest: Xmm, src: Operand },
    Ucomisd { a: Operand, b: Operand },
    Movsd { dest: Operand, src: Operand },
    Movaps { dest: Operand, src: Operand },

    // SIMD
    Pxor { dest: Xmm, src: Operand },
    Pcmpeqd { dest: Xmm, src: Operand },
    Pcmpgtd { dest: Xmm, src: Operand },
    Paddb { dest: Xmm, src: Operand },
    Paddw { dest: Xmm, src: Operand },
    Paddd { dest: Xmm, src: Operand },
    Paddq { dest: Xmm, src: Operand },
    Psubb { dest: Xmm, src: Operand },
    Psubw { dest: Xmm, src: Operand },
    Psubd { dest: Xmm, src: Operand },
    Psubq { dest: Xmm, src: Operand },
    Pmuludq { dest: Xmm, src: Operand },
    Pmaddwd { dest: Xmm, src: Operand },

    // Other
    Nop,
    Hlt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Xmm {
    Xmm0, Xmm1, Xmm2, Xmm3, Xmm4, Xmm5, Xmm6, Xmm7,
    Xmm8, Xmm9, Xmm10, Xmm11, Xmm12, Xmm13, Xmm14, Xmm15,
}

/// AArch64 instruction
#[derive(Debug, Clone)]
pub enum A64Inst {
    // Data movement
    Mov { dest: A64, src: Operand },
    Mvn { dest: A64, src: Operand },
    Ldr { dest: A64, src: Operand },
    Str { src: A64, dest: Operand },
    Ldp { dest: (A64, A64), src: Operand },
    Stp { src: (A64, A64), dest: Operand },
    Ldrb { dest: A64, src: Operand },
    Strb { src: A64, dest: Operand },
    Ldrh { dest: A64, src: Operand },
    Strh { src: A64, dest: Operand },

    // Arithmetic
    Add { dest: A64, a: A64, b: Operand },
    Adds { dest: A64, a: A64, b: Operand }, // Set flags
    Sub { dest: A64, a: A64, b: Operand },
    Subs { dest: A64, a: A64, b: Operand }, // Set flags
    Mul { dest: A64, a: A64, b: A64 },
    Mneg { dest: A64, a: A64, b: A64 },
    Sdiv { dest: A64, a: A64, b: A64 },
    Udiv { dest: A64, a: A64, b: A64 },
    Srem { dest: A64, a: A64, b: A64 },
    Urem { dest: A64, a: A64, b: A64 },
    Neg { dest: A64, src: A64 },
    Negs { dest: A64, src: A64 }, // Set flags

    // Logical
    And { dest: A64, a: A64, b: Operand },
    Orr { dest: A64, a: A64, b: Operand },
    Eor { dest: A64, a: A64, b: Operand },
    Bic { dest: A64, a: A64, b: Operand },
    Tst { a: A64, b: Operand },
    Mvn { dest: A64, src: Operand },

    // Shifts
    Lsl { dest: A64, src: A64, shift: Operand },
    Lsr { dest: A64, src: A64, shift: Operand },
    Asr { dest: A64, src: A64, shift: Operand },
    Ror { dest: A64, src: A64, shift: Operand },

    // Comparison
    Cmp { a: A64, b: Operand },
    Cmps { a: A64, b: Operand }, // Set flags

    // Control flow
    B { target: String },
    BEq { target: String },
    BNeq { target: String },
    BLt { target: String },
    BLe { target: String },
    BGt { target: String },
    BGe { target: String },
    BCs { target: String }, // Carry set
    BHi { target: String }, // Unsigned higher
    BHs { target: String }, // Unsigned higher or same
    BLo { target: String }, // Unsigned lower
    BLs { target: String }, // Unsigned lower or same
    BPl { target: String }, // Positive
    BMi { target: String }, // Negative
    BVs { target: String }, // Overflow set
    BVc { target: String }, // Overflow clear
    Bl { target: String }, // Branch with link
    Br { target: Register },
    Ret,
    Cbz { dest: A64, target: String },
    Cbnz { dest: A64, target: String },
    Tbnz { dest: A64, bit: u8, target: String },
    Tbz { dest: A64, bit: u8, target: String },

    // Floating point
    Fmov { dest: V, src: Operand },
    Fcvtzs { dest: A64, src: V },
    Scvtf { dest: V, src: A64 },
    Fcmp { a: V, b: V },
    Fadd { dest: V, a: V, b: V },
    Fsub { dest: V, a: V, b: V },
    Fmul { dest: V, a: V, b: V },
    Fdiv { dest: V, a: V, b: V },
    Fneg { dest: V, src: V },
    Fabs { dest: V, src: V },
    Fsqrt { dest: V, src: V },

    // SIMD
    AdvSimd { inst: String, dest: V, a: V, b: V },

    // Other
    Nop,
    Brk,
   Svc { imm: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum V {
    V0, V1, V2, V3, V4, V5, V6, V7,
    V8, V9, V10, V11, V12, V13, V14, V15,
    V16, V17, V18, V19, V20, V21, V22, V23,
    V24, V25, V26, V27, V28, V29, V30, V31,
}

impl V {
    pub fn from_idx(idx: u8) -> Option<V> {
        match idx {
            0 => Some(V::V0), 1 => Some(V::V1), 2 => Some(V::V2), 3 => Some(V::V3),
            4 => Some(V::V4), 5 => Some(V::V5), 6 => Some(V::V6), 7 => Some(V::V7),
            8 => Some(V::V8), 9 => Some(V::V9), 10 => Some(V::V10), 11 => Some(V::V11),
            12 => Some(V::V12), 13 => Some(V::V13), 14 => Some(V::V14), 15 => Some(V::V15),
            16 => Some(V::V16), 17 => Some(V::V17), 18 => Some(V::V18), 19 => Some(V::V19),
            20 => Some(V::V20), 21 => Some(V::V21), 22 => Some(V::V22), 23 => Some(V::V23),
            24 => Some(V::V24), 25 => Some(V::V25), 26 => Some(V::V26), 27 => Some(V::V27),
            28 => Some(V::V28), 29 => Some(V::V29), 30 => Some(V::V30), 31 => Some(V::V31),
            _ => None,
        }
    }
}

/// Function compiled to native code
#[derive(Debug, Clone)]
pub struct NativeFunction {
    pub name: String,
    pub params: Vec<String>,
    pub locals: Vec<String>,
    pub instructions: Vec<CompiledInst>,
    pub frame_size: usize,
    pub align: usize,
}

/// Compiled instruction
#[derive(Debug, Clone)]
pub struct CompiledInst {
    pub inst: Instruction,
    pub size: usize,
}

/// Architecture-independent instruction
#[derive(Debug, Clone)]
pub enum Instruction {
    X86_64(X86Inst),
    AArch64(A64Inst),
}

/// Native code generator
pub struct NativeCodegen {
    target: Target,
    functions: HashMap<String, NativeFunction>,
    current_func: Option<String>,
    next_vreg: u16,
    label_counter: usize,
}

impl NativeCodegen {
    pub fn new(target: Target) -> Self {
        NativeCodegen {
            target,
            functions: HashMap::new(),
            current_func: None,
            next_vreg: 0,
            label_counter: 0,
        }
    }

    /// Allocate a virtual register
    pub fn alloc_vreg(&mut self) -> Register {
        let reg = Register::Virtual(self.next_vreg);
        self.next_vreg += 1;
        reg
    }

    /// Generate a unique label
    pub fn new_label(&mut self) -> String {
        let label = format!(".L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Compile a function
    pub fn compile_function(&mut self, func: &Function) -> Result<NativeFunction, CodegenError> {
        println!("TUMC: NativeCodegen - Compiling function '{}' for {:?}", func.name, self.target.arch);

        let mut native_func = NativeFunction {
            name: func.name.clone(),
            params: func.params.iter().map(|p| p.name.clone()).collect(),
            locals: vec![],
            instructions: vec![],
            frame_size: 0,
            align: 16,
        };

        // Emit function prologue
        self.emit_prologue(&mut native_func);

        // Compile each statement
        for stmt in &func.body {
            self.compile_stmt(stmt, &mut native_func)?;
        }

        // Emit function epilogue
        self.emit_epilogue(&mut native_func);

        Ok(native_func)
    }

    /// Emit function prologue
    fn emit_prologue(&self, func: &mut NativeFunction) {
        match self.target.arch {
            Arch::X86_64 => {
                // push rbp
                func.instructions.push(CompiledInst {
                    inst: Instruction::X86_64(X86Inst::Push { src: Operand::Reg(Register::X86_64(R8::Rbp)) }),
                    size: 1,
                });
                // mov rbp, rsp
                func.instructions.push(CompiledInst {
                    inst: Instruction::X86_64(X86Inst::Mov { dest: R8::Rbp, src: Operand::Reg(Register::X86_64(R8::Rsp)) }),
                    size: 3,
                });
            }
            Arch::AArch64 => {
                // stp x29, x30, [sp, #-16]!
                func.instructions.push(CompiledInst {
                    inst: Instruction::AArch64(A64Inst::Stp {
                        src: (A64::X29, A64::X30),
                        dest: Operand::Mem { base: Register::AArch64(A64::Sp), offset: -16, scale: 1, index: None },
                    }),
                    size: 4,
                });
                // mov x29, sp
                func.instructions.push(CompiledInst {
                    inst: Instruction::AArch64(A64Inst::Mov {
                        dest: A64::X29,
                        src: Operand::Reg(Register::AArch64(A64::Sp)),
                    }),
                    size: 4,
                });
            }
            _ => {}
        }
    }

    /// Emit function epilogue
    fn emit_epilogue(&self, func: &mut NativeFunction) {
        match self.target.arch {
            Arch::X86_64 => {
                // pop rbp
                func.instructions.push(CompiledInst {
                    inst: Instruction::X86_64(X86Inst::Pop { dest: R8::Rbp }),
                    size: 1,
                });
                // ret
                func.instructions.push(CompiledInst {
                    inst: Instruction::X86_64(X86Inst::Ret),
                    size: 1,
                });
            }
            Arch::AArch64 => {
                // ldp x29, x30, [sp], #16
                func.instructions.push(CompiledInst {
                    inst: Instruction::AArch64(A64Inst::Ldp {
                        dest: (A64::X29, A64::X30),
                        src: Operand::Mem { base: Register::AArch64(A64::Sp), offset: 0, scale: 1, index: None },
                    }),
                    size: 4,
                });
                // ret
                func.instructions.push(CompiledInst {
                    inst: Instruction::AArch64(A64Inst::Ret),
                    size: 4,
                });
            }
            _ => {}
        }
    }

    /// Compile a statement
    fn compile_stmt(&self, stmt: &Stmt, func: &mut NativeFunction) -> Result<(), CodegenError> {
        match &stmt.kind {
            StmtKind::Expr(expr) => {
                self.compile_expr(expr, func)?;
            }
            StmtKind::Let(pattern, _, value, body) => {
                if let Some(val) = value {
                    self.compile_expr(val, func)?;
                }
                for s in body {
                    self.compile_stmt(s, func)?;
                }
            }
            StmtKind::Return(Some(expr)) => {
                self.compile_expr(expr, func)?;
            }
            StmtKind::If(condition, then_block, else_block) => {
                self.compile_expr(condition, func)?;
                for s in then_block {
                    self.compile_stmt(s, func)?;
                }
                if let Some((_, else_stmts)) = else_block {
                    for s in else_stmts {
                        self.compile_stmt(s, func)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Compile an expression
    fn compile_expr(&self, expr: &Expr, func: &mut NativeFunction) -> Result<(), CodegenError> {
        match &expr.kind {
            ExprKind::Literal(lit) => {
                self.emit_load_literal(lit, func);
            }
            ExprKind::Binary { op, lhs, rhs } => {
                self.compile_expr(lhs, func)?;
                self.compile_expr(rhs, func)?;
                self.emit_binary_op(op, func);
            }
            ExprKind::Identifier(name) => {
                self.emit_load_var(name, func);
            }
            ExprKind::Call { function, arguments } => {
                // Emit arguments
                for arg in arguments {
                    self.compile_expr(arg, func)?;
                }
                // Call function
                if let ExprKind::Identifier(name) = &function.kind {
                    self.emit_call(name, arguments.len(), func);
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// Emit load literal
    fn emit_load_literal(&self, lit: &Literal, func: &mut NativeFunction) {
        match self.target.arch {
            Arch::X86_64 => {
                match lit {
                    Literal::Int(n) => {
                        func.instructions.push(CompiledInst {
                            inst: Instruction::X86_64(X86Inst::Mov {
                                dest: R8::Rax,
                                src: Operand::Imm(*n as i64),
                            }),
                            size: 10,
                        });
                    }
                    Literal::String(s) => {
                        // Would load string address
                        func.instructions.push(CompiledInst {
                            inst: Instruction::X86_64(X86Inst::Lea {
                                dest: R8::Rax,
                                src: Operand::Label(format!(".str_{}", s.len())),
                            }),
                            size: 7,
                        });
                    }
                    _ => {}
                }
            }
            Arch::AArch64 => {
                match lit {
                    Literal::Int(n) => {
                        func.instructions.push(CompiledInst {
                            inst: Instruction::AArch64(A64Inst::Mov {
                                dest: A64::X0,
                                src: Operand::Imm(*n as i64),
                            }),
                            size: 4,
                        });
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    /// Emit load variable
    fn emit_load_var(&self, name: &str, func: &mut NativeFunction) {
        match self.target.arch {
            Arch::X86_64 => {
                func.instructions.push(CompiledInst {
                    inst: Instruction::X86_64(X86Inst::Mov {
                        dest: R8::Rax,
                        src: Operand::Mem {
                            base: Register::X86_64(R8::Rbp),
                            offset: -8,
                            scale: 1,
                            index: None,
                        },
                    }),
                    size: 3,
                });
            }
            Arch::AArch64 => {
                func.instructions.push(CompiledInst {
                    inst: Instruction::AArch64(A64Inst::Ldr {
                        dest: A64::X0,
                        src: Operand::Mem {
                            base: Register::AArch64(A64::X29),
                            offset: -8,
                            scale: 1,
                            index: None,
                        },
                    }),
                    size: 4,
                });
            }
            _ => {}
        }
    }

    /// Emit binary operation
    fn emit_binary_op(&self, op: &BinOp, func: &mut NativeFunction) {
        match self.target.arch {
            Arch::X86_64 => {
                let inst = match op {
                    BinOp::Add => X86Inst::Add { dest: R8::Rax, src: Operand::Reg(Register::X86_64(R8::Rbx)) },
                    BinOp::Sub => X86Inst::Sub { dest: R8::Rax, src: Operand::Reg(Register::X86_64(R8::Rbx)) },
                    BinOp::Mul => X86Inst::Imul { dest: R8::Rax, src: Operand::Reg(Register::X86_64(R8::Rbx)) },
                    BinOp::Div => X86Inst::Idiv { src: Operand::Reg(Register::X86_64(R8::Rbx)) },
                    BinOp::And => X86Inst::And { dest: R8::Rax, src: Operand::Reg(Register::X86_64(R8::Rbx)) },
                    BinOp::Or => X86Inst::Or { dest: R8::Rax, src: Operand::Reg(Register::X86_64(R8::Rbx)) },
                    BinOp::Xor => X86Inst::Xor { dest: R8::Rax, src: Operand::Reg(Register::X86_64(R8::Rbx)) },
                    BinOp::Shl => X86Inst::Shl { dest: R8::Rax, src: Operand::Reg(Register::X86_64(R8::Rcx)) },
                    BinOp::Shr => X86Inst::Shr { dest: R8::Rax, src: Operand::Reg(Register::X86_64(R8::Rcx)) },
                    _ => X86Inst::Nop,
                };
                func.instructions.push(CompiledInst {
                    inst: Instruction::X86_64(inst),
                    size: 3,
                });
            }
            Arch::AArch64 => {
                let inst = match op {
                    BinOp::Add => A64Inst::Add { dest: A64::X0, a: A64::X0, b: Operand::Reg(Register::AArch64(A64::X1)) },
                    BinOp::Sub => A64Inst::Sub { dest: A64::X0, a: A64::X0, b: Operand::Reg(Register::AArch64(A64::X1)) },
                    BinOp::Mul => A64Inst::Mul { dest: A64::X0, a: A64::X0, b: A64::X1 },
                    BinOp::And => A64Inst::And { dest: A64::X0, a: A64::X0, b: Operand::Reg(Register::AArch64(A64::X1)) },
                    BinOp::Or => A64Inst::Orr { dest: A64::X0, a: A64::X0, b: Operand::Reg(Register::AArch64(A64::X1)) },
                    BinOp::Xor => A64Inst::Eor { dest: A64::X0, a: A64::X0, b: Operand::Reg(Register::AArch64(A64::X1)) },
                    _ => A64Inst::Nop,
                };
                func.instructions.push(CompiledInst {
                    inst: Instruction::AArch64(inst),
                    size: 4,
                });
            }
            _ => {}
        }
    }

    /// Emit function call
    fn emit_call(&self, name: &str, _argc: usize, func: &mut NativeFunction) {
        match self.target.arch {
            Arch::X86_64 => {
                func.instructions.push(CompiledInst {
                    inst: Instruction::X86_64(X86Inst::Call { target: name.to_string() }),
                    size: 5,
                });
            }
            Arch::AArch64 => {
                func.instructions.push(CompiledInst {
                    inst: Instruction::AArch64(A64Inst::Bl { target: name.to_string() }),
                    size: 4,
                });
            }
            _ => {}
        }
    }

    /// Emit machine code to bytes
    pub fn emit_bytes(&self, func: &NativeFunction) -> Vec<u8> {
        let mut bytes = Vec::new();

        for inst in &func.instructions {
            match &inst.inst {
                Instruction::X86_64(i) => {
                    bytes.extend_from_slice(&self.emit_x86_inst(i));
                }
                Instruction::AArch64(i) => {
                    bytes.extend_from_slice(&self.emit_a64_inst(i));
                }
            }
        }

        bytes
    }

    /// Emit x86_64 instruction bytes
    fn emit_x86_inst(&self, inst: &X86Inst) -> Vec<u8> {
        let mut bytes = Vec::new();

        match inst {
            X86Inst::Mov { dest, src } => {
                bytes.push(0x48); // REX.W
                bytes.push(0x89);
                bytes.extend_from_slice(&self.emit_x86_modrm(*dest, src));
            }
            X86Inst::Add { dest, src } => {
                bytes.push(0x48); // REX.W
                bytes.push(0x01);
                bytes.extend_from_slice(&self.emit_x86_modrm(*dest, src));
            }
            X86Inst::Sub { dest, src } => {
                bytes.push(0x48); // REX.W
                bytes.push(0x29);
                bytes.extend_from_slice(&self.emit_x86_modrm(*dest, src));
            }
            X86Inst::Ret => {
                bytes.push(0xC3);
            }
            X86Inst::Push { src } => {
                bytes.push(0x50 + self.x86_reg_num(src));
            }
            X86Inst::Pop { dest } => {
                bytes.push(0x58 + self.x86_reg_num(dest));
            }
            X86Inst::Call { target } => {
                bytes.push(0xE8);
                // Would add relative offset
                bytes.extend_from_slice(&0i32.to_le_bytes());
            }
            X86Inst::Jmp { target } => {
                bytes.push(0xE9);
                bytes.extend_from_slice(&0i32.to_le_bytes());
            }
            X86Inst::Nop => {
                bytes.push(0x90);
            }
            _ => {}
        }

        bytes
    }

    /// Emit AArch64 instruction bytes
    fn emit_a64_inst(&self, inst: &A64Inst) -> Vec<u8> {
        let mut bytes = Vec::new();

        match inst {
            A64Inst::Ret => {
                // ret
                bytes.extend_from_slice(&0xD65F03C0u32.to_le_bytes());
            }
            A64Inst::Mov { dest, src } => {
                if let Operand::Imm(imm) = src {
                    // movz
                    let enc = 0xD2800000u32
                        | ((*imm as u32) << 5)
                        | (*dest as u32);
                    bytes.extend_from_slice(&enc.to_le_bytes());
                } else {
                    // orr xd, xzr, xs
                    let enc = 0xAA2003E0u32
                        | (*dest as u32)
                        | ((src as u32) << 5);
                    bytes.extend_from_slice(&enc.to_le_bytes());
                }
            }
            A64Inst::Add { dest, a, b } => {
                if let Operand::Imm(imm) = b {
                    let enc = 0x91000000u32
                        | ((*dest as u32) << 0)
                        | ((*a as u32) << 5)
                        | ((*imm as u32 & 0xFFF) << 10);
                    bytes.extend_from_slice(&enc.to_le_bytes());
                } else {
                    let enc = 0x8B000000u32
                        | ((*dest as u32) << 0)
                        | ((*a as u32) << 5)
                        | ((*b as u32) << 16);
                    bytes.extend_from_slice(&enc.to_le_bytes());
                }
            }
            A64Inst::Bl { target } => {
                // bl
                bytes.extend_from_slice(&0x94000000u32.to_le_bytes());
            }
            A64Inst::B { target } => {
                // b
                bytes.extend_from_slice(&0x14000000u32.to_le_bytes());
            }
            A64Inst::Nop => {
                bytes.extend_from_slice(&0xD503201Fu32.to_le_bytes());
            }
            _ => {}
        }

        bytes
    }

    /// Get x86_64 register number
    fn x86_reg_num(&self, reg: &Operand) -> u8 {
        match reg {
            Operand::Reg(Register::X86_64(r)) => match r {
                R8::Rax => 0, R8::Rcx => 1, R8::Rdx => 2, R8::Rbx => 3,
                R8::Rsp => 4, R8::Rbp => 5, R8::Rsi => 6, R8::Rdi => 7,
                R8::R8 => 8, R8::R9 => 9, R8::R10 => 10, R8::R11 => 11,
                R8::R12 => 12, R8::R13 => 13, R8::R14 => 14, R8::R15 => 15,
            },
            _ => 0,
        }
    }

    /// Emit x86 ModRM byte
    fn emit_x86_modrm(&self, dest: R8, src: &Operand) -> Vec<u8> {
        let mut bytes = vec![];

        let (mod_, rm, reg) = match src {
            Operand::Reg(Register::X86_64(r)) => (0xC0, self.x86_reg_num(src), self.x86_reg_num(&Operand::Reg(Register::X86_64(dest)))),
            Operand::Imm(_) => (0xC0, self.x86_reg_num(&Operand::Reg(Register::X86_64(dest))), 0),
            _ => (0x00, 0x00, 0x00),
        };

        bytes.push((mod_ << 6) | (reg << 3) | rm);

        if let Operand::Imm(imm) = src {
            bytes.extend_from_slice(&(*imm as i32).to_le_bytes());
        }

        bytes
    }
}

/// Code generation error
#[derive(Debug, Clone)]
pub enum CodegenError {
    UnsupportedArch(Arch),
    UnsupportedOS(OS),
    RegisterAllocationFailed,
    OutOfRegisters,
    InvalidOperand,
}

impl fmt::Display for CodegenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodegenError::UnsupportedArch(arch) => write!(f, "Unsupported architecture: {:?}", arch),
            CodegenError::UnsupportedOS(os) => write!(f, "Unsupported OS: {:?}", os),
            CodegenError::RegisterAllocationFailed => write!(f, "Register allocation failed"),
            CodegenError::OutOfRegisters => write!(f, "Out of registers"),
            CodegenError::InvalidOperand => write!(f, "Invalid operand"),
        }
    }
}

impl std::error::Error for CodegenError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_codegen_x86_64() {
        let target = Target::x86_64_linux_gnu();
        let mut codegen = NativeCodegen::new(target);

        let func = NativeFunction {
            name: "test".to_string(),
            params: vec![],
            locals: vec![],
            instructions: vec![],
            frame_size: 0,
            align: 16,
        };

        let bytes = codegen.emit_bytes(&func);
        assert!(bytes.is_empty());
    }

    #[test]
    fn test_native_codegen_aarch64() {
        let target = Target::aarch64_linux_gnu();
        let codegen = NativeCodegen::new(target);
        assert_eq!(codegen.target.arch, Arch::AArch64);
    }
}
