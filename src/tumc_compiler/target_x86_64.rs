//! TUMC x86_64 Native Code Generation
//!
//! Code generation for x86_64 architecture with AVX/AVX2/SSE support.

use std::collections::HashMap;

/// x86_64 register
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum X86Reg {
    // 64-bit general purpose
    Rax, Rcx, Rdx, Rbx, Rsp, Rbp, Rsi, Rdi,
    R8, R9, R10, R11, R12, R13, R14, R15,
    // 32-bit general purpose
    Eax, Ecx, Edx, Ebx, Esp, Ebp, Esi, Edi,
    R8d, R9d, R10d, R11d, R12d, R13d, R14d, R15d,
    // 16-bit
    Ax, Cx, Dx, Bx, Sp, Bp, Si, Di,
    R8w, R9w, R10w, R11w, R12w, R13w, R14w, R15w,
    // 8-bit
    Al, Cl, Dl, Bl, Ah, Ch, Dh, Bh,
    Spl, Bpl, Sil, Dil,
    R8b, R9b, R10b, R11b, R12b, R13b, R14b, R15b,
    // XMM registers (SSE/AVX)
    Xmm0, Xmm1, Xmm2, Xmm3, Xmm4, Xmm5, Xmm6, Xmm7,
    Xmm8, Xmm9, Xmm10, Xmm11, Xmm12, Xmm13, Xmm14, Xmm15,
    // YMM registers (AVX)
    Ymm0, Ymm1, Ymm2, Ymm3, Ymm4, Ymm5, Ymm6, Ymm7,
    Ymm8, Ymm9, Ymm10, Ymm11, Ymm12, Ymm13, Ymm14, Ymm15,
    // ZMM registers (AVX-512)
    Zmm0, Zmm1, Zmm2, Zmm3, Zmm4, Zmm5, Zmm6, Zmm7,
    Zmm8, Zmm9, Zmm10, Zmm11, Zmm12, Zmm13, Zmm14, Zmm15,
    Zmm16, Zmm17, Zmm18, Zmm19, Zmm20, Zmm21, Zmm22, Zmm23,
    Zmm24, Zmm25, Zmm26, Zmm27, Zmm28, Zmm29, Zmm30, Zmm31,
}

impl X86Reg {
    /// Get encoding byte for REX-encoded registers
    pub fn encode(&self) -> u8 {
        match self {
            // 64-bit registers
            X86Reg::Rax => 0,
            X86Reg::Rcx => 1,
            X86Reg::Rdx => 2,
            X86Reg::Rbx => 3,
            X86Reg::Rsp => 4,
            X86Reg::Rbp => 5,
            X86Reg::Rsi => 6,
            X86Reg::Rdi => 7,
            X86Reg::R8 => 8,
            X86Reg::R9 => 9,
            X86Reg::R10 => 10,
            X86Reg::R11 => 11,
            X86Reg::R12 => 12,
            X86Reg::R13 => 13,
            X86Reg::R14 => 14,
            X86Reg::R15 => 15,
            // 32-bit registers
            X86Reg::Eax => 0,
            X86Reg::Ecx => 1,
            X86Reg::Edx => 2,
            X86Reg::Ebx => 3,
            X86Reg::Esp => 4,
            X86Reg::Ebp => 5,
            X86Reg::Esi => 6,
            X86Reg::Edi => 7,
            X86Reg::R8d => 8,
            X86Reg::R9d => 9,
            X86Reg::R10d => 10,
            X86Reg::R11d => 11,
            X86Reg::R12d => 12,
            X86Reg::R13d => 13,
            X86Reg::R14d => 14,
            X86Reg::R15d => 15,
            _ => 0,
        }
    }

