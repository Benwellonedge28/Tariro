//! TUMC LLVM IR Generation
//!
//! Intermediate representation generation targeting LLVM for optimization
//! and native code generation via LLVM backends.

use std::collections::HashMap;

/// LLVM integer types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LlvmIntType {
    I1,   // 1-bit
    I8,   // 8-bit
    I16,  // 16-bit
    I32,  // 32-bit
    I64,  // 64-bit
    I128, // 128-bit
}

impl LlvmIntType {
    /// Get LLVM type name
    pub fn name(&self) -> &'static str {
        match self {
            LlvmIntType::I1 => "i1",
            LlvmIntType::I8 => "i8",
            LlvmIntType::I16 => "i16",
            LlvmIntType::I32 => "i32",
            LlvmIntType::I64 => "i64",
            LlvmIntType::I128 => "i128",
        }
    }
}

/// LLVM floating point types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LlvmFloatType {
    Half,   // 16-bit (IEEE 754)
    Float,  // 32-bit
    Double, // 64-bit
    Fp128,  // 128-bit
    X86Fp80, // x87 80-bit
}

impl LlvmFloatType {
    /// Get LLVM type name
    pub fn name(&self) -> &'static str {
        match self {
            LlvmFloatType::Half => "half",
            LlvmFloatType::Float => "float",
            LlvmFloatType::Double => "double",
            LlvmFloatType::Fp128 => "fp128",
            LlvmFloatType::X86Fp80 => "x86_fp80",
        }
    }
}

/// LLVM type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LlvmType {
    /// Integer
    Int(LlvmIntType),
    /// Floating point
    Float(LlvmFloatType),
    /// Void
    Void,
    /// Function
    Function(Box<LlvmType>, Vec<LlvmType>),
    /// Pointer
    Pointer(Box<LlvmType>),
    /// Array
    Array(usize, Box<LlvmType>),
    /// Vector (SIMD)
    Vector(usize, Box<LlvmType>),
    /// Structure
    Struct(Vec<LlvmType>),
    /// Label
    Label,
    /// Metadata
    Metadata,
    /// Token
    Token,
    /// Opaque (forward declaration)
    Opaque,
}

impl LlvmType {
    /// Get LLVM type string
    pub fn to_string(&self) -> String {
        match self {
            LlvmType::Int(t) => t.name().to_string(),
            LlvmType::Float(t) => t.name().to_string(),
            LlvmType::Void => "void".to_string(),
            LlvmType::Function(ret, params) => {
                let params_str = params
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} ({})", ret.to_string(), params_str)
            }
            LlvmType::Pointer(t) => format!("{}*", t.to_string()),
            LlvmType::Array(len, t) => format!("[{} x {}]", len, t.to_string()),
            LlvmType::Vector(len, t) => format!("<{} x {}>", len, t.to_string()),
            LlvmType::Struct(types) => {
                let fields = types
                    .iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{ {} }}", fields)
            }
            LlvmType::Label => "label".to_string(),
            LlvmType::Metadata => "metadata".to_string(),
            LlvmType::Token => "token".to_string(),
            LlvmType::Opaque => "opaque".to_string(),
        }
    }
}

/// LLVM linkage
#[derive(Debug, Clone, Copy)]
pub enum LdLinkage {
    Private,
    Internal,
    AvailableExternally,
    LinkOnce,
    Weak,
    Common,
    Appending,
    ExternWeak,
    LinkOnceODR,
    WeakODR,
    External,
}

impl LdLinkage {
    pub fn to_string(&self) -> &'static str {
        match self {
            LdLinkage::Private => "private",
            LdLinkage::Internal => "internal",
            LdLinkage::AvailableExternally => "available_externally",
            LdLinkage::LinkOnce => "linkonce",
            LdLinkage::Weak => "weak",
            LdLinkage::Common => "common",
            LdLinkage::Appending => "appending",
            LdLinkage::ExternWeak => "extern_weak",
            LdLinkage::LinkOnceODR => "linkonce_odr",
            LdLinkage::WeakODR => "weak_odr",
            LdLinkage::External => "external",
        }
    }
}

/// LLVM visibility
#[derive(Debug, Clone, Copy)]
pub enum LdVisibility {
    Default,
    Hidden,
    Protected,
}

