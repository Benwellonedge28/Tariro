// GPU Dialect - GPU Kernel Operations

use super::*;

/// GPU dialect operations
#[derive(Debug, Clone)]
pub enum GpuOp {
    /// GPU kernel launch
    Launch {
        kernel: ValueId,
        grid_size: (ValueId, ValueId, ValueId),
        block_size: (ValueId, ValueId, ValueId),
        args: Vec<ValueId>,
    },
    /// GPU module
    Module {
        name: String,
        kernels: Vec<GpuKernel>,
    },
    /// Allocate GPU memory
    Alloc(ValueId, GpuMemorySpace),
    /// Deallocate GPU memory
    Dealloc(ValueId),
    /// Host to device memory copy
    Memcpy(ValueId, ValueId, ValueId), // dst, src, size
    /// Set device
    SetDevice(usize),
    /// Synchronize
    Sync,
    /// GPU barrier
    Barrier(Option<u32>),
    /// Read/Write lane operations
    Shuffle(Bool, ValueId, ValueId, ValueId), // width, mask, src_lane
}

/// GPU kernel definition
#[derive(Debug, Clone)]
pub struct GpuKernel {
    pub name: String,
    pub body: Vec<GpuKernelOp>,
    pub grid_size: (usize, usize, usize),
    pub block_size: (usize, usize, usize),
    pub shared_mem_size: usize,
}

#[derive(Debug, Clone)]
pub enum GpuKernelOp {
    /// Get thread/block index
    ThreadIdx(GpuDim),
    BlockIdx(GpuDim),
    BlockDim(GpuDim),
    GridDim(GpuDim),
    /// Shared memory operations
    SharedLoad(ValueId, ValueId),  // memref, index
    SharedStore(ValueId, ValueId, ValueId), // value, memref, index
    /// Synchronization
    Barrier,
    /// Memory fence
    Fence(MemoryOrdering),
    /// Vector lane operations
    Vote(VoteOp, ValueId),
    /// Atomic operations
    AtomicRMW(AtomicRMWOp, ValueId, ValueId, MemoryOrdering),
}

#[derive(Debug, Clone, Copy)]
pub enum GpuDim {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub enum VoteOp {
    All,
    Any,
    Ballot,
}

#[derive(Debug, Clone)]
pub enum GpuMemorySpace {
    Global,
    Shared,
    Private,
    Constant,
    Local,
}

impl Dialect for GpuDialect {
    fn name(&self) -> &'static str {
        "gpu"
    }

    fn from_standard(&mut self, module: &Module) -> Result<(), DialectError> {
        // Convert standard IR to GPU operations
        for func in &module.functions {
            if func.attributes.hot || func.name.contains("kernel") {
                // This might be a GPU kernel
                let kernel = GpuKernel {
                    name: func.name.clone(),
                    body: Vec::new(),
                    grid_size: (1, 1, 1),
                    block_size: (256, 1, 1),
                    shared_mem_size: 0,
                };
                self.kernels.push(kernel);
            }
        }
        Ok(())
    }

    fn to_standard(&self) -> Result<Module, DialectError> {
        let mut module = Module::new("gpu_conversion");

        for kernel in &self.kernels {
            let mut func = Function::new(FunctionId(0), &kernel.name, IrType::Void);
            func.attributes.hot = true;

            let mut block = BasicBlock::new(BasicBlockId(0));

            // Emit kernel body
            for op in &kernel.body {
                match op {
                    GpuKernelOp::ThreadIdx(dim) => {
                        // Intrinsic call to get thread index
                        let inst = Instruction::new(
                            Operation::Intrinsic(
                                format!("llvm.nvvm.read.ptx.s.tid.{}", match dim {
                                    GpuDim::X => "x",
                                    GpuDim::Y => "y",
                                    GpuDim::Z => "z",
                                }).to_string(),
                                Vec::new(),
                            ),
                            IrType::Uint32,
                        );
                        block.instructions.push(inst);
                    }
                    GpuKernelOp::BlockIdx(dim) => {
                        let inst = Instruction::new(
                            Operation::Intrinsic(
                                format!("llvm.nvvm.read.ptx.s.ctaid.{}", match dim {
                                    GpuDim::X => "x",
                                    GpuDim::Y => "y",
                                    GpuDim::Z => "z",
                                }).to_string(),
                                Vec::new(),
                            ),
                            IrType::Uint32,
                        );
                        block.instructions.push(inst);
                    }
                    _ => {}
                }
            }

            block.terminator = Terminator::Ret(None);
            func.body.push(block);
            module.functions.push(func);
        }

        Ok(module)
    }
}

pub struct GpuDialect {
    pub kernels: Vec<GpuKernel>,
    pub current_device: usize,
}

impl GpuDialect {
    pub fn new() -> Self {
        Self {
            kernels: Vec::new(),
            current_device: 0,
        }
    }

    pub fn add_kernel(&mut self, kernel: GpuKernel) {
        self.kernels.push(kernel);
    }

    pub fn launch(&mut self, kernel: ValueId, grid: (ValueId, ValueId, ValueId), block: (ValueId, ValueId, ValueId), args: Vec<ValueId>) {
        self.operations.push(GpuOp::Launch {
            kernel,
            grid_size: grid,
            block_size: block,
            args,
        });
    }
}

impl Default for GpuDialect {
    fn default() -> Self {
        Self::new()
    }
}

/// GPU backend targets
#[derive(Debug, Clone)]
pub enum GpuBackend {
    /// NVIDIA CUDA
    Cuda,
    /// AMD ROCm/HIP
    Rocm,
    /// OpenCL
    OpenCL,
    /// Level Zero
    LevelZero,
    /// SPIR-V for general GPU
    SpirV,
}

impl GpuBackend {
    pub fn triple(&self) -> &'static str {
        match self {
            GpuBackend::Cuda => "nvptx64-nvidia-cuda",
            GpuBackend::Rocm => "amdgcn-amd-amdhsa",
            GpuBackend::OpenCL => "spir64-unknown-opencl",
            GpuBackend::LevelZero => "spir64-unknown-unknown",
            GpuBackend::SpirV => "spirv64-unknown-vulkan",
        }
    }
}