    /// Check if register needs REX prefix
    pub fn needs_rex(&self) -> bool {
        matches!(
            self,
            X86Reg::R8 | X86Reg::R9 | X86Reg::R10 | X86Reg::R11 |
            X86Reg::R12 | X86Reg::R13 | X86Reg::R14 | X86Reg::R15 |
            X86Reg::R8d | X86Reg::R9d | X86Reg::R10d | X86Reg::R11d |
            X86Reg::R12d | X86Reg::R13d | X86Reg::R14d | X86Reg::R15d |
            X86Reg::R8w | X86Reg::R9w | X86Reg::R10w | X86Reg::R11w |
            X86Reg::R12w | X86Reg::R13w | X86Reg::R14w | X86Reg::R15w |
            X86Reg::R8b | X86Reg::R9b | X86Reg::R10b | X86Reg::R11b |
            X86Reg::R12b | X86Reg::R13b | X86Reg::R14b | X86Reg::R15b |
            X86Reg::Spl | X86Reg::Bpl | X86Reg::Sil | X86Reg::Dil
        )
    }
}

/// Memory operand
#[derive(Debug, Clone)]
pub struct MemOperand {
    /// Base register
    pub base: Option<X86Reg>,
    /// Index register
    pub index: Option<X86Reg>,
    /// Scale (1, 2, 4, 8)
    pub scale: u8,
    /// Displacement
    pub disp: i32,
}

impl MemOperand {
    /// Create new memory operand
    pub fn new(disp: i32) -> Self {
        Self {
            base: None,
            index: None,
            scale: 1,
            disp,
        }
    }

    /// With base register
    pub fn with_base(mut self, reg: X86Reg) -> Self {
        self.base = Some(reg);
        self
    }

    /// With index and scale
    pub fn with_index(mut self, reg: X86Reg, scale: u8) -> Self {
        self.index = Some(reg);
        self.scale = scale;
        self
    }
}

/// Operand for x86_64 instructions
#[derive(Debug, Clone)]
pub enum X86Operand {
    /// Register
    Reg(X86Reg),
    /// Memory reference
    Mem(MemOperand),
    /// Immediate value (8-bit)
    Imm8(u8),
    /// Immediate value (16-bit)
    Imm16(u16),
    /// Immediate value (32-bit)
    Imm32(u32),
    /// Immediate value (64-bit)
    Imm64(u64),
    /// Relative offset (for jumps/calls)
    Rel32(i32),
    /// RIP-relative address
    RipRel(i32),
}

/// x86_64 instruction
#[derive(Debug, Clone)]
pub struct X86Inst {
    /// Opcode bytes
    pub opcode: Vec<u8>,
    /// Operands
    pub operands: Vec<X86Operand>,
    /// Prefix bytes
    pub prefixes: Vec<u8>,
}

impl X86Inst {
    /// Create new instruction
    pub fn new(opcode: Vec<u8>) -> Self {
        Self {
            opcode,
            operands: Vec::new(),
            prefixes: Vec::new(),
        }
    }

    /// Add operand
    pub fn op(mut self, operand: X86Operand) -> Self {
        self.operands.push(operand);
        self
    }

    /// Add prefix
    pub fn prefix(mut self, p: u8) -> Self {
        self.prefixes.push(p);
        self
    }
}

/// x86_64 assembler
pub struct X86Asm {
    /// Output buffer
    code: Vec<u8>,
    /// Labels
    labels: HashMap<String, usize>,
    /// Pending relocations
    relocations: Vec<Relocation>,
    /// Current position
    pos: usize,
}

