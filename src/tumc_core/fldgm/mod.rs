//! FLDGM System: Formal Language Design & Generation Module.
//!
//! This module enables Tariro to formally define, create, and generate new programming languages,
//! including their syntax, semantics, type systems, and runtime models, along with automated
//! tooling generation.

pub mod gdl_processor;
pub mod ast_generator;
pub mod semantic_model;
pub mod tooling_generator; // NEW: Automated Tooling Generator

pub struct FldgmCore {
    gdl_processor: gdl_processor::GdlProcessor,
    ast_generator: ast_generator::AstGenerator,
    semantic_model_builder: semantic_model::SemanticModelBuilder,
    tooling_gen: tooling_generator::ToolingGenerator, // NEW: ToolingGenerator instance
}

impl FldgmCore {
    pub fn new() -> Self {
        FldgmCore {
            gdl_processor: gdl_processor::GdlProcessor::new(),
            ast_generator: ast_generator::AstGenerator::new(),
            semantic_model_builder: semantic_model::SemanticModelBuilder::new(),
            tooling_gen: tooling_generator::ToolingGenerator::new(), // NEW: Initialize ToolingGenerator
        }
    }

    /// Designs a new formal language based on the provided specification.
    pub fn design_language(&self, language_spec: &str) -> String {
        println!("FLDGM: Designing language with spec: '{}'", language_spec);
        let processed_grammar = self.gdl_processor.process_grammar_spec(language_spec);

        // Simulate AST generation after grammar processing
        let generated_ast = self.ast_generator.generate_ast(&processed_grammar, "sample_source_code");

        // Simulate semantic model definition
        let semantic_rules = self.semantic_model_builder.define_semantic_rules(&generated_ast);

        // Simulate tooling generation
        let generated_tooling = self.tooling_gen.generate_all_tooling(&processed_grammar);

        format!("Language design initiated. Processed grammar: {}. Generated AST: {}. Semantic Rules: {}. Generated Tooling: {}.", processed_grammar, generated_ast, semantic_rules, generated_tooling)
    }

    /// Generates language tooling (lexer, parser, AST builder) for a given language.
    pub fn generate_tooling(&self, language_name: &str, grammar_id: &str) -> String {
        println!("FLDGM: Generating tooling for language '{}' from grammar '{}'.", language_name, grammar_id);
        self.tooling_gen.generate_all_tooling(grammar_id)
    }
}
