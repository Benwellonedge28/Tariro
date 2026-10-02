//! DWARF Debug Information Generation for TUMC
//! Generates DWARF format debug info for debugging support

use crate::prelude::*;
use super::ast::*;
use std::collections::{HashMap, HashSet};

/// DWARF version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DwarfVersion {
    /// DWARF 2 (legacy)
    V2,
    /// DWARF 3
    V3,
    /// DWARF 4 (common)
    V4,
    /// DWARF 5
    V5,
}

impl Default for DwarfVersion {
    fn default() -> Self {
        DwarfVersion::V4
    }
}

/// DWARF section
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DwarfSection {
    /// .debug_abbrev
    Abbrev,
    /// .debug_info
    Info,
    /// .debug_line
    Line,
    /// .debug_frame
    Frame,
    /// .debug_str
    Str,
    /// .debug_ranges
    Ranges,
    /// .debug_loc
    Loc,
    /// .debug_pubtypes
    PubTypes,
    /// .debug_pubnames
    PubNames,
}

/// DWARF abbreviation entry
#[derive(Debug, Clone)]
pub struct Abbreviation {
    /// Abbreviation code
    pub code: u64,
    /// Tag (what kind of thing this is)
    pub tag: DwarfTag,
    /// Whether children are present
    pub has_children: bool,
    /// Attribute specifications
    pub attributes: Vec<AttributeSpec>,
}

impl Abbreviation {
    pub fn new(code: u64, tag: DwarfTag, has_children: bool) -> Self {
        Abbreviation {
            code,
            tag,
            has_children,
            attributes: Vec::new(),
        }
    }

    pub fn with_attr(mut self, name: DwarfAttr, form: DwarfForm) -> Self {
        self.attributes.push(AttributeSpec { name, form });
        self
    }
}

/// Attribute specification
#[derive(Debug, Clone)]
pub struct AttributeSpec {
    pub name: DwarfAttr,
    pub form: DwarfForm,
}

/// DWARF tags (what kind of thing)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DwarfTag {
    None,
    /// Array type
    ArrayType,
    /// Class/struct type
    ClassType,
    /// Entry point
    EntryPoint,
    /// Enumeration type
    EnumerationType,
    /// Formal parameter
    FormalParameter,
    /// Imported declaration
    ImportedDeclaration,
    /// Label
    Label,
    /// Lexical block
    LexicalBlock,
    /// Member
    Member,
    /// Pointer type
    PointerType,
    /// Reference type
    ReferenceType,
    /// Compile unit
    CompileUnit,
    /// Condition
    Condition,
    /// Constant
    Constant,
    /// File type
    FileType,
    /// Function
    Subprogram,
    /// String type
    StringType,
    /// Structure type
    StructureType,
    /// Subroutine type
    SubroutineType,
    /// Union type
    UnionType,
    /// Variable
    Variable,
    /// Common block
    CommonBlock,
    /// Common inclusion
    CommonInclusion,
    /// Inheritance
    Inheritance,
    /// Inlined subroutine
    InlinedSubroutine,
    /// Module
    Module,
    /// Pointer to member type
    PointerToMemberType,
    /// Reference to member type
    ReferenceToMemberType,
    /// Dynamic type
    DynamicType,
    /// With statement
    WithStatement,
    /// Access declaration
    AccessDeclaration,
    /// Base type
    BaseType,
    /// Catch block
    CatchBlock,
    /// Const type
    ConstType,
    /// Constant parameter
    ConstantParameter,
    /// Entry point
    DwarfEntryPoint,
    /// Enumerator
    Enumerator,
    /// File type
    FileType2,
    /// Friend
    Friend,
    /// Namelist
    Namelist,
    /// Namelist item
    NamelistItem,
    /// Packed type
    PackedType,
    /// Template type parameter
    TemplateTypeParameter,
    /// Template value parameter
    TemplateValueParameter,
    /// Thrown type
    ThrownType,
    /// Try block
    TryBlock,
    /// Variant
    Variant,
    /// Variable parameter
    VariableParameter,
    /// Variant part
    VariantPart,
    /// Variable template parameter
    VariableTemplateParameter,
}

impl DwarfTag {
    pub fn to_uleb128(&self) -> u64 {
        *self as u64
    }
}