impl X86Asm {
    /// Create new assembler
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            labels: HashMap::new(),
            relocations: Vec::new(),
            pos: 0,
        }
    }

    /// Emit byte
    pub fn emit_byte(&mut self, b: u8) {
        self.code.push(b);
        self.pos += 1;
    }

    /// Emit bytes
    pub fn emit_bytes(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
        self.pos += bytes.len();
    }

    /// Emit 16-bit value
    pub fn emit_word(&mut self, val: u16) {
        self.code.extend_from_slice(&val.to_le_bytes());
        self.pos += 2;
    }

    /// Emit 32-bit value
    pub fn emit_dword(&mut self, val: u32) {
        self.code.extend_from_slice(&val.to_le_bytes());
        self.pos += 4;
    }

    /// Emit 64-bit value
    pub fn emit_qword(&mut self, val: u64) {
        self.code.extend_from_slice(&val.to_le_bytes());
        self.pos += 8;
    }

    /// Define label
    pub fn label(&mut self, name: &str) {
        self.labels.insert(name.to_string(), self.pos);
    }

    /// Get current position
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Add relocation
    pub fn add_relocation(&mut self, reloc: Relocation) {
        self.relocations.push(reloc);
    }

    /// Emit REX prefix
    pub fn emit_rex(&mut self, w: bool, r: bool, x: bool, b: bool) {
        let mut rex = 0x40u8;
        if w { rex |= 0x08; }
        if r { rex |= 0x04; }
        if x { rex |= 0x02; }
        if b { rex |= 0x01; }
        if rex != 0x40 {
            self.emit_byte(rex);
        }
    }

    /// Emit ModR/M byte
    pub fn emit_modrm(&mut self, mod_: u8, reg: u8, rm: u8) {
        let byte = ((mod_ & 0x03) << 6) | ((reg & 0x07) << 3) | (rm & 0x07);
        self.emit_byte(byte);
    }

    /// Emit SIB byte
    pub fn emit_sib(&mut self, scale: u8, index: u8, base: u8) {
        let byte = ((scale & 0x03) << 6) | ((index & 0x07) << 3) | (base & 0x07);
        self.emit_byte(byte);
    }

    /// Get generated code
    pub fn finalize(self) -> X86Binary {
        X86Binary {
            code: self.code,
            labels: self.labels,
            relocations: self.relocations,
        }
    }

    // ==== Instruction Emission ====

    /// MOV r/m64, r64
    pub fn mov_r64_r64(&mut self, dst: X86Reg, src: X86Reg) {
        self.emit_rex(true, false, false, false);
        self.emit_byte(0x89);
        self.emit_modrm(3, dst.encode() & 7, src.encode() & 7);
    }

    /// MOV r64, imm32
    pub fn mov_r64_imm32(&mut self, dst: X86Reg, imm: u32) {
        self.emit_rex(true, false, false, dst.needs_rex());
        self.emit_byte(0xC7);
        self.emit_modrm(3, 0, dst.encode() & 7);
        self.emit_dword(imm);
    }

    /// MOV r64, imm64
    pub fn mov_r64_imm64(&mut self, dst: X86Reg, imm: u64) {
        self.emit_rex(true, false, false, dst.needs_rex());
        self.emit_byte(0xB8 + (dst.encode() & 7));
        self.emit_qword(imm);
    }

    /// ADD r64, r64
    pub fn add_r64_r64(&mut self, dst: X86Reg, src: X86Reg) {
        self.emit_rex(true, false, false, false);
        self.emit_byte(0x01);
        self.emit_modrm(3, dst.encode() & 7, src.encode() & 7);
    }

    /// SUB r64, r64
    pub fn sub_r64_r64(&mut self, dst: X86Reg, src: X86Reg) {
        self.emit_rex(true, false, false, false);
        self.emit_byte(0x29);
        self.emit_modrm(3, dst.encode() & 7, src.encode() & 7);
    }

    /// IMUL r64, r64
    pub fn imul_r64_r64(&mut self, dst: X86Reg, src: X86Reg) {
        self.emit_rex(true, false, false, false);
        self.emit_bytes(&[0x0F, 0xAF]);
        self.emit_modrm(3, dst.encode() & 7, src.encode() & 7);
    }

    /// AND r64, r64
    pub fn and_r64_r64(&mut self, dst: X86Reg, src: X86Reg) {
        self.emit_rex(true, false, false, false);
        self.emit_byte(0x21);
        self.emit_modrm(3, dst.encode() & 7, src.encode() & 7);
    }

    /// OR r64, r64
    pub fn or_r64_r64(&mut self, dst: X86Reg, src: X86Reg) {
        self.emit_rex(true, false, false, false);
        self.emit_byte(0x09);
        self.emit_modrm(3, dst.encode() & 7, src.encode() & 7);
    }

    /// XOR r64, r64
    pub fn xor_r64_r64(&mut self, dst: X86Reg, src: X86Reg) {
        self.emit_rex(true, false, false, false);
        self.emit_byte(0x31);
        self.emit_modrm(3, dst.encode() & 7, src.encode() & 7);
    }

    /// CMP r64, r64
    pub fn cmp_r64_r64(&mut self, dst: X86Reg, src: X86Reg) {
        self.emit_rex(true, false, false, false);
        self.emit_byte(0x39);
        self.emit_modrm(3, dst.encode() & 7, src.encode() & 7);
    }

    /// TEST r64, r64
    pub fn test_r64_r64(&mut self, dst: X86Reg, src: X86Reg) {
        self.emit_rex(true, false, false, false);
        self.emit_bytes(&[0x85]);
        self.emit_modrm(3, dst.encode() & 7, src.encode() & 7);
    }

    /// PUSH r64
    pub fn push_r64(&mut self, src: X86Reg) {
        if src.needs_rex() {
            self.emit_byte(0x41);
        }
        self.emit_byte(0x50 + (src.encode() & 7));
    }

    /// POP r64
    pub fn pop_r64(&mut self, dst: X86Reg) {
        if dst.needs_rex() {
            self.emit_byte(0x41);
        }
        self.emit_byte(0x58 + (dst.encode() & 7));
    }

    /// JMP rel32
    pub fn jmp_rel32(&mut self, offset: i32) {
        self.emit_byte(0xE9);
        self.emit_dword(offset as u32);
    }

    /// JZ rel32
    pub fn jz_rel32(&mut self, offset: i32) {
        self.emit_bytes(&[0x0F, 0x84]);
        self.emit_dword(offset as u32);
    }

    /// JNZ rel32
    pub fn jnz_rel32(&mut self, offset: i32) {
        self.emit_bytes(&[0x0F, 0x85]);
        self.emit_dword(offset as u32);
    }

    /// CALL rel32
    pub fn call_rel32(&mut self, offset: i32) {
        self.emit_byte(0xE8);
        self.emit_dword(offset as u32);
    }

    /// RET
    pub fn ret(&mut self) {
        self.emit_byte(0xC3);
    }

    /// NOP
    pub fn nop(&mut self) {
        self.emit_byte(0x90);
    }

    /// INT3 (breakpoint)
    pub fn int3(&mut self) {
        self.emit_byte(0xCC);
    }

    /// SYSCALL
    pub fn syscall(&mut self) {
        self.emit_bytes(&[0x0F, 0x05]);
    }

    /// CPUID
    pub fn cpuid(&mut self) {
        self.emit_bytes(&[0x0F, 0xA2]);
    }
}

