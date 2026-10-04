// WebAssembly Target

use super::*;

/// WebAssembly target
pub struct WasmTarget {
    pub features: WasmFeatures,
}

#[derive(Debug, Clone)]
pub struct WasmFeatures {
    pub reference_types: bool,
    pub simd: bool,
    pub threads: bool,
    pub tail_call: bool,
    pub exceptions: bool,
    pub memory64: bool,
    pub relaxed_simd: bool,
    pub extended_const: bool,
    pub multivalue: bool,
}

impl Default for WasmFeatures {
    fn default() -> Self {
        Self {
            reference_types: true,
            simd: true,
            threads: false,
            tail_call: true,
            exceptions: false,
            memory64: false,
            relaxed_simd: false,
            extended_const: false,
            multivalue: true,
        }
    }
}

impl WasmTarget {
    pub fn new() -> Self {
        Self {
            features: WasmFeatures::default(),
        }
    }

    /// Generate WebAssembly binary
    pub fn emit_binary(&self, module: &Module) -> Result<Vec<u8>, TargetError> {
        let mut wasm = WasmBinary::new();

        // Magic number and version
        wasm.push_u32(0x0061736D); // magic
        wasm.push_u32(0x01000000); // version 1

        // Type section
        wasm.push_section_id(1);
        let type_section_start = wasm.len();
        wasm.push_u32(0); // placeholder

        let mut type_indices: HashMap<FunctionId, u32> = HashMap::new();

        for func in &module.functions {
            let type_idx = wasm.add_type(&func.params, &func.ret_type);
            type_indices.insert(func.id, type_idx);
        }

        wasm.write_section_size(type_section_start);

        // Function section
        wasm.push_section_id(3);
        let func_section_start = wasm.len();
        wasm.push_u32(0); // placeholder

        for func in &module.functions {
            if let Some(type_idx) = type_indices.get(&func.id) {
                wasm.push_u32(*type_idx);
            }
        }

        wasm.write_section_size(func_section_start);

        // Export section
        wasm.push_section_id(7);
        let export_section_start = wasm.len();
        wasm.push_u32(0); // placeholder

        for func in &module.functions {
            if func.name == "main" || func.name == "_start" {
                wasm.push_name(&func.name);
                wasm.push_byte(0); // function export
                wasm.push_u32(func.id.0 as u32);
            }
        }

        wasm.write_section_size(export_section_start);

        // Code section
        wasm.push_section_id(10);
        let code_section_start = wasm.len();
        wasm.push_u32(0); // placeholder

        for func in &module.functions {
            let code_start = wasm.len();
            wasm.push_u32(0); // placeholder

            // Generate function body
            self.emit_function_body(func, &mut wasm);

            wasm.write_function_size(code_start);
        }

        wasm.write_section_size(code_section_start);

        Ok(wasm.into_bytes())
    }

    fn emit_function_body(&self, func: &Function, wasm: &mut WasmBinary) {
        // Local declarations
        wasm.push_u32(func.params.len() as u32); // number of locals

        // Generate instructions
        for block in &func.body {
            for inst in &block.instructions {
                self.emit_instruction(inst, wasm);
            }
        }

        // End function
        wasm.push_byte(0x0B); // end
    }

    fn emit_instruction(&self, inst: &Instruction, wasm: &mut WasmBinary) {
        match &inst.op {
            Operation::Binary(BinOp::Add, _, _) => {
                wasm.push_byte(0x6A); // i32.add
            }
            Operation::Binary(BinOp::Sub, _, _) => {
                wasm.push_byte(0x6B); // i32.sub
            }
            Operation::Binary(BinOp::Mul, _, _) => {
                wasm.push_byte(0x6C); // i32.mul
            }
            Operation::Binary(BinOp::Div, _, _) => {
                wasm.push_byte(0x6D); // i32.div_s
            }
            Operation::Binary(BinOp::Eq, _, _) => {
                wasm.push_byte(0x46); // i32.eq
            }
            Operation::Binary(BinOp::Lt, _, _) => {
                wasm.push_byte(0x48); // i32.lt_s
            }
            Operation::Ret(_) => {
                wasm.push_byte(0x0F); // return
            }
            Operation::Call(func_id, _) => {
                wasm.push_byte(0x10); // call
                wasm.push_u32(func_id.0 as u32);
            }
            _ => {
                wasm.push_byte(0x01); // nop
            }
        }
    }