/// DWARF attributes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DwarfAttr {
    None,
    /// Name of the entity
    Name,
    /// Byte size of entity
    ByteSize,
    /// Location description
    Location,
    /// Type description
    Type,
    /// Upper bound index
    UpperBound,
    /// Bit size
    BitSize,
    /// Bit offset
    BitOffset,
    /// Data member location
    DataMemberLocation,
    /// Addressing mode
    AddressingMode,
    /// Artificial flag
    Artificial,
    /// Backend CSSpec
    BackendCsspec,
    /// Binary scale
    BinaryScale,
    /// Bit stride
    BitStride,
    /// Byte stride
    ByteStride,
    /// Common reference
    CommonReference,
    /// Containing type
    ContainingType,
    /// Decimal scale
    DecimalScale,
    /// Decimal sign
    DecimalSign,
    /// Description
    Description,
    /// Digit count
    DigitCount,
    /// Discr
    Discr,
    /// Discr value
    DiscrValue,
    /// Encoding
    Encoding,
    /// Endianity
    Endianity,
    /// External
    External,
    /// Frame base
    FrameBase,
    /// Friend name
    FriendName,
    /// Identifier case
    IdentifierCase,
    /// Import
    Import,
    /// Inline
    Inline,
    /// Is optional
    IsOptional,
    /// Lower bound
    LowerBound,
    /// Memory location
    MemoryLocation,
    /// Priority
    Priority,
    /// Prototyped
    Prototyped,
    /// Pure
    Pure,
    /// Ranges,
    Ranges,
    /// Return address
    ReturnAddress,
    /// Segment
    Segment,
    /// Small
    Small,
    /// Specification,
    Specification,
    /// Start scope
    StartScope,
    /// Static link
    StaticLink,
    /// Thread local
    ThreadLocal,
    /// Trampoline,
    Trampoline,
    /// Type param
    TypeParam,
    /// Upper bound expr
    UpperBoundExpr,
    /// Use location
    UseLocation,
    /// Use str
    UseStr,
    /// Var params
    VarParams,
    /// Visibility
    Visibility,
    /// Vtable elem location
    VtableElemLocation,
    /// Word size
    WordSize,
    /// Language
    Language,
    /// Producer
    Producer,
    /// Compilation unit
    CompilationUnit,
    /// Event
    Event,
    /// Deleted
    Deleted,
}

impl DwarfAttr {
    pub fn to_uleb128(&self) -> u64 {
        *self as u64
    }
}

/// DWARF attribute forms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DwarfForm {
    /// No form
    None,
    /// Address
    Addr,
    /// 1-byte unsigned
    Data1,
    /// 2-byte unsigned
    Data2,
    /// 4-byte unsigned
    Data4,
    /// 8-byte unsigned
    Data8,
    /// 1-byte signed
    Sdata,
    /// 4-byte signed
    String,
    /// Index into .debug_str
    Strp,
    /// uleb128
    Uleb128,
    /// Reference to another DIE
    Ref1,
    Ref2,
    Ref4,
    Ref8,
    /// Reference to block
    RefAddr,
    /// Expression block
    Exprloc,
    /// Flag present
    FlagPresent,
    /// Reference flag
    Flag,
    /// Section offset
    SecOffset,
    /// Expr/Bloc implicit
    Indirect,
}

impl DwarfForm {
    pub fn to_uleb128(&self) -> u64 {
        *self as u64
    }
}

/// DWARF location expression opcode
#[derive(Debug, Clone)]
pub enum LocationOp {
    /// No operation
    Nop,
    /// Address
    Addr(u64),
    /// Dereference
    Deref,
    /// Read memory
    Fbreg(i64),
    /// Frame base register
   breg(u8, i64),
    /// Register
    Reg(u8),
    /// Register based
    Regx(u64),
    /// Implicit value
    Const1u(u8),
    Const1s(i8),
    Const2u(u16),
    Const2s(i16),
    Const4u(u32),
    Const4s(i32),
    Const8u(u64),
    Const8s(i64),
    /// Stack operations
    Dup,
    Drop,
    Over,
    Pick(u8),
    Swap,
    Rot,
    /// Arithmetic
    Plus,
    PlusUconst(u64),
    Minus,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Not,
    /// Branch
    Lt,
    Gt,
    Le,
    Ge,
    Eq,
    Ne,
    Shl,
    Shr,
    Shra,
    /// Special
    Bra(u16),
    Skip(u16),
    Littype,
    Bind,
    /// DW_OP 2.x
    Regvaltype(u8),
    Dereftype(u8),
    Xdereftype(u8),
    Abs,
    Bitpiece(u64),
    Convert,
    Reinterpret,
    /// DW_OP 5
    ParameterRef(u64),
    FrameRef,
    Trim,
    ConvertToType,
    ReinterpretAsType,
    ImplicitValue,
    StackValue,
    AppendTree,
    ElseOp,
    Switch,
    Continuation,
    GenericDwarfExpr,
}

