// JVM Target

use super::*;

/// JVM bytecode target
pub struct JvmTarget {
    pub version: JvmVersion,
}

#[derive(Debug, Clone, Copy)]
pub enum JvmVersion {
    Java8,
    Java11,
    Java17,
    Java21,
}

impl Default for JvmVersion {
    fn default() -> Self {
        JvmVersion::Java11
    }
}

impl JvmTarget {
    pub fn new() -> Self {
        Self {
            version: JvmVersion::default(),
        }
    }

    /// Generate JVM class file
    pub fn emit_class(&self, class: &JvmClass) -> Result<Vec<u8>, TargetError> {
        let mut bytecode = JvmBytecode::new();

        // Magic number
        bytecode.push_u32(0xCAFEBABE);

        // Version
        bytecode.push_u16(0x0000); // minor
        bytecode.push_u16(match self.version {
            JvmVersion::Java8 => 0x0034,
            JvmVersion::Java11 => 0x0037,
            JvmVersion::Java17 => 0x003B,
            JvmVersion::Java21 => 0x003F,
        });

        // Constant pool
        let cp_start = bytecode.len();
        bytecode.push_u16(0); // placeholder

        let mut constant_pool = Vec::new();
        self.build_constant_pool(class, &mut constant_pool, &mut bytecode);

        bytecode.write_u16_at(cp_start, (constant_pool.len() + 1) as u16);

        // Access flags
        bytecode.push_u16(class.access_flags);

        // This class
        bytecode.push_u16(1); // index to CONSTANT_Class

        // Super class
        bytecode.push_u16(2); // index to CONSTANT_Class for java/lang/Object

        // Interfaces
        bytecode.push_u16(0); // count

        // Fields
        bytecode.push_u16(0); // count

        // Methods
        bytecode.push_u16(class.methods.len() as u16);
        for method in &class.methods {
            self.emit_method(method, &mut bytecode);
        }

        // Attributes
        bytecode.push_u16(0); // count

        Ok(bytecode.into_bytes())
    }

    fn build_constant_pool(
        &self,
        class: &JvmClass,
        _pool: &mut Vec<ConstantPoolEntry>,
        bytecode: &mut JvmBytecode,
    ) {
        // CONSTANT_Class for this class
        bytecode.push_u8(7); // CONSTANT_Class
        bytecode.push_u16(3); // name index

        // CONSTANT_Class for java/lang/Object
        bytecode.push_u8(7);
        bytecode.push_u16(4); // name index

        // CONSTANT_Utf8 for class name
        bytecode.push_u8(1); // CONSTANT_Utf8
        bytecode.push_u16(class.name.len() as u16);
        bytecode.extend_from_slice(class.name.as_bytes());

        // CONSTANT_Utf8 for java/lang/Object
        bytecode.push_u8(1);
        bytecode.push_u16(16);
        bytecode.extend_from_slice(b"java/lang/Object".as_bytes());
    }

    fn emit_method(&self, method: &JvmMethod, bytecode: &mut JvmBytecode) {
        // Access flags
        bytecode.push_u16(method.access_flags);

        // Name index
        bytecode.push_u16(5); // would resolve from constant pool

        // Descriptor index
        bytecode.push_u16(6);

        // Attributes count
        bytecode.push_u16(1);

        // Code attribute
        bytecode.push_u16(1); // Code attribute name index
        let code_start = bytecode.len();
        bytecode.push_u32(0); // attribute length placeholder

        // Max stack
        bytecode.push_u16(4);

        // Max locals
        bytecode.push_u16(method.locals);

        // Code length
        bytecode.push_u32(method.code.len() as u32);

        // Code
        bytecode.extend_from_slice(&method.code);

        // Exception table
        bytecode.push_u16(0);

        // Attributes count
        bytecode.push_u16(0);

        bytecode.write_u32_at(code_start, (bytecode.len() - code_start - 4) as u32);
    }

