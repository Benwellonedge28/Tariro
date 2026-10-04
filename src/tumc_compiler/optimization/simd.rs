// SIMD Optimization

use super::*;

/// SIMD operations
#[derive(Debug, Clone)]
pub enum SimdOp {
    // Arithmetic
    Add(IrType),
    Sub(IrType),
    Mul(IrType),
    Div(IrType),
    Min(IrType),
    Max(IrType),
    Neg(IrType),
    Abs(IrType),
    Sqrt(IrType),

    // Logical
    And(IrType),
    Or(IrType),
    Xor(IrType),
    Not,

    // Comparison
    CmpGt,
    CmpGe,
    CmpLt,
    CmpLe,
    CmpEq,
    CmpNe,

    // Shuffles
    Shuffle(Vec<i32>),
    Blend,
    Permute,

    // Memory
    Load { align: usize },
    Store { align: usize },
    Gather { align: usize },
    Scatter { align: usize },

    // Conversions
    Trunc,
    ZExt,
    SExt,
    FPTrunc,
    FPExt,
    SIToFP,
    UIToFP,
    FPToSI,
    FPToUI,
}

/// SIMD intrinsic
#[derive(Debug, Clone)]
pub struct SimdIntrinsic {
    pub name: String,
    pub op: SimdOp,
    pub signature: SimdSignature,
}

#[derive(Debug, Clone)]
pub struct SimdSignature {
    pub input_types: Vec<IrType>,
    pub output_type: IrType,
    pub mask_type: Option<IrType>,
}

/// SIMD target feature detection
pub struct SimdFeatureDetector {
    pub features: SimdFeatures,
}

#[derive(Debug, Clone, Default)]
pub struct SimdFeatures {
    pub sse: bool,
    pub sse2: bool,
    pub sse3: bool,
    pub ssse3: bool,
    pub sse4_1: bool,
    pub sse4_2: bool,
    pub avx: bool,
    pub avx2: bool,
    pub avx512f: bool,
    pub avx512bw: bool,
    pub avx512vl: bool,
    pub neon: bool,
    pub sve: bool,
    pub sve2: bool,
    pub rvv: bool,
}

impl SimdFeatures {
    pub fn detect() -> Self {
        Self::default()
    }

    pub fn supports(&self, feature: &str) -> bool {
        match feature {
            "sse" => self.sse,
            "sse2" => self.sse2,
            "avx" => self.avx,
            "avx2" => self.avx2,
            "avx512f" => self.avx512f,
            "neon" => self.neon,
            "sve" => self.sve,
            "rvv" => self.rvv,
            _ => false,
        }
    }

    pub fn max_vector_bits(&self) -> usize {
        if self.avx512f { 512 }
        else if self.avx2 { 256 }
        else if self.sse { 128 }
        else { 0 }
    }
}

/// SIMD code generator
pub struct SimdCodeGenerator {
    pub target: SimdTarget,
    pub features: SimdFeatures,
}

impl SimdCodeGenerator {
    pub fn new(target: SimdTarget) -> Self {
        Self {
            target,
            features: SimdFeatures::detect(),
        }
    }

    /// Generate SIMD operation
    pub fn generate(&self, op: &SimdOp) -> Vec<String> {
        match op {
            SimdOp::Add(_) => self.generate_add(),
            SimdOp::Mul(_) => self.generate_mul(),
            SimdOp::Load { align } => self.generate_load(*align),
            SimdOp::Store { align } => self.generate_store(*align),
            _ => vec![],
        }
    }

    fn generate_add(&self) -> Vec<String> {
        match self.target {
            SimdTarget::SSE => vec!["addps %xmm0, %xmm1".to_string()],
            SimdTarget::AVX | SimdTarget::AVX2 => vec!["vaddps %ymm0, %ymm1, %ymm2".to_string()],
            SimdTarget::AVX512 => vec!["vaddps %zmm0, %zmm1, %zmm2".to_string()],
            SimdTarget::NEON => vec!["fadd v0.4s, v1.4s, v2.4s".to_string()],
        }
    }

    fn generate_mul(&self) -> Vec<String> {
        match self.target {
            SimdTarget::SSE => vec!["mulps %xmm0, %xmm1".to_string()],
            SimdTarget::AVX | SimdTarget::AVX2 => vec!["vmulps %ymm0, %ymm1, %ymm2".to_string()],
            SimdTarget::AVX512 => vec!["vmulps %zmm0, %zmm1, %zmm2".to_string()],
            SimdTarget::NEON => vec!["fmul v0.4s, v1.4s, v2.4s".to_string()],
        }
    }

    fn generate_load(&self, align: usize) -> Vec<String> {
        match self.target {
            SimdTarget::SSE => {
                if align >= 16 {
                    vec!["movaps (%rdi), %xmm0".to_string()]
                } else {
                    vec!["movups (%rdi), %xmm0".to_string()]
                }
            }
            SimdTarget::AVX | SimdTarget::AVX2 => {
                if align >= 32 {
                    vec!["vmovaps (%rdi), %ymm0".to_string()]
                } else {
                    vec!["vmovups (%rdi), %ymm0".to_string()]
                }
            }
            SimdTarget::AVX512 => vec!["vmovaps (%rdi), %zmm0".to_string()],
            SimdTarget::NEON => vec!["ldr q0, [x0]".to_string()],
        }
    }