impl LocationOp {
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            LocationOp::Nop => bytes.push(0x00),
            LocationOp::Addr(v) => { bytes.push(0x03); bytes.extend_from_slice(&v.to_le_bytes()); }
            LocationOp::Deref => bytes.push(0x06),
            LocationOp::Fbreg(o) => { bytes.push(0x91); bytes.extend_from_slice(&encode_sleb128(*o)); }
            LocationOp::Reg(r) => bytes.push(0x50 + r),
            LocationOp::Regx(r) => { bytes.push(0x90); bytes.extend_from_slice(&encode_uleb128(*r)); }
            LocationOp::Const1u(v) => { bytes.push(0x10); bytes.push(*v); }
            LocationOp::Const2u(v) => { bytes.push(0x11); bytes.extend_from_slice(&v.to_le_bytes()); }
            LocationOp::Const4u(v) => { bytes.push(0x12); bytes.extend_from_slice(&v.to_le_bytes()); }
            LocationOp::Plus => bytes.push(0x22),
            LocationOp::Minus => bytes.push(0x23),
            LocationOp::Mul => bytes.push(0x27),
            LocationOp::Eq => bytes.push(0x29),
            LocationOp::Lt => bytes.push(0x2c),
            LocationOp::Bra(target) => { bytes.push(0x38); bytes.extend_from_slice(&target.to_le_bytes()); }
            LocationOp::Skip(target) => { bytes.push(0x2f); bytes.extend_from_slice(&target.to_le_bytes()); }
            LocationOp::Dup => bytes.push(0x12),
            LocationOp::Drop => bytes.push(0x13),
            _ => {}
        }
        bytes
    }
}

/// Line number program opcodes
#[derive(Debug, Clone)]
pub enum LineOp {
    /// Extended opcode
    Extended(u8, Vec<u8>),
    /// Copy
    Copy,
    /// Advance PC
    AdvancePc(u64),
    /// Advance line
    AdvanceLine(i64),
    /// Set file
    SetFile(u64),
    /// Set column
    SetColumn(u64),
    /// Negate stmt
    NegateStmt,
    /// Set basic block
    SetBasicBlock,
    /// Const add pc
    ConstAddPc,
    /// Fixed advance pc
    FixedAdvancePc(u16),
    /// Set prologue end
    SetPrologueEnd,
    /// Set epilogue begin
    SetEpilogueBegin,
    /// Set isa
    SetIsa(u64),
}

impl LineOp {
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            LineOp::Extended(len, data) => {
                bytes.push(0x00);
                bytes.push(*len);
                bytes.extend_from_slice(data);
            }
            LineOp::Copy => bytes.push(0x01),
            LineOp::AdvancePc(v) => {
                bytes.push(0x02);
                bytes.extend_from_slice(&encode_uleb128(*v));
            }
            LineOp::AdvanceLine(v) => {
                bytes.push(0x03);
                bytes.extend_from_slice(&encode_sleb128(*v));
            }
            LineOp::SetFile(v) => {
                bytes.push(0x04);
                bytes.extend_from_slice(&encode_uleb128(*v));
            }
            LineOp::SetColumn(v) => {
                bytes.push(0x05);
                bytes.extend_from_slice(&encode_uleb128(*v));
            }
            LineOp::NegateStmt => bytes.push(0x06),
            LineOp::SetBasicBlock => bytes.push(0x07),
            LineOp::ConstAddPc => bytes.push(0x08),
            LineOp::FixedAdvancePc(v) => {
                bytes.push(0x09);
                bytes.extend_from_slice(&v.to_le_bytes());
            }
            _ => {}
        }
        bytes
    }
}

/// Line number program state
#[derive(Debug, Clone, Default)]
pub struct LineState {
    pub address: u64,
    pub file: u64,
    pub line: u64,
    pub column: u64,
    pub is_stmt: bool,
    pub basic_block: bool,
    pub end_sequence: bool,
    pub prologue_end: bool,
    pub epilogue_begin: bool,
    pub isa: u64,
}

