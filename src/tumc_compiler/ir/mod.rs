// TUMC Intermediate Representations Module
// Provides multiple IR levels: SSA, Control Flow Graph, and Sea of Nodes

pub mod ssa;
pub mod cfg;
pub mod sea_of_nodes;
pub mod dialects;

pub use ssa::*;
pub use cfg::*;
pub use sea_of_nodes::*;
pub use dialects::*;

// ============================================================================
// Common IR Types
// ============================================================================

/// Value types in the IR
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrType {
    /// Void/no value type
    Void,
    /// Boolean
    Bool,
    /// 8-bit integer
    Int8,
    /// 16-bit integer
    Int16,
    /// 32-bit integer
    Int32,
    /// 64-bit integer
    Int64,
    /// 128-bit integer
    Int128,
    /// Arbitrary width integer
    Int(usize),
    /// 8-bit unsigned integer
    Uint8,
    /// 16-bit unsigned integer
    Uint16,
    /// 32-bit unsigned integer
    Uint32,
    /// 64-bit unsigned integer
    Uint64,
    /// 128-bit unsigned integer
    Uint128,
    /// Pointer type
    Pointer(Box<IrType>),
    /// Floating point
    Float16,
    Float32,
    Float64,
    /// SIMD vector type
    Vector(Box<IrType>, usize),
    /// Function pointer
    FuncPtr(Vec<IrType>, Box<IrType>),
    /// Array type
    Array(Box<IrType>, usize),
    /// Structure type
    Struct(Vec<IrType>),
    /// Named type reference
    Named(String),
    /// Runtime type (dynamic)
    Runtime,
}

impl IrType {
    pub fn size(&self) -> Option<usize> {
        match self {
            IrType::Void => Some(0),
            IrType::Bool | IrType::Int8 | IrType::Uint8 => Some(1),
            IrType::Int16 | IrType::Uint16 => Some(2),
            IrType::Int32 | IrType::Uint32 | IrType::Float32 => Some(4),
            IrType::Int64 | IrType::Uint64 | IrType::Float64 => Some(8),
            IrType::Int128 | IrType::Uint128 => Some(16),
            IrType::Float16 => Some(2),
            IrType::Pointer(_) => Some(8), // 64-bit pointer
            IrType::Vector(ety, len) => ety.size().map(|s| s * len),
            IrType::Array(ety, len) => ety.size().map(|s| s * len),
            IrType::Struct(types) => types.iter().map(|t| t.size()).fold(Some(0), |acc, s| {
                acc.and_then(|a| s.map(|s| a + s))
            }),
            _ => None,
        }
    }

    pub fn alignment(&self) -> usize {
        self.size().unwrap_or(1).next_power_of_two().min(16)
    }

    pub fn is_integer(&self) -> bool {
        matches!(self,
            IrType::Int8 | IrType::Int16 | IrType::Int32 | IrType::Int64 |
            IrType::Int128 | IrType::Int(_) |
            IrType::Uint8 | IrType::Uint16 | IrType::Uint32 | IrType::Uint64 | IrType::Uint128
        )
    }

    pub fn is_float(&self) -> bool {
        matches!(self, IrType::Float16 | IrType::Float32 | IrType::Float64)
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, IrType::Pointer(_))
    }

    pub fn is_vector(&self) -> bool {
        matches!(self, IrType::Vector(_, _))
    }
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOp {
    // Arithmetic
    Add, Sub, Mul, Div, Rem,
    // Bitwise
    And, Or, Xor, Shl, Shr, Rotl, Rotr,
    // Comparison
    Eq, Ne, Lt, Le, Gt, Ge,
    // Floating point
    FAdd, FSub, FMul, FDiv, FRem,
    FEq, FNe, FLt, FLe, FGt, FGe,
}

/// Unary operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnOp {
    Neg, Not, BitNot,
    // Floating point
    FNeg, FAbs, FSqrt, FCeil, FFloor, FRound, FTrunc, FExp, FLog, FPow,
}

/// Memory operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemOp {
    Load, Store,
    LoadAtomic, StoreAtomic,
    Cmpxchg, AtomicRMW,
}

/// Memory ordering for atomics
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryOrdering {
    NotAtomic,
    Unordered,
    Monotonic,
    Acquire,
    Release,
    AcqRel,
    SeqCst,
}

