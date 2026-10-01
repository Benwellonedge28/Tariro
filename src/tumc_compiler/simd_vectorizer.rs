//! SIMD vectorization for TUMC
//! Generates vectorized code using SIMD instructions (SSE, AVX, NEON)

use crate::prelude::*;
use super::ast::*;
use std::collections::HashMap;

/// SIMD instruction set
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimdIsa {
    /// No SIMD
    None,
    /// SSE (128-bit)
    SSE,
    /// SSE2 (128-bit)
    SSE2,
    /// SSE3 (128-bit)
    SSE3,
    /// SSSE3 (128-bit)
    SSSE3,
    /// SSE4.1 (128-bit)
    SSE4_1,
    /// SSE4.2 (128-bit)
    SSE4_2,
    /// AVX (256-bit)
    AVX,
    /// AVX2 (256-bit with more integer ops)
    AVX2,
    /// AVX-512 (512-bit)
    AVX512,
    /// NEON (ARM SIMD, 128-bit)
    Neon,
    /// SVE (ARM Scalable Vector Extension)
    SVE,
}

/// Vector width in bits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VectorWidth {
    W128,
    W256,
    W512,
    W1024, // SVE
}

/// Lane type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

impl LaneType {
    /// Get size in bytes
    pub fn size(&self) -> usize {
        match self {
            LaneType::I8 | LaneType::U8 => 1,
            LaneType::I16 | LaneType::U16 => 2,
            LaneType::I32 | LaneType::U32 | LaneType::F32 => 4,
            LaneType::I64 | LaneType::U64 | LaneType::F64 => 8,
        }
    }

    /// Get number of lanes for a given width
    pub fn lanes(&self, width: VectorWidth) -> usize {
        let bits = match width {
            VectorWidth::W128 => 128,
            VectorWidth::W256 => 256,
            VectorWidth::W512 => 512,
            VectorWidth::W1024 => 1024,
        };
        bits / (self.size() * 8)
    }
}

/// SIMD operation kind
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimdOp {
    // Load/Store
    Load(VectorWidth, LaneType),
    Store(VectorWidth, LaneType),
    Gather(VectorWidth, LaneType),
    Scatter(VectorWidth, LaneType),

    // Arithmetic
    Add(VectorWidth, LaneType),
    Sub(VectorWidth, LaneType),
    Mul(VectorWidth, LaneType),
    Div(VectorWidth, LaneType),
    Neg(VectorWidth, LaneType),
    Abs(VectorWidth, LaneType),
    Sqrt(VectorWidth, LaneType),
    Fma(VectorWidth, LaneType), // Fused multiply-add

    // Comparison
    CmpEq(VectorWidth, LaneType),
    CmpNe(VectorWidth, LaneType),
    CmpLt(VectorWidth, LaneType),
    CmpLe(VectorWidth, LaneType),
    CmpGt(VectorWidth, LaneType),
    CmpGe(VectorWidth, LaneType),

    // Logical
    And(VectorWidth),
    Or(VectorWidth),
    Xor(VectorWidth),
    Not(VectorWidth),
    AndNot(VectorWidth),

    // Shift
    ShiftLeft(VectorWidth, LaneType),
    ShiftRight(VectorWidth, LaneType),

    // Reduction
    ReduceAdd(VectorWidth, LaneType),
    ReduceMul(VectorWidth, LaneType),
    ReduceMin(VectorWidth, LaneType),
    ReduceMax(VectorWidth, LaneType),

    // Type conversion
    Cast(VectorWidth, LaneType, LaneType), // Cast between lane types
    Promote(VectorWidth, LaneType),         // e.g., i32 -> i64
    Demote(VectorWidth, LaneType),         // e.g., i64 -> i32
    FloatToInt(VectorWidth),
    IntToFloat(VectorWidth),

    // Permutation
    Shuffle(VectorWidth),
    Blend(VectorWidth),
    Broadcast(VectorWidth, LaneType),
    Splat(VectorWidth, LaneType),
}

/// SIMD vector value
#[derive(Debug, Clone)]
pub struct SimdValue {
    pub lanes: Vec<Value>,
    pub width: VectorWidth,
    pub lane_type: LaneType,
}