impl LdVisibility {
    pub fn to_string(&self) -> &'static str {
        match self {
            LdVisibility::Default => "default",
            LdVisibility::Hidden => "hidden",
            LdVisibility::Protected => "protected",
        }
    }
}

/// LLVM calling convention
#[derive(Debug, Clone, Copy)]
pub enum CallConv {
    C,
    Fast,
    Cold,
    Gc,
    X86Stdcall,
    X86Fastcall,
    ArmApcs,
    ArmAAPCS,
    Msp430Intr,
    X86ThisCall,
    PtXKernel,
    MipsEabi,
    MipsMsa,
    SolarisCcd,
    Swift,
    X86VectorCall,
    Hhvm,
    HhvmC,
    AmdGpuKernel,
    AmdGpuHs,
}

impl CallConv {
    pub fn to_string(&self) -> &'static str {
        match self {
            CallConv::C => "ccc",
            CallConv::Fast => "fastcc",
            CallConv::Cold => "coldcc",
            CallConv::Gc => "cc 10",
            CallConv::X86Stdcall => "x86_stdcallcc",
            CallConv::X86Fastcall => "x86_fastcallcc",
            CallConv::ArmApcs => "arm_apcscc",
            CallConv::ArmAAPCS => "arm_aapcscc",
            CallConv::Msp430Intr => "msp430_intrcc",
            CallConv::X86ThisCall => "x86_thiscallcc",
            CallConv::PtXKernel => "ptx_kernelcc",
            CallConv::MipsEabi => "mips_eabicc",
            CallConv::MipsMsa => "mips_msacc",
            CallConv::SolarisCcd => "sparc_cvcallcc",
            CallConv::Swift => "swiftcc",
            CallConv::X86VectorCall => "x86_vectorcallcc",
            CallConv::Hhvm => "hhvmcc",
            CallConv::HhvmC => "hhvm_ccc",
            CallConv::AmdGpuKernel => "amdgpu_kernelcc",
            CallConv::AmdGpuHs => "amdgpu_hscc",
        }
    }
}

/// LLVM binary operation
#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    FAdd,
    Sub,
    FSub,
    Mul,
    FMul,
    UDiv,
    SDiv,
    FDiv,
    URem,
    SRem,
    FRem,
    Shl,
    LShr,
    AShr,
    And,
    Or,
    Xor,
}

impl BinOp {
    pub fn to_string(&self) -> &'static str {
        match self {
            BinOp::Add => "add",
            BinOp::FAdd => "fadd",
            BinOp::Sub => "sub",
            BinOp::FSub => "fsub",
            BinOp::Mul => "mul",
            BinOp::FMul => "fmul",
            BinOp::UDiv => "udiv",
            BinOp::SDiv => "sdiv",
            BinOp::FDiv => "fdiv",
            BinOp::URem => "urem",
            BinOp::SRem => "srem",
            BinOp::FRem => "frem",
            BinOp::Shl => "shl",
            BinOp::LShr => "lshr",
            BinOp::AShr => "ashr",
            BinOp::And => "and",
            BinOp::Or => "or",
            BinOp::Xor => "xor",
        }
    }
}

/// LLVM comparison predicate
#[derive(Debug, Clone, Copy)]
pub enum CmpPredicate {
    // Integer comparisons
    Eq,  // Equal
    Ne,  // Not equal
    Ugt, // Unsigned greater than
    Uge, // Unsigned greater or equal
    Ult, // Unsigned less than
    Ule, // Unsigned less or equal
    Sgt, // Signed greater than
    Sge, // Signed greater or equal
    Slt, // Signed less than
    Sle, // Signed less or equal
    // Floating point comparisons
    Oeq, // Ordered and equal
    One, // Ordered and not equal
    Ogt, // Ordered and greater than
    Oge, // Ordered and greater or equal
    Olt, // Ordered and less than
    Ole, // Ordered and less or equal
    Ord, // Ordered (no NaNs)
    Uno, // Unordered (any NaN)
    Ueq, // Unordered and equal
    Ugt2, // Unordered and greater than
    Uge2, // Unordered and greater or equal
    Ult2, // Unordered and less than
    Ule2, // Unordered and less or equal
    Nglt, // Not greater than
    Ngle, // Not greater than or equal
    Ngt, // Not greater than
    Nge, // Not greater than or equal
    Nlt, // Not less than
    Nle, // Not less than or equal
    Nge2, // Not greater than or equal
    Sgt2, // Signed greater than
}