/// Calling conventions
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CallConv {
    /// C calling convention
    C,
    /// Fastcall (Windows x64)
    Fastcall,
    /// System V AMD64 ABI
    SysV64,
    /// Microsoft x64 ABI
    MsAbi,
    /// ARM AAPCS
    AAPCS,
    /// WebAssembly
    Wasm,
    /// JVM bytecode
    JVM,
    /// TUMC internal
    Tumc,
    /// Tail call optimized
    Tail,
}

/// Function attributes
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct FunctionAttributes {
    pub inline: Option<InlineAttribute>,
    pub optimize: bool,
    pub cold: bool,
    pub hot: bool,
    pub no_return: bool,
    pub no_unwind: bool,
    pub read_none: bool,
    pub read_only: bool,
    pub write_only: bool,
    pub stack_align: Option<usize>,
    pub guard: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InlineAttribute {
    Always,
    Never,
    Hint,
}

/// Linkage types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Linkage {
    Private,
    Internal,
    AvailableExternally,
    Common,
    LinkOnce,
    Weak,
    LinkOnceODR,
    WeakODR,
    External,
    DLLImport,
    DLLExport,
}

/// Visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Visibility {
    Default,
    Hidden,
    Protected,
}

/// Section for code placement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Section {
    Text,
    Data,
    Bss,
    Rodata,
    Tdata,
    Tbss,
    Custom(String),
}

// ============================================================================
// Value Representations
// ============================================================================

/// A value in the IR (SSA variable)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Value {
    pub id: ValueId,
    pub name: Option<String>,
    pub ty: IrType,
}

