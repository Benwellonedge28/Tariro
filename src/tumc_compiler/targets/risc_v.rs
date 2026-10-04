// RISC-V Target

use super::*;

/// RISC-V target
pub struct RiscVTarget {
    pub extensions: RiscVExtensions,
    pub xlen: usize,
}

#[derive(Debug, Clone)]
pub struct RiscVExtensions {
    pub m: bool, // Integer multiplication/division
    pub a: bool, // Atomic instructions
    pub f: bool, // Single precision FP
    pub d: bool, // Double precision FP
    pub c: bool, // Compressed instructions
    pub v: bool, // Vector extension
    pub b: bool, // Bit manipulation
}

impl Default for RiscVExtensions {
    fn default() -> Self {
        Self {
            m: true,
            a: true,
            f: true,
            d: true,
            c: true,
            v: false,
            b: false,
        }
    }
}

impl RiscVTarget {
    pub fn new() -> Self {
        Self {
            extensions: RiscVExtensions::default(),
            xlen: 64,
        }
    }

    /// Emit RISC-V instruction
    pub fn emit_instruction(&self, inst: &Instruction, buf: &mut Vec<u8>) {
        match &inst.op {
            Operation::Binary(BinOp::Add, lhs, rhs) => {
                // add xd, xn, xm = 0 + (rd << 7) + (rs1 << 15) + (rs2 << 20) + (funct7 << 25)
                let rd = 10; // a0
                let rs1 = 11; // a1
                let rs2 = 12; // a2
                let opcode = 0x33; // OP
                let funct3 = 0x0; // add
                let funct7 = 0x00;

                let encoding = (funct7 as u32) << 25 |
                               (rs2 as u32) << 20 |
                               (rs1 as u32) << 15 |
                               (funct3 as u32) << 12 |
                               (rd as u32) << 7 |
                               opcode;

                buf.extend_from_slice(&encoding.to_le_bytes());
                let _ = (lhs, rhs);
            }
            Operation::Ret(_) => {
                // jalr x0, x1, 0 = ret
                let rd = 0;
                let rs1 = 1; // ra (return address)
                let jalr_opcode = 0x67;
                let funct3 = 0x0;

                let encoding = (rd as u32) << 7 | (funct3 as u32) << 12 | (rs1 as u32) << 15 | (jalr_opcode as u32);
                buf.extend_from_slice(&encoding.to_le_bytes());
            }
            _ => {}
        }
    }

    /// Emit function prologue
    pub fn emit_prologue(&self, frame_size: usize, buf: &mut Vec<u8>) {
        // addi sp, sp, -frame_size
        let imm = (-(frame_size as i32)) as u32;
        let rd = 2; // sp
        let rs1 = 2; // sp
        let addi_opcode = 0x13;
        let funct3 = 0x0;

        let encoding = ((imm & 0xFFF) as u32) << 20 |
                       (rs1 as u32) << 15 |
                       (funct3 as u32) << 12 |
                       (rd as u32) << 7 |
                       addi_opcode;
        buf.extend_from_slice(&encoding.to_le_bytes());

        // Save return address if needed
        // sw ra, frame_size-4(sp)
    }

    /// Emit function epilogue
    pub fn emit_epilogue(&self, frame_size: usize, buf: &mut Vec<u8>) {
        // addi sp, sp, frame_size
        let imm = frame_size as u32;
        let rd = 2; // sp
        let rs1 = 2; // sp
        let addi_opcode = 0x13;
        let funct3 = 0x0;

        let encoding = ((imm & 0xFFF) as u32) << 20 |
                       (rs1 as u32) << 15 |
                       (funct3 as u32) << 12 |
                       (rd as u32) << 7 |
                       addi_opcode;
        buf.extend_from_slice(&encoding.to_le_bytes());

        // jalr x0, x1, 0 (ret)
        let jalr_opcode = 0x67;
        let encoding = (2u32) << 7 | 0x0 << 12 | (1u32) << 15 | jalr_opcode;
        buf.extend_from_slice(&encoding.to_le_bytes());
    }
}

impl Target for RiscVTarget {
    fn name(&self) -> &'static str {
        "riscv64"
    }

    fn triple(&self) -> &str {
        "riscv64-unknown-unknown"
    }

    fn data_layout(&self) -> DataLayout {
        DataLayout {
            endian: Endianness::Little,
            pointer_size: 8,
            pointer_abi_align: 8,
            aggregate_align: 8,
            vector_align: 16,
            function_align: 16,
            stack_align: 16,
            primitive_sizes: HashMap::new(),
        }
    }

    fn codegen(&self, function: &Function) -> Result<TargetCode, TargetError> {
        let mut code = Vec::new();

        // Emit prologue
        self.emit_prologue(0, &mut code);

        // Emit instructions
        for block in &function.body {
            for inst in &block.instructions {
                self.emit_instruction(inst, &mut code);
            }
        }

        // Emit epilogue
        self.emit_epilogue(0, &mut code);

        Ok(TargetCode {
            data: code,
            relocations: Vec::new(),
            debug_info: DebugInfo::default(),
        })
    }

    fn verify(&self, code: &TargetCode) -> bool {
        // Check that code length is multiple of 4 (RISC-V instructions are 4 bytes)
        code.data.len() % 4 == 0
    }
}

impl Default for RiscVTarget {
    fn default() -> Self {
        Self::new()
    }
}
