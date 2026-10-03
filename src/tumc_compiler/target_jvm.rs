//! TUMC JVM Bytecode Generation
//!
//! Code generation for Java Virtual Machine bytecode targeting the JVM.

use std::collections::HashMap;

/// JVM type descriptor
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JvmType {
    /// Byte (8-bit signed integer)
    Byte,
    /// Short (16-bit signed integer)
    Short,
    /// Int (32-bit signed integer)
    Int,
    /// Long (64-bit signed integer)
    Long,
    /// Float (32-bit IEEE 754)
    Float,
    /// Double (64-bit IEEE 754)
    Double,
    /// Boolean
    Boolean,
    /// Char (16-bit unsigned Unicode)
    Char,
    /// Object reference
    Object(String),
    /// Array
    Array(Box<JvmType>),
    /// Return address (for jsr/ret)
    ReturnAddress,
    /// Void
    Void,
}

impl JvmType {
    /// Get type descriptor string
    pub fn descriptor(&self) -> &'static str {
        match self {
            JvmType::Byte => "B",
            JvmType::Short => "S",
            JvmType::Int => "I",
            JvmType::Long => "J",
            JvmType::Float => "F",
            JvmType::Double => "D",
            JvmType::Boolean => "Z",
            JvmType::Char => "C",
            JvmType::Object(_) => "L",
            JvmType::Array(_) => "[",
            JvmType::ReturnAddress => "J",
            JvmType::Void => "V",
        }
    }

    /// Get stack size required
    pub fn stack_size(&self) -> u8 {
        match self {
            JvmType::Double | JvmType::Long => 2,
            _ => 1,
        }
    }

    /// Get category (1 or 2)
    pub fn category(&self) -> u8 {
        match self {
            JvmType::Double | JvmType::Long => 2,
            _ => 1,
        }
    }
}

/// JVM opcode
#[derive(Debug, Clone, Copy)]
pub enum JvmOp {
    // Constants
    AconstNull,
    IconstM1,
    Iconst0,
    Iconst1,
    Iconst2,
    Iconst3,
    Iconst4,
    Iconst5,
    Lconst0,
    Lconst1,
    Fconst0,
    Fconst1,
    Fconst2,
    Dconst0,
    Dconst1,
    Bipush(u8),
    Sipush(u16),
    Ldc(u8),
    LdcW(u16),
    Ldc2W(u8),

    // Loads
    Iload(u8),
    Lload(u8),
    Fload(u8),
    Dload(u8),
    Aload(u8),
    Iload0,
    Iload1,
    Iload2,
    Iload3,
    Lload0,
    Lload1,
    Lload2,
    Lload3,
    Fload0,
    Fload1,
    Fload2,
    Fload3,
    Dload0,
    Dload1,
    Dload2,
    Dload3,
    Aload0,
    Aload1,
    Aload2,
    Aload3,
    Iaload,
    Laload,
    Faload,
    Daload,
    Aaload,
    Baload,
    Caload,
    Saload,

    // Stores
    Istore(u8),
    Lstore(u8),
    Fstore(u8),
    Dstore(u8),
    Astore(u8),
    Istore0,
    Istore1,
    Istore2,
    Istore3,
    Lstore0,
    Lstore1,
    Lstore2,
    Lstore3,
    Fstore0,
    Fstore1,
    Fstore2,
    Fstore3,
    Dstore0,
    Dstore1,
    Dstore2,
    Dstore3,
    Astore0,
    Astore1,
    Astore2,
    Astore3,
    Iastore,
    Lastore,
    Fastore,
    Dastore,
    Aastore,
    Bastore,
    Castore,
    Sastore,

    // Stack
    Pop,
    Pop2,
    Dup,
    DupX1,
    DupX2,
    Dup2,
    Dup2X1,
    Dup2X2,
    Swap,

