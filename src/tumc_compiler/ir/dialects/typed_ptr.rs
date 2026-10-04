// Typed Pointer Dialect

use super::*;

/// Typed pointer operations
#[derive(Debug, Clone)]
pub enum TypedPtrOp {
    /// Allocate with type
    TypedAlloc(IrType),
    /// Load with type
    TypedLoad(ValueId, IrType),
    /// Store with type
    TypedStore(ValueId, ValueId, IrType),
    /// GEP with result type
    TypedGep(ValueId, Vec<ValueId>, IrType),
    /// Type cast
    TypeBitcast(ValueId, IrType),
}

impl Dialect for TypedPtrDialect {
    fn name(&self) -> &'static str {
        "typed_ptr"
    }

    fn from_standard(&mut self, module: &Module) -> Result<(), DialectError> {
        // Convert standard IR load/store to typed operations
        for func in &module.functions {
            for block in &func.body {
                for inst in &block.instructions {
                    match &inst.op {
                        Operation::Load(_, ptr, ty, _) => {
                            self.operations.push(TypedPtrOp::TypedLoad(*ptr, ty.clone()));
                        }
                        Operation::Store(_, ptr, val, _) => {
                            self.operations.push(TypedPtrOp::TypedStore(*ptr, *val, inst.result_type.clone()));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn to_standard(&self) -> Result<Module, DialectError> {
        let mut module = Module::new("converted");
        let mut func = Function::new(FunctionId(0), "converted_fn", IrType::Void);

        let entry = BasicBlock::new(BasicBlockId(0));
        func.body.push(entry);

        for op in &self.operations {
            match op {
                TypedPtrOp::TypedAlloc(ty) => {
                    func.body[0].instructions.push(Instruction::new(
                        Operation::Alloca(ty.clone(), None),
                        IrType::Pointer(Box::new(ty.clone())),
                    ));
                }
                TypedPtrOp::TypedLoad(ptr, ty) => {
                    func.body[0].instructions.push(Instruction::new(
                        Operation::Load(MemOp::Load, *ptr, ty.clone(), MemoryOrdering::NotAtomic),
                        ty.clone(),
                    ));
                }
                TypedPtrOp::TypedStore(ptr, val, ty) => {
                    func.body[0].instructions.push(Instruction::new(
                        Operation::Store(MemOp::Store, *ptr, *val, MemoryOrdering::NotAtomic),
                        IrType::Void,
                    ));
                }
                TypedPtrOp::TypedGep(ptr, indices, result_ty) => {
                    func.body[0].instructions.push(Instruction::new(
                        Operation::LoadElementPtr(*ptr, indices.clone()),
                        result_ty.clone(),
                    ));
                }
                TypedPtrOp::TypeBitcast(val, ty) => {
                    func.body[0].instructions.push(Instruction::new(
                        Operation::Bitcast(*val, ty.clone()),
                        ty.clone(),
                    ));
                }
            }
        }

        module.functions.push(func);
        Ok(module)
    }
}

pub struct TypedPtrDialect {
    pub operations: Vec<TypedPtrOp>,
}

impl TypedPtrDialect {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }
}

impl Default for TypedPtrDialect {
    fn default() -> Self {
        Self::new()
    }
}
