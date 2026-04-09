//! Automated Tooling Generator (Lexer/Parser) for FLDGM.
//!
//! This module automates the creation of essential language tooling, specifically
//! lexical analyzers (lexers) and parsers, directly from formal grammar definitions
//! provided by the GDL Processor. 

#[derive(Debug, Clone, PartialEq)]
pub enum ToolType {
    Lexer,
    Parser,
    Combined,
}

#[derive(Debug, Clone)]
pub struct GeneratedTool {
    pub tool_type: ToolType,
    pub language_name: String,
    pub source_code_path: String, // Path to generated source file
    pub description: String,
}

pub struct ToolingGenerator;

impl ToolingGenerator {
    pub fn new() -> Self {
        ToolingGenerator {}
    }

    /// Generates a lexical analyzer (lexer) from a grammar definition.
    pub fn generate_lexer(&self, grammar_definition: &str, language_name: &str) -> String {
        println!("Tooling Generator: Generating lexer for '{}' from grammar:\n{}", language_name, grammar_definition.lines().next().unwrap_or(""));
        // Simulate lexer generation
        format!("Lexer generated for language '{}'. Output code: (conceptual lexer code)", language_name)
    }

    /// Generates a parser from a grammar definition.
    pub fn generate_parser(&self, grammar_definition: &str, language_name: &str) -> String {
        println!("Tooling Generator: Generating parser for '{}' from grammar:\n{}", language_name, grammar_definition.lines().next().unwrap_or(""));
        // Simulate parser generation
        format!("Parser generated for language '{}'. Output code: (conceptual parser code)", language_name)
    }

    /// Generates both lexer and parser.
    pub fn generate_all_tooling(&self, grammar_definition: &str) -> String {
        let language_name = "NewLang_Auto"; // Simplified for simulation
        let lexer_output = self.generate_lexer(grammar_definition, language_name);
        let parser_output = self.generate_parser(grammar_definition, language_name);

        let generated_lexer_tool = GeneratedTool {
            tool_type: ToolType::Lexer,
            language_name: language_name.to_string(),
            source_code_path: format!("/fldgm_output/{}_lexer.rs", language_name),
            description: "Automatically generated lexical analyzer.".to_string(),
        };

        let generated_parser_tool = GeneratedTool {
            tool_type: ToolType::Parser,
            language_name: language_name.to_string(),
            source_code_path: format!("/fldgm_output/{}_parser.rs", language_name),
            description: "Automatically generated parser.".to_string(),
        };

        println!("Tooling Generator: Generated Tools: Lexer: {:?}, Parser: {:?}", generated_lexer_tool, generated_parser_tool);
        format!("Lexer and parser generated for new language. Lexer: '{}', Parser: '{}'.", lexer_output, parser_output)
    }
}