impl SimdValue {
    pub fn new(width: VectorWidth, lane_type: LaneType) -> Self {
        let lanes = vec![Value::I32(0); lane_type.lanes(width)];
        SimdValue { lanes, width, lane_type }
    }

    pub fn num_lanes(&self) -> usize {
        self.lane_type.lanes(self.width)
    }
}

/// SIMD analysis result
#[derive(Debug, Clone)]
pub struct SimdAnalysis {
    /// Whether loop can be vectorized
    pub can_vectorize: bool,
    /// Estimated speedup from vectorization
    pub estimated_speedup: f64,
    /// Number of iterations
    pub trip_count: Option<usize>,
    /// Memory access pattern
    pub memory_pattern: MemoryPattern,
    /// Dependencies that prevent vectorization
    pub dependencies: Vec<Dependency>,
    /// Recommended vector width
    pub recommended_width: VectorWidth,
    /// Recommended lane type
    pub recommended_lanes: LaneType,
}

/// Memory access pattern
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryPattern {
    /// Contiguous access (stride 1)
    Contiguous,
    /// Strided access
    Strided { stride: usize },
    /// Gather/scatter (indexed)
    Indexed,
    /// Unknown
    Unknown,
}

/// Dependency type
#[derive(Debug, Clone)]
pub struct Dependency {
    pub kind: DependencyKind,
    pub from: String,
    pub to: String,
    pub distance: Option<usize>,
    pub can_hoist: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyKind {
    /// Loop-carried dependence
    LoopCarried,
    /// Loop-independent dependence
    LoopIndependent,
    /// Memory alias
    Alias,
    /// Scalar dependency
    Scalar,
}

/// Vectorization pass
pub struct SimdVectorizer {
    /// Target ISA
    isa: SimdIsa,
    /// Available vector widths
    available_widths: Vec<VectorWidth>,
    /// Vectorization cost model
    cost_model: SimdCostModel,
    /// Statistics
    stats: SimdStats,
}

/// Cost model for vectorization decisions
#[derive(Debug, Clone)]
pub struct SimdCostModel {
    /// Cost of scalar operation
    pub scalar_op_cost: usize,
    /// Cost of SIMD operation (per lane)
    pub simd_op_cost: usize,
    /// Cost of gather/scatter
    pub gather_scatter_cost: usize,
    /// Cost of shuffles
    pub shuffle_cost: usize,
    /// Cost of mask operations
    pub mask_cost: usize,
    /// Memory bandwidth (bytes per cycle)
    pub memory_bandwidth: usize,
}

impl Default for SimdCostModel {
    fn default() -> Self {
        SimdCostModel {
            scalar_op_cost: 1,
            simd_op_cost: 1,
            gather_scatter_cost: 4,
            shuffle_cost: 2,
            mask_cost: 3,
            memory_bandwidth: 32, // bytes per cycle
        }
    }
}

/// SIMD vectorization statistics
#[derive(Debug, Clone, Default)]
pub struct SimdStats {
    pub loops_analyzed: usize,
    pub loops_vectorized: usize,
    pub total_lanes_vectorized: usize,
    pub operations_vectorized: usize,
}

impl SimdVectorizer {
    pub fn new(isa: SimdIsa) -> Self {
        let available_widths = match isa {
            SimdIsa::None => vec![],
            SimdIsa::SSE | SimdIsa::SSE2 | SimdIsa::SSE3 | SimdIsa::SSSE3
            | SimdIsa::SSE4_1 | SimdIsa::SSE4_2 => vec![VectorWidth::W128],
            SimdIsa::AVX | SimdIsa::AVX2 => vec![VectorWidth::W128, VectorWidth::W256],
            SimdIsa::AVX512 => vec![VectorWidth::W128, VectorWidth::W256, VectorWidth::W512],
            SimdIsa::Neon => vec![VectorWidth::W128],
            SimdIsa::SVE => vec![VectorWidth::W128, VectorWidth::W256, VectorWidth::W512, VectorWidth::W1024],
        };

        SimdVectorizer {
            isa,
            available_widths,
            cost_model: SimdCostModel::default(),
            stats: SimdStats::default(),
        }
    }

