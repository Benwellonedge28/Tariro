//! TUMC ARM64 (AArch64) Native Code Generation
//!
//! Code generation for ARM64 architecture with NEON/SVE support.

use std::collections::HashMap;

/// ARM64 register (64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arm64Reg {
    // General purpose 64-bit
    X0, X1, X2, X3, X4, X5, X6, X7,
    X8, X9, X10, X11, X12, X13, X14, X15,
    X16, X17, X18, X19, X20, X21, X22, X23,
    X24, X25, X26, X27, X28, X29, X30, Xsp, Xzr,
    // 32-bit general purpose
    W0, W1, W2, W3, W4, W5, W6, W7,
    W8, W9, W10, W11, W12, W13, W14, W15,
    W16, W17, W18, W19, W20, W21, W22, W23,
    W24, W25, W26, W27, W28, W29, W30, Wsp, Wzr,
    // SIMD/Floating point
    B0, B1, B2, B3, B4, B5, B6, B7,
    B8, B9, B10, B11, B12, B13, B14, B15,
    B16, B17, B18, B19, B20, B21, B22, B23,
    B24, B25, B26, B27, B28, B29, B30, B31,
    H0, H1, H2, H3, H4, H5, H6, H7,
    H8, H9, H10, H11, H12, H13, H14, H15,
    H16, H17, H18, H19, H20, H21, H22, H23,
    H24, H25, H26, H27, H28, H29, H30, H31,
    S0, S1, S2, S3, S4, S5, S6, S7,
    S8, S9, S10, S11, S12, S13, S14, S15,
    S16, S17, S18, S19, S20, S21, S22, S23,
    S24, S25, S26, S27, S28, S29, S30, S31,
    D0, D1, D2, D3, D4, D5, D6, D7,
    D8, D9, D10, D11, D12, D13, D14, D15,
    D16, D17, D18, D19, D20, D21, D22, D23,
    D24, D25, D26, D27, D28, D29, D30, D31,
    Q0, Q1, Q2, Q3, Q4, Q5, Q6, Q7,
    Q8, Q9, Q10, Q11, Q12, Q13, Q14, Q15,
    Q16, Q17, Q18, Q19, Q20, Q21, Q22, Q23,
    Q24, Q25, Q26, Q27, Q28, Q29, Q30, Q31,
}

impl Arm64Reg {
    /// Get encoding (0-31)
    pub fn encode(&self) -> u8 {
        match self {
            // 64-bit registers
            Arm64Reg::X0  | Arm64Reg::W0  => 0,
            Arm64Reg::X1  | Arm64Reg::W1  => 1,
            Arm64Reg::X2  | Arm64Reg::W2  => 2,
            Arm64Reg::X3  | Arm64Reg::W3  => 3,
            Arm64Reg::X4  | Arm64Reg::W4  => 4,
            Arm64Reg::X5  | Arm64Reg::W5  => 5,
            Arm64Reg::X6  | Arm64Reg::W6  => 6,
            Arm64Reg::X7  | Arm64Reg::W7  => 7,
            Arm64Reg::X8  | Arm64Reg::W8  => 8,
            Arm64Reg::X9  | Arm64Reg::W9  => 9,
            Arm64Reg::X10 | Arm64Reg::W10 => 10,
            Arm64Reg::X11 | Arm64Reg::W11 => 11,
            Arm64Reg::X12 | Arm64Reg::W12 => 12,
            Arm64Reg::X13 | Arm64Reg::W13 => 13,
            Arm64Reg::X14 | Arm64Reg::W14 => 14,
            Arm64Reg::X15 | Arm64Reg::W15 => 15,
            Arm64Reg::X16 | Arm64Reg::W16 => 16,
            Arm64Reg::X17 | Arm64Reg::W17 => 17,
            Arm64Reg::X18 | Arm64Reg::W18 => 18,
            Arm64Reg::X19 | Arm64Reg::W19 => 19,
            Arm64Reg::X20 | Arm64Reg::W20 => 20,
            Arm64Reg::X21 | Arm64Reg::W21 => 21,
            Arm64Reg::X22 | Arm64Reg::W22 => 22,
            Arm64Reg::X23 | Arm64Reg::W23 => 23,
            Arm64Reg::X24 | Arm64Reg::W24 => 24,
            Arm64Reg::X25 | Arm64Reg::W25 => 25,
            Arm64Reg::X26 | Arm64Reg::W26 => 26,
            Arm64Reg::X27 | Arm64Reg::W27 => 27,
            Arm64Reg::X28 | Arm64Reg::W28 => 28,
            Arm64Reg::X29 | Arm64Reg::W29 => 29,
            Arm64Reg::X30 | Arm64Reg::W30 => 30,
            Arm64Reg::Xsp | Arm64Reg::Wsp => 31,
            Arm64Reg::Xzr | Arm64Reg::Wzr => 31,
            // SIMD registers
            _ => 0,
        }
    }

