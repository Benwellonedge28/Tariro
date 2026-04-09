//! Meta-Library Generation Sub-Module for TUMC.
//!
//! This module is responsible for automatically generating libraries and SDKs for new languages
//! and target platforms defined within the TUMC ecosystem. It ensures that newly designed
//! languages are practical and usable by developers by providing them with necessary tooling
//! for API/ABI interaction, code templating, and documentation.

pub mod api_abi_parser;
pub mod code_template_engine;
pub mod documentation_generator; // NEW: Documentation Generator

pub struct MetaLibraryGenCore {
    api_abi_parser: api_abi_parser::ApiAbiParser,
    code_template_engine: code_template_engine::CodeTemplateEngine,
    documentation_generator: documentation_generator::DocumentationGenerator, // NEW: DocumentationGenerator instance
}

impl MetaLibraryGenCore {
    pub fn new() -> Self {
        MetaLibraryGenCore {
            api_abi_parser: api_abi_parser::ApiAbiParser::new(),
            code_template_engine: code_template_engine::CodeTemplateEngine::new(),
            documentation_generator: documentation_generator::DocumentationGenerator::new(), // NEW: Initialize DocumentationGenerator
        }
    }

    /// Generates a meta-library (SDK, bindings, wrappers) for a given artifact and target platform.
    pub fn generate_meta_library(&self, artifact_spec: &str, target_platform: &str) -> String {
        println!("Meta-Library Generator: Generating meta-library for artifact: '{}' on platform: '{}'", artifact_spec, target_platform);
        let parsed_abi = self.api_abi_parser.parse_api_abi_definition("sample_api_spec");

        // Simulate code generation
        let generated_code = self.code_template_engine.generate_code_from_template(&parsed_abi, "rust");

        // Generate documentation for the generated code
        let generated_docs = self.documentation_generator.generate_documentation(&parsed_abi, &generated_code);

        format!("Meta-library generated for '{}' on platform '{}' based on parsed ABI and code template. Generated code: {}. Generated docs: {}.", artifact_spec, target_platform, generated_code, generated_docs)
    }
}