/// Debug Information Entry (DIE)
#[derive(Debug, Clone)]
pub struct Die {
    /// Tag
    pub tag: DwarfTag,
    /// Abbreviation code
    pub abbrev_code: u64,
    /// Attributes
    pub attributes: HashMap<DwarfAttr, DieAttribute>,
    /// Children
    pub children: Vec<Die>,
    /// Offset in debug_info
    pub offset: u64,
}

impl Die {
    pub fn new(tag: DwarfTag, abbrev_code: u64) -> Self {
        Die {
            tag,
            abbrev_code,
            attributes: HashMap::new(),
            children: Vec::new(),
            offset: 0,
        }
    }

    pub fn with_attr(mut self, attr: DwarfAttr, value: DieAttribute) -> Self {
        self.attributes.insert(attr, value);
        self
    }

    pub fn add_child(&mut self, child: Die) {
        self.children.push(child);
    }
}

/// Die attribute value
#[derive(Debug, Clone)]
pub enum DieAttribute {
    /// Address
    Addr(u64),
    /// Block
    Block(Vec<u8>),
    /// Constant data
    Data(u64),
    /// String
    String(String),
    /// Reference to another DIE
    Ref(u64),
    /// Location expression
    Exprloc(Vec<u8>),
    /// Flag
    Flag(bool),
    /// Reference to string
    Strp(u64),
}

impl DieAttribute {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            DieAttribute::Addr(v) => v.to_le_bytes().to_vec(),
            DieAttribute::Block(v) => v.clone(),
            DieAttribute::Data(v) => v.to_le_bytes().to_vec(),
            DieAttribute::String(s) => s.as_bytes().to_vec(),
            DieAttribute::Ref(v) => v.to_le_bytes().to_vec(),
            DieAttribute::Exprloc(v) => v.clone(),
            DieAttribute::Flag(_) => vec![],
            DieAttribute::Strp(v) => v.to_le_bytes().to_vec(),
        }
    }
}

/// Source file info
#[derive(Debug, Clone)]
pub struct SourceFile {
    pub name: String,
    pub path: String,
    pub directory: String,
    pub mtime: u64,
    pub size: u64,
}

/// Line number table entry
#[derive(Debug, Clone)]
pub struct LineEntry {
    pub address: u64,
    pub file: u64,
    pub line: u64,
    pub column: u64,
    pub is_stmt: bool,
    pub basic_block: bool,
    pub prologue_end: bool,
    pub epilogue_begin: bool,
    pub isa: u64,
}

/// DWARF generator
pub struct DwarfGenerator {
    /// Target DWARF version
    version: DwarfVersion,
    /// Compilation unit
    compile_unit: Option<Die>,
    /// Abbreviation table
    abbreviations: Vec<Abbreviation>,
    /// Source files
    source_files: Vec<SourceFile>,
    /// Line program entries
    line_entries: Vec<LineEntry>,
    /// String table
    strings: HashMap<String, u64>,
    /// Next abbreviation code
    next_abbrev_code: u64,
    /// Output buffer
    buffers: HashMap<DwarfSection, Vec<u8>>,
}

impl DwarfGenerator {
    /// Create new DWARF generator
    pub fn new(version: DwarfVersion) -> Self {
        let mut gen = DwarfGenerator {
            version,
            compile_unit: None,
            abbreviations: Vec::new(),
            source_files: Vec::new(),
            line_entries: Vec::new(),
            strings: HashMap::new(),
            next_abbrev_code: 1,
            buffers: HashMap::new(),
        };

        // Initialize abbreviations table
        gen.init_abbreviations();

        gen
    }