impl Default for X86Asm {
    fn default() -> Self {
        Self::new()
    }
}

/// Relocation entry
#[derive(Debug, Clone)]
pub struct Relocation {
    /// Offset in code
    pub offset: usize,
    /// Relocation type
    pub kind: RelocKind,
    /// Symbol name
    pub symbol: String,
    /// Addend
    pub addend: i64,
}

/// Relocation kind
#[derive(Debug, Clone, Copy)]
pub enum RelocKind {
    /// Absolute 64-bit
    Absolute64,
    /// PC-relative 32-bit
    Relative32,
    /// RIP-relative 32-bit
    RipRel32,
    /// GOT entry
    GotEntry,
    /// PLT entry
    PltEntry,
}

/// Generated x86_64 binary
#[derive(Debug, Clone)]
pub struct X86Binary {
    /// Machine code
    pub code: Vec<u8>,
    /// Label offsets
    pub labels: HashMap<String, usize>,
    /// Relocations
    pub relocations: Vec<Relocation>,
}

impl X86Binary {
    /// Get the machine code
    pub fn code(&self) -> &[u8] {
        &self.code
    }

    /// Apply relocations (for position-independent code)
    pub fn apply_relocations(&mut self, base_addr: u64) {
        for reloc in &self.relocations {
            match reloc.kind {
                RelocKind::Absolute64 => {
                    let target = if let Some(&offset) = self.labels.get(&reloc.symbol) {
                        (base_addr as i64 + offset as i64 + reloc.addend) as u64
                    } else {
                        reloc.addend as u64
                    };
                    let bytes = target.to_le_bytes();
                    self.code[reloc.offset..reloc.offset + 8].copy_from_slice(&bytes);
                }
                RelocKind::Relative32 => {
                    let target = if let Some(&offset) = self.labels.get(&reloc.symbol) {
                        offset as i32 - reloc.offset as i32 - 4 + reloc.addend as i32
                    } else {
                        reloc.addend as i32
                    };
                    let bytes = target.to_le_bytes();
                    self.code[reloc.offset..reloc.offset + 4].copy_from_slice(&bytes);
                }
                _ => {}
            }
        }
    }
}

