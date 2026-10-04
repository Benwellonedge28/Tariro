//! TUMC .NET/CLR Code Generation
//!
//! Code generation targeting the .NET Common Language Runtime (CLR).

use std::collections::HashMap;

/// CLR type representation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClrType {
    /// Boolean
    Boolean,
    /// Char
    Char,
    /// Int8
    Int8,
    /// UInt8
    UInt8,
    /// Int16
    Int16,
    /// UInt16
    UInt16,
    /// Int32
    Int32,
    /// UInt32
    UInt32,
    /// Int64
    Int64,
    /// UInt64
    UInt64,
    /// Float32
    Float32,
    /// Float64
    Float64,
    /// Decimal
    Decimal,
    /// String
    String,
    /// Object
    Object,
    /// Void
    Void,
    /// Typed reference
    TypedReference,
    /// IntPtr
    IntPtr,
    /// UIntPtr
    UIntPtr,
    /// Array
    Array(Box<ClrType>),
    /// Pointer
    Pointer(Box<ClrType>),
    /// By reference
    ByRef(Box<ClrType>),
    /// Custom type
    Custom(String),
}

impl ClrType {
    /// Get CLR type descriptor
    pub fn descriptor(&self) -> String {
        match self {
            ClrType::Boolean => "bool".to_string(),
            ClrType::Char => "char".to_string(),
            ClrType::Int8 => "int8".to_string(),
            ClrType::UInt8 => "uint8".to_string(),
            ClrType::Int16 => "int16".to_string(),
            ClrType::UInt16 => "uint16".to_string(),
            ClrType::Int32 => "int32".to_string(),
            ClrType::UInt32 => "uint32".to_string(),
            ClrType::Int64 => "int64".to_string(),
            ClrType::UInt64 => "uint64".to_string(),
            ClrType::Float32 => "float32".to_string(),
            ClrType::Float64 => "float64".to_string(),
            ClrType::Decimal => "decimal".to_string(),
            ClrType::String => "string".to_string(),
            ClrType::Object => "object".to_string(),
            ClrType::Void => "void".to_string(),
            ClrType::TypedReference => "typedref".to_string(),
            ClrType::IntPtr => "intptr".to_string(),
            ClrType::UIntPtr => "uintptr".to_string(),
            ClrType::Array(t) => format!("{}[]", t.descriptor()),
            ClrType::Pointer(t) => format!("{}*", t.descriptor()),
            ClrType::ByRef(t) => format!("{}&", t.descriptor()),
            ClrType::Custom(name) => name.clone(),
        }
    }

    /// Get CLI type code
    pub fn type_code(&self) -> u8 {
        match self {
            ClrType::Boolean => 2,
            ClrType::Char => 3,
            ClrType::Int8 => 4,
            ClrType::UInt8 => 5,
            ClrType::Int16 => 6,
            ClrType::UInt16 => 7,
            ClrType::Int32 => 8,
            ClrType::UInt32 => 9,
            ClrType::Int64 => 10,
            ClrType::UInt64 => 11,
            ClrType::Float32 => 12,
            ClrType::Float64 => 13,
            ClrType::Decimal => 14,
            ClrType::DateTime => 16,
            ClrType::String => 18,
            _ => 0,
        }
    }
}

/// IL instruction opcodes
#[derive(Debug, Clone, Copy)]
pub enum IlOpcode {
    // Stack operations
    Nop,
    Break,
   Ldarg(u16),
    Ldarga(u16),
    Starg(u16),
    Ldloc(u16),
    Ldloca(u16),
    Stloc(u16),
    Ldnull,
    LdcI4(i32),
    LdcI8(i64),
    LdcR4(f32),
    LdcR8(f64),
    LdcI4M1,
    LdcI40,
    LdcI41,
    LdcI42,
    LdcI43,
    LdcI44,
    LdcI45,
    LdcI46,
    LdcI47,
    LdcI48,
    LdcI4S(i8),

    // Binary operations
    Add,
    AddOvf,
    AddOvfUn,
    Sub,
    SubOvf,
    SubOvfUn,
    Mul,
    MulOvf,
    MulOvfUn,
    Div,
    DivUn,
    Rem,
    RemUn,

    // Logical operations
    And,
    Or,
    Xor,
    Not,
    Neg,

    // Shifts
    Shl,
    Shr,
    ShrUn,

    // Conversions
    ConvI1,
    ConvI2,
    ConvI4,
    ConvI8,
    ConvR4,
    ConvR8,
    ConvU4,
    ConvU8,
    ConvU,
    ConvRUn,

    // Comparisons
    Ceq,
    Cgt,
    CgtUn,
    Clt,
    CltUn,