    /// Initialize standard abbreviations
    fn init_abbreviations(&mut self) {
        // DW_TAG_compile_unit
        self.abbreviations.push(
            Abbreviation::new(1, DwarfTag::CompileUnit, true)
                .with_attr(DwarfAttr::Language, DwarfForm::Uleb128)
                .with_attr(DwarfAttr::Producer, DwarfForm::Strp)
                .with_attr(DwarfAttr::Name, DwarfForm::Strp)
                .with_attr(DwarfAttr::CompDir, DwarfForm::Strp)
                .with_attr(DwarfAttr::LowPc, DwarfForm::Addr)
                .with_attr(DwarfAttr::HighPc, DwarfForm::Addr)
        );

        // DW_TAG_base_type
        self.abbreviations.push(
            Abbreviation::new(2, DwarfTag::BaseType, false)
                .with_attr(DwarfAttr::Name, DwarfForm::String)
                .with_attr(DwarfAttr::ByteSize, DwarfForm::Data1)
                .with_attr(DwarfAttr::Encoding, DwarfForm::Data1)
        );

        // DW_TAG_pointer_type
        self.abbreviations.push(
            Abbreviation::new(3, DwarfTag::PointerType, false)
                .with_attr(DwarfAttr::Type, DwarfForm::Ref4)
                .with_attr(DwarfAttr::ByteSize, DwarfForm::Data1)
        );

        // DW_TAG_structure_type
        self.abbreviations.push(
            Abbreviation::new(4, DwarfTag::StructureType, true)
                .with_attr(DwarfAttr::Name, DwarfForm::String)
                .with_attr(DwarfAttr::ByteSize, DwarfForm::Data4)
        );

        // DW_TAG_member
        self.abbreviations.push(
            Abbreviation::new(5, DwarfTag::Member, false)
                .with_attr(DwarfAttr::Name, DwarfForm::String)
                .with_attr(DwarfAttr::Type, DwarfForm::Ref4)
                .with_attr(DwarfAttr::DataMemberLocation, DwarfForm::Data1)
        );

        // DW_TAG_subprogram
        self.abbreviations.push(
            Abbreviation::new(6, DwarfTag::Subprogram, true)
                .with_attr(DwarfAttr::Name, DwarfForm::String)
                .with_attr(DwarfAttr::LowPc, DwarfForm::Addr)
                .with_attr(DwarfAttr::HighPc, DwarfForm::Addr)
                .with_attr(DwarfAttr::FrameBase, DwarfForm::Exprloc)
                .with_attr(DwarfAttr::External, DwarfForm::FlagPresent)
        );

        // DW_TAG_formal_parameter
        self.abbreviations.push(
            Abbreviation::new(7, DwarfTag::FormalParameter, false)
                .with_attr(DwarfAttr::Name, DwarfForm::String)
                .with_attr(DwarfAttr::Type, DwarfForm::Ref4)
                .with_attr(DwarfAttr::Location, DwarfForm::Exprloc)
        );

        // DW_TAG_variable
        self.abbreviations.push(
            Abbreviation::new(8, DwarfTag::Variable, false)
                .with_attr(DwarfAttr::Name, DwarfForm::String)
                .with_attr(DwarfAttr::Type, DwarfForm::Ref4)
                .with_attr(DwarfAttr::Location, DwarfForm::Exprloc)
        );

        // DW_TAG_enumerator
        self.abbreviations.push(
            Abbreviation::new(9, DwarfTag::Enumerator, false)
                .with_attr(DwarfAttr::Name, DwarfForm::String)
                .with_attr(DwarfAttr::ConstValue, DwarfForm::Data8)
        );
    }

    /// Add a source file
    pub fn add_source_file(&mut self, name: &str, path: &str) {
        self.source_files.push(SourceFile {
            name: name.to_string(),
            path: path.to_string(),
            directory: std::path::Path::new(path)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            mtime: 0,
            size: 0,
        });
    }

    /// Start compilation unit
    pub fn start_compile_unit(&mut self, name: &str, lang: u64) -> &mut Die {
        self.strings.insert(name.to_string(), self.add_string(name));

        let mut die = Die::new(DwarfTag::CompileUnit, 1);
        die.attributes.insert(DwarfAttr::Language, DieAttribute::Data(lang));
        die.attributes.insert(DwarfAttr::Name, DieAttribute::Strp(self.strings[name]));
        die.attributes.insert(DwarfAttr::Producer, DieAttribute::String("TUMC 0.1.0".to_string()));
        die.attributes.insert(DwarfAttr::CompDir, DieAttribute::Strp(0)); // Current directory

        self.compile_unit = Some(die);
        self.compile_unit.as_mut().unwrap()
    }

    /// Add base type
    pub fn add_base_type(&mut self, name: &str, size: u8, encoding: u8) -> Die {
        let mut die = Die::new(DwarfTag::BaseType, 2);
        die.attributes.insert(DwarfAttr::Name, DieAttribute::String(name.to_string()));
        die.attributes.insert(DwarfAttr::ByteSize, DieAttribute::Data(size as u64));
        die.attributes.insert(DwarfAttr::Encoding, DieAttribute::Data(encoding as u64));
        die
    }