    /// Analyze a loop for vectorization potential
    pub fn analyze_loop(&self, loop_: &Stmt) -> SimdAnalysis {
        self.stats.loops_analyzed += 1;

        // Check for simple for loops
        if let StmtKind::For(var, iterable, body) = &loop_.kind {
            let memory_pattern = self.analyze_memory_pattern(body);

            // Check for loop-carried dependencies
            let dependencies = self.find_dependencies(body);

            // Estimate trip count
            let trip_count = self.estimate_trip_count(iterable);

            // Check if vectorization is beneficial
            let can_vectorize = dependencies.iter()
                .all(|d| d.can_hoist || d.kind == DependencyKind::LoopIndependent);

            // Calculate estimated speedup
            let estimated_speedup = if can_vectorize {
                let lanes = self.max_lanes();
                let scalar_ops = self.count_scalar_ops(body);
                let vector_ops = self.count_vector_ops(body, lanes);

                if vector_ops > 0 {
                    (scalar_ops as f64) / (vector_ops as f64)
                } else {
                    1.0
                }
            } else {
                1.0
            };

            // Determine recommended vector width
            let recommended_width = if let Some(trip) = trip_count {
                self.available_widths.iter()
                    .find(|w| {
                        let lanes = LaneType::I32.lanes(**w);
                        lanes as usize <= trip
                    })
                    .copied()
                    .unwrap_or(VectorWidth::W128)
            } else {
                *self.available_widths.last().unwrap_or(&VectorWidth::W128)
            };

            SimdAnalysis {
                can_vectorize,
                estimated_speedup,
                trip_count,
                memory_pattern,
                dependencies,
                recommended_width,
                recommended_lanes: LaneType::I32,
            }
        } else {
            SimdAnalysis {
                can_vectorize: false,
                estimated_speedup: 1.0,
                trip_count: None,
                memory_pattern: MemoryPattern::Unknown,
                dependencies: vec![],
                recommended_width: VectorWidth::W128,
                recommended_lanes: LaneType::I32,
            }
        }
    }

    /// Analyze memory access pattern
    fn analyze_memory_pattern(&self, body: &[Stmt]) -> MemoryPattern {
        // Simplified: check for array accesses with induction variable
        let mut has_induction_access = false;

        for stmt in body {
            if let StmtKind::Assignment(lhs, rhs) = &stmt.kind {
                if let ExprKind::Index(_, _) = lhs.kind {
                    has_induction_access = true;
                }
            }
        }

        if has_induction_access {
            MemoryPattern::Contiguous
        } else {
            MemoryPattern::Unknown
        }
    }

    /// Find dependencies in loop body
    fn find_dependencies(&self, body: &[Stmt]) -> Vec<Dependency> {
        let mut deps = Vec::new();

        // Simplified: check for scalar dependencies
        for stmt in body {
            if let StmtKind::Assignment(Expr { kind: ExprKind::Identifier(name), .. }, rhs) = &stmt.kind {
                if let ExprKind::Identifier(_) = rhs.kind {
                    deps.push(Dependency {
                        kind: DependencyKind::Scalar,
                        from: name.clone(),
                        to: name.clone(),
                        distance: Some(1),
                        can_hoist: false,
                    });
                }
            }
        }

        deps
    }

    /// Estimate loop trip count
    fn estimate_trip_count(&self, iterable: &Expr) -> Option<usize> {
        if let ExprKind::Range(start, end, _) = &iterable.kind {
            let start_val = if let ExprKind::Literal(Literal::Int(n)) = start.kind {
                *n as usize
            } else {
                return None;
            };

            let end_val = if let ExprKind::Literal(Literal::Int(n)) = end.kind {
                *n as usize
            } else {
                return None;
            };

            Some(end_val.saturating_sub(start_val))
        } else {
            None
        }
    }

