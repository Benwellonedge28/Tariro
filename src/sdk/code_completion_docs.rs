use std::collections::HashMap;
use crate::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

// Re-using SdkSpecification from sdk/code_generator.rs
// Represents the core specification of an SDK API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkSpecification {
    pub spec_id: String,
    pub name: String,
    pub version: String,
    pub endpoints: Vec<EndpointDefinition>,
    pub data_models: Vec<DataModelDefinition>,
    pub authentication_methods: Vec<String>, // e.g., "API_Key", "OAuth2"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointDefinition {
    pub path: String,
    pub method: String, // e.g., "GET", "POST"
    pub description: String,
    pub request_body_model_id: Option<String>,
    pub response_body_model_id: Option<String>,
    pub query_parameters: HashMap<String, String>, // param_name -> type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataModelDefinition {
    pub model_id: String,
    pub name: String,
    pub fields: HashMap<String, String>, // field_name -> type (e.g., "string", "integer", "boolean")
}


// Represents a context-aware code completion suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionSuggestion {
    pub suggestion_id: String,
    pub sdk_spec_id: String,
    pub context_path: String, // e.g., "Sdk.Users.get_by_id("
    pub suggested_text: String,
    pub detail: String, // e.g., "Function call to retrieve user by ID"
    pub type_info: String, // e.g., "fn(id: String) -> User"
    pub sort_rank: u32,
}

// Represents a comprehensive documentation snippet for an SDK element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentationSnippet {
    pub doc_id: String,
    pub sdk_spec_id: String,
    pub element_path: String, // e.g., "Sdk.Users.get_by_id", "Sdk.User"
    pub language: String, // Target language for the documentation format
    pub snippet_content: String, // Markdown or reStructuredText format
    pub last_generated: DateTime<Utc>,
}

pub struct SdkIntellisenseGenerator {
    sdk_specifications: Mutex<HashMap<String, SdkSpecification>>, // key: spec_id
    completion_suggestions_cache: Mutex<HashMap<String, Vec<CompletionSuggestion>>>,
    documentation_snippets_cache: Mutex<HashMap<String, Vec<DocumentationSnippet>>>,
}

impl SdkIntellisenseGenerator {
    pub fn new() -> Self {
        SdkIntellisenseGenerator {
            sdk_specifications: Mutex::new(HashMap::new()),
            completion_suggestions_cache: Mutex::new(HashMap::new()),
            documentation_snippets_cache: Mutex::new(HashMap::new()),
        }
    }

    // Registers an SDK specification for which to generate intellisense
    pub fn register_sdk_specification(&self, spec: SdkSpecification) {
        println!("SDK Intellisense Generator: Registering SDK specification: {}", spec.name);
        self.sdk_specifications.lock().unwrap().insert(spec.spec_id.clone(), spec);
    }

    // Generates context-aware code completion suggestions for a given SDK
    pub fn generate_completion_suggestions(&self, spec_id: &str, target_language: &str) -> Vec<CompletionSuggestion> {
        println!("SDK Intellisense Generator: Generating completion suggestions for {} in {}...", spec_id, target_language);
        let specs_locked = self.sdk_specifications.lock().unwrap();
        let spec = specs_locked.get(spec_id).expect("SDK Specification not found.");

        let mut suggestions = Vec::new();

        // Simulate suggestions for data models
        for model in &spec.data_models {
            suggestions.push(CompletionSuggestion {
                suggestion_id: Uuid::new_v4().to_string(),
                sdk_spec_id: spec.spec_id.clone(),
                context_path: format!("{}.", spec.name),
                suggested_text: model.name.clone(),
                detail: format!("Data model for {}", model.name),
                type_info: format!("struct {}", model.name),
                sort_rank: 10,
            });
        }

        // Simulate suggestions for endpoints
        for endpoint in &spec.endpoints {
            let func_name = endpoint.path.replace("/", "_").trim_start_matches('_').to_string();
            let return_type = if endpoint.response_body_model_id.is_some() { "Response" } else { "void" };
            suggestions.push(CompletionSuggestion {
                suggestion_id: Uuid::new_v4().to_string(),
                sdk_spec_id: spec.spec_id.clone(),
                context_path: format!("{}.{}", spec.name, func_name),
                suggested_text: func_name.clone(),
                detail: endpoint.description.clone(),
                type_info: format!("fn({:?}) -> {}", endpoint.query_parameters.keys().collect::<Vec<&String>>(), return_type),
                sort_rank: 20,
            });
        }

        self.completion_suggestions_cache.lock().unwrap().insert(spec_id.to_string(), suggestions.clone());
        println!("SDK Intellisense Generator: Generated {} completion suggestions.", suggestions.len());
        suggestions
    }