    /// Add pointer type
    pub fn add_pointer_type(&mut self, pointed_to: u64) -> Die {
        let mut die = Die::new(DwarfTag::PointerType, 3);
        die.attributes.insert(DwarfAttr::Type, DieAttribute::Ref(pointed_to));
        die.attributes.insert(DwarfAttr::ByteSize, DieAttribute::Data(8)); // 64-bit pointer
        die
    }

    /// Add structure type
    pub fn add_struct(&mut self, name: &str, size: u32) -> Die {
        let mut die = Die::new(DwarfTag::StructureType, 4);
        die.attributes.insert(DwarfAttr::Name, DieAttribute::String(name.to_string()));
        die.attributes.insert(DwarfAttr::ByteSize, DieAttribute::Data(size as u64));
        die
    }

    /// Add member field
    pub fn add_member(&mut self, name: &str, type_ref: u64, offset: u8) -> Die {
        let mut die = Die::new(DwarfTag::Member, 5);
        die.attributes.insert(DwarfAttr::Name, DieAttribute::String(name.to_string()));
        die.attributes.insert(DwarfAttr::Type, DieAttribute::Ref(type_ref));
        die.attributes.insert(DwarfAttr::DataMemberLocation, DieAttribute::Data(offset as u64));
        die
    }

    /// Add function
    pub fn add_function(&mut self, name: &str, low_pc: u64, high_pc: u64, external: bool) -> Die {
        let mut die = Die::new(DwarfTag::Subprogram, 6);
        die.attributes.insert(DwarfAttr::Name, DieAttribute::String(name.to_string()));
        die.attributes.insert(DwarfAttr::LowPc, DieAttribute::Addr(low_pc));
        die.attributes.insert(DwarfAttr::HighPc, DieAttribute::Addr(high_pc));

        // Frame base location
        let mut loc = Vec::new();
        loc.push(0x9c); // DW_OP_fbreg(0)
        loc.extend_from_slice(&encode_sleb128(0));
        die.attributes.insert(DwarfAttr::FrameBase, DieAttribute::Exprloc(loc));

        if external {
            die.attributes.insert(DwarfAttr::External, DieAttribute::Flag(true));
        }

        die
    }

    /// Add formal parameter
    pub fn add_parameter(&mut self, name: &str, type_ref: u64, location: Vec<u8>) -> Die {
        let mut die = Die::new(DwarfTag::FormalParameter, 7);
        die.attributes.insert(DwarfAttr::Name, DieAttribute::String(name.to_string()));
        die.attributes.insert(DwarfAttr::Type, DieAttribute::Ref(type_ref));
        die.attributes.insert(DwarfAttr::Location, DieAttribute::Exprloc(location));
        die
    }

    /// Add variable
    pub fn add_variable(&mut self, name: &str, type_ref: u64, location: Vec<u8>) -> Die {
        let mut die = Die::new(DwarfTag::Variable, 8);
        die.attributes.insert(DwarfAttr::Name, DieAttribute::String(name.to_string()));
        die.attributes.insert(DwarfAttr::Type, DieAttribute::Ref(type_ref));
        die.attributes.insert(DwarfAttr::Location, DieAttribute::Exprloc(location));
        die
    }

    /// Add line number entry
    pub fn add_line(&mut self, address: u64, file: u64, line: u64, column: u64) {
        self.line_entries.push(LineEntry {
            address,
            file,
            line,
            column,
            is_stmt: true,
            basic_block: false,
            prologue_end: false,
            epilogue_begin: false,
            isa: 0,
        });
    }

    /// Add string to string table
    pub fn add_string(&mut self, s: &str) -> u64 {
        let offset = self.strings.len() as u64;
        offset
    }

    /// Generate debug sections
    pub fn generate(&mut self) -> HashMap<DwarfSection, Vec<u8>> {
        let mut sections = HashMap::new();

        // Generate .debug_abbrev
        sections.insert(DwarfSection::Abbrev, self.generate_abbrev_section());

        // Generate .debug_info
        sections.insert(DwarfSection::Info, self.generate_info_section());

        // Generate .debug_line
        sections.insert(DwarfSection::Line, self.generate_line_section());

        // Generate .debug_str
        sections.insert(DwarfSection::Str, self.generate_str_section());

        self.buffers = sections.clone();
        sections
    }