    /// Check if this is a 64-bit register
    pub fn is_64bit(&self) -> bool {
        matches!(self, Arm64Reg::X0..=Arm64Reg::Xzr)
    }

    /// Get 64-bit counterpart
    pub fn to_64bit(&self) -> Self {
        match self {
            Arm64Reg::W0  => Arm64Reg::X0,
            Arm64Reg::W1  => Arm64Reg::X1,
            Arm64Reg::W2  => Arm64Reg::X2,
            Arm64Reg::W3  => Arm64Reg::X3,
            Arm64Reg::W4  => Arm64Reg::X4,
            Arm64Reg::W5  => Arm64Reg::X5,
            Arm64Reg::W6  => Arm64Reg::X6,
            Arm64Reg::W7  => Arm64Reg::X7,
            Arm64Reg::W8  => Arm64Reg::X8,
            Arm64Reg::W9  => Arm64Reg::X9,
            Arm64Reg::W10 => Arm64Reg::X10,
            Arm64Reg::W11 => Arm64Reg::X11,
            Arm64Reg::W12 => Arm64Reg::X12,
            Arm64Reg::W13 => Arm64Reg::X13,
            Arm64Reg::W14 => Arm64Reg::X14,
            Arm64Reg::W15 => Arm64Reg::X15,
            Arm64Reg::W16 => Arm64Reg::X16,
            Arm64Reg::W17 => Arm64Reg::X17,
            Arm64Reg::W18 => Arm64Reg::X18,
            Arm64Reg::W19 => Arm64Reg::X19,
            Arm64Reg::W20 => Arm64Reg::X20,
            Arm64Reg::W21 => Arm64Reg::X21,
            Arm64Reg::W22 => Arm64Reg::X22,
            Arm64Reg::W23 => Arm64Reg::X23,
            Arm64Reg::W24 => Arm64Reg::X24,
            Arm64Reg::W25 => Arm64Reg::X25,
            Arm64Reg::W26 => Arm64Reg::X26,
            Arm64Reg::W27 => Arm64Reg::X27,
            Arm64Reg::W28 => Arm64Reg::X28,
            Arm64Reg::W29 => Arm64Reg::X29,
            Arm64Reg::W30 => Arm64Reg::X30,
            Arm64Reg::Wsp => Arm64Reg::Xsp,
            Arm64Reg::Wzr => Arm64Reg::Xzr,
            _ => *self,
        }
    }
}

/// ARM64 condition flags
#[derive(Debug, Clone, Copy)]
pub enum Cond {
    Eq,  // Equal
    Ne,  // Not equal
    Cs,  // Carry set (unsigned higher or same)
    Cc,  // Carry clear (unsigned lower)
    Mi,  // Minus (negative)
    Pl,  // Plus (positive or zero)
    Vs,  // Overflow
    Vc,  // No overflow
    Hi,  // Unsigned higher
    Ls,  // Unsigned lower or same
    Ge,  // Signed greater than or equal
    Lt,  // Signed less than
    Gt,  // Signed greater than
    Le,  // Signed less than or equal
    Al,  // Always
}