/// Function frame layout for x86_64 System V ABI
#[derive(Debug, Clone)]
pub struct X86Frame {
    /// Frame size (red zone + locals + spills)
    pub size: usize,
    /// Red zone size (128 bytes)
    pub red_zone: usize,
    /// Local variable area
    pub locals: usize,
    /// Register saves area
    pub saved_regs: Vec<X86Reg>,
}

impl X86Frame {
    /// Create new frame
    pub fn new(locals: usize, saved_regs: Vec<X86Reg>) -> Self {
        let saved_size = saved_regs.len() * 8;
        let frame_size = 128 + locals + saved_size; // 128 for red zone
        Self {
            size: frame_size,
            red_zone: 128,
            locals,
            saved_regs,
        }
    }

    /// Get stack offset for local
    pub fn local_offset(&self, index: usize) -> i32 {
        -(self.red_zone as i32 + (index + 1) as i32 * 8)
    }

    /// Get stack offset for saved register
    pub fn saved_reg_offset(&self, index: usize) -> i32 {
        -(self.red_zone as i32 + self.locals as i32 + (index as i32 + 1) * 8)
    }
}

/// x86_64 code generator
pub struct X86CodeGen {
    /// Assembler
    asm: X86Asm,
    /// Current frame
    frame: Option<X86Frame>,
    /// Function prologue/epilogue
    emit_prologue: bool,
    emit_epilogue: bool,
}

impl X86CodeGen {
    /// Create new code generator
    pub fn new() -> Self {
        Self {
            asm: X86Asm::new(),
            frame: None,
            emit_prologue: true,
            emit_epilogue: true,
        }
    }

    /// Begin function
    pub fn begin_function(&mut self, frame: X86Frame) {
        self.frame = Some(frame.clone());

        if self.emit_prologue {
            // Push rbp
            self.asm.push_r64(X86Reg::Rbp);
            // mov rbp, rsp
            self.asm.mov_r64_r64(X86Reg::Rbp, X86Reg::Rsp);

            // Allocate frame
            if frame.size > 0 {
                // sub rsp, frame_size
                self.asm.emit_rex(true, false, false, false);
                self.asm.emit_byte(0x81);
                self.asm.emit_modrm(3, 5, 4); // sub rsp, imm32
                self.asm.emit_dword(frame.size as u32);
            }

            // Save callee-saved registers
            for (i, reg) in frame.saved_regs.iter().enumerate() {
                let offset = frame.saved_reg_offset(i);
                // mov [rbp + offset], reg
                let mem = MemOperand::new(offset).with_base(X86Reg::Rbp);
                self.emit_mov_r64_mem(reg.clone(), mem);
            }
        }
    }