    /// Generate abbreviation section
    fn generate_abbrev_section(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for abbrev in &self.abbreviations {
            // Write code
            bytes.extend_from_slice(&encode_uleb128(abbrev.code));

            // Write tag
            bytes.extend_from_slice(&encode_uleb128(abbrev.tag.to_uleb128()));

            // Write children flag
            bytes.push(if abbrev.has_children { 1 } else { 0 });

            // Write attributes
            for attr in &abbrev.attributes {
                bytes.extend_from_slice(&encode_uleb128(attr.name.to_uleb128()));
                bytes.extend_from_slice(&encode_uleb128(attr.form.to_uleb128()));
            }

            // End of attributes
            bytes.push(0);
            bytes.push(0);
        }

        // End of abbreviation table
        bytes.push(0);

        bytes
    }

    /// Generate info section
    fn generate_info_section(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        if let Some(ref cu) = self.compile_unit {
            self.write_die(&cu, &mut bytes);
        }

        bytes
    }

    /// Write DIE to bytes
    fn write_die(&self, die: &Die, bytes: &mut Vec<u8>) {
        // Write abbreviation code
        bytes.extend_from_slice(&encode_uleb128(die.abbrev_code));

        // Write attributes
        for (attr, value) in &die.attributes {
            bytes.extend_from_slice(&encode_uleb128(attr.to_uleb128()));
            match value {
                DieAttribute::Addr(v) => bytes.extend_from_slice(&v.to_le_bytes()),
                DieAttribute::Data(v) => bytes.extend_from_slice(&encode_uleb128(*v)),
                DieAttribute::String(s) => {
                    bytes.push(0); // null-terminated
                }
                DieAttribute::Ref(v) => bytes.extend_from_slice(&v.to_le_bytes()),
                DieAttribute::Exprloc(v) => {
                    bytes.extend_from_slice(&encode_uleb128(v.len() as u64));
                    bytes.extend_from_slice(v);
                }
                DieAttribute::Flag(_) => bytes.push(0),
                DieAttribute::Strp(v) => bytes.extend_from_slice(&v.to_le_bytes()),
                DieAttribute::Block(v) => {
                    bytes.extend_from_slice(&encode_uleb128(v.len() as u64));
                    bytes.extend_from_slice(v);
                }
            }
        }

        // End of attributes
        bytes.push(0);

        // Write children
        if !die.children.is_empty() {
            for child in &die.children {
                self.write_die(child, bytes);
            }
            // End of children
            bytes.push(0);
        }
    }

    /// Generate line section
    fn generate_line_section(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Unit header
        let mut unit_start = bytes.len();

        // Unit length (placeholder)
        bytes.extend_from_slice(&0u32.to_le_bytes());

        // Version
        bytes.extend_from_slice(&2u16.to_le_bytes()); // DWARF 2

        // Header length
        let header_length_pos = bytes.len();
        bytes.extend_from_slice(&0u32.to_le_bytes());

        // Minimum instruction length
        bytes.push(1);

        // Default is stmt
        bytes.push(1);

        // Line base
        bytes.push(-5i8 as u8);

        // Line range
        bytes.push(14);

        // Opcode base
        bytes.push(13);

        // Standard opcode lengths
        bytes.push(0); // LNS_copy
        bytes.push(1); // LNS_advance_pc
        bytes.push(1); // LNS_advance_line
        bytes.push(1); // LNS_set_file
        bytes.push(1); // LNS_set_column
        bytes.push(0); // LNS_negate_stmt
        bytes.push(0); // LNS_set_basic_block
        bytes.push(0); // LNS_const_add_pc
        bytes.push(1); // LNS_fixed_advance_pc

        // Include directories
        bytes.push(0);

        // File names
        for (i, file) in self.source_files.iter().enumerate() {
            // File name
            bytes.extend_from_slice(file.name.as_bytes());
            bytes.push(0);

            // Directory index
            bytes.extend_from_slice(&encode_uleb128(0));

            // Modification time
            bytes.extend_from_slice(&encode_uleb128(file.mtime));

            // File size
            bytes.extend_from_slice(&encode_uleb128(file.size));
        }
        bytes.push(0);

        // Update header length
        let header_length = (bytes.len() - header_length_pos - 4) as u32;
        let end_pos = bytes.len();
        let _ = bytes.splice(header_length_pos..header_length_pos + 4, header_length.to_le_bytes());
        let _ = end_pos;

        // Line program
        for entry in &self.line_entries {
            bytes.extend_from_slice(&encode_uleb128(entry.address));
            bytes.extend_from_slice(&encode_uleb128(entry.file));
            bytes.extend_from_slice(&encode_sleb128(entry.line as i64));
            bytes.extend_from_slice(&encode_uleb128(entry.column));
            bytes.push(if entry.is_stmt { 1 } else { 0 });
        }

        // Update unit length
        let unit_length = (bytes.len() - unit_start - 4) as u32;
        let _ = bytes.splice(unit_start..unit_start + 4, unit_length.to_le_bytes());

        bytes
    }