    // Generates comprehensive documentation snippets for a given SDK
    pub fn generate_documentation(&self, spec_id: &str, language: &str) -> Vec<DocumentationSnippet> {
        println!("SDK Intellisense Generator: Generating documentation for {} in {}...", spec_id, language);
        let specs_locked = self.sdk_specifications.lock().unwrap();
        let spec = specs_locked.get(spec_id).expect("SDK Specification not found.");

        let mut docs = Vec::new();

        // Simulate documentation for data models
        for model in &spec.data_models {
            let fields_doc = model.fields.iter()
                .map(|(name, type_str)| format!("- `{}`: {}", name, type_str))
                .collect::<Vec<String>>()
                .join("\n");
            let content = format!(
                "## {} Model\n\nRepresents the `{}` data structure.\n\n### Fields\n{}",
                model.name, model.name, fields_doc
            );
            docs.push(DocumentationSnippet {
                doc_id: Uuid::new_v4().to_string(),
                sdk_spec_id: spec.spec_id.clone(),
                element_path: format!("{}.{}", spec.name, model.name),
                language: language.to_string(),
                snippet_content: content,
                last_generated: Utc::now(),
            });
        }

        // Simulate documentation for endpoints
        for endpoint in &spec.endpoints {
            let params_doc = endpoint.query_parameters.iter()
                .map(|(name, type_str)| format!("- `{}` ({}): Parameter for this endpoint.", name, type_str))
                .collect::<Vec<String>>()
                .join("\n");
            let content = format!(
                "## {} `{}`\n\n{}\n\n### Request Parameters\n{}",
                endpoint.method, endpoint.path, endpoint.description, params_doc
            );
            docs.push(DocumentationSnippet {
                doc_id: Uuid::new_v4().to_string(),
                sdk_spec_id: spec.spec_id.clone(),
                element_path: format!("{}.{}", spec.name, endpoint.path.replace("/", "_").trim_start_matches('_')),
                language: language.to_string(),
                snippet_content: content,
                last_generated: Utc::now(),
            });
        }

        self.documentation_snippets_cache.lock().unwrap().insert(spec_id.to_string(), docs.clone());
        println!("SDK Intellisense Generator: Generated {} documentation snippets.", docs.len());
        docs
    }

    pub fn get_completion_suggestions(&self, spec_id: &str) -> Option<Vec<CompletionSuggestion>> {
        self.completion_suggestions_cache.lock().unwrap().get(spec_id).cloned()
    }

    pub fn get_documentation_snippets(&self, spec_id: &str) -> Option<Vec<DocumentationSnippet>> {
        self.documentation_snippets_cache.lock().unwrap().get(spec_id).cloned()
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*; 

    #[test]
    fn test_sdk_intellisense_generation() {
        let generator = SdkIntellisenseGenerator::new();

        // Define a sample SDK specification (re-used from code_generator)
        let spec = SdkSpecification {
            spec_id: Uuid::new_v4().to_string(),
            name: "ExampleAPI".to_string(),
            version: "1.0.0".to_string(),
            endpoints: vec![
                EndpointDefinition {
                    path: "/users".to_string(),
                    method: "GET".to_string(),
                    description: "Retrieve a list of users".to_string(),
                    request_body_model_id: None,
                    response_body_model_id: Some("UserList".to_string()),
                    query_parameters: HashMap::from([("limit".to_string(), "integer".to_string())]),
                },
                EndpointDefinition {
                    path: "/users".to_string(),
                    method: "POST".to_string(),
                    description: "Create a new user".to_string(),
                    request_body_model_id: Some("UserCreate".to_string()),
                    response_body_model_id: Some("User".to_string()),
                    query_parameters: HashMap::new(),
                },
            ],
            data_models: vec![
                DataModelDefinition {
                    model_id: "User".to_string(),
                    name: "User".to_string(),
                    fields: HashMap::from([
                        ("id".to_string(), "string".to_string()),
                        ("name".to_string(), "string".to_string()),
                        ("email".to_string(), "string".to_string()),
                    ]),
                },
                DataModelDefinition {
                    model_id: "UserCreate".to_string(),
                    name: "UserCreate".to_string(),
                    fields: HashMap::from([
                        ("name".to_string(), "string".to_string()),
                        ("email".to_string(), "string".to_string()),
                    ]),
                },
            ],
            authentication_methods: vec!["API_Key".to_string()],
        };

        generator.register_sdk_specification(spec.clone());

        // Generate completion suggestions
        let suggestions = generator.generate_completion_suggestions(&spec.spec_id, "Rust");
        println!("Generated Suggestions: {:?}", suggestions);
        assert!(!suggestions.is_empty());
        assert!(suggestions.iter().any(|s| s.suggested_text == "User"));
        assert!(suggestions.iter().any(|s| s.suggested_text == "users"));

        // Generate documentation snippets
        let docs = generator.generate_documentation(&spec.spec_id, "Markdown");
        println!("Generated Documentation: {:?}", docs);
        assert!(!docs.is_empty());
        assert!(docs.iter().any(|d| d.element_path.contains("User") && d.snippet_content.contains("## User Model")));
        assert!(docs.iter().any(|d| d.element_path.contains("users") && d.snippet_content.contains("## GET `/users`")));

        let cached_suggestions = generator.get_completion_suggestions(&spec.spec_id).unwrap();
        assert_eq!(cached_suggestions.len(), suggestions.len());

        let cached_docs = generator.get_documentation_snippets(&spec.spec_id).unwrap();
        assert_eq!(cached_docs.len(), docs.len());
    }
}
*/