    fn generate_store(&self, align: usize) -> Vec<String> {
        match self.target {
            SimdTarget::SSE => {
                if align >= 16 {
                    vec!["movaps %xmm0, (%rdi)".to_string()]
                } else {
                    vec!["movups %xmm0, (%rdi)".to_string()]
                }
            }
            SimdTarget::AVX | SimdTarget::AVX2 => {
                if align >= 32 {
                    vec!["vmovaps %ymm0, (%rdi)".to_string()]
                } else {
                    vec!["vmovups %ymm0, (%rdi)".to_string()]
                }
            }
            SimdTarget::AVX512 => vec!["vmovaps %zmm0, (%rdi)".to_string()],
            SimdTarget::NEON => vec!["str q0, [x0]".to_string()],
        }
    }

    /// Generate gather operation
    pub fn generate_gather(&self, indices: &[ValueId]) -> Vec<String> {
        match self.target {
            SimdTarget::AVX2 => {
                // vpgatherdd
                vec!["vpgatherdd %ymm0, (%rdi,%ymm1,4), %ymm2".to_string()]
            }
            SimdTarget::AVX512 => {
                // vpgatherdd zmm
                vec!["vpgatherdd (%rdi,%zmm1,4), %zmm0".to_string()]
            }
            SimdTarget::NEON => {
                // LDAPR/LD1R for broadcast
                vec!["ld1r {v0.4s}, [x0]".to_string()]
            }
            _ => vec![],
        }
    }

    /// Generate horizontal operations (e.g., hadd for reduction)
    pub fn generate_horizontal(&self, op: &str) -> Vec<String> {
        match self.target {
            SimdTarget::SSE3 | SimdTarget::SSSE3 => {
                vec![format!("h{}", op)]
            }
            SimdTarget::AVX | SimdTarget::AVX2 => {
                // AVX doesn't have hadd, need workaround
                vec![]
            }
            SimdTarget::AVX512 => {
                // Reduction intrinsics
                vec![]
            }
            SimdTarget::NEON => {
                vec![format!("addp v0.4s, v0.4s",))]
            }
        }
    }
}

/// Reduction operation generation
pub struct SimdReduction {
    pub target: SimdTarget,
}

impl SimdReduction {
    pub fn new(target: SimdTarget) -> Self {
        Self { target }
    }

    /// Generate sum reduction
    pub fn gen_sum_reduction(&self, ty: &IrType) -> Vec<String> {
        match ty {
            IrType::Float32 => {
                match self.target {
                    SimdTarget::NEON => vec![
                        "faddp v0.4s, v0.4s".to_string(),
                        "faddp s0, v0.4s".to_string(),
                    ],
                    _ => vec![],
                }
            }
            IrType::Int32 => {
                match self.target {
                    SimdTarget::NEON => vec![
                        "addv s0, v0.4s".to_string(),
                    ],
                    _ => vec![],
                }
            }
            _ => vec![],
        }
    }

    /// Generate min reduction
    pub fn gen_min_reduction(&self, ty: &IrType) -> Vec<String> {
        match ty {
            IrType::Float32 => {
                match self.target {
                    SimdTarget::NEON => vec![
                        "fminnmv s0, v0.4s".to_string(),
                    ],
                    _ => vec![],
                }
            }
            IrType::Int32 => {
                match self.target {
                    SimdTarget::NEON => vec![
                        "minv s0, v0.4s".to_string(),
                    ],
                    _ => vec![],
                }
            }
            _ => vec![],
        }
    }

    /// Generate max reduction
    pub fn gen_max_reduction(&self, ty: &IrType) -> Vec<String> {
        match ty {
            IrType::Float32 => {
                match self.target {
                    SimdTarget::NEON => vec![
                        "fmaxnmv s0, v0.4s".to_string(),
                    ],
                    _ => vec![],
                }
            }
            IrType::Int32 => {
                match self.target {
                    SimdTarget::NEON => vec![
                        "maxv s0, v0.4s".to_string(),
                    ],
                    _ => vec![],
                }
            }
            _ => vec![],
        }
    }
}

/// Masked SIMD operations
pub struct MaskedSimdOperations {
    pub target: SimdTarget,
}

impl MaskedSimdOperations {
    pub fn new(target: SimdTarget) -> Self {
        Self { target }
    }

    /// Generate masked load
    pub fn gen_masked_load(&self, mask_reg: &str, passthru: &str) -> Vec<String> {
        match self.target {
            SimdTarget::AVX512 => vec![
                format!("vmovaps.zmm{{k1}}{{z}}, (%rdi), %zmm0",),
            ],
            _ => vec![],
        }
    }

    /// Generate masked store
    pub fn gen_masked_store(&self, mask_reg: &str) -> Vec<String> {
        match self.target {
            SimdTarget::AVX512 => vec![
                format!("vmovaps.zmm %zmm0, (%rdi){{k1}}",),
            ],
            _ => vec![],
        }
    }
}