impl Cond {
    /// Encode as 4-bit field
    pub fn encode(&self) -> u8 {
        match self {
            Cond::Eq => 0b0000,
            Cond::Ne => 0b0001,
            Cond::Cs => 0b0010,
            Cond::Cc => 0b0011,
            Cond::Mi => 0b0100,
            Cond::Pl => 0b0101,
            Cond::Vs => 0b0110,
            Cond::Vc => 0b0111,
            Cond::Hi => 0b1000,
            Cond::Ls => 0b1001,
            Cond::Ge => 0b1010,
            Cond::Lt => 0b1011,
            Cond::Gt => 0b1100,
            Cond::Le => 0b1101,
            Cond::Al => 0b1110,
        }
    }

    /// Negate condition
    pub fn negate(&self) -> Self {
        match self {
            Cond::Eq => Cond::Ne,
            Cond::Ne => Cond::Eq,
            Cond::Cs => Cond::Cc,
            Cond::Cc => Cond::Cs,
            Cond::Mi => Cond::Pl,
            Cond::Pl => Cond::Mi,
            Cond::Vs => Cond::Vc,
            Cond::Vc => Cond::Vs,
            Cond::Hi => Cond::Ls,
            Cond::Ls => Cond::Hi,
            Cond::Ge => Cond::Lt,
            Cond::Lt => Cond::Ge,
            Cond::Gt => Cond::Le,
            Cond::Le => Cond::Gt,
            Cond::Al => Cond::Al,
        }
    }
}

/// Memory operand (simplified)
#[derive(Debug, Clone)]
pub struct Arm64Mem {
    /// Base register
    pub base: Arm64Reg,
    /// Immediate offset
    pub offset: i32,
    /// Pre-index
    pub pre_index: bool,
    /// Post-index
    pub post_index: bool,
}

impl Arm64Mem {
    /// Create new memory operand
    pub fn new(base: Arm64Reg, offset: i32) -> Self {
        Self {
            base,
            offset,
            pre_index: false,
            post_index: false,
        }
    }

    /// Pre-index variant
    pub fn pre_indexed(mut self) -> Self {
        self.pre_index = true;
        self
    }

    /// Post-index variant
    pub fn post_indexed(mut self) -> Self {
        self.post_index = true;
        self
    }
}

/// Shift types
#[derive(Debug, Clone, Copy)]
pub enum Shift {
    Lsl, // Logical shift left
    Lsr, // Logical shift right
    Asr, // Arithmetic shift right
    Ror, // Rotate right
}

impl Shift {
    pub fn encode(&self) -> u8 {
        match self {
            Shift::Lsl => 0b00,
            Shift::Lsr => 0b01,
            Shift::Asr => 0b10,
            Shift::Ror => 0b11,
        }
    }
}

/// Extend types
#[derive(Debug, Clone, Copy)]
pub enum Extend {
    Uxtb, // Unsigned extend byte
    Uxth, // Unsigned extend halfword
    Uxtw, // Unsigned extend word
    Uxtx, // Unsigned extend word (64-bit)
    Sxtb, // Signed extend byte
    Sxth, // Signed extend halfword
    Sxtw, // Signed extend word
    Sxtx, // Signed extend word (64-bit)
}

impl Extend {
    pub fn encode(&self) -> u8 {
        match self {
            Extend::Uxtb => 0b000,
            Extend::Uxth => 0b001,
            Extend::Uxtw => 0b010,
            Extend::Uxtx => 0b011,
            Extend::Sxtb => 0b100,
            Extend::Sxth => 0b101,
            Extend::Sxtw => 0b110,
            Extend::Sxtx => 0b111,
        }
    }
}

/// ARM64 assembler
pub struct Arm64Asm {
    /// Output buffer
    code: Vec<u32>,
    /// Labels
    labels: HashMap<String, usize>,
    /// Relocations
    relocations: Vec<Arm64Relocation>,
}