    /// Count scalar operations in body
    fn count_scalar_ops(&self, body: &[Stmt]) -> usize {
        let mut count = 0;

        for stmt in body {
            count += match &stmt.kind {
                StmtKind::Expr(expr) => self.count_ops_in_expr(expr),
                StmtKind::Assignment(_, rhs) => self.count_ops_in_expr(rhs),
                _ => 0,
            };
        }

        count
    }

    /// Count operations in expression
    fn count_ops_in_expr(&self, expr: &Expr) -> usize {
        match &expr.kind {
            ExprKind::Binary { lhs, rhs, .. } => {
                1 + self.count_ops_in_expr(lhs) + self.count_ops_in_expr(rhs)
            }
            ExprKind::Unary { operand, .. } => {
                1 + self.count_ops_in_expr(operand)
            }
            ExprKind::Call { arguments, .. } => {
                arguments.iter().map(|a| self.count_ops_in_expr(a)).sum()
            }
            _ => 0,
        }
    }

    /// Count vector operations (per lane)
    fn count_vector_ops(&self, body: &[Stmt], lanes: usize) -> usize {
        let scalar_ops = self.count_scalar_ops(body);
        if scalar_ops == 0 {
            return 0;
        }

        // Account for vector overhead
        let vector_overhead = 2; // Load and store
        let lane_factor = if lanes > 0 { lanes } else { 1 };

        (scalar_ops / lane_factor) + vector_overhead
    }

    /// Get maximum vector lanes available
    fn max_lanes(&self) -> usize {
        self.available_widths.iter()
            .map(|w| LaneType::I32.lanes(*w))
            .max()
            .unwrap_or(1)
    }

    /// Vectorize a loop
    pub fn vectorize_loop(&self, loop_: &Stmt, analysis: &SimdAnalysis) -> Result<Stmt, VectorizeError> {
        if !analysis.can_vectorize {
            return Err(VectorizeError::CannotVectorize(
                "Loop has dependencies that prevent vectorization".to_string(),
            ));
        }

        // Create vectorized version
        let vectorized = self.create_vectorized_loop(loop_, analysis)?;

        self.stats.loops_vectorized += 1;

        Ok(vectorized)
    }

    /// Create vectorized loop body
    fn create_vectorized_loop(&self, loop_: &Stmt, analysis: &SimdAnalysis) -> Result<Stmt, VectorizeError> {
        if let StmtKind::For(var, iterable, body) = &loop_.kind {
            let width = analysis.recommended_width;
            let lane_type = analysis.recommended_lanes;
            let lanes = lane_type.lanes(width);

            // Create vectorized body
            let mut vectorized_body = Vec::new();

            for stmt in body {
                let vs = self.vectorize_stmt(stmt, width, lane_type)?;
                vectorized_body.push(vs);
            }

            // Return vectorized loop
            Ok(Stmt {
                kind: StmtKind::For(
                    var.clone(),
                    Box::new(iterable.clone()),
                    vectorized_body,
                ),
                span: loop_.span.clone(),
            })
        } else {
            Err(VectorizeError::InvalidLoop)
        }
    }

    /// Vectorize a single statement
    fn vectorize_stmt(&self, stmt: &Stmt, width: VectorWidth, lane_type: LaneType) -> Result<Stmt, VectorizeError> {
        match &stmt.kind {
            StmtKind::Expr(expr) => {
                Ok(Stmt {
                    kind: StmtKind::Expr(self.vectorize_expr(expr, width, lane_type)?),
                    span: stmt.span.clone(),
                })
            }
            StmtKind::Assignment(lhs, rhs) => {
                Ok(Stmt {
                    kind: StmtKind::Assignment(
                        lhs.clone(),
                        Box::new(self.vectorize_expr(rhs, width, lane_type)?),
                    ),
                    span: stmt.span.clone(),
                })
            }
            StmtKind::Let(p, t, v, body) => {
                Ok(Stmt {
                    kind: StmtKind::Let(
                        p.clone(),
                        t.clone(),
                        v.as_ref().map(|e| Box::new(self.vectorize_expr(e, width, lane_type)?)),
                        body.iter().map(|s| self.vectorize_stmt(s, width, lane_type)).collect::<Result<Vec<_>, _>>()?,
                    ),
                    span: stmt.span.clone(),
                })
            }
            _ => Ok(stmt.clone()),
        }
    }