    // Arithmetic
    Iadd,
    Ladd,
    Fadd,
    Dadd,
    Isub,
    Lsub,
    Fsub,
    Dsub,
    Imul,
    Lmul,
    Fmul,
    Dmul,
    Idiv,
    Ldiv,
    Fdiv,
    Ddiv,
    Irem,
    Lrem,
    Frem,
    Drem,
    Ineg,
    Lneg,
    Fneg,
    Dneg,
    Ishl,
    Lshl,
    Ishr,
    Lshr,
    Iushr,
    Lushr,
    Iand,
    Land,
    Ior,
    Lor,
    Ixor,
    Lxor,
    Iinc(u8, i8),

    // Comparison
    Lcmp,
    Fcmpl,
    Fcmpg,
    Dcmpl,
    Dcmpg,
    IfEq(i16),
    IfNe(i16),
    IfLt(i16),
    IfGe(i16),
    IfGt(i16),
    IfLe(i16),
    IfIcmpEq(i16),
    IfIcmpNe(i16),
    IfIcmpLt(i16),
    IfIcmpGe(i16),
    IfIcmpGt(i16),
    IfIcmpLe(i16),

    // Control transfer
    Jmp(i16),
    JmpW(i32),
    IfNull(i16),
    IfNonNull(i16),
    IfAcmpeq(i16),
    IfAcmpne(i16),
    Ret(u8),
    Tableswitch(i8, Vec<i32>),
    Lookupswitch(i8, Vec<(i32, i32)>),

    // Returns
    Ireturn,
    Lreturn,
    Freturn,
    Dreturn,
    Areturn,
    Return,

    // Field access
    Getstatic(u16),
    Putstatic(u16),
    Getfield(u16),
    Putfield(u16),

    // Method invocation
    Invokevirtual(u16),
    Invokespecial(u16),
    Invokestatic(u16),
    Invokeinterface(u16, u8),
    Invokedynamic(u16, u8, u8),

    // Object/array
    New(u16),
    Newarray(u8),
    Anewarray(u16),
    Arraylength,
    Athrow,
    Checkcast(u16),
    Instanceof(u16),

    // Monitor
    Monitorenter,
    Monitorexit,

    // Multi-dimensional array
    Multianewarray(u16, u8),

    // Ifcmp
    IfIcmpeq(i16),
    IfIcmpne(i16),
    IfIcmplt(i16),
    IfIcmpge(i16),
    IfIcmpgt(i16),
    IfIcmple(i16),

    // Load from array
    Iconst0,
}