impl Arm64Asm {
    /// Create new assembler
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            labels: HashMap::new(),
            relocations: Vec::new(),
        }
    }

    /// Emit 32-bit instruction
    fn emit(&mut self, inst: u32) {
        self.code.push(inst.to_le());
    }

    /// Define label
    pub fn label(&mut self, name: &str) {
        self.labels.insert(name.to_string(), self.code.len());
    }

    /// Add relocation
    pub fn add_relocation(&mut self, reloc: Arm64Relocation) {
        self.relocations.push(reloc);
    }

    // ==== Data Processing Instructions ====

    /// ADD (immediate)
    pub fn add_imm(&mut self, rd: Arm64Reg, rn: Arm64Reg, imm: u16, shift: Option<(Shift, u8)>) {
        let sf = if rd.is_64bit() { 1 } else { 0 };
        let (imm12, shift_encode, shift_imm) = shift
            .map(|(s, i)| (imm as u32, s.encode() as u32, i as u32))
            .unwrap_or((imm as u32, 0, 0));

        let mut inst = 0u32;
        inst |= (sf as u32) << 31;            // sf
        inst |= 0b100100 << 20;               // opc
        inst |= (shift_encode as u32) << 22;  // shift
        inst |= (imm12 & 0xFFF) << 10;        // imm12
        inst |= (rn.encode() as u32) << 5;    // Rn
        inst |= rd.encode() as u32;           // Rd
        self.emit(inst);
    }

    /// SUB (immediate)
    pub fn sub_imm(&mut self, rd: Arm64Reg, rn: Arm64Reg, imm: u16, shift: Option<(Shift, u8)>) {
        let sf = if rd.is_64bit() { 1 } else { 0 };
        let (imm12, shift_encode, shift_imm) = shift
            .map(|(s, i)| (imm as u32, s.encode() as u32, i as u32))
            .unwrap_or((imm as u32, 0, 0));

        let mut inst = 0u32;
        inst |= (sf as u32) << 31;            // sf
        inst |= 0b101100 << 20;               // opc
        inst |= (shift_encode as u32) << 22;  // shift
        inst |= (imm12 & 0xFFF) << 10;        // imm12
        inst |= (rn.encode() as u32) << 5;    // Rn
        inst |= rd.encode() as u32;            // Rd
        self.emit(inst);
    }

    /// MOV (register)
    pub fn mov(&mut self, rd: Arm64Reg, rm: Arm64Reg) {
        if rd.is_64bit() && rm.is_64bit() {
            self.orr_shifted(rd, Arm64Reg::Xzr, rm, Shift::Lsl, 0);
        } else {
            self.orr_shifted_32(rd, Arm64Reg::Wzr, rm, Shift::Lsl, 0);
        }
    }

    /// ORR (shifted register)
    pub fn orr_shifted(&mut self, rd: Arm64Reg, rn: Arm64Reg, rm: Arm64Reg, shift: Shift, imm: u8) {
        let mut inst = 0u32;
        inst |= 1 << 31;                      // sf
        inst |= 0b011010 << 24;               // opcode
        inst |= (shift.encode() as u32) << 22; // shift
        inst |= (imm as u32) << 10;            // imm6
        inst |= (rn.encode() as u32) << 5;     // Rn
        inst |= (rm.encode() as u32) << 16;    // Rm
        inst |= rd.encode() as u32;            // Rd
        self.emit(inst);
    }

    /// ORR (32-bit shifted register)
    pub fn orr_shifted_32(&mut self, rd: Arm64Reg, rn: Arm64Reg, rm: Arm64Reg, shift: Shift, imm: u8) {
        let mut inst = 0u32;
        inst |= 0b011010 << 24;               // opcode
        inst |= (shift.encode() as u32) << 22; // shift
        inst |= (imm as u32) << 10;            // imm6
        inst |= (rn.encode() as u32) << 5;     // Rn
        inst |= (rm.encode() as u32) << 16;    // Rm
        inst |= rd.encode() as u32;            // Rd
        self.emit(inst);
    }

    /// AND (register)
    pub fn and_reg(&mut self, rd: Arm64Reg, rn: Arm64Reg, rm: Arm64Reg) {
        let sf = if rd.is_64bit() { 1 } else { 0 };
        let mut inst = 0u32;
        inst |= (sf as u32) << 31;
        inst |= 0b000100 << 24;
        inst |= (rn.encode() as u32) << 5;
        inst |= (rm.encode() as u32) << 16;
        inst |= rd.encode() as u32;
        self.emit(inst);
    }

    /// EOR (register)
    pub fn eor_reg(&mut self, rd: Arm64Reg, rn: Arm64Reg, rm: Arm64Reg) {
        let sf = if rd.is_64bit() { 1 } else { 0 };
        let mut inst = 0u32;
        inst |= (sf as u32) << 31;
        inst |= 0b100100 << 24;
        inst |= (rn.encode() as u32) << 5;
        inst |= (rm.encode() as u32) << 16;
        inst |= rd.encode() as u32;
        self.emit(inst);
    }

    // ==== Load/Store Instructions ====

    /// LDR (immediate, unsigned offset)
    pub fn ldr_imm(&mut self, rt: Arm64Reg, rn: Arm64Reg, offset: u16) {
        let sf = if rt.is_64bit() { 1 } else { 0 };
        let imm12 = offset / if rt.is_64bit() { 8 } else { 4 };

        let mut inst = 0u32;
        inst |= (sf as u32) << 31;
        inst |= 0b11100 << 24;
        inst |= (imm12 as u32) << 10;
        inst |= (rn.encode() as u32) << 5;
        inst |= rt.encode() as u32;
        self.emit(inst);
    }

    /// LDR (register)
    pub fn ldr_reg(&mut self, rt: Arm64Reg, rn: Arm64Reg, rm: Arm64Reg) {
        let sf = if rt.is_64bit() { 1 } else { 0 };
        let mut inst = 0u32;
        inst |= (sf as u32) << 31;
        inst |= 0b11100 << 24;
        inst |= 0b010000 << 16;
        inst |= (rn.encode() as u32) << 5;
        inst |= (rm.encode() as u32) << 16;
        inst |= rt.encode() as u32;
        self.emit(inst);
    }

    /// STR (immediate, unsigned offset)
    pub fn str_imm(&mut self, rt: Arm64Reg, rn: Arm64Reg, offset: u16) {
        let sf = if rt.is_64bit() { 1 } else { 0 };
        let imm12 = offset / if rt.is_64bit() { 8 } else { 4 };

        let mut inst = 0u32;
        inst |= (sf as u32) << 31;
        inst |= 0b11100 << 24;
        inst |= 0b01 << 22;
        inst |= (imm12 as u32) << 10;
        inst |= (rn.encode() as u32) << 5;
        inst |= rt.encode() as u32;
        self.emit(inst);
    }

    // ==== Branch Instructions ====

    /// B (unconditional branch)
    pub fn b(&mut self, target: i32) {
        let offset = ((target as i64 - 4) / 4) as u32 & 0x03FF_FFFF;
        let mut inst = 0u32;
        inst |= 0b000101 << 26;
        inst |= offset;
        self.emit(inst);
    }

    /// B.cond (conditional branch)
    pub fn b_cond(&mut self, cond: Cond, target: i32) {
        let offset = ((target as i64 - 4) / 4) as u32 & 0x7FF;
        let mut inst = 0u32;
        inst |= 0b010101 << 26;
        inst |= 0 << 24;
        inst |= (cond.encode() as u32) << 0;
        inst |= (offset & 0x7FFF) << 5;
        inst |= ((offset >> 15) & 0x1F) << 19;
        self.emit(inst);
    }

    /// BL (branch with link)
    pub fn bl(&mut self, target: i32) {
        let offset = ((target as i64 - 4) / 4) as u32 & 0x03FF_FFFF;
        let mut inst = 0u32;
        inst |= 0b100101 << 26;
        inst |= offset;
        self.emit(inst);
    }

    /// BR (branch to register)
    pub fn br(&mut self, rn: Arm64Reg) {
        let mut inst = 0u32;
        inst |= 1 << 31;                      // sf
        inst |= 0b1101011 << 23;
        inst |= 0b00000 << 16;
        inst |= (rn.encode() as u32) << 5;
        self.emit(inst);
    }

    /// RET
    pub fn ret(&mut self) {
        let mut inst = 0u32;
        inst |= 1 << 31;                      // sf
        inst |= 0b1101011 << 23;
        inst |= 0b00000 << 16;
        inst |= 0b11111 << 5;                 // X30
        self.emit(inst);
    }

    // ==== Compare Instructions ====

    /// CMP (register)
    pub fn cmp_reg(&mut self, rn: Arm64Reg, rm: Arm64Reg) {
        let sf = if rn.is_64bit() { 1 } else { 0 };
        let mut inst = 0u32;
        inst |= (sf as u32) << 31;
        inst |= 0b100100 << 24;
        inst |= (rn.encode() as u32) << 5;
        inst |= (rm.encode() as u32) << 16;
        inst |= 0b11111;                      // Rd = XZR/WZR
        self.emit(inst);
    }

    // ==== Stack Instructions ====

    /// STP (pair store)
    pub fn stp(&mut self, rt1: Arm64Reg, rt2: Arm64Reg, rn: Arm64Reg, offset: i32) {
        let sf = if rt1.is_64bit() { 1 } else { 0 };
        let (imm7, pre_index, post_index) = if offset >= 0 {
            ((offset / 8) as u32, 0b01, false)
        } else {
            (((-offset) / 8) as u32, 0b11, false)
        };

        let mut inst = 0u32;
        inst |= (sf as u32) << 31;
        inst |= 0b10100 << 26;
        inst |= (pre_index as u32) << 23;
        inst |= (imm7 & 0x7F) << 15;
        inst |= (rt2.encode() as u32) << 10;
        inst |= (rn.encode() as u32) << 5;
        inst |= rt1.encode() as u32;
        self.emit(inst);
    }

    /// LDP (pair load)
    pub fn ldp(&mut self, rt1: Arm64Reg, rt2: Arm64Reg, rn: Arm64Reg, offset: i32) {
        let sf = if rt1.is_64bit() { 1 } else { 0 };
        let (imm7, pre_index, post_index) = if offset >= 0 {
            ((offset / 8) as u32, 0b01, false)
        } else {
            (((-offset) / 8) as u32, 0b11, false)
        };

        let mut inst = 0u32;
        inst |= (sf as u32) << 31;
        inst |= 0b10101 << 26;
        inst |= (pre_index as u32) << 23;
        inst |= (imm7 & 0x7F) << 15;
        inst |= (rt2.encode() as u32) << 10;
        inst |= (rn.encode() as u32) << 5;
        inst |= rt1.encode() as u32;
        self.emit(inst);
    }

    // ==== NOP and Breakpoint ====

    /// NOP
    pub fn nop(&mut self) {
        self.emit(0xD503201F);
    }

    /// BRK (breakpoint)
    pub fn brk(&mut self, imm: u16) {
        let mut inst = 0u32;
        inst |= 0b11010100 << 24;
        inst |= 0b00001 << 19;
        inst |= (imm as u32) << 5;
        self.emit(inst);
    }

    /// SVC (system call)
    pub fn svc(&mut self, imm: u16) {
        let mut inst = 0u32;
        inst |= 0b11010100 << 24;
        inst |= 0b00000 << 19;
        inst |= (imm as u32) << 5;
        self.emit(inst);
    }

    /// Finalize and get binary
    pub fn finalize(self) -> Arm64Binary {
        Arm64Binary {
            code: self.code,
            labels: self.labels,
            relocations: self.relocations,
        }
    }
}

