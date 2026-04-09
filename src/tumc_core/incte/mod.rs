//! INCTE System: Integrated Native Compilation & Transpilation Engine.
//!
//! This module provides the core capabilities for transforming Universal Intermediate Representations (UIRs)
//! into highly optimized native executable code or transpilating them into other target languages.
//! It is responsible for bridging the gap between language definitions (from FLDGM) and deployable artifacts.

pub mod ir_generator;
pub mod optimization_pipeline;
pub mod code_generator; // NEW: Code Generator Backends

pub struct IncteCore {
    ir_generator: ir_generator::IrGenerator,
    optimization_pipeline: optimization_pipeline::OptimizationPipeline,
    code_generator: code_generator::CodeGenerator, // NEW: CodeGenerator instance
}

impl IncteCore {
    pub fn new() -> Self {
        IncteCore {
            ir_generator: ir_generator::IrGenerator::new(),
            optimization_pipeline: optimization_pipeline::OptimizationPipeline::new(),
            code_generator: code_generator::CodeGenerator::new(), // NEW: Initialize CodeGenerator
        }
    }

    /// Compiles an optimized Universal Intermediate Representation (UIR) into a native executable artifact.
    pub fn compile_to_native(&self, uir_representation: &str, target_platform: &str) -> String {
        println!("INCTE: Compiling UIR to native code for platform: '{}'", target_platform);
        let generated_ir = self.ir_generator.generate_ir("sample_ast_or_semantic_graph");

        // Apply optimizations to the generated IR
        let optimized_ir = self.optimization_pipeline.apply_optimizations(&generated_ir);

        // Generate native code from the optimized IR
        let native_code = self.code_generator.generate_native_code(&optimized_ir, target_platform);

        format!("Native executable for '{}' generated. Code: {}.", target_platform, native_code)
    }

    /// Transpiles an optimized UIR into another high-level programming language.
    pub fn transpile_to_language(&self, uir_representation: &str, target_language: &str) -> String {
        println!("INCTE: Transpiling UIR to language: '{}'", target_language);
        let generated_ir = self.ir_generator.generate_ir("sample_ast_or_semantic_graph");

        // Apply optimizations to the generated IR
        let optimized_ir = self.optimization_pipeline.apply_optimizations(&generated_ir);

        // Transpile to the target language
        let transpiled_code = self.code_generator.transpile_code(&optimized_ir, target_language);

        format!("Code in '{}' generated. Code: {}.", target_language, transpiled_code)
    }
}