    // Branching
    Br(Label),
    BrS(Label),
    BrTrue(Label),
    BrFalse(Label),
    Beq(Label),
    Bge(Label),
    Bgt(Label),
    Ble(Label),
    Blt(Label),
    BneUn(Label),
    BgeUn(Label),
    BgtUn(Label),
    BleUn(Label),
    BltUn(Label),
    Switch(Vec<Label>),

    // Return
    Ret,
    Endfinally,

    // Memory
    LdindI1,
    LdindU1,
    LdindI2,
    LdindU2,
    LdindI4,
    LdindU4,
    LdindI8,
    LdindR4,
    LdindR8,
    LdindRef,
    StindI1,
    StindI2,
    StindI4,
    StindI8,
    StindR4,
    StindR8,
    StindRef,

    // Object model
    Ldobj(String),
    Stobj(String),
    Ldlen,
    Ldelema(String),
    LdelemI1,
    LdelemU1,
    LdelemI2,
    LdelemU2,
    LdelemI4,
    LdelemU4,
    LdelemI8,
    LdelemR4,
    LdelemR8,
    LdelemRef,
    StelemI1,
    StelemI2,
    StelemI4,
    StelemI8,
    StelemR4,
    StelemR8,
    StelemRef,

    // Methods
    Call(String),
    Callvirt(String),
    Calli(String),
    Newobj(String),
    Ldftn(String),

    // Fields
    Ldfld(String),
    Ldflda(String),
    Stfld(String),
    Ldsfld(String),
    Ldsflda(String),
    Stsfld(String),

    // Array creation
    Newarr(String),
    Initobj(String),
    Initblk,

    // Exception handling
    Throw,
    Rethrow,
    Leave(Label),
    Endfault,

    // Other
    Localloc,
    Unaligned(u8),
    Volatile,
    Tail,
    Constrained(String),
    Readonly,
}

impl IlOpcode {
    /// Get opcode bytes
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            IlOpcode::Nop => vec![0xFE, 0x00],
            IlOpcode::Break => vec![0xCD],
            IlOpcode::Ret => vec![0x2A],
            IlOpcode::Ldnull => vec![0x14],
            IlOpcode::Add => vec![0x58],
            IlOpcode::Sub => vec![0x59],
            IlOpcode::Mul => vec![0x5A],
            IlOpcode::Div => vec![0x5B],
            IlOpcode::Rem => vec![0x5D],
            IlOpcode::And => vec![0x5F],
            IlOpcode::Or => vec![0x60],
            IlOpcode::Xor => vec![0x61],
            IlOpcode::Not => vec![0x67],
            IlOpcode::Neg => vec![0x68],
            IlOpcode::Shl => vec![0x62],
            IlOpcode::Shr => vec![0x63],
            IlOpcode::ShrUn => vec![0x64],
            IlOpcode::Ceq => vec![0xFE, 0x01],
            IlOpcode::Cgt => vec![0xFE, 0x02],
            IlOpcode::CgtUn => vec![0xFE, 0x03],
            IlOpcode::Clt => vec![0xFE, 0x04],
            IlOpcode::CltUn => vec![0xFE, 0x05],
            IlOpcode::Ldlen => vec![0x8E],
            IlOpcode::Throw => vec![0x7A],
            IlOpcode::Pop => vec![0x26],
            IlOpcode::Dup => vec![0x25],
            IlOpcode::Localloc => vec![0xFE, 0x0D],
            _ => vec![0x00],
        }
    }
}

/// Label for branching
#[derive(Debug, Clone, Copy)]
pub struct Label(usize);

/// IL assembler
pub struct IlAssembler {
    /// Instructions
    instructions: Vec<IlOpcode>,
    /// Labels
    labels: HashMap<String, usize>,
    /// Current position
    pos: usize,
    /// Constants
    constants: Vec<IlConstant>,
    /// Strings
    strings: Vec<String>,
}