impl CmpPredicate {
    pub fn to_string(&self) -> &'static str {
        match self {
            CmpPredicate::Eq => "eq",
            CmpPredicate::Ne => "ne",
            CmpPredicate::Ugt => "ugt",
            CmpPredicate::Uge => "uge",
            CmpPredicate::Ult => "ult",
            CmpPredicate::Ule => "ule",
            CmpPredicate::Sgt | CmpPredicate::Sgt2 => "sgt",
            CmpPredicate::Sge | CmpPredicate::Sge2 => "sge",
            CmpPredicate::Slt => "slt",
            CmpPredicate::Sle => "sle",
            CmpPredicate::Oeq => "oeq",
            CmpPredicate::One => "one",
            CmpPredicate::Ogt => "ogt",
            CmpPredicate::Oge => "oge",
            CmpPredicate::Olt => "olt",
            CmpPredicate::Ole => "ole",
            CmpPredicate::Ord => "ord",
            CmpPredicate::Uno => "uno",
            CmpPredicate::Ueq => "ueq",
            CmpPredicate::Ugt2 => "ugt",
            CmpPredicate::Uge2 => "uge",
            CmpPredicate::Ult2 => "ult",
            CmpPredicate::Ule2 => "ule",
            CmpPredicate::Nglt => "ngt",
            CmpPredicate::Ngle => "ngt",
            CmpPredicate::Ngt => "ngt",
            CmpPredicate::Nge => "nge",
            CmpPredicate::Nlt => "nlt",
            CmpPredicate::Nle => "nle",
            CmpPredicate::Nge2 => "nge",
        }
    }
}

/// LLVM value reference
#[derive(Debug, Clone)]
pub enum LlvmValue {
    /// Named value
    Named(String),
    /// Immediate integer
    ImmediateInt(i64, LlvmIntType),
    /// Immediate float
    ImmediateFloat(f64, LlvmFloatType),
    /// Null
    Null,
    /// Undef
    Undef,
    /// True
    True,
    /// False
    False,
    /// Zero initializer
    ZeroInitializer,
}

/// LLVM operand
#[derive(Debug, Clone)]
pub enum LlvmOperand {
    /// Value
    Value(LlvmValue),
    /// Constant expression
    Const(LlvmConstExpr),
}

/// LLVM constant expression
#[derive(Debug, Clone)]
pub enum LlvmConstExpr {
    /// Binary operation
    BinOp(BinOp, Box<LlvmOperand>, Box<LlvmOperand>),
    /// Comparison
    ICmp(CmpPredicate, Box<LlvmOperand>, Box<LlvmOperand>),
    /// FCmp(CmpPredicate, Box<LlvmOperand>, Box<LlvmOperand>),
    /// Cast
    Cast(LlvmCastOp, Box<LlvmOperand>, LlvmType),
    /// GetElementPtr
    GEP(bool, Box<LlvmOperand>, Vec<LlvmOperand>),
}

/// LLVM cast operation
#[derive(Debug, Clone, Copy)]
pub enum LlvmCastOp {
    Trunc,
    ZExt,
    SExt,
    FpTrunc,
    FpExt,
    FpToUi,
    FpToSi,
    UiToFp,
    SiToFp,
    PtrToInt,
    IntToPtr,
    BitCast,
}

impl LlvmCastOp {
    pub fn to_string(&self) -> &'static str {
        match self {
            LlvmCastOp::Trunc => "trunc",
            LlvmCastOp::ZExt => "zext",
            LlvmCastOp::SExt => "sext",
            LlvmCastOp::FpTrunc => "fptrunc",
            LlvmCastOp::FpExt => "fpext",
            LlvmCastOp::FpToUi => "fptoui",
            LlvmCastOp::FpToSi => "fptosi",
            LlvmCastOp::UiToFp => "uitofp",
            LlvmCastOp::SiToFp => "sitofp",
            LlvmCastOp::PtrToInt => "ptrtoint",
            LlvmCastOp::IntToPtr => "inttoptr",
            LlvmCastOp::BitCast => "bitcast",
        }
    }
}