    /// Generate string section
    fn generate_str_section(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(0); // Empty string at offset 0

        for s in self.strings.keys() {
            bytes.extend_from_slice(s.as_bytes());
            bytes.push(0);
        }

        bytes
    }

    /// Emit ELF/DWARF format
    pub fn emit_elf(&self) -> Vec<u8> {
        // Simplified ELF generation
        let mut elf = Vec::new();

        // ELF header
        elf.extend_from_slice(&[0x7f, b'E', b'L', b'F']); // Magic
        elf.push(2); // 64-bit
        elf.push(1); // Little endian
        elf.push(1); // ELF version
        elf.extend_from_slice(&[0; 8]); // OS/ABI, padding
        elf.extend_from_slice(&[0; 16]); // Padding

        elf.extend_from_slice(&1u16.to_le_bytes()); // e_type
        elf.extend_from_slice(&62u16.to_le_bytes()); // e_machine (x86_64)
        elf.extend_from_slice(&1u32.to_le_bytes()); // e_version
        elf.extend_from_slice(&0u64.to_le_bytes()); // e_entry
        elf.extend_from_slice(&0u64.to_le_bytes()); // e_phoff
        elf.extend_from_slice(&0u64.to_le_bytes()); // e_shoff
        elf.extend_from_slice(&0u32.to_le_bytes()); // e_flags
        elf.extend_from_slice(&64u16.to_le_bytes()); // e_ehsize
        elf.extend_from_slice(&0u16.to_le_bytes()); // e_phentsize
        elf.extend_from_slice(&0u16.to_le_bytes()); // e_phnum
        elf.extend_from_slice(&64u16.to_le_bytes()); // e_shentsize
        elf.extend_from_slice(&0u16.to_le_bytes()); // e_shnum
        elf.extend_from_slice(&0u16.to_le_bytes()); // e_shstrndx

        elf
    }
}

// ==================== Helper Functions ====================

/// Encode unsigned LEB128
fn encode_uleb128(mut value: u64) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            bytes.push(byte | 0x80);
        } else {
            bytes.push(byte);
            break;
        }
    }
    bytes
}

/// Encode signed LEB128
fn encode_sleb128(mut value: i64) -> Vec<u8> {
    let mut bytes = Vec::new();
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        if (value == 0 && byte & 0x40 == 0) || (value == -1 && byte & 0x40 != 0) {
            bytes.push(byte);
            break;
        } else {
            bytes.push(byte | 0x80);
        }
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uleb128_encoding() {
        assert_eq!(encode_uleb128(0), vec![0]);
        assert_eq!(encode_uleb128(1), vec![1]);
        assert_eq!(encode_uleb128(127), vec![127]);
        assert_eq!(encode_uleb128(128), vec![0x80, 1]);
        assert_eq!(encode_uleb128(300), vec![0xAC, 2]);
    }

    #[test]
    fn test_sleb128_encoding() {
        assert_eq!(encode_sleb128(0), vec![0]);
        assert_eq!(encode_sleb128(-1), vec![0x7F]);
        assert_eq!(encode_sleb128(127), vec![0x7F, 0]);
        assert_eq!(encode_sleb128(-128), vec![0x80, 0x7F]);
    }

    #[test]
    fn test_dwarf_generator() {
        let mut gen = DwarfGenerator::new(DwarfVersion::V4);
        gen.add_source_file("test.tumc", "/src/test.tumc");

        let cu = gen.start_compile_unit("test.tumc", 12); // Rust-like

        let sections = gen.generate();
        assert!(sections.contains_key(&DwarfSection::Abbrev));
        assert!(sections.contains_key(&DwarfSection::Info));
        assert!(sections.contains_key(&DwarfSection::Line));
    }
}