impl IlAssembler {
    /// Create new assembler
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            labels: HashMap::new(),
            pos: 0,
            constants: Vec::new(),
            strings: Vec::new(),
        }
    }

    /// Define label
    pub fn define_label(&mut self, name: &str) {
        self.labels.insert(name.to_string(), self.pos);
    }

    /// Emit instruction
    pub fn emit(&mut self, op: IlOpcode) {
        self.instructions.push(op);
        self.pos += op.bytes().len();
    }

    /// Load argument
    pub fn ldarg(&mut self, index: u16) {
        if index <= 255 {
            self.emit(IlOpcode::Ldarg(index as u8 as u16));
        }
    }

    /// Load local
    pub fn ldloc(&mut self, index: u16) {
        self.emit(IlOpcode::Ldloc(index));
    }

    /// Store local
    pub fn stloc(&mut self, index: u16) {
        self.emit(IlOpcode::Stloc(index));
    }

    /// Load constant integer
    pub fn ldc_i4(&mut self, value: i32) {
        match value {
            -1 => self.emit(IlOpcode::LdcI4M1),
            0 => self.emit(IlOpcode::LdcI40),
            1 => self.emit(IlOpcode::LdcI41),
            2 => self.emit(IlOpcode::LdcI42),
            3 => self.emit(IlOpcode::LdcI43),
            4 => self.emit(IlOpcode::LdcI44),
            5 => self.emit(IlOpcode::LdcI45),
            6 => self.emit(IlOpcode::LdcI46),
            7 => self.emit(IlOpcode::LdcI47),
            8 => self.emit(IlOpcode::LdcI48),
            v if v >= i8::MIN as i32 && v <= i8::MAX as i32 => {
                self.emit(IlOpcode::LdcI4S(v as i8));
            }
            _ => self.emit(IlOpcode::LdcI4(value)),
        }
    }

    /// Load constant integer 64
    pub fn ldc_i8(&mut self, value: i64) {
        self.emit(IlOpcode::LdcI8(value));
    }

    /// Load constant float
    pub fn ldc_r4(&mut self, value: f32) {
        self.emit(IlOpcode::LdcR4(value));
    }

    /// Load constant double
    pub fn ldc_r8(&mut self, value: f64) {
        self.emit(IlOpcode::LdcR8(value));
    }

    /// Binary operations
    pub fn add(&mut self) { self.emit(IlOpcode::Add); }
    pub fn sub(&mut self) { self.emit(IlOpcode::Sub); }
    pub fn mul(&mut self) { self.emit(IlOpcode::Mul); }
    pub fn div(&mut self) { self.emit(IlOpcode::Div); }
    pub fn rem(&mut self) { self.emit(IlOpcode::Rem); }

    /// Logical operations
    pub fn and(&mut self) { self.emit(IlOpcode::And); }
    pub fn or(&mut self) { self.emit(IlOpcode::Or); }
    pub fn xor(&mut self) { self.emit(IlOpcode::Xor); }
    pub fn not(&mut self) { self.emit(IlOpcode::Not); }
    pub fn neg(&mut self) { self.emit(IlOpcode::Neg); }

    /// Comparisons
    pub fn ceq(&mut self) { self.emit(IlOpcode::Ceq); }
    pub fn cgt(&mut self) { self.emit(IlOpcode::Cgt); }
    pub fn clt(&mut self) { self.emit(IlOpcode::Clt); }

    /// Branches
    pub fn br(&mut self, label: &str) {
        let target = self.labels.get(label).copied().unwrap_or(0);
        self.emit(IlOpcode::Br(Label(target)));
    }

    pub fn br_if(&mut self, label: &str) {
        let target = self.labels.get(label).copied().unwrap_or(0);
        self.emit(IlOpcode::BrTrue(Label(target)));
    }

    pub fn br_if_false(&mut self, label: &str) {
        let target = self.labels.get(label).copied().unwrap_or(0);
        self.emit(IlOpcode::BrFalse(Label(target)));
    }

    /// Return
    pub fn ret(&mut self) {
        self.emit(IlOpcode::Ret);
    }

    /// Call method
    pub fn call(&mut self, method: &str) {
        self.emit(IlOpcode::Call(method.to_string()));
    }

    /// Call virtual method
    pub fn callvirt(&mut self, method: &str) {
        self.emit(IlOpcode::Callvirt(method.to_string()));
    }

    /// Create new object
    pub fn newobj(&mut self, ctor: &str) {
        self.emit(IlOpcode::Newobj(ctor.to_string()));
    }

    /// Load field
    pub fn ldfld(&mut self, field: &str) {
        self.emit(IlOpcode::Ldfld(field.to_string()));
    }

    /// Store field
    pub fn stfld(&mut self, field: &str) {
        self.emit(IlOpcode::Stfld(field.to_string()));
    }

    /// Load string
    pub fn ldstr(&mut self, s: &str) -> usize {
        if let Some(idx) = self.strings.iter().position(|x| x == s) {
            return idx;
        }
        self.strings.push(s.to_string());
        self.strings.len() - 1
    }

    /// Finalize
    pub fn finalize(self) -> IlModule {
        let mut code = Vec::new();
        for inst in &self.instructions {
            code.extend_from_slice(&inst.bytes());
        }

        IlModule {
            code,
            strings: self.strings,
            constants: self.constants,
        }
    }
}