/// LLVM instruction
#[derive(Debug, Clone)]
pub enum LlvmInstruction {
    /// Binary operation
    BinOp(BinOp, LlvmType, LlvmOperand, LlvmOperand),
    /// Integer comparison
    ICmp(CmpPredicate, LlvmOperand, LlvmOperand),
    /// Float comparison
    FCmp(CmpPredicate, LlvmOperand, LlvmOperand),
    /// Cast
    Cast(LlvmCastOp, LlvmOperand, LlvmType),
    /// Call
    Call(LlvmType, Option<String>, String, Vec<(LlvmType, LlvmOperand)>, Option<LlvmOperand>),
    /// Return
    Ret(Option<LlvmOperand>),
    /// Branch
    Br(String),
    /// Conditional branch
    CondBr(LlvmOperand, String, String),
    /// Switch
    Switch(LlvmOperand, String, Vec<(LlvmOperand, String)>),
    /// Indirect branch
    IndirectBr(LlvmOperand, Vec<String>),
    /// Invoke
    Invoke(LlvmType, String, Vec<(LlvmType, LlvmOperand)>, String, String),
    /// Resume
    Resume(LlvmOperand),
    /// Unreachable
    Unreachable,
    /// Alloca
    Alloca(LlvmType, Option<u32>),
    /// Load
    Load(LlvmType, LlvmOperand),
    /// Store
    Store(LlvmOperand, LlvmOperand),
    /// GetElementPtr
    GEP(LlvmType, LlvmOperand, Vec<LlvmOperand>),
    /// ExtractValue
    ExtractValue(LlvmOperand, Vec<u32>),
    /// InsertValue
    InsertValue(LlvmOperand, LlvmOperand, Vec<u32>),
    /// ExtractElement
    ExtractElement(LlvmOperand, LlvmOperand),
    /// InsertElement
    InsertElement(LlvmOperand, LlvmOperand, LlvmOperand),
    /// ShuffleVector
    ShuffleVector(LlvmOperand, LlvmOperand, LlvmOperand),
    /// Phi
    Phi(LlvmType, Vec<(LlvmOperand, String)>),
    /// Select
    Select(LlvmOperand, LlvmOperand, LlvmOperand),
    /// LandingPad
    LandingPad(LlvmType, usize, Option<String>),
}

/// Basic block
#[derive(Debug, Clone)]
pub struct LlvmBasicBlock {
    /// Name
    pub name: String,
    /// Instructions
    pub instructions: Vec<LlvmInstruction>,
    /// Terminator (if different from instruction)
    pub terminator: Option<LlvmInstruction>,
}

impl LlvmBasicBlock {
    /// Create new basic block
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            instructions: Vec::new(),
            terminator: None,
        }
    }

    /// Add instruction
    pub fn add(&mut self, inst: LlvmInstruction) {
        self.instructions.push(inst);
    }

    /// Set terminator
    pub fn set_terminator(&mut self, term: LlvmInstruction) {
        self.terminator = Some(term);
    }
}

/// Function argument
#[derive(Debug, Clone)]
pub struct LlvmArg {
    /// Name
    pub name: String,
    /// Type
    pub ty: LlvmType,
    /// Attributes
    pub attrs: Vec<LlvmArgAttr>,
}

impl LlvmArg {
    /// Create new argument
    pub fn new(name: &str, ty: LlvmType) -> Self {
        Self {
            name: name.to_string(),
            ty,
            attrs: Vec::new(),
        }
    }
}

/// Function argument attribute
#[derive(Debug, Clone, Copy)]
pub enum LlvmArgAttr {
    ByVal,
    InReg,
    Nest,
    NoAlias,
    NoCapture,
    NonNull,
    ReadNone,
    ReadOnly,
    Returned,
    SignExt,
    Sret,
    ZeroExt,
}