impl JvmOp {
    /// Encode opcode to bytes
    pub fn encode(&self) -> Vec<u8> {
        match self {
            // Single-byte opcodes
            JvmOp::AconstNull => vec![0x01],
            JvmOp::IconstM1 => vec![0x02],
            JvmOp::Iconst0 => vec![0x03],
            JvmOp::Iconst1 => vec![0x04],
            JvmOp::Iconst2 => vec![0x05],
            JvmOp::Iconst3 => vec![0x06],
            JvmOp::Iconst4 => vec![0x07],
            JvmOp::Iconst5 => vec![0x08],
            JvmOp::Lconst0 => vec![0x09],
            JvmOp::Lconst1 => vec![0x0A],
            JvmOp::Fconst0 => vec![0x0B],
            JvmOp::Fconst1 => vec![0x0C],
            JvmOp::Fconst2 => vec![0x0D],
            JvmOp::Dconst0 => vec![0x0E],
            JvmOp::Dconst1 => vec![0x0F],
            JvmOp::Pop => vec![0x57],
            JvmOp::Pop2 => vec![0x58],
            JvmOp::Dup => vec![0x59],
            JvmOp::DupX1 => vec![0x5A],
            JvmOp::DupX2 => vec![0x5B],
            JvmOp::Dup2 => vec![0x5C],
            JvmOp::Dup2X1 => vec![0x5D],
            JvmOp::Dup2X2 => vec![0x5E],
            JvmOp::Swap => vec![0x5F],
            JvmOp::Iadd => vec![0x60],
            JvmOp::Ladd => vec![0x61],
            JvmOp::Fadd => vec![0x62],
            JvmOp::Dadd => vec![0x63],
            JvmOp::Isub => vec![0x64],
            JvmOp::Lsub => vec![0x65],
            JvmOp::Fsub => vec![0x66],
            JvmOp::Dsub => vec![0x67],
            JvmOp::Imul => vec![0x68],
            JvmOp::Lmul => vec![0x69],
            JvmOp::Fmul => vec![0x6A],
            JvmOp::Dmul => vec![0x6B],
            JvmOp::Idiv => vec![0x6C],
            JvmOp::Ldiv => vec![0x6D],
            JvmOp::Fdiv => vec![0x6E],
            JvmOp::Ddiv => vec![0x6F],
            JvmOp::Irem => vec![0x70],
            JvmOp::Lrem => vec![0x71],
            JvmOp::Frem => vec![0x72],
            JvmOp::Drem => vec![0x73],
            JvmOp::Ineg => vec![0x74],
            JvmOp::Lneg => vec![0x75],
            JvmOp::Fneg => vec![0x76],
            JvmOp::Dneg => vec![0x77],
            JvmOp::Ishl => vec![0x78],
            JvmOp::Lshl => vec![0x79],
            JvmOp::Ishr => vec![0x7A],
            JvmOp::Lshr => vec![0x7B],
            JvmOp::Iushr => vec![0x7C],
            JvmOp::Lushr => vec![0x7D],
            JvmOp::Iand => vec![0x7E],
            JvmOp::Land => vec![0x7F],
            JvmOp::Ior => vec![0x80],
            JvmOp::Lor => vec![0x81],
            JvmOp::Ixor => vec![0x82],
            JvmOp::Lxor => vec![0x83],
            JvmOp::Ireturn => vec![0xAC],
            JvmOp::Lreturn => vec![0xAD],
            JvmOp::Freturn => vec![0xAE],
            JvmOp::Dreturn => vec![0xAF],
            JvmOp::Areturn => vec![0xB0],
            JvmOp::Return => vec![0xB1],
            JvmOp::Arraylength => vec![0xBE],
            JvmOp::Athrow => vec![0xBF],
            JvmOp::Monitorenter => vec![0xC2],
            JvmOp::Monitorexit => vec![0xC3],
            JvmOp::Iaload => vec![0x2E],
            JvmOp::Laload => vec![0x2F],
            JvmOp::Faload => vec![0x30],
            JvmOp::Daload => vec![0x31],
            JvmOp::Aaload => vec![0x32],
            JvmOp::Baload => vec![0x33],
            JvmOp::Caload => vec![0x34],
            JvmOp::Saload => vec![0x35],
            JvmOp::Iastore => vec![0x4F],
            JvmOp::Lastore => vec![0x50],
            JvmOp::Fastore => vec![0x51],
            JvmOp::Dastore => vec![0x52],
            JvmOp::Aastore => vec![0x53],
            JvmOp::Bastore => vec![0x54],
            JvmOp::Castore => vec![0x55],
            JvmOp::Sastore => vec![0x56],
            JvmOp::Lcmp => vec![0x94],
            JvmOp::Fcmpl => vec![0x95],
            JvmOp::Fcmpg => vec![0x96],
            JvmOp::Dcmpl => vec![0x97],
            JvmOp::Dcmpg => vec![0x98],
            JvmOp::Iload0 => vec![0x1A],
            JvmOp::Iload1 => vec![0x1B],
            JvmOp::Iload2 => vec![0x1C],
            JvmOp::Iload3 => vec![0x1D],
            JvmOp::Lload0 => vec![0x1E],
            JvmOp::Lload1 => vec![0x1F],
            JvmOp::Lload2 => vec![0x20],
            JvmOp::Lload3 => vec![0x21],
            JvmOp::Fload0 => vec![0x22],
            JvmOp::Fload1 => vec![0x23],
            JvmOp::Fload2 => vec![0x24],
            JvmOp::Fload3 => vec![0x25],
            JvmOp::Dload0 => vec![0x26],
            JvmOp::Dload1 => vec![0x27],
            JvmOp::Dload2 => vec![0x28],
            JvmOp::Dload3 => vec![0x29],
            JvmOp::Aload0 => vec![0x2A],
            JvmOp::Aload1 => vec![0x2B],
            JvmOp::Aload2 => vec![0x2C],
            JvmOp::Aload3 => vec![0x2D],
            JvmOp::Istore0 => vec![0x3B],
            JvmOp::Istore1 => vec![0x3C],
            JvmOp::Istore2 => vec![0x3D],
            JvmOp::Istore3 => vec![0x3E],
            JvmOp::Lstore0 => vec![0x3F],
            JvmOp::Lstore1 => vec![0x40],
            JvmOp::Lstore2 => vec![0x41],
            JvmOp::Lstore3 => vec![0x42],
            JvmOp::Fstore0 => vec![0x43],
            JvmOp::Fstore1 => vec![0x44],
            JvmOp::Fstore2 => vec![0x45],
            JvmOp::Fstore3 => vec![0x46],
            JvmOp::Dstore0 => vec![0x47],
            JvmOp::Dstore1 => vec![0x48],
            JvmOp::Dstore2 => vec![0x49],
            JvmOp::Dstore3 => vec![0x4A],
            JvmOp::Astore0 => vec![0x4B],
            JvmOp::Astore1 => vec![0x4C],
            JvmOp::Astore2 => vec![0x4D],
            JvmOp::Astore3 => vec![0x4E],

            // Two-byte opcodes
            JvmOp::Bipush(v) => vec![0x10, *v],
            JvmOp::Iload(v) => vec![0x15, *v],
            JvmOp::Lload(v) => vec![0x16, *v],
            JvmOp::Fload(v) => vec![0x17, *v],
            JvmOp::Dload(v) => vec![0x18, *v],
            JvmOp::Aload(v) => vec![0x19, *v],
            JvmOp::Istore(v) => vec![0x36, *v],
            JvmOp::Lstore(v) => vec![0x37, *v],
            JvmOp::Fstore(v) => vec![0x38, *v],
            JvmOp::Dstore(v) => vec![0x39, *v],
            JvmOp::Astor(v) => vec![0x3A, *v],
            JvmOp::Ret(v) => vec![0xB9, *v],
            JvmOp::Iinc(v1, v2) => vec![0x84, *v1, *v2 as u8],
            JvmOp::IfEq(v) => vec![0x99, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfNe(v) => vec![0x9A, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfLt(v) => vec![0x9B, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfGe(v) => vec![0x9C, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfGt(v) => vec![0x9D, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfLe(v) => vec![0x9E, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfIcmpEq(v) => vec![0x9F, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfIcmpNe(v) => vec![0xA0, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfIcmpLt(v) => vec![0xA1, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfIcmpGe(v) => vec![0xA2, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfIcmpGt(v) => vec![0xA3, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfIcmpLe(v) => vec![0xA4, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfNull(v) => vec![0xC7, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfNonNull(v) => vec![0xC8, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfAcmpeq(v) => vec![0xA5, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::IfAcmpne(v) => vec![0xA6, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Jmp(v) => vec![0xA7, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Ldc(v) => vec![0x12, *v],
            JvmOp::LdcW(v) => vec![0x13, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::New(v) => vec![0xBB, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Newarray(v) => vec![0xBC, *v],
            JvmOp::Anewarray(v) => vec![0xBD, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Checkcast(v) => vec![0xC0, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Instanceof(v) => vec![0xC1, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Getstatic(v) => vec![0xB2, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Putstatic(v) => vec![0xB3, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Getfield(v) => vec![0xB4, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Putfield(v) => vec![0xB5, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Invokevirtual(v) => vec![0xB6, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Invokespecial(v) => vec![0xB7, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            JvmOp::Invokestatic(v) => vec![0xB8, &v.to_le_bytes()[0], &v.to_le_bytes()[1]].iter().map(|x| *x).collect(),
            _ => vec![],
        }
    }
}

/// JVM constant pool entry
#[derive(Debug, Clone)]
pub enum ConstantPoolEntry {
    /// UTF-8 string
    Utf8(String),
    /// Integer
    Integer(i32),
    /// Float
    Float(f32),
    /// Long
    Long(i64),
    /// Double
    Double(f64),
    /// Class reference
    Class { name_index: u16 },
    /// String reference
    String { string_index: u16 },
    /// Field reference
    Fieldref { class_index: u16, name_and_type_index: u16 },
    /// Method reference
    Methodref { class_index: u16, name_and_type_index: u16 },
    /// Interface method reference
    InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
    /// Name and type descriptor
    NameAndType { name_index: u16, descriptor_index: u16 },
    /// Method handle
    MethodHandle { reference_kind: u8, reference_index: u16 },
    /// Method type
    MethodType { descriptor_index: u16 },
    /// Dynamic
    Dynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },
    /// Invoke dynamic
    InvokeDynamic { bootstrap_method_attr_index: u16, name_and_type_index: u16 },
}

/// JVM method
#[derive(Debug, Clone)]
pub struct JvmMethod {
    /// Access flags
    pub access_flags: u16,
    /// Name
    pub name: String,
    /// Descriptor
    pub descriptor: String,
    /// Attributes
    pub attributes: Vec<JvmAttribute>,
    /// Code (if present)
    pub code: Option<JvmCode>,
}

impl JvmMethod {
    /// Create new method
    pub fn new(name: &str, descriptor: &str) -> Self {
        Self {
            access_flags: 0,
            name: name.to_string(),
            descriptor: descriptor.to_string(),
            attributes: Vec::new(),
            code: None,
        }
    }

    /// Set code
    pub fn with_code(mut self, max_stack: u16, max_locals: u16, code: Vec<u8>) -> Self {
        self.code = Some(JvmCode {
            max_stack,
            max_locals,
            code,
            exception_table: Vec::new(),
            attributes: Vec::new(),
        });
        self
    }
}

/// JVM code attribute
#[derive(Debug, Clone)]
pub struct JvmCode {
    /// Maximum stack size
    pub max_stack: u16,
    /// Maximum local variables
    pub max_locals: u16,
    /// Bytecode
    pub code: Vec<u8>,
    /// Exception table
    pub exception_table: Vec<JvmExceptionEntry>,
    /// Attributes
    pub attributes: Vec<JvmAttribute>,
}

/// JVM exception table entry
#[derive(Debug, Clone)]
pub struct JvmExceptionEntry {
    /// Start PC
    pub start_pc: u16,
    /// End PC
    pub end_pc: u16,
    /// Handler PC
    pub handler_pc: u16,
    /// Catch type
    pub catch_type: u16,
}

/// JVM attribute
#[derive(Debug, Clone)]
pub struct JvmAttribute {
    /// Attribute name
    pub name: String,
    /// Data
    pub data: Vec<u8>,
}

/// JVM field
#[derive(Debug, Clone)]
pub struct JvmField {
    /// Access flags
    pub access_flags: u16,
    /// Name
    pub name: String,
    /// Descriptor
    pub descriptor: String,
    /// Attributes
    pub attributes: Vec<JvmAttribute>,
}

impl JvmField {
    /// Create new field
    pub fn new(name: &str, descriptor: &str) -> Self {
        Self {
            access_flags: 0,
            name: name.to_string(),
            descriptor: descriptor.to_string(),
            attributes: Vec::new(),
        }
    }
}

/// JVM class file builder
pub struct JvmClassBuilder {
    /// Minor version
    minor_version: u16,
    /// Major version (e.g., 52 = Java 8)
    major_version: u16,
    /// Constant pool
    constant_pool: Vec<ConstantPoolEntry>,
    /// Access flags
    access_flags: u16,
    /// This class
    this_class: u16,
    /// Super class
    super_class: u16,
    /// Interfaces
    interfaces: Vec<u16>,
    /// Fields
    fields: Vec<JvmField>,
    /// Methods
    methods: Vec<JvmMethod>,
    /// Attributes
    attributes: Vec<JvmAttribute>,
}

impl JvmClassBuilder {
    /// Create new class builder
    pub fn new(name: &str) -> Self {
        let mut builder = Self {
            minor_version: 0,
            major_version: 52, // Java 8
            constant_pool: Vec::new(),
            access_flags: 0,
            this_class: 0,
            super_class: 0,
            interfaces: Vec::new(),
            fields: Vec::new(),
            methods: Vec::new(),
            attributes: Vec::new(),
        };
        builder.add_utf8(name);
        builder
    }

    /// Add UTF-8 to constant pool
    pub fn add_utf8(&mut self, s: &str) -> u16 {
        self.constant_pool.push(ConstantPoolEntry::Utf8(s.to_string()));
        self.constant_pool.len() as u16
    }

    /// Add class reference
    pub fn add_class(&mut self, name_index: u16) -> u16 {
        self.constant_pool.push(ConstantPoolEntry::Class { name_index });
        self.constant_pool.len() as u16
    }

    /// Add string
    pub fn add_string(&mut self, string_index: u16) -> u16 {
        self.constant_pool.push(ConstantPoolEntry::String { string_index });
        self.constant_pool.len() as u16
    }

    /// Add name and type
    pub fn add_name_and_type(&mut self, name_index: u16, descriptor_index: u16) -> u16 {
        self.constant_pool.push(ConstantPoolEntry::NameAndType {
            name_index,
            descriptor_index,
        });
        self.constant_pool.len() as u16
    }

    /// Add method reference
    pub fn add_methodref(&mut self, class_index: u16, name_and_type_index: u16) -> u16 {
        self.constant_pool
            .push(ConstantPoolEntry::Methodref {
                class_index,
                name_and_type_index,
            });
        self.constant_pool.len() as u16
    }

    /// Add field reference
    pub fn add_fieldref(&mut self, class_index: u16, name_and_type_index: u16) -> u16 {
        self.constant_pool
            .push(ConstantPoolEntry::Fieldref {
                class_index,
                name_and_type_index,
            });
        self.constant_pool.len() as u16
    }

    /// Set access flags
    pub fn set_access_flags(&mut self, flags: u16) {
        self.access_flags = flags;
    }

    /// Set this class
    pub fn set_this_class(&mut self, index: u16) {
        self.this_class = index;
    }

    /// Set super class
    pub fn set_super_class(&mut self, index: u16) {
        self.super_class = index;
    }

    /// Add method
    pub fn add_method(&mut self, method: JvmMethod) {
        self.methods.push(method);
    }

    /// Add field
    pub fn add_field(&mut self, field: JvmField) {
        self.fields.push(field);
    }

    /// Build class file
    pub fn build(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Magic
        bytes.extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]);

        // Version
        bytes.extend_from_slice(&self.minor_version.to_be_bytes());
        bytes.extend_from_slice(&self.major_version.to_be_bytes());

        // Constant pool count (includes null entry)
        bytes.extend_from_slice(&(self.constant_pool.len() as u16 + 1).to_be_bytes());

        // Constant pool entries
        for entry in &self.constant_pool {
            Self::write_constant_pool_entry(&mut bytes, entry);
        }

        // Access flags
        bytes.extend_from_slice(&self.access_flags.to_be_bytes());

        // This class
        bytes.extend_from_slice(&self.this_class.to_be_bytes());

        // Super class
        bytes.extend_from_slice(&self.super_class.to_be_bytes());

        // Interfaces count
        bytes.extend_from_slice(&(self.interfaces.len() as u16).to_be_bytes());

        // Interfaces
        for iface in &self.interfaces {
            bytes.extend_from_slice(&iface.to_be_bytes());
        }

        // Fields count
        bytes.extend_from_slice(&(self.fields.len() as u16).to_be_bytes());

        // Fields
        for field in &self.fields {
            bytes.extend_from_slice(&field.access_flags.to_be_bytes());
            let name_idx = self.constant_pool.iter().position(|e| {
                if let ConstantPoolEntry::Utf8(s) = e {
                    s == &field.name
                } else {
                    false
                }
            }).unwrap_or(0) as u16 + 1;
            let desc_idx = self.constant_pool.iter().position(|e| {
                if let ConstantPoolEntry::Utf8(s) = e {
                    s == &field.descriptor
                } else {
                    false
                }
            }).unwrap_or(0) as u16 + 1;
            bytes.extend_from_slice(&name_idx.to_be_bytes());
            bytes.extend_from_slice(&desc_idx.to_be_bytes());
            bytes.extend_from_slice(&(field.attributes.len() as u16).to_be_bytes());
        }

        // Methods count
        bytes.extend_from_slice(&(self.methods.len() as u16).to_be_bytes());

        // Methods
        for method in &self.methods {
            Self::write_method(&mut bytes, method);
        }

        // Attributes count
        bytes.extend_from_slice(&(self.attributes.len() as u16).to_be_bytes());

        bytes
    }

    /// Write constant pool entry
    fn write_constant_pool_entry(bytes: &mut Vec<u8>, entry: &ConstantPoolEntry) {
        match entry {
            ConstantPoolEntry::Utf8(s) => {
                bytes.push(0x01);
                let s_bytes = s.as_bytes();
                bytes.extend_from_slice(&(s_bytes.len() as u16).to_be_bytes());
                bytes.extend_from_slice(s_bytes);
            }
            ConstantPoolEntry::Integer(i) => {
                bytes.push(0x04);
                bytes.extend_from_slice(&i.to_be_bytes());
            }
            ConstantPoolEntry::Float(f) => {
                bytes.push(0x04);
                bytes.extend_from_slice(&f.to_be_bytes());
            }
            ConstantPoolEntry::Long(l) => {
                bytes.push(0x05);
                bytes.extend_from_slice(&l.to_be_bytes());
            }
            ConstantPoolEntry::Double(d) => {
                bytes.push(0x06);
                bytes.extend_from_slice(&d.to_be_bytes());
            }
            ConstantPoolEntry::Class { name_index } => {
                bytes.push(0x07);
                bytes.extend_from_slice(&name_index.to_be_bytes());
            }
            ConstantPoolEntry::String { string_index } => {
                bytes.push(0x08);
                bytes.extend_from_slice(&string_index.to_be_bytes());
            }
            ConstantPoolEntry::Fieldref {
                class_index,
                name_and_type_index,
            } => {
                bytes.push(0x09);
                bytes.extend_from_slice(&class_index.to_be_bytes());
                bytes.extend_from_slice(&name_and_type_index.to_be_bytes());
            }
            ConstantPoolEntry::Methodref {
                class_index,
                name_and_type_index,
            } => {
                bytes.push(0x0A);
                bytes.extend_from_slice(&class_index.to_be_bytes());
                bytes.extend_from_slice(&name_and_type_index.to_be_bytes());
            }
            ConstantPoolEntry::InterfaceMethodref {
                class_index,
                name_and_type_index,
            } => {
                bytes.push(0x0B);
                bytes.extend_from_slice(&class_index.to_be_bytes());
                bytes.extend_from_slice(&name_and_type_index.to_be_bytes());
            }
            ConstantPoolEntry::NameAndType {
                name_index,
                descriptor_index,
            } => {
                bytes.push(0x0C);
                bytes.extend_from_slice(&name_index.to_be_bytes());
                bytes.extend_from_slice(&descriptor_index.to_be_bytes());
            }
            _ => {}
        }
    }

    /// Write method
    fn write_method(bytes: &mut Vec<u8>, method: &JvmMethod) {
        bytes.extend_from_slice(&method.access_flags.to_be_bytes());
        let name_idx = 1u16; // Would need proper lookup
        let desc_idx = 2u16;
        bytes.extend_from_slice(&name_idx.to_be_bytes());
        bytes.extend_from_slice(&desc_idx.to_be_bytes());

        if let Some(code) = &method.code {
            bytes.extend_from_slice(&1u16.to_be_bytes()); // 1 attribute
            // SourceFile attribute
            bytes.extend_from_slice(&3u16.to_be_bytes()); // "Code" index
            let attr_len = 12 + code.code.len();
            bytes.extend_from_slice(&(attr_len as u32).to_be_bytes());
            bytes.extend_from_slice(&code.max_stack.to_be_bytes());
            bytes.extend_from_slice(&code.max_locals.to_be_bytes());
            bytes.extend_from_slice(&(code.code.len() as u32).to_be_bytes());
            bytes.extend_from_slice(&code.code);
            bytes.extend_from_slice(&(code.exception_table.len() as u16).to_be_bytes());
        } else {
            bytes.extend_from_slice(&0u16.to_be_bytes()); // No attributes
        }
    }
}

/// JVM bytecode emitter
pub struct JvmEmitter {
    /// Bytecode buffer
    code: Vec<u8>,
    /// Labels
    labels: HashMap<String, usize>,
    /// Pending jumps
    pending_jumps: Vec<(String, usize)>,
}

impl JvmEmitter {
    /// Create new emitter
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            labels: HashMap::new(),
            pending_jumps: Vec::new(),
        }
    }

    /// Emit opcode
    pub fn emit(&mut self, op: JvmOp) {
        self.code.extend_from_slice(&op.encode());
    }

    /// Define label
    pub fn label(&mut self, name: &str) {
        let pos = self.code.len();
        self.labels.insert(name.to_string(), pos);

        // Resolve pending jumps
        let to_resolve: Vec<_> = self
            .pending_jumps
            .iter()
            .filter(|(n, _)| *n == name)
            .cloned()
            .collect();

        for (_, offset) in to_resolve {
            let delta = pos as i32 - offset as i32 - 1;
            if delta >= i16::MIN as i32 && delta <= i16::MAX as i32 {
                self.code[offset] = (delta >> 8) as u8;
                self.code[offset + 1] = delta as u8;
            }
        }
    }

    /// Get current position
    pub fn pos(&self) -> usize {
        self.code.len()
    }

    /// Load constant
    pub fn load_const(&mut self, val: i32) {
        match val {
            -1 => self.emit(JvmOp::IconstM1),
            0 => self.emit(JvmOp::Iconst0),
            1 => self.emit(JvmOp::Iconst1),
            2 => self.emit(JvmOp::Iconst2),
            3 => self.emit(JvmOp::Iconst3),
            4 => self.emit(JvmOp::Iconst4),
            5 => self.emit(JvmOp::Iconst5),
            v if v >= i8::MIN as i32 && v <= i8::MAX as i32 => {
                self.emit(JvmOp::Bipush(v as u8));
            }
            v if v >= i16::MIN as i32 && v <= i16::MAX as i32 => {
                self.emit(JvmOp::Sipush(v as u16));
            }
            _ => {
                self.emit(JvmOp::Ldc((self.pos() + 1) as u8));
            }
        }
    }

    /// Return
    pub fn ret(&mut self, ty: &JvmType) {
        match ty {
            JvmType::Void => self.emit(JvmOp::Return),
            JvmType::Int | JvmType::Byte | JvmType::Short | JvmType::Char | JvmType::Boolean => {
                self.emit(JvmOp::Ireturn)
            }
            JvmType::Long => self.emit(JvmOp::Lreturn),
            JvmType::Float => self.emit(JvmOp::Freturn),
            JvmType::Double => self.emit(JvmOp::Dreturn),
            _ => self.emit(JvmOp::Areturn),
        }
    }

    /// Get bytecode
    pub fn finalize(self) -> Vec<u8> {
        self.code
    }
}

impl Default for JvmEmitter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_constants() {
        let mut emitter = JvmEmitter::new();
        emitter.load_const(0);
        emitter.load_const(42);
        emitter.load_const(300);

        let code = emitter.finalize();
        assert!(!code.is_empty());
    }

    #[test]
    fn test_label_definition() {
        let mut emitter = JvmEmitter::new();
        emitter.label("start");
        emitter.emit(JvmOp::Iconst0);
        emitter.label("end");

        assert!(emitter.labels.contains_key("start"));
        assert!(emitter.labels.contains_key("end"));
    }

    #[test]
    fn test_class_builder() {
        let mut builder = JvmClassBuilder::new("Test");
        builder.set_access_flags(0x21); // PUBLIC | SUPER
        let class_idx = builder.add_utf8("Test");
        let this_idx = builder.add_class(class_idx);
        builder.set_this_class(this_idx);
        builder.set_super_class(0); // java/lang/Object

        let mut method = JvmMethod::new("main", "([Ljava/lang/String;)V");
        let code = vec![0xB1]; // RETURN
        method = method.with_code(1, 2, code);
        builder.add_method(method);

        let class_data = builder.build();
        assert_eq!(&class_data[0..4], &[0xCA, 0xFE, 0xBA, 0xBE]);
    }
}