    /// Generate WebAssembly text format
    pub fn emit_text(&self, module: &Module) -> String {
        let mut wat = String::new();

        wat.push_str("(module\n\n");

        // Type definitions
        for func in &module.functions {
            wat.push_str("  (func ");
            wat.push_str(&format!("${}", func.name));

            // Parameters
            for (name, _) in &func.params {
                wat.push_str(&format!(" (param ${})", name));
            }

            // Return type
            if !matches!(func.ret_type, IrType::Void) {
                wat.push_str(" (result i32)");
            }

            wat.push_str("\n");

            // Generate instructions
            for block in &func.body {
                for inst in &block.instructions {
                    wat.push_str(&format!("    ; {:?}\n", inst.op));
                }
            }

            wat.push_str("  )\n\n");
        }

        // Exports
        for func in &module.functions {
            if func.name == "main" {
                wat.push_str(&format!("  (export \"main\" (func ${}))\n", func.name));
            }
        }

        wat.push_str(")\n");

        wat
    }
}

impl Target for WasmTarget {
    fn name(&self) -> &'static str {
        "wasm32"
    }

    fn triple(&self) -> &str {
        "wasm32-unknown-unknown"
    }

    fn data_layout(&self) -> DataLayout {
        DataLayout {
            endian: Endianness::Little,
            pointer_size: 4,
            pointer_abi_align: 4,
            aggregate_align: 4,
            vector_align: 16,
            function_align: 16,
            stack_align: 16,
            primitive_sizes: HashMap::new(),
        }
    }

    fn codegen(&self, function: &Function) -> Result<TargetCode, TargetError> {
        let mut module = Module::new("wasm_module");
        module.functions.push(function.clone());

        let binary = self.emit_binary(&module)?;
        let text = self.emit_text(&module);

        Ok(TargetCode {
            data: binary,
            relocations: Vec::new(),
            debug_info: DebugInfo {
                dwarf: None,
                source_map: None,
            },
        })
    }

    fn verify(&self, code: &TargetCode) -> bool {
        // Check magic number
        code.data.len() >= 8 &&
        code.data[0..4] == [0x00, 0x61, 0x73, 0x6D]
    }
}

impl Default for WasmTarget {
    fn default() -> Self {
        Self::new()
    }
}

/// WebAssembly binary writer
struct WasmBinary {
    data: Vec<u8>,
}

impl WasmBinary {
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    fn push_byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    fn push_u32(&mut self, val: u32) {
        self.data.extend_from_slice(&val.to_le_bytes());
    }

    fn push_section_id(&mut self, id: u8) {
        self.data.push(id);
    }

    fn push_name(&mut self, name: &str) {
        self.push_u32(name.len() as u32);
        self.data.extend_from_slice(name.as_bytes());
    }

    fn push_section_size(&mut self, start: usize) {
        let size = self.data.len() - start - 4;
        let bytes = (size as u32).to_le_bytes();
        self.data[start..start + 4].copy_from_slice(&bytes);
    }

    fn write_section_size(&mut self, start: usize) {
        self.push_section_size(start);
    }

    fn write_function_size(&mut self, start: usize) {
        self.push_section_size(start);
    }

    fn add_type(&mut self, _params: &[(String, IrType)], _ret: &IrType) -> u32 {
        // Simplified - returns index
        0
    }
}

impl Default for WasmBinary {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;