/// LLVM function
#[derive(Debug, Clone)]
pub struct LlvmFunction {
    /// Name
    pub name: String,
    /// Linkage
    pub linkage: LdLinkage,
    /// Visibility
    pub visibility: LdVisibility,
    /// Return type
    pub ret_type: LlvmType,
    /// Arguments
    pub args: Vec<LlvmArg>,
    /// Basic blocks
    pub blocks: Vec<LlvmBasicBlock>,
    /// Calling convention
    pub call_conv: CallConv,
    /// Attributes
    pub attrs: Vec<LlvmFuncAttr>,
}

impl LlvmFunction {
    /// Create new function
    pub fn new(name: &str, ret_type: LlvmType) -> Self {
        Self {
            name: name.to_string(),
            linkage: LdLinkage::External,
            visibility: LdVisibility::Default,
            ret_type,
            args: Vec::new(),
            blocks: Vec::new(),
            call_conv: CallConv::C,
            attrs: Vec::new(),
        }
    }

    /// Add argument
    pub fn add_arg(&mut self, arg: LlvmArg) {
        self.args.push(arg);
    }

    /// Add basic block
    pub fn add_block(&mut self, block: LlvmBasicBlock) {
        self.blocks.push(block);
    }
}

/// Function attribute
#[derive(Debug, Clone)]
pub enum LlvmFuncAttr {
    AlignStack(u32),
    AlwaysInline,
    Builtin,
    Cold,
    InlineHint,
    JumpTable,
    MinimizeSize,
    Naked,
    NoBuiltin,
    NoCanonical,
    NoDuplicate,
    NoImplicitFloat,
    NoInline,
    NoMerge,
    NoRecurse,
    NoRedZone,
    NoReturn,
    NoSee,
    NoUnwind,
    OptimizeForSize,
    OptSize,
    ReadNone,
    ReadOnly,
    ArgMemOnly,
    ReturnsTwice,
    SafeStack,
    SanitizeAddress,
    SanitizeMemory,
    SanitizeThread,
    StackAlignReq(u32),
    StackProtect,
    StackProtectReq,
    StackProtectStrong,
    StructRet,
    UWTable,
}

/// Global variable
#[derive(Debug, Clone)]
pub struct LlvmGlobal {
    /// Name
    pub name: String,
    /// Linkage
    pub linkage: LdLinkage,
    /// Visibility
    pub visibility: LdVisibility,
    /// Type
    pub ty: LlvmType,
    /// Is constant
    pub constant: bool,
    /// Initial value
    pub initializer: Option<LlvmOperand>,
    /// Alignment
    pub alignment: u32,
}

/// LLVM module
#[derive(Debug, Clone)]
pub struct LlvmModule {
    /// Module name
    pub name: String,
    /// Data layout
    pub data_layout: String,
    /// Target triple
    pub target_triple: String,
    /// Functions
    pub functions: Vec<LlvmFunction>,
    /// Global variables
    pub globals: Vec<LlvmGlobal>,
    /// Named types
    pub named_types: HashMap<String, LlvmType>,
}

impl LlvmModule {
    /// Create new module
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            data_layout: String::new(),
            target_triple: String::new(),
            functions: Vec::new(),
            globals: Vec::new(),
            named_types: HashMap::new(),
        }
    }

    /// Set target
    pub fn set_target(&mut self, triple: &str, layout: &str) {
        self.target_triple = triple.to_string();
        self.data_layout = layout.to_string();
    }

    /// Add function
    pub fn add_function(&mut self, func: LlvmFunction) {
        self.functions.push(func);
    }

    /// Add global
    pub fn add_global(&mut self, global: LlvmGlobal) {
        self.globals.push(global);
    }

    /// Set named type
    pub fn set_named_type(&mut self, name: &str, ty: LlvmType) {
        self.named_types.insert(name.to_string(), ty);
    }
}

/// LLVM IR builder
pub struct LlvmIrBuilder {
    /// Current function
    current_function: Option<String>,
    /// Current block
    current_block: Option<String>,
    /// Value names
    value_names: HashMap<String, String>,
    /// Next value number
    next_value_num: usize,
    /// Basic blocks
    blocks: HashMap<String, Vec<LlvmInstruction>>,
    /// Pending instructions (for later processing)
    pending: Vec<String>,
}