impl Default for Arm64Asm {
    fn default() -> Self {
        Self::new()
    }
}

/// ARM64 relocation
#[derive(Debug, Clone)]
pub struct Arm64Relocation {
    pub offset: usize,
    pub kind: Arm64RelocKind,
    pub symbol: String,
    pub addend: i64,
}

/// ARM64 relocation kind
#[derive(Debug, Clone, Copy)]
pub enum Arm64RelocKind {
    /// Absolute 64-bit
    Absolute64,
    /// PC-relative 19-bit
    Pcrel19,
    /// PC-relative 26-bit (for branches)
    Pcrel26,
    /// ADRP (page-aligned 21-bit)
    Adrp21,
    /// ADD (12-bit immediate)
    Add12,
}

/// Generated ARM64 binary
#[derive(Debug, Clone)]
pub struct Arm64Binary {
    /// Machine code (as 32-bit instructions)
    pub code: Vec<u32>,
    /// Label offsets (in instruction count)
    pub labels: HashMap<String, usize>,
    /// Relocations
    pub relocations: Vec<Arm64Relocation>,
}

impl Arm64Binary {
    /// Get the machine code as bytes
    pub fn code(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.code.len() * 4);
        for &inst in &self.code {
            bytes.extend_from_slice(&inst.to_le_bytes());
        }
        bytes
    }

    /// Convert to big-endian (for cross-compilation)
    pub fn to_big_endian(&self) -> Self {
        Self {
            code: self.code.iter().map(|&i| i.to_be()).collect(),
            labels: self.labels.clone(),
            relocations: self.relocations.clone(),
        }
    }
}