impl Default for IlAssembler {
    fn default() -> Self {
        Self::new()
    }
}

/// IL constant
#[derive(Debug, Clone)]
pub enum IlConstant {
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(String),
    Null,
}

/// IL module
#[derive(Debug, Clone)]
pub struct IlModule {
    pub code: Vec<u8>,
    pub strings: Vec<String>,
    pub constants: Vec<IlConstant>,
}

/// .NET metadata
#[derive(Debug, Clone)]
pub struct ClrMetadata {
    pub version: String,
    pub assembly_name: String,
    pub assembly_version: String,
    pub types: Vec<ClrTypeDef>,
}

/// CLR type definition
#[derive(Debug, Clone)]
pub struct ClrTypeDef {
    pub name: String,
    pub namespace: String,
    pub flags: u32,
    pub base_type: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<ClrField>,
    pub methods: Vec<ClrMethod>,
}

/// CLR field
#[derive(Debug, Clone)]
pub struct ClrField {
    pub name: String,
    pub field_type: ClrType,
    pub flags: u32,
}

/// CLR method
#[derive(Debug, Clone)]
pub struct ClrMethod {
    pub name: String,
    pub return_type: ClrType,
    pub parameters: Vec<(String, ClrType)>,
    pub flags: u32,
    pub body: Option<IlModule>,
}

/// .NET code generator
pub struct ClrCodeGen {
    /// Assembler
    asm: IlAssembler,
    /// Metadata
    metadata: ClrMetadata,
    /// Local variables
    locals: Vec<(String, ClrType)>,
}

impl ClrCodeGen {
    /// Create new code generator
    pub fn new(assembly_name: &str) -> Self {
        Self {
            asm: IlAssembler::new(),
            metadata: ClrMetadata {
                version: "v4.0".to_string(),
                assembly_name: assembly_name.to_string(),
                assembly_version: "1.0.0.0".to_string(),
                types: Vec::new(),
            },
            locals: Vec::new(),
        }
    }

    /// Define method entry
    pub fn begin_method(&mut self, name: &str, return_type: ClrType, params: Vec<(String, ClrType)>) {
        // Method prologue
        self.locals.clear();
    }

    /// End method
    pub fn end_method(&mut self) {
        self.asm.ret();
    }

    /// Load argument
    pub fn ldarg(&mut self, name: &str) {
        let idx = self.locals.iter().position(|(n, _)| n == name);
        if let Some(i) = idx {
            self.asm.ldarg(i as u16);
        }
    }

    /// Load local
    pub fn ldloc(&mut self, name: &str) {
        let idx = self.locals.iter().position(|(n, _)| n == name);
        if let Some(i) = idx {
            self.asm.ldloc(i as u16);
        }
    }

    /// Store local
    pub fn stloc(&mut self, name: &str) {
        let idx = self.locals.iter().position(|(n, _)| n == name);
        if let Some(i) = idx {
            self.asm.stloc(i as u16);
        } else {
            self.locals.push((name.to_string(), ClrType::Object));
            let i = self.locals.len() - 1;
            self.asm.stloc(i as u16);
        }
    }

    /// Call method
    pub fn call(&mut self, method: &str) {
        self.asm.call(method);
    }

    /// Return value
    pub fn ret(&mut self) {
        self.asm.ret();
    }

    /// Add method to type
    pub fn add_method(&mut self, method: ClrMethod) {
        if let Some(typ) = self.metadata.types.last_mut() {
            typ.methods.push(method);
        }
    }

    /// Finalize code generator
    pub fn finalize(self) -> ClrMetadata {
        self.metadata
    }
}

/// CIL instruction stream
pub struct CilStream {
    data: Vec<u8>,
}

impl CilStream {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn write_u8(&mut self, v: u8) {
        self.data.push(v);
    }

    pub fn write_u16(&mut self, v: u16) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u32(&mut self, v: u32) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_u64(&mut self, v: u64) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_i32(&mut self, v: i32) {
        self.data.extend_from_slice(&v.to_le_bytes());
    }

    pub fn write_blob(&mut self, data: &[u8]) {
        let len = data.len() as u32;
        self.write_u32(len);
        self.data.extend_from_slice(data);
    }

    pub fn write_string(&mut self, s: &str) {
        let bytes = s.as_bytes();
        self.write_u32(bytes.len() as u32 + 1);
        self.data.push(b'"');
        self.data.extend_from_slice(bytes);
        self.data.push(b'"');
    }

    pub fn write_guid(&mut self) {
        self.data.extend_from_slice(&[0u8; 16]);
    }