impl LlvmIrBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            current_function: None,
            current_block: None,
            value_names: HashMap::new(),
            next_value_num: 0,
            blocks: HashMap::new(),
            pending: Vec::new(),
        }
    }

    /// Get next value name
    pub fn next_name(&mut self, hint: &str) -> String {
        let name = format!("%{}", hint);
        if !self.value_names.contains_key(&name) {
            self.value_names.insert(name.clone(), name);
            return name;
        }
        let name = format!("%{}.{}", hint, self.next_value_num);
        self.next_value_num += 1;
        self.value_names.insert(name.clone(), name.clone());
        name
    }

    /// Set insertion point
    pub fn set_insert_point(&mut self, block: &str) {
        self.current_block = Some(block.to_string());
        if !self.blocks.contains_key(block) {
            self.blocks.insert(block.to_string(), Vec::new());
        }
    }

    /// Add instruction
    pub fn add_instruction(&mut self, inst: LlvmInstruction) -> String {
        let name = self.next_name("v");
        if let Some(block) = &self.current_block {
            if let Some(insts) = self.blocks.get_mut(block) {
                insts.push(inst);
            }
        }
        name
    }

    /// Build module
    pub fn build(self) -> LlvmModule {
        LlvmModule {
            name: "generated".to_string(),
            data_layout: String::new(),
            target_triple: String::new(),
            functions: Vec::new(),
            globals: Vec::new(),
            named_types: HashMap::new(),
        }
    }

    // ==== Instruction builders ====

    /// Build alloca
    pub fn build_alloca(&mut self, ty: LlvmType) -> String {
        self.add_instruction(LlvmInstruction::Alloca(ty, None))
    }

    /// Build load
    pub fn build_load(&mut self, ptr: &str, ty: LlvmType) -> String {
        let ptr_op = LlvmOperand::Value(LlvmValue::Named(ptr.to_string()));
        self.add_instruction(LlvmInstruction::Load(ty, ptr_op))
    }

    /// Build store
    pub fn build_store(&mut self, value: &str, ptr: &str) {
        let val_op = LlvmOperand::Value(LlvmValue::Named(value.to_string()));
        let ptr_op = LlvmOperand::Value(LlvmValue::Named(ptr.to_string()));
        self.add_instruction(LlvmInstruction::Store(val_op, ptr_op));
    }

    /// Build add
    pub fn build_add(&mut self, lhs: &str, rhs: &str, ty: &LlvmType) -> String {
        let lhs_op = LlvmOperand::Value(LlvmValue::Named(lhs.to_string()));
        let rhs_op = LlvmOperand::Value(LlvmValue::Named(rhs.to_string()));
        self.add_instruction(LlvmInstruction::BinOp(BinOp::Add, ty.clone(), lhs_op, rhs_op))
    }

    /// Build sub
    pub fn build_sub(&mut self, lhs: &str, rhs: &str, ty: &LlvmType) -> String {
        let lhs_op = LlvmOperand::Value(LlvmValue::Named(lhs.to_string()));
        let rhs_op = LlvmOperand::Value(LlvmValue::Named(rhs.to_string()));
        self.add_instruction(LlvmInstruction::BinOp(BinOp::Sub, ty.clone(), lhs_op, rhs_op))
    }

    /// Build mul
    pub fn build_mul(&mut self, lhs: &str, rhs: &str, ty: &LlvmType) -> String {
        let lhs_op = LlvmOperand::Value(LlvmValue::Named(lhs.to_string()));
        let rhs_op = LlvmOperand::Value(LlvmValue::Named(rhs.to_string()));
        self.add_instruction(LlvmInstruction::BinOp(BinOp::Mul, ty.clone(), lhs_op, rhs_op))
    }

    /// Build icmp
    pub fn build_icmp(&mut self, pred: CmpPredicate, lhs: &str, rhs: &str) -> String {
        let lhs_op = LlvmOperand::Value(LlvmValue::Named(lhs.to_string()));
        let rhs_op = LlvmOperand::Value(LlvmValue::Named(rhs.to_string()));
        self.add_instruction(LlvmInstruction::ICmp(pred, lhs_op, rhs_op))
    }

    /// Build ret
    pub fn build_ret(&mut self, value: Option<&str>) {
        let op = value.map(|v| LlvmOperand::Value(LlvmValue::Named(v.to_string())));
        self.add_instruction(LlvmInstruction::Ret(op));
    }

    /// Build br
    pub fn build_br(&mut self, dest: &str) {
        self.add_instruction(LlvmInstruction::Br(dest.to_string()));
    }

    /// Build cond br
    pub fn build_cond_br(&mut self, cond: &str, then_dest: &str, else_dest: &str) {
        let cond_op = LlvmOperand::Value(LlvmValue::Named(cond.to_string()));
        self.add_instruction(LlvmInstruction::CondBr(cond_op, then_dest.to_string(), else_dest.to_string()));
    }

    /// Build call
    pub fn build_call(&mut self, func: &str, args: Vec<(&str, LlvmType)>) -> String {
        let args_ops = args
            .into_iter()
            .map(|(n, t)| (t, LlvmOperand::Value(LlvmValue::Named(n.to_string()))))
            .collect();
        self.add_instruction(LlvmInstruction::Call(
            LlvmType::Int(LlvmIntType::I32),
            None,
            func.to_string(),
            args_ops,
            None,
        ))
    }

    /// Build GEP
    pub fn build_gep(&mut self, ptr: &str, indices: Vec<&str>, ty: LlvmType) -> String {
        let ptr_op = LlvmOperand::Value(LlvmValue::Named(ptr.to_string()));
        let indices_ops: Vec<_> = indices
            .iter()
            .map(|i| LlvmOperand::Value(LlvmValue::Named(i.to_string())))
            .collect();
        self.add_instruction(LlvmInstruction::GEP(ty, ptr_op, indices_ops))
    }
}