/// ARM64 function frame layout
#[derive(Debug, Clone)]
pub struct Arm64Frame {
    /// Frame size
    pub size: usize,
    /// Callee-saved registers
    pub saved_regs: Vec<Arm64Reg>,
}

impl Arm64Frame {
    /// Create new frame
    pub fn new(locals: usize, saved_regs: Vec<Arm64Reg>) -> Self {
        let saved_size = saved_regs.len() * 8;
        let frame_size = locals + saved_size + 16; // +16 for potential 16-byte alignment
        Self {
            size: (frame_size + 15) & !15, // 16-byte align
            saved_regs,
        }
    }
}

/// ARM64 code generator
pub struct Arm64CodeGen {
    /// Assembler
    asm: Arm64Asm,
    /// Current frame
    frame: Option<Arm64Frame>,
}

impl Arm64CodeGen {
    /// Create new code generator
    pub fn new() -> Self {
        Self {
            asm: Arm64Asm::new(),
            frame: None,
        }
    }

    /// Begin function
    pub fn begin_function(&mut self, frame: Arm64Frame) {
        self.frame = Some(frame.clone());

        // STP X29, X30, [SP, #-16]!
        self.asm.stp(Arm64Reg::X29, Arm64Reg::X30, Arm64Reg::Xsp, -16);

        // MOV X29, SP
        self.asm.mov(Arm64Reg::X29, Arm64Reg::Xsp);

        // Allocate frame
        if frame.size > 0 {
            self.asm.sub_imm(Arm64Reg::Xsp, Arm64Reg::Xsp, (frame.size as u16), Some((Shift::Lsl, 0)));
        }

        // Save callee-saved registers
        for (i, reg) in frame.saved_regs.iter().enumerate() {
            let offset = 16 + i * 8;
            self.asm.str_imm(*reg, Arm64Reg::X29, offset as u16);
        }
    }