    pub fn finalize(self) -> Vec<u8> {
        self.data
    }
}

impl Default for CilStream {
    fn default() -> Self {
        Self::new()
    }
}

/// .NET portable executable header
pub struct PortableExecutable {
    /// DOS header
    dos_header: Vec<u8>,
    /// PE header
    pe_header: Vec<u8>,
    /// Optional header
    optional_header: Vec<u8>,
    /// Section headers
    sections: Vec<SectionHeader>,
    /// Section data
    section_data: HashMap<String, Vec<u8>>,
}

/// Section header
#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub name: String,
    pub virtual_size: u32,
    pub virtual_address: u32,
    pub raw_size: u32,
    pub raw_offset: u32,
    pub characteristics: u32,
}

impl PortableExecutable {
    /// Create new PE
    pub fn new() -> Self {
        Self {
            dos_header: vec![0; 64],
            pe_header: Vec::new(),
            optional_header: Vec::new(),
            sections: Vec::new(),
            section_data: HashMap::new(),
        }
    }

    /// Add section
    pub fn add_section(&mut self, name: &str, data: Vec<u8>) {
        self.section_data.insert(name.to_string(), data);
    }

    /// Generate PE file
    pub fn generate(mut self) -> Vec<u8> {
        let mut output = Vec::new();

        // DOS header
        output.extend_from_slice(&self.dos_header);
        output.extend_from_slice(b"PE\0\0");

        // PE header (simplified)
        output.extend_from_slice(&[0x14, 0x00]); // Machine (x86)
        output.extend_from_slice(&[0x02, 0x00]); // Number of sections
        output.extend_from_slice(&[0xE0, 0x00]); // Timestamp
        output.extend_from_slice(&[0x00, 0x00]); // Symbol table
        output.extend_from_slice(&[0x00, 0x00]); // Symbols
        output.extend_from_slice(&[0xE0, 0x00]); // Optional header size
        output.extend_from_slice(&[0x02, 0x01]); // Characteristics

        // Section headers and data
        let mut offset = (output.len() + self.sections.len() * 40) as u32;
        for (name, data) in &self.section_data {
            let header = SectionHeader {
                name: name.clone(),
                virtual_size: data.len() as u32,
                virtual_address: offset,
                raw_size: data.len() as u32,
                raw_offset: offset,
                characteristics: 0x40000040,
            };

            // Section header
            let mut name_bytes = name.bytes().take(8).collect::<Vec<_>>();
            name_bytes.resize(8, 0);
            output.extend_from_slice(&name_bytes);
            output.extend_from_slice(&header.virtual_size.to_le_bytes());
            output.extend_from_slice(&header.virtual_address.to_le_bytes());
            output.extend_from_slice(&header.raw_size.to_le_bytes());
            output.extend_from_slice(&header.raw_offset.to_le_bytes());
            output.extend_from_slice(&[0u8; 12]);
            output.extend_from_slice(&header.characteristics.to_le_bytes());

            // Section data
            output.extend_from_slice(data);

            // Align to 512 bytes
            while output.len() % 512 != 0 {
                output.push(0);
            }

            offset += data.len() as u32;
        }

        output
    }
}

impl Default for PortableExecutable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_il_assembler() {
        let mut asm = IlAssembler::new();
        asm.ldc_i4(42);
        asm.ret();

        let module = asm.finalize();
        assert!(!module.code.is_empty());
    }

    #[test]
    fn test_binary_ops() {
        let mut asm = IlAssembler::new();
        asm.ldc_i4(1);
        asm.ldc_i4(2);
        asm.add();
        asm.ret();

        let module = asm.finalize();
        assert!(module.code.contains(&0x58)); // add opcode
    }

    #[test]
    fn test_labels() {
        let mut asm = IlAssembler::new();
        asm.define_label("start");
        asm.ldc_i4(1);
        asm.br("start");
        asm.ret();

        assert!(asm.labels.contains_key("start"));
    }

    #[test]
    fn test_clr_code_gen() {
        let mut gen = ClrCodeGen::new("TestAssembly");
        gen.begin_method("Add", ClrType::Int32, vec![
            ("a".to_string(), ClrType::Int32),
            ("b".to_string(), ClrType::Int32),
        ]);
        gen.ldarg("a");
        gen.ldarg("b");
        gen.call("int32 Add(int32, int32)");
        gen.ret();
    }

    #[test]
    fn test_pe_generation() {
        let mut pe = PortableExecutable::new();
        pe.add_section(".text", vec![0xCC; 256]);
        let data = pe.generate();
        assert!(!data.is_empty());
    }
}