    /// End function
    pub fn end_function(&mut self) {
        if self.emit_epilogue {
            // Leave (mov rsp, rbp; pop rbp)
            self.asm.emit_byte(0xC9);
            self.asm.ret();
        }
    }

    /// Emit MOV r64, r/m64
    pub fn emit_mov_r64_rm64(&mut self, dst: X86Reg, src: MemOperand) {
        self.asm.emit_rex(true, false, false, src.base.map_or(false, |r| r.needs_rex()));
        self.asm.emit_byte(0x8B);
        self.emit_mem_operand(dst.encode(), src);
    }

    /// Emit MOV r/m64, r64
    pub fn emit_mov_r64_mem(&mut self, src: X86Reg, dst: MemOperand) {
        self.asm.emit_rex(true, false, false, dst.base.map_or(false, |r| r.needs_rex()));
        self.asm.emit_byte(0x89);
        self.emit_mem_operand(src.encode(), dst);
    }

    /// Emit memory operand with ModR/M
    fn emit_mem_operand(&mut self, reg: u8, mem: MemOperand) {
        match (mem.base, mem.index) {
            (None, None) => {
                // [disp32]
                self.asm.emit_modrm(0, reg, 4);
                self.asm.emit_sib(0, 4, 5);
                self.asm.emit_dword(mem.disp as u32);
            }
            (Some(base), None) => {
                if mem.disp == 0 && base != X86Reg::Rbp {
                    // [base]
                    self.asm.emit_modrm(0, reg, base.encode() & 7);
                    if base == X86Reg::Rsp {
                        self.asm.emit_sib(1, 4, 4);
                    }
                } else if mem.disp >= -128 && mem.disp <= 127 {
                    // [base + disp8]
                    self.asm.emit_modrm(1, reg, base.encode() & 7);
                    if base == X86Reg::Rsp {
                        self.asm.emit_sib(1, 4, 4);
                    }
                    self.asm.emit_byte(mem.disp as u8);
                } else {
                    // [base + disp32]
                    self.asm.emit_modrm(2, reg, base.encode() & 7);
                    if base == X86Reg::Rsp {
                        self.asm.emit_sib(1, 4, 4);
                    }
                    self.asm.emit_dword(mem.disp as u32);
                }
            }
            (Some(base), Some(index)) => {
                self.asm.emit_modrm(0, reg, 4);
                self.asm.emit_sib(mem.scale, index.encode() & 7, base.encode() & 7);
                self.asm.emit_dword(mem.disp as u32);
            }
            (None, Some(index)) => {
                self.asm.emit_modrm(0, reg, 4);
                self.asm.emit_sib(mem.scale, index.encode() & 7, 5);
                self.asm.emit_dword(mem.disp as u32);
            }
        }
    }

    /// Finalize and get binary
    pub fn finalize(self) -> X86Binary {
        self.asm.finalize()
    }
}

impl Default for X86CodeGen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler_basic() {
        let mut asm = X86Asm::new();
        asm.mov_r64_imm64(X86Reg::Rax, 42);
        asm.ret();

        let binary = asm.finalize();
        assert!(!binary.code.is_empty());
        assert_eq!(binary.code[0], 0xB8); // mov rax, imm64
    }

    #[test]
    fn test_prologue_epilogue() {
        let mut gen = X86CodeGen::new();
        let frame = X86Frame::new(16, vec![X86Reg::Rbx, X86Reg::R12]);
        gen.begin_function(frame);
        gen.end_function();

        let binary = gen.finalize();
        assert!(!binary.code.is_empty());
    }

    #[test]
    fn test_label_resolution() {
        let mut asm = X86Asm::new();
        asm.label("start");
        asm.mov_r64_imm64(X86Reg::Rax, 1);
        asm.jmp_rel32(0); // Will be fixed later
        asm.label("end");

        let binary = asm.finalize();
        assert!(binary.labels.contains_key("start"));
        assert!(binary.labels.contains_key("end"));
    }
}