    /// End function
    pub fn end_function(&mut self) {
        // Restore callee-saved registers
        if let Some(ref frame) = self.frame {
            for (i, reg) in frame.saved_regs.iter().enumerate() {
                let offset = 16 + i * 8;
                self.asm.ldr_imm(*reg, Arm64Reg::X29, offset as u16);
            }
        }

        // MOV SP, X29
        self.asm.mov(Arm64Reg::Xsp, Arm64Reg::X29);

        // LDP X29, X30, [SP], #16
        self.asm.ldp(Arm64Reg::X29, Arm64Reg::X30, Arm64Reg::Xsp, 16);

        // RET
        self.asm.ret();
    }

    /// Finalize and get binary
    pub fn finalize(self) -> Arm64Binary {
        self.asm.finalize()
    }
}

impl Default for Arm64CodeGen {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_instruction() {
        let mut asm = Arm64Asm::new();
        asm.add_imm(Arm64Reg::X0, Arm64Reg::X1, 42, None);

        let binary = asm.finalize();
        assert_eq!(binary.code.len(), 1);
    }

    #[test]
    fn test_function_prologue() {
        let mut gen = Arm64CodeGen::new();
        let frame = Arm64Frame::new(32, vec![Arm64Reg::X19, Arm64Reg::X20]);
        gen.begin_function(frame);
        gen.end_function();

        let binary = gen.finalize();
        assert!(binary.code.len() >= 4);
    }

    #[test]
    fn test_label_resolution() {
        let mut asm = Arm64Asm::new();
        asm.label("start");
        asm.mov(Arm64Reg::X0, Arm64Reg::Xzr);
        asm.b_cond(Cond::Al, 0);
        asm.label("end");

        let binary = asm.finalize();
        assert!(binary.labels.contains_key("start"));
        assert!(binary.labels.contains_key("end"));
    }
}
