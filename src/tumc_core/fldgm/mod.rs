//! FLDGM System: Formal Language Design & Generation Module.
//!
//! This module enables Tariro to formally define, create, and generate new programming languages,
//! including their syntax, semantics, type systems, and runtime models, along with automated
//! tooling generation.

pub mod gdl_processor;
pub mod ast_generator; // NEW: Abstract Syntax Tree Generator

pub struct FldgmCore {
    gdl_processor: gdl_processor::GdlProcessor,
    ast_generator: ast_generator::AstGenerator, // NEW: AstGenerator instance
}

impl FldgmCore {
    pub fn new() -> Self {
        FldgmCore {
            gdl_processor: gdl_processor::GdlProcessor::new(),
            ast_generator: ast_generator::AstGenerator::new(), // NEW: Initialize AstGenerator
        }
    }

    /// Designs a new formal language based on the provided specification.
    pub fn design_language(&self, language_spec: &str) -> String {
        println!("FLDGM: Designing language with spec: '{}'", language_spec);
        let processed_grammar = self.gdl_processor.process_grammar_spec(language_spec);

        // Simulate AST generation after grammar processing
        let generated_ast = self.ast_generator.generate_ast(&processed_grammar, "sample_source_code");

        format!("Language design initiated. Processed grammar: {}. Generated AST: {}. Ready for tooling generation.", processed_grammar, generated_ast)
    }

    /// Generates language tooling (lexer, parser, AST builder) for a given language.
    pub fn generate_tooling(&self, language_name: &str, grammar_id: &str) -> String {
        println!("FLDGM: Generating tooling for language '{}' from grammar '{}'.", language_name, grammar_id);
        // Simulate tooling generation
        format!("Tooling (lexer, parser, AST builder) generated for language {}.", language_name)
    }
}