    /// Vectorize an expression
    fn vectorize_expr(&self, expr: &Expr, width: VectorWidth, lane_type: LaneType) -> Result<Expr, VectorizeError> {
        match &expr.kind {
            ExprKind::Binary { op, lhs, rhs } => {
                let vlhs = self.vectorize_expr(lhs, width, lane_type)?;
                let vrhs = self.vectorize_expr(rhs, width, lane_type)?;

                // Determine SIMD operation
                let simd_op = match op {
                    BinOp::Add => SimdOp::Add(width, lane_type),
                    BinOp::Sub => SimdOp::Sub(width, lane_type),
                    BinOp::Mul => SimdOp::Mul(width, lane_type),
                    BinOp::Div => SimdOp::Div(width, lane_type),
                    BinOp::And => SimdOp::And(width),
                    BinOp::Or => SimdOp::Or(width),
                    BinOp::Xor => SimdOp::Xor(width),
                    _ => return Ok(expr.clone()),
                };

                self.stats.operations_vectorized += 1;

                Ok(Expr {
                    kind: ExprKind::Call {
                        function: Box::new(Expr {
                            kind: ExprKind::Identifier(format!("__simd_{:?}", simd_op)),
                            span: expr.span.clone(),
                            annotations: vec![],
                        }),
                        arguments: vec![vlhs, vrhs],
                    },
                    span: expr.span.clone(),
                    annotations: vec![],
                })
            }
            ExprKind::Call { function, arguments } => {
                let vargs: Result<Vec<Expr>, _> = arguments.iter()
                    .map(|a| self.vectorize_expr(a, width, lane_type))
                    .collect();

                Ok(Expr {
                    kind: ExprKind::Call {
                        function: function.clone(),
                        arguments: vargs?,
                    },
                    span: expr.span.clone(),
                    annotations: vec![],
                })
            }
            _ => Ok(expr.clone()),
        }
    }

    /// Emit SIMD instruction
    pub fn emit_simd_instruction(&self, op: &SimdOp) -> Vec<String> {
        match self.isa {
            SimdIsa::SSE2 | SimdIsa::SSE3 | SimdIsa::SSSE3
            | SimdIsa::SSE4_1 | SimdIsa::SSE4_2 => {
                self.emit_sse_instructions(op)
            }
            SimdIsa::AVX | SimdIsa::AVX2 => {
                self.emit_avx_instructions(op)
            }
            SimdIsa::AVX512 => {
                self.emit_avx512_instructions(op)
            }
            SimdIsa::Neon => {
                self.emit_neon_instructions(op)
            }
            _ => vec![],
        }
    }

    /// Emit SSE instructions
    fn emit_sse_instructions(&self, op: &SimdOp) -> Vec<String> {
        match op {
            SimdOp::Add(width, lane_type) => {
                let suffix = match (width, lane_type) {
                    (VectorWidth::W128, LaneType::I32) => "addps",
                    (VectorWidth::W128, LaneType::F32) => "addps",
                    (VectorWidth::W128, LaneType::F64) => "addpd",
                    _ => "addps",
                };
                vec![suffix.to_string()]
            }
            SimdOp::Mul(width, lane_type) => {
                let suffix = match (width, lane_type) {
                    (VectorWidth::W128, LaneType::F32) => "mulps",
                    (VectorWidth::W128, LaneType::F64) => "mulpd",
                    _ => "mulps",
                };
                vec![suffix.to_string()]
            }
            SimdOp::Load(width, lane_type) => {
                vec![format!("movdqu {:?} {:?}", width, lane_type)]
            }
            SimdOp::Store(width, lane_type) => {
                vec![format!("movdqu {:?} {:?}", width, lane_type)]
            }
            _ => vec![],
        }
    }