impl Default for LlvmIrBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// LLVM IR printer
pub struct LlvmPrinter {
    /// Output string
    output: String,
    /// Indentation level
    indent: usize,
}

impl LlvmPrinter {
    /// Create new printer
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }

    /// Print module
    pub fn print_module(&mut self, module: &LlvmModule) {
        // Header
        if !module.name.is_empty() {
            self.line(&format!("; Module: {}", module.name));
        }

        if !module.target_triple.is_empty() {
            self.line(&format!("target triple = \"{}\"", module.target_triple));
        }

        if !module.data_layout.is_empty() {
            self.line(&format!("target datalayout = \"{}\"", module.data_layout));
        }

        self.empty();

        // Named types
        for (name, ty) in &module.named_types {
            self.line(&format!("%{} = type {}", name, ty.to_string()));
        }

        self.empty();

        // Globals
        for global in &module.globals {
            self.print_global(global);
        }

        self.empty();

        // Functions
        for func in &module.functions {
            self.print_function(func);
        }
    }

    /// Print global variable
    fn print_global(&mut self, global: &LlvmGlobal) {
        let linkage = global.linkage.to_string();
        let vis = global.visibility.to_string();
        let linkage_type = if global.constant { "constant" } else { "global" };
        self.line(&format!(
            "@{} = {} {} {} {}",
            global.name,
            linkage,
            vis,
            linkage_type,
            global.ty.to_string()
        ));
    }

    /// Print function
    fn print_function(&mut self, func: &LlvmFunction) {
        // Function header
        let args = func
            .args
            .iter()
            .map(|a| format!("{} %{}", a.ty.to_string(), a.name))
            .collect::<Vec<_>>()
            .join(", ");

        self.line(&format!(
            "define {} {} @{}({}) {{",
            func.linkage.to_string(),
            func.ret_type.to_string(),
            func.name,
            args
        ));

        self.indent += 1;

        // Basic blocks
        for block in &func.blocks {
            self.print_block(block);
        }

        self.indent -= 1;
        self.line("}");
        self.empty();
    }

    /// Print basic block
    fn print_block(&mut self, block: &LlvmBasicBlock) {
        self.line(&format!("{}:", block.name));

        self.indent += 1;

        for inst in &block.instructions {
            self.print_instruction(inst);
        }

        if let Some(term) = &block.terminator {
            self.print_instruction(term);
        }

        self.indent -= 1;
        self.empty();
    }

    /// Print instruction
    fn print_instruction(&mut self, inst: &LlvmInstruction) {
        let line = match inst {
            LlvmInstruction::BinOp(op, ty, lhs, rhs) => {
                format!("{} = {} {} {}, {}", "%", op.to_string(), ty.to_string(), self.operand_to_string(lhs), self.operand_to_string(rhs))
            }
            LlvmInstruction::Ret(Some(val)) => {
                format!("ret {} {}", LlvmType::Int(LlvmIntType::I32).to_string(), self.operand_to_string(val))
            }
            LlvmInstruction::Ret(None) => "ret void".to_string(),
            LlvmInstruction::Br(dest) => format!("br label %{}", dest),
            LlvmInstruction::CondBr(cond, then_dest, else_dest) => {
                format!("br {} {}, label %{}, label %{}",
                    self.operand_to_string(cond),
                    self.operand_to_string(cond),
                    then_dest,
                    else_dest)
            }
            LlvmInstruction::Alloca(ty, _) => format!("%alloca = alloca {}", ty.to_string()),
            LlvmInstruction::Load(ty, ptr) => {
                format!("%load = load {}, {}* {}", ty.to_string(), ty.to_string(), self.operand_to_string(ptr))
            }
            LlvmInstruction::Store(val, ptr) => {
                format!("store {} {}, {}* {}",
                    LlvmType::Int(LlvmIntType::I32).to_string(),
                    self.operand_to_string(val),
                    LlvmType::Int(LlvmIntType::I32).to_string(),
                    self.operand_to_string(ptr))
            }
            _ => "; unsupported instruction".to_string(),
        };

        self.line(&line);
    }

    /// Convert operand to string
    fn operand_to_string(&self, op: &LlvmOperand) -> String {
        match op {
            LlvmOperand::Value(v) => match v {
                LlvmValue::Named(n) => format!("%{}", n),
                LlvmValue::ImmediateInt(i, _) => i.to_string(),
                LlvmValue::ImmediateFloat(f, _) => format!("{}", f),
                LlvmValue::Null => "null".to_string(),
                LlvmValue::Undef => "undef".to_string(),
                LlvmValue::True => "true".to_string(),
                LlvmValue::False => "false".to_string(),
                LlvmValue::ZeroInitializer => "zeroinitializer".to_string(),
            },
            LlvmOperand::Const(_) => "const".to_string(),
        }
    }

    /// Add line
    fn line(&mut self, s: &str) {
        for _ in 0..self.indent {
            self.output.push_str("  ");
        }
        self.output.push_str(s);
        self.output.push('\n');
    }

    /// Add empty line
    fn empty(&mut self) {
        self.output.push('\n');
    }

    /// Get output
    pub fn finalize(self) -> String {
        self.output
    }
}