    /// Generate bytecode for instruction
    pub fn emit_instruction(&self, inst: &Instruction, buf: &mut Vec<u8>) {
        match &inst.op {
            Operation::Binary(BinOp::Add, _, _) => {
                if matches!(inst.result_type, IrType::Int32) {
                    buf.push(0x60); // iadd
                } else {
                    buf.push(0x94); // ladd
                }
            }
            Operation::Binary(BinOp::Sub, _, _) => {
                buf.push(0x64); // isub
            }
            Operation::Binary(BinOp::Mul, _, _) => {
                buf.push(0x68); // imul
            }
            Operation::Binary(BinOp::Div, _, _) => {
                buf.push(0x6C); // idiv
            }
            Operation::Ret(_) => {
                buf.push(0xAC); // ireturn
            }
            Operation::Load(_, ptr, ty, _) => {
                buf.push(0x2E); // aload_0 (simplified)
                let _ = (ptr, ty);
            }
            Operation::Call(func_id, _) => {
                buf.push(0xB8); // invokestatic
                buf.push_u16(func_id.0 as u16);
            }
            _ => {}
        }
    }
}

impl Target for JvmTarget {
    fn name(&self) -> &'static str {
        "jvm"
    }

    fn triple(&self) -> &str {
        "jvm-unknown-unknown"
    }

    fn data_layout(&self) -> DataLayout {
        DataLayout {
            endian: Endianness::Big,
            pointer_size: 8,
            pointer_abi_align: 8,
            aggregate_align: 8,
            vector_align: 16,
            function_align: 8,
            stack_align: 8,
            primitive_sizes: HashMap::new(),
        }
    }

    fn codegen(&self, function: &Function) -> Result<TargetCode, TargetError> {
        let mut code = Vec::new();
        for block in &function.body {
            for inst in &block.instructions {
                self.emit_instruction(inst, &mut code);
            }
        }

        let mut method = JvmMethod::new(&function.name);
        method.code = code;

        let mut class = JvmClass::new(&function.name);
        class.methods.push(method);

        let class_bytes = self.emit_class(&class)?;

        Ok(TargetCode {
            data: class_bytes,
            relocations: Vec::new(),
            debug_info: DebugInfo::default(),
        })
    }

    fn verify(&self, code: &TargetCode) -> bool {
        code.data.len() >= 8 && code.data[0..4] == [0xCA, 0xFE, 0xBA, 0xBE]
    }
}

impl Default for JvmTarget {
    fn default() -> Self {
        Self::new()
    }
}

/// JVM class representation
#[derive(Debug, Clone)]
pub struct JvmClass {
    pub name: String,
    pub access_flags: u16,
    pub fields: Vec<JvmField>,
    pub methods: Vec<JvmMethod>,
}

impl JvmClass {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            access_flags: 0x0021, // ACC_PUBLIC | ACC_SUPER
            fields: Vec::new(),
            methods: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct JvmField {
    pub name: String,
    pub descriptor: String,
    pub access_flags: u16,
}

#[derive(Debug, Clone)]
pub struct JvmMethod {
    pub name: String,
    pub descriptor: String,
    pub access_flags: u16,
    pub code: Vec<u8>,
    pub locals: u16,
}

impl JvmMethod {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            descriptor: String::new(),
            access_flags: 0x0001, // ACC_PUBLIC
            code: Vec::new(),
            locals: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ConstantPoolEntry {
    Class(String),
    FieldRef(String, String, String),
    MethodRef(String, String, String),
    InterfaceMethodRef(String, String, String),
    String(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NameAndType(String, String),
    Utf8(String),
}

/// JVM bytecode writer
struct JvmBytecode {
    data: Vec<u8>,
}

impl JvmBytecode {
    fn new() -> Self {
        Self {
            data: Vec::new(),
        }
    }

    fn push_u8(&mut self, val: u8) {
        self.data.push(val);
    }

    fn push_u16(&mut self, val: u16) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    fn push_u32(&mut self, val: u32) {
        self.data.extend_from_slice(&val.to_be_bytes());
    }

    fn extend_from_slice(&mut self, slice: &[u8]) {
        self.data.extend_from_slice(slice);
    }

    fn write_u16_at(&mut self, offset: usize, val: u16) {
        self.data[offset..offset + 2].copy_from_slice(&val.to_be_bytes());
    }

    fn write_u32_at(&mut self, offset: usize, val: u32) {
        self.data[offset..offset + 4].copy_from_slice(&val.to_be_bytes());
    }

    fn into_bytes(self) -> Vec<u8> {
        self.data
    }
}

impl Default for JvmBytecode {
    fn default() -> Self {
        Self::new()
    }
}

use std::collections::HashMap;