    /// Emit AVX instructions
    fn emit_avx_instructions(&self, op: &SimdOp) -> Vec<String> {
        match op {
            SimdOp::Add(VectorWidth::W256, LaneType::F32) => {
                vec!["vaddps ymm0, ymm1, ymm2".to_string()]
            }
            SimdOp::Mul(VectorWidth::W256, LaneType::F32) => {
                vec!["vmulps ymm0, ymm1, ymm2".to_string()]
            }
            SimdOp::Load(VectorWidth::W256, _) => {
                vec!["vmovdqu ymm0, [rdi]".to_string()]
            }
            _ => self.emit_sse_instructions(op),
        }
    }

    /// Emit AVX-512 instructions
    fn emit_avx512_instructions(&self, op: &SimdOp) -> Vec<String> {
        match op {
            SimdOp::Add(VectorWidth::W512, LaneType::F32) => {
                vec!["vaddps zmm0, zmm1, zmm2".to_string()]
            }
            SimdOp::Mul(VectorWidth::W512, LaneType::F32) => {
                vec!["vmulps zmm0, zmm1, zmm2".to_string()]
            }
            SimdOp::Fma(VectorWidth::W512, LaneType::F32) => {
                vec!["vfmadd132ps zmm0, zmm1, zmm2".to_string()]
            }
            _ => self.emit_avx_instructions(op),
        }
    }

    /// Emit NEON instructions (ARM)
    fn emit_neon_instructions(&self, op: &SimdOp) -> Vec<String> {
        match op {
            SimdOp::Add(VectorWidth::W128, lane_type) => {
                let suffix = match lane_type {
                    LaneType::I32 => "add.4s",
                    LaneType::I64 => "add.2d",
                    LaneType::F32 => "fadd.4s",
                    LaneType::F64 => "fadd.2d",
                    _ => "add.8b",
                };
                vec![format!("{} v0, v1, v2", suffix)]
            }
            SimdOp::Mul(VectorWidth::W128, lane_type) => {
                let suffix = match lane_type {
                    LaneType::I32 => "mul.4s",
                    LaneType::F32 => "fmul.4s",
                    _ => "mul.8h",
                };
                vec![format!("{} v0, v1, v2", suffix)]
            }
            SimdOp::Load(VectorWidth::W128, _) => {
                vec!["ld1 {v0.4s}, [x0]".to_string()]
            }
            SimdOp::Store(VectorWidth::W128, _) => {
                vec!["st1 {v0.4s}, [x0]".to_string()]
            }
            _ => vec![],
        }
    }

    /// Get vectorization statistics
    pub fn stats(&self) -> &SimdStats {
        &self.stats
    }
}

/// Vectorization error
#[derive(Debug, Clone)]
pub enum VectorizeError {
    CannotVectorize(String),
    InvalidLoop,
    UnsupportedOperation(String),
    IsaNotSupported,
}

impl fmt::Display for VectorizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VectorizeError::CannotVectorize(msg) => write!(f, "Cannot vectorize: {}", msg),
            VectorizeError::InvalidLoop => write!(f, "Invalid loop structure"),
            VectorizeError::UnsupportedOperation(op) => write!(f, "Unsupported operation: {}", op),
            VectorizeError::IsaNotSupported => write!(f, "Target ISA does not support SIMD"),
        }
    }
}

impl std::error::Error for VectorizeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lane_type_size() {
        assert_eq!(LaneType::I8.size(), 1);
        assert_eq!(LaneType::I32.size(), 4);
        assert_eq!(LaneType::I64.size(), 8);
    }

    #[test]
    fn test_lane_count() {
        assert_eq!(LaneType::I32.lanes(VectorWidth::W128), 4);
        assert_eq!(LaneType::I32.lanes(VectorWidth::W256), 8);
        assert_eq!(LaneType::I32.lanes(VectorWidth::W512), 16);
    }

    #[test]
    fn test_simd_vectorizer_creation() {
        let vectorizer = SimdVectorizer::new(SimdIsa::AVX2);
        assert!(vectorizer.max_lanes() >= 4);
    }

    #[test]
    fn test_simd_instruction_emission() {
        let vectorizer = SimdVectorizer::new(SimdIsa::Neon);
        let instrs = vectorizer.emit_simd_instruction(&SimdOp::Add(VectorWidth::W128, LaneType::F32));
        assert!(!instrs.is_empty());
    }
}