impl Value {
    pub fn new(id: ValueId, ty: IrType) -> Self {
        Self { id, name: None, ty }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
}

/// Unique identifier for values
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ValueId(pub usize);

/// Extended SSA value that can represent constants, instructions, etc.
#[derive(Debug, Clone)]
pub enum ValueDef {
    /// Undefined value
    Undef,
    /// Null pointer
    Null,
    /// Integer constant
    ConstInt(i64, IrType),
    /// Float constant
    ConstFloat(f64, IrType),
    /// String constant
    ConstString(String),
    /// Vector constant
    ConstVector(Vec<ValueDef>),
    /// Zero initializer
    ZeroInit,
    /// Reference to an instruction result
    Instruction(InstId),
    /// Reference to a function argument
    Argument(usize),
    /// Reference to a global value
    Global(GlobalId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlobalId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BasicBlockId(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FunctionId(pub usize);

// ============================================================================
// Module and Program Representation
// ============================================================================

/// Global variable
#[derive(Debug, Clone)]
pub struct GlobalVar {
    pub id: GlobalId,
    pub name: String,
    pub ty: IrType,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub thread_local: bool,
    pub constant: bool,
    pub initializer: Option<Vec<ValueDef>>,
    pub section: Option<Section>,
}

/// Function definition
#[derive(Debug, Clone)]
pub struct Function {
    pub id: FunctionId,
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub ret_type: IrType,
    pub body: Vec<BasicBlock>,
    pub attributes: FunctionAttributes,
    pub linkage: Linkage,
    pub visibility: Visibility,
    pub calling_conv: CallConv,
    pub metadata: HashMap<String, Metadata>,
}

impl Function {
    pub fn new(id: FunctionId, name: impl Into<String>, ret_type: IrType) -> Self {
        Self {
            id,
            name: name.into(),
            params: Vec::new(),
            ret_type,
            body: Vec::new(),
            attributes: FunctionAttributes::default(),
            linkage: Linkage::External,
            visibility: Visibility::Default,
            calling_conv: CallConv::C,
            metadata: HashMap::new(),
        }
    }

    pub fn add_param(&mut self, name: impl Into<String>, ty: IrType) -> usize {
        let idx = self.params.len();
        self.params.push((name.into(), ty));
        idx
    }

    pub fn entry_block(&self) -> BasicBlockId {
        BasicBlockId(0)
    }

    pub fn basic_blocks(&self) -> &[BasicBlock] {
        &self.body
    }

    pub fn basic_blocks_mut(&mut self) -> &mut Vec<BasicBlock> {
        &mut self.body
    }
}

/// Basic block
#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub name: Option<String>,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

impl BasicBlock {
    pub fn new(id: BasicBlockId) -> Self {
        Self {
            id,
            name: None,
            instructions: Vec::new(),
            terminator: Terminator::Unreachable,
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn add_inst(&mut self, inst: Instruction) -> InstId {
        let id = InstId(self.instructions.len());
        self.instructions.push(inst);
        id
    }
}

/// Instruction representation
#[derive(Debug, Clone)]
pub struct Instruction {
    pub id: InstId,
    pub op: Operation,
    pub result_type: IrType,
    pub comments: Vec<String>,
}

impl Instruction {
    pub fn new(op: Operation, result_type: IrType) -> Self {
        Self {
            id: InstId(0), // Set by basic block
            op,
            result_type,
            comments: Vec::new(),
        }
    }
}

/// Operations (SSA instructions)
#[derive(Debug, Clone)]
pub enum Operation {
    // Control flow
    /// Phi node for SSA merging
    Phi(Vec<(ValueId, BasicBlockId)>, IrType),
    /// Parameter reference
    Param(usize),

    // Arithmetic
    Binary(BinOp, ValueId, ValueId),
    Unary(UnOp, ValueId),

    // Memory operations
    Alloca(IrType, Option<usize>),  // type, array size
    Load(MemOp, ValueId, IrType, MemoryOrdering),
    Store(MemOp, ValueId, ValueId, MemoryOrdering),
    LoadElementPtr(ValueId, Vec<ValueId>),
    LoadFieldPtr(ValueId, usize),

    // Function calls
    Call(FunctionId, Vec<ValueId>),
    CallIndirect(ValueId, Vec<ValueId>),  // fn ptr, args
    InlineAsm(String, Vec<ValueId>, Vec<(String, ValueId)>),  // asm, args, clobbers

    // Type conversions
    Cast(ValueId, IrType, CastKind),
    Bitcast(ValueId, IrType),
    IntToPtr(ValueId),
    PtrToInt(ValueId),
    Trunc(ValueId, IrType),
    ZExt(ValueId, IrType),
    SExt(ValueId, IrType),
    FPTrunc(ValueId, IrType),
    FPExt(ValueId, IrType),
    FPToInt(ValueId, IrType),
    IntToFP(ValueId, IrType),

    // Comparison
    ICmp(IntCmpOp, ValueId, ValueId),
    FCmp(FloatCmpOp, ValueId, ValueId),

    // Vector operations
    ExtractElement(ValueId, ValueId),
    InsertElement(ValueId, ValueId, ValueId),
    ShuffleVector(ValueId, ValueId, Vec<i32>),

    // Aggregate operations
    ExtractValue(ValueId, Vec<usize>),
    InsertValue(ValueId, ValueId, Vec<usize>),

    // Atomic operations
    AtomicRMW(AtomicRMWOp, ValueId, ValueId, MemoryOrdering),
    Cmpxchg(ValueId, ValueId, ValueId, MemoryOrdering, bool),

    // Fence
    Fence(MemoryOrdering),

    // Virtual register operations
    PhiPseudo(Vec<(ValueId, BasicBlockId)>, IrType),  // For non-SSA lowering

    // Debug information
    DebugValue(ValueId, DbgInfo),

    // Intrinsic calls
    Intrinsic(String, Vec<ValueId>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CastKind {
    Bitcast,
    IntToPtr,
    PtrToInt,
    IntToFP,
    FPToInt,
    FPTrunc,
    FPExt,
    Trunc,
    ZExt,
    SExt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntCmpOp {
    Eq, Ne, UgT, UgE, ULt, ULe,
    SgT, SgE, SLt, SLe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatCmpOp {
    False, OEq, Ogt, Oge, Olt, Ole, ONE,
    ORd, UGT, UGE, ULT, ULE, UNE, OR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AtomicRMWOp {
    Xchg, Add, Sub, And, Nand, Or, Xor, Max, Min, UMax, UMin,
}

/// Terminator instructions
#[derive(Debug, Clone)]
pub enum Terminator {
    /// Unconditional branch
    Br(BasicBlockId),
    /// Conditional branch
    BrCond(ValueId, BasicBlockId, BasicBlockId),
    /// Switch statement
    Switch(ValueId, BasicBlockId, Vec<(ValueId, BasicBlockId)>),
    /// Indirect branch (jump table)
    IndirectBr(ValueId, Vec<BasicBlockId>),
    /// Return value
    Ret(Option<ValueId>),
    /// Return from function with unwind
    RetUnwind(Option<ValueId>),
    /// Resume unwinding
    Resume(ValueId),
    /// Catch switch for exception handling
    CatchSwitch(ValueId, BasicBlockId, Vec<BasicBlockId>),
    /// Return from landing pad
    CatchRet(ValueId, BasicBlockId),
    /// Cleanup return
    CleanupRet(ValueId, Option<BasicBlockId>),
    /// Unreachable
    Unreachable,
    /// Invoke (with exception handling)
    Invoke(FunctionId, Vec<ValueId>, BasicBlockId, BasicBlockId),
}

/// Debug information
#[derive(Debug, Clone)]
pub struct DbgInfo {
    pub source_file: String,
    pub line: u32,
    pub column: u32,
    pub discriminator: u32,
}

/// Metadata
#[derive(Debug, Clone)]
pub enum Metadata {
    String(String),
    Int(i64),
    Float(f64),
    Type(IrType),
    Value(ValueId),
    Node(Vec<(String, Metadata)>),
    DIHandle(DIHandle),
}

#[derive(Debug, Clone)]
pub struct DIHandle(pub usize);

/// Complete compilation unit/module
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub source_language: String,
    pub target_triple: String,
    pub data_layout: DataLayout,
    pub globals: Vec<GlobalVar>,
    pub functions: Vec<Function>,
    pub metadata: HashMap<String, Metadata>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            source_language: "TUMC".to_string(),
            target_triple: "x86_64-unknown-unknown".to_string(),
            data_layout: DataLayout::default(),
            globals: Vec::new(),
            functions: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, func: Function) -> FunctionId {
        let id = FunctionId(self.functions.len());
        self.functions.push(func);
        id
    }

    pub fn add_global(&mut self, global: GlobalVar) -> GlobalId {
        let id = GlobalId(self.globals.len());
        self.globals.push(global);
        id
    }
}

/// Target data layout
#[derive(Debug, Clone)]
pub struct DataLayout {
    pub endian: Endianness,
    pub pointer_size: usize,
    pub pointer_abi_align: usize,
    pub aggregate_align: usize,
    pub vector_align: usize,
    pub function_align: usize,
    pub stack_align: usize,
    pub primitive_sizes: HashMap<IrType, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
}

impl Default for DataLayout {
    fn default() -> Self {
        let mut primitive_sizes = HashMap::new();
        primitive_sizes.insert(IrType::Int8, 1);
        primitive_sizes.insert(IrType::Int16, 2);
        primitive_sizes.insert(IrType::Int32, 4);
        primitive_sizes.insert(IrType::Int64, 8);
        primitive_sizes.insert(IrType::Float32, 4);
        primitive_sizes.insert(IrType::Float64, 8);

        Self {
            endian: Endianness::Little,
            pointer_size: 8,
            pointer_abi_align: 8,
            aggregate_align: 8,
            vector_align: 16,
            function_align: 16,
            stack_align: 16,
            primitive_sizes,
        }
    }
}

// ============================================================================
// Transformation Helpers
// ============================================================================

/// Dominator tree information
#[derive(Debug, Clone)]
pub struct DominatorTree {
    pub tree: Vec<DominatorNode>,
}

#[derive(Debug, Clone)]
pub struct DominatorNode {
    pub block: BasicBlockId,
    pub parent: Option<BasicBlockId>,
    pub children: Vec<BasicBlockId>,
    pub immediate_dominator: Option<BasicBlockId>,
}

impl DominatorTree {
    pub fn new(function: &Function) -> Self {
        let mut tree = Vec::with_capacity(function.body.len());
        for (i, block) in function.body.iter().enumerate() {
            tree.push(DominatorNode {
                block: block.id,
                parent: None,
                children: Vec::new(),
                immediate_dominator: if i == 0 { None } else { Some(BasicBlockId(0)) },
            });
        }
        Self { tree }
    }

    pub fn idom(&self, block: BasicBlockId) -> Option<BasicBlockId> {
        self.tree.get(block.0)?.immediate_dominator
    }

    pub fn dominates(&self, dominator: BasicBlockId, dominated: BasicBlockId) -> bool {
        let mut current = Some(dominated);
        while let Some(block) = current {
            if block == dominator {
                return true;
            }
            current = self.idom(block);
        }
        false
    }
}

/// Loop tree information
#[derive(Debug, Clone)]
pub struct LoopTree {
    pub loops: Vec<Loop>,
}

#[derive(Debug, Clone)]
pub struct Loop {
    pub header: BasicBlockId,
    pub blocks: Vec<BasicBlockId>,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl LoopTree {
    pub fn new() -> Self {
        Self { loops: Vec::new() }
    }
}

/// CFG analysis results
#[derive(Debug, Clone)]
pub struct CfgAnalysis {
    pub predecessor_map: HashMap<BasicBlockId, Vec<BasicBlockId>>,
    pub successor_map: HashMap<BasicBlockId, Vec<BasicBlockId>>,
    pub dominator_tree: DominatorTree,
    pub post_dominator_tree: DominatorTree,
    pub loop_tree: LoopTree,
    pub topological_order: Vec<BasicBlockId>,
    pub reverse_post_order: Vec<BasicBlockId>,
}

impl CfgAnalysis {
    pub fn new(function: &Function) -> Self {
        let mut predecessor_map: HashMap<BasicBlockId, Vec<BasicBlockId>> = HashMap::new();
        let mut successor_map: HashMap<BasicBlockId, Vec<BasicBlockId>> = HashMap::new();

        for block in &function.body {
            predecessor_map.entry(block.id).or_default();
            successor_map.entry(block.id).or_default();
        }

        for block in &function.body {
            let successors = match &block.terminator {
                Terminator::Br(target) => vec![*target],
                Terminator::BrCond(_, then, else_) => vec![*then, *else_],
                Terminator::Switch(_, default, cases) => {
                    let mut targets: Vec<_> = cases.iter().map(|(_, b)| *b).collect();
                    targets.push(*default);
                    targets
                }
                Terminator::IndirectBr(_, targets) => targets.clone(),
                Terminator::Invoke(_, _, normal, unwind) => vec![*normal, *unwind],
                _ => Vec::new(),
            };

            for &target in &successors {
                predecessor_map.entry(target).or_default().push(block.id);
                successor_map.get_mut(&block.id).unwrap().push(target);
            }
        }

        Self {
            predecessor_map,
            successor_map,
            dominator_tree: DominatorTree::new(function),
            post_dominator_tree: DominatorTree::new(function),
            loop_tree: LoopTree::new(),
            topological_order: Vec::new(),
            reverse_post_order: Vec::new(),
        }
    }
}

// ============================================================================
// IR Builder
// ============================================================================

/// Builder for constructing IR
pub struct IrBuilder {
    pub current_block: BasicBlockId,
    pub function: Option<FunctionId>,
    instructions: Vec<Instruction>,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self {
            current_block: BasicBlockId(0),
            function: None,
            instructions: Vec::new(),
        }
    }

    pub fn set_block(&mut self, block: BasicBlockId) {
        self.current_block = block;
    }

    pub fn build_alloca(&mut self, ty: IrType, name: Option<String>) -> ValueId {
        let inst = Instruction::new(
            Operation::Alloca(ty.clone(), None),
            IrType::Pointer(Box::new(ty)),
        );
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_load(&mut self, ptr: ValueId, ty: IrType) -> ValueId {
        let inst = Instruction::new(
            Operation::Load(MemOp::Load, ptr, ty.clone(), MemoryOrdering::NotAtomic),
            ty,
        );
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_store(&mut self, ptr: ValueId, val: ValueId) {
        let inst = Instruction::new(
            Operation::Store(MemOp::Store, ptr, val, MemoryOrdering::NotAtomic),
            IrType::Void,
        );
        self.instructions.push(inst);
    }

    pub fn build_add(&mut self, lhs: ValueId, rhs: ValueId, ty: IrType) -> ValueId {
        let op = if ty.is_float() { BinOp::FAdd } else { BinOp::Add };
        let inst = Instruction::new(Operation::Binary(op, lhs, rhs), ty);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_sub(&mut self, lhs: ValueId, rhs: ValueId, ty: IrType) -> ValueId {
        let op = if ty.is_float() { BinOp::FSub } else { BinOp::Sub };
        let inst = Instruction::new(Operation::Binary(op, lhs, rhs), ty);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_mul(&mut self, lhs: ValueId, rhs: ValueId, ty: IrType) -> ValueId {
        let op = if ty.is_float() { BinOp::FMul } else { BinOp::Mul };
        let inst = Instruction::new(Operation::Binary(op, lhs, rhs), ty);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_div(&mut self, lhs: ValueId, rhs: ValueId, ty: IrType) -> ValueId {
        let op = if ty.is_float() { BinOp::FDiv } else { BinOp::Div };
        let inst = Instruction::new(Operation::Binary(op, lhs, rhs), ty);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_icmp(&mut self, op: IntCmpOp, lhs: ValueId, rhs: ValueId) -> ValueId {
        let inst = Instruction::new(Operation::ICmp(op, lhs, rhs), IrType::Bool);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_fcmp(&mut self, op: FloatCmpOp, lhs: ValueId, rhs: ValueId) -> ValueId {
        let inst = Instruction::new(Operation::FCmp(op, lhs, rhs), IrType::Bool);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_br(&mut self, target: BasicBlockId) -> Terminator {
        Terminator::Br(target)
    }

    pub fn build_cond_br(&mut self, cond: ValueId, then: BasicBlockId, else_: BasicBlockId) -> Terminator {
        Terminator::BrCond(cond, then, else_)
    }

    pub fn build_ret(&mut self, val: Option<ValueId>) -> Terminator {
        Terminator::Ret(val)
    }

    pub fn build_call(&mut self, func: FunctionId, args: Vec<ValueId>, ret_type: IrType) -> ValueId {
        let inst = Instruction::new(Operation::Call(func, args), ret_type);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_sext(&mut self, val: ValueId, to_type: IrType) -> ValueId {
        let inst = Instruction::new(Operation::SExt(val, to_type.clone()), to_type);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_zext(&mut self, val: ValueId, to_type: IrType) -> ValueId {
        let inst = Instruction::new(Operation::ZExt(val, to_type.clone()), to_type);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_trunc(&mut self, val: ValueId, to_type: IrType) -> ValueId {
        let inst = Instruction::new(Operation::Trunc(val, to_type.clone()), to_type);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_bitcast(&mut self, val: ValueId, to_type: IrType) -> ValueId {
        let inst = Instruction::new(Operation::Bitcast(val, to_type.clone()), to_type);
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_gep(&mut self, ptr: ValueId, indices: Vec<ValueId>) -> ValueId {
        let inst = Instruction::new(
            Operation::LoadElementPtr(ptr, indices),
            IrType::Pointer(Box::new(IrType::Void)), // Computed from types
        );
        let id = ValueId(self.instructions.len());
        self.instructions.push(inst);
        id
    }

    pub fn build_unreachable(&mut self) -> Terminator {
        Terminator::Unreachable
    }

    pub fn finalize(self) -> Vec<Instruction> {
        self.instructions
    }
}

impl Default for IrBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Pass Infrastructure
// ============================================================================

/// Trait for IR passes
pub trait IrPass {
    fn name(&self) -> &str;
    fn run(&self, module: &mut Module) -> bool;
}

/// Pass manager
pub struct PassManager {
    passes: Vec<Box<dyn IrPass>>,
}

impl PassManager {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn add_pass<P: IrPass + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    pub fn run(&self, module: &mut Module) {
        for pass in &self.passes {
            pass.run(module);
        }
    }
}

impl Default for PassManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ir_type_sizing() {
        assert_eq!(IrType::Int32.size(), Some(4));
        assert_eq!(IrType::Float64.size(), Some(8));
        assert_eq!(IrType::Pointer(Box::new(IrType::Int32)).size(), Some(8));
        assert_eq!(IrType::Vector(Box::new(IrType::Float32), 4).size(), Some(16));
    }

    #[test]
    fn test_data_layout() {
        let layout = DataLayout::default();
        assert_eq!(layout.pointer_size, 8);
        assert_eq!(layout.endian, Endianness::Little);
    }

    #[test]
    fn test_function_builder() {
        let mut func = Function::new(FunctionId(0), "test", IrType::Int32);
        func.add_param("x", IrType::Int32);
        func.add_param("y", IrType::Int32);

        assert_eq!(func.params.len(), 2);
        assert_eq!(func.ret_type, IrType::Int32);
    }

    #[test]
    fn test_ir_builder() {
        let mut builder = IrBuilder::new();

        let alloca = builder.build_alloca(IrType::Int32, None);
        assert!(matches!(builder.instructions[0].op, Operation::Alloca(_, _)));

        let load = builder.build_load(alloca, IrType::Int32);
        assert!(matches!(builder.instructions[1].op, Operation::Load(_, _, _, _)));
    }
}