impl Default for LlvmPrinter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_llvm_type_to_string() {
        assert_eq!(LlvmType::Int(LlvmIntType::I32).to_string(), "i32");
        assert_eq!(LlvmType::Float(LlvmFloatType::Double).to_string(), "double");
        assert_eq!(LlvmType::Pointer(Box::new(LlvmType::Int(LlvmIntType::I8))).to_string(), "i8*");
    }

    #[test]
    fn test_function_creation() {
        let mut func = LlvmFunction::new("add", LlvmType::Int(LlvmIntType::I32));
        func.add_arg(LlvmArg::new("a", LlvmType::Int(LlvmIntType::I32)));
        func.add_arg(LlvmArg::new("b", LlvmType::Int(LlvmIntType::I32)));

        let mut block = LlvmBasicBlock::new("entry");
        block.add(LlvmInstruction::Alloca(LlvmType::Int(LlvmIntType::I32), None));
        func.add_block(block);

        assert_eq!(func.name, "add");
        assert_eq!(func.args.len(), 2);
    }

    #[test]
    fn test_module_creation() {
        let mut module = LlvmModule::new("test");
        module.set_target("x86_64-unknown-linux-gnu", "e-m:e-i64:64-f80:128-n8:16:32:64-S128");

        let func = LlvmFunction::new("main", LlvmType::Int(LlvmIntType::I32));
        module.add_function(func);

        assert_eq!(module.name, "test");
        assert_eq!(module.functions.len(), 1);
    }

    #[test]
    fn test_ir_builder() {
        let mut builder = LlvmIrBuilder::new();
        let name = builder.next_name("v");
        builder.set_insert_point("entry");
        builder.build_ret(Some(&name));

        assert_eq!(name, "%v");
    }
}
