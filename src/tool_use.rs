// Tariro AGI - Tool Use Engine Module
// Enables AGI to use external tools and APIs

use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::sync::Arc;

// ============================================================================
// Tool Types
// ============================================================================

/// Tool definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub tool_id: String,
    pub name: String,
    pub description: String,
    pub tool_type: ToolType,
    pub input_schema: Schema,
    pub output_schema: Schema,
    pub capabilities: Vec<String>,
    pub limitations: Vec<String>,
    pub version: String,
    pub auth_required: bool,
    pub rate_limit: Option<RateLimit>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolType {
    WebSearch,
    WebFetch,
    Calculator,
    CodeExecution,
    FileSystem,
    Database,
    API,
    External,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    pub fields: Vec<SchemaField>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaField {
    pub name: String,
    pub field_type: FieldType,
    pub description: String,
    pub default: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Integer,
    Float,
    Boolean,
    Array,
    Object,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimit {
    pub requests_per_minute: usize,
    pub requests_per_hour: usize,
    pub requests_per_day: usize,
}

// ============================================================================
// Tool Execution
// ============================================================================

/// Tool execution request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolRequest {
    pub request_id: String,
    pub tool_id: String,
    pub parameters: HashMap<String, ParameterValue>,
    pub timeout: Duration,
    pub retry_on_failure: bool,
    pub max_retries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterValue {
    pub value: serde_json::Value,
    pub validated: bool,
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub request_id: String,
    pub tool_id: String,
    pub success: bool,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
    pub execution_time: Duration,
    pub retries: usize,
    pub timestamp: DateTime<Utc>,
}

// ============================================================================
// Tool Manager
// ============================================================================

/// Tool use management system
pub struct ToolUseEngine {
    pub tools: HashMap<String, Tool>,
    pub execution_history: VecDeque<ToolExecution>,
    pub tool_permissions: HashMap<String, ToolPermission>,
    pub config: ToolConfig,
    pub execution_stats: ToolStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolExecution {
    pub execution_id: String,
    pub tool_id: String,
    pub request: ToolRequest,
    pub result: Option<ToolResult>,
    pub status: ExecutionStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Timeout,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolPermission {
    pub tool_id: String,
    pub allowed: bool,
    pub restricted_parameters: Vec<String>,
    pub max_usage_per_day: Option<usize>,
    pub requires_confirmation: bool,
}

#[derive(Debug, Clone)]
pub struct ToolConfig {
    pub enable_all_tools: bool,
    pub default_timeout: Duration,
    pub max_concurrent_executions: usize,
    pub enable_retry: bool,
    pub max_retries: usize,
    pub require_permission: bool,
    pub sandbox_execution: bool,
}

impl Default for ToolConfig {
    fn default() -> Self {
        Self {
            enable_all_tools: true,
            default_timeout: Duration::from_secs(30),
            max_concurrent_executions: 10,
            enable_retry: true,
            max_retries: 3,
            require_permission: true,
            sandbox_execution: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolStats {
    pub total_executions: usize,
    pub successful_executions: usize,
    pub failed_executions: usize,
    pub average_execution_time: Duration,
    pub most_used_tools: Vec<(String, usize)>,
}

impl ToolUseEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            tools: HashMap::new(),
            execution_history: VecDeque::new(),
            tool_permissions: HashMap::new(),
            config: ToolConfig::default(),
            execution_stats: ToolStats {
                total_executions: 0,
                successful_executions: 0,
                failed_executions: 0,
                average_execution_time: Duration::from_secs(0),
                most_used_tools: Vec::new(),
            },
        };

        engine.register_default_tools();
        engine
    }

    fn register_default_tools(&mut self) {
        // Web Search Tool
        self.register_tool(Tool {
            tool_id: "web_search".to_string(),
            name: "Web Search".to_string(),
            description: "Search the web for information".to_string(),
            tool_type: ToolType::WebSearch,
            input_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "query".to_string(),
                        field_type: FieldType::String,
                        description: "Search query".to_string(),
                        default: None,
                    },
                    SchemaField {
                        name: "num_results".to_string(),
                        field_type: FieldType::Integer,
                        description: "Number of results".to_string(),
                        default: Some("10".to_string()),
                    },
                ],
                required: vec!["query".to_string()],
            },
            output_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "results".to_string(),
                        field_type: FieldType::Array,
                        description: "Search results".to_string(),
                        default: None,
                    },
                ],
                required: vec!["results".to_string()],
            },
            capabilities: vec!["search".to_string(), "information_retrieval".to_string()],
            limitations: vec!["may_not_find_all_results".to_string()],
            version: "1.0.0".to_string(),
            auth_required: false,
            rate_limit: Some(RateLimit {
                requests_per_minute: 60,
                requests_per_hour: 1000,
                requests_per_day: 10000,
            }),
            enabled: true,
        });

        // Web Fetch Tool
        self.register_tool(Tool {
            tool_id: "web_fetch".to_string(),
            name: "Web Fetch".to_string(),
            description: "Fetch content from URLs".to_string(),
            tool_type: ToolType::WebFetch,
            input_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "url".to_string(),
                        field_type: FieldType::String,
                        description: "URL to fetch".to_string(),
                        default: None,
                    },
                ],
                required: vec!["url".to_string()],
            },
            output_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "content".to_string(),
                        field_type: FieldType::String,
                        description: "Fetched content".to_string(),
                        default: None,
                    },
                ],
                required: vec!["content".to_string()],
            },
            capabilities: vec!["fetch".to_string(), "download".to_string()],
            limitations: vec!["only_http_https".to_string(), "size_limit".to_string()],
            version: "1.0.0".to_string(),
            auth_required: false,
            rate_limit: Some(RateLimit {
                requests_per_minute: 30,
                requests_per_hour: 500,
                requests_per_day: 5000,
            }),
            enabled: true,
        });

        // Calculator Tool
        self.register_tool(Tool {
            tool_id: "calculator".to_string(),
            name: "Calculator".to_string(),
            description: "Perform mathematical calculations".to_string(),
            tool_type: ToolType::Calculator,
            input_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "expression".to_string(),
                        field_type: FieldType::String,
                        description: "Mathematical expression".to_string(),
                        default: None,
                    },
                ],
                required: vec!["expression".to_string()],
            },
            output_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "result".to_string(),
                        field_type: FieldType::Float,
                        description: "Calculation result".to_string(),
                        default: None,
                    },
                ],
                required: vec!["result".to_string()],
            },
            capabilities: vec!["math".to_string(), "calculation".to_string()],
            limitations: vec!["basic_operations".to_string()],
            version: "1.0.0".to_string(),
            auth_required: false,
            rate_limit: None,
            enabled: true,
        });

        // Code Execution Tool
        self.register_tool(Tool {
            tool_id: "code_executor".to_string(),
            name: "Code Executor".to_string(),
            description: "Execute code in a sandboxed environment".to_string(),
            tool_type: ToolType::CodeExecution,
            input_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "code".to_string(),
                        field_type: FieldType::String,
                        description: "Code to execute".to_string(),
                        default: None,
                    },
                    SchemaField {
                        name: "language".to_string(),
                        field_type: FieldType::String,
                        description: "Programming language".to_string(),
                        default: Some("python".to_string()),
                    },
                ],
                required: vec!["code".to_string()],
            },
            output_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "output".to_string(),
                        field_type: FieldType::String,
                        description: "Execution output".to_string(),
                        default: None,
                    },
                    SchemaField {
                        name: "error".to_string(),
                        field_type: FieldType::String,
                        description: "Error message if any".to_string(),
                        default: None,
                    },
                ],
                required: vec!["output".to_string()],
            },
            capabilities: vec!["execution".to_string(), "programming".to_string()],
            limitations: vec!["sandboxed".to_string(), "time_limit".to_string(), "memory_limit".to_string()],
            version: "1.0.0".to_string(),
            auth_required: false,
            rate_limit: Some(RateLimit {
                requests_per_minute: 10,
                requests_per_hour: 100,
                requests_per_day: 500,
            }),
            enabled: true,
        });

        // File System Tool
        self.register_tool(Tool {
            tool_id: "filesystem".to_string(),
            name: "File System".to_string(),
            description: "Read and write files".to_string(),
            tool_type: ToolType::FileSystem,
            input_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "operation".to_string(),
                        field_type: FieldType::String,
                        description: "Operation: read, write, list".to_string(),
                        default: None,
                    },
                    SchemaField {
                        name: "path".to_string(),
                        field_type: FieldType::String,
                        description: "File path".to_string(),
                        default: None,
                    },
                ],
                required: vec!["operation".to_string(), "path".to_string()],
            },
            output_schema: Schema {
                fields: vec![
                    SchemaField {
                        name: "content".to_string(),
                        field_type: FieldType::String,
                        description: "File content".to_string(),
                        default: None,
                    },
                ],
                required: vec!["content".to_string()],
            },
            capabilities: vec!["read".to_string(), "write".to_string(), "list".to_string()],
            limitations: vec!["sandboxed".to_string(), "allowed_paths_only".to_string()],
            version: "1.0.0".to_string(),
            auth_required: false,
            rate_limit: Some(RateLimit {
                requests_per_minute: 100,
                requests_per_hour: 1000,
                requests_per_day: 10000,
            }),
            enabled: true,
        });
    }

    /// Register a new tool
    pub fn register_tool(&mut self, tool: Tool) {
        self.tools.insert(tool.tool_id.clone(), tool);
    }

    /// Unregister a tool
    pub fn unregister_tool(&mut self, tool_id: &str) -> bool {
        self.tools.remove(tool_id).is_some()
    }

    /// Get tool by ID
    pub fn get_tool(&self, tool_id: &str) -> Option<&Tool> {
        self.tools.get(tool_id)
    }

    /// Execute a tool
    pub async fn execute_tool(&mut self, request: ToolRequest) -> ToolResult {
        let start_time = DateTime::<Utc>::now();
        let execution_id = generate_id();

        // Check if tool exists
        let tool = match self.tools.get(&request.tool_id) {
            Some(t) => t,
            None => {
                return ToolResult {
                    request_id: request.request_id.clone(),
                    tool_id: request.tool_id.clone(),
                    success: false,
                    output: None,
                    error: Some(format!("Tool '{}' not found", request.tool_id)),
                    execution_time: start_time.elapsed(),
                    retries: 0,
                    timestamp: DateTime<Utc>::now(),
                };
            }
        };

        // Check permissions
        if let Some(permission) = self.tool_permissions.get(&request.tool_id) {
            if !permission.allowed {
                return ToolResult {
                    request_id: request.request_id.clone(),
                    tool_id: request.tool_id.clone(),
                    success: false,
                    output: None,
                    error: Some("Tool use not permitted".to_string()),
                    execution_time: start_time.elapsed(),
                    retries: 0,
                    timestamp: DateTime<Utc>::now(),
                };
            }
        }

        // Validate parameters
        let validation_error = self.validate_parameters(&request.parameters, &tool.input_schema);
        if let Some(error) = validation_error {
            return ToolResult {
                request_id: request.request_id.clone(),
                tool_id: request.tool_id.clone(),
                success: false,
                output: None,
                error: Some(error),
                execution_time: start_time.elapsed(),
                retries: 0,
                timestamp: DateTime<Utc>::now(),
            };
        }

        // Execute tool (simplified - in production would call actual tool)
        let result = self.execute_tool_logic(tool, &request.parameters).await;

        // Update stats
        self.update_stats(&result, &request.tool_id);

        // Record execution
        let execution = ToolExecution {
            execution_id,
            tool_id: request.tool_id.clone(),
            request,
            result: Some(result.clone()),
            status: if result.success { ExecutionStatus::Completed } else { ExecutionStatus::Failed },
            started_at: start_time,
            completed_at: Some(DateTime::<Utc>::now()),
        };

        self.execution_history.push_back(execution);

        // Keep bounded
        while self.execution_history.len() > 1000 {
            self.execution_history.pop_front();
        }

        result
    }

    async fn execute_tool_logic(&self, tool: &Tool, params: &HashMap<String, ParameterValue>) -> ToolResult {
        // Simplified execution - in production would implement actual tool logic
        match tool.tool_type {
            ToolType::WebSearch => {
                let query = params.get("query")
                    .map(|p| p.value.as_str().unwrap_or(""))
                    .unwrap_or("");

                ToolResult {
                    request_id: generate_id(),
                    tool_id: tool.tool_id.clone(),
                    success: true,
                    output: Some(serde_json::json!({
                        "results": [
                            {"title": "Result 1", "url": "https://example.com/1", "snippet": "Sample result for "},
                            {"title": "Result 2", "url": "https://example.com/2", "snippet": "Another result"}
                        ]
                    })),
                    error: None,
                    execution_time: Duration::from_millis(100),
                    retries: 0,
                    timestamp: DateTime<Utc>::now(),
                }
            }
            ToolType::Calculator => {
                let expr = params.get("expression")
                    .map(|p| p.value.as_str().unwrap_or(""))
                    .unwrap_or("");

                // Simple evaluation
                let result = self.evaluate_expression(expr);

                ToolResult {
                    request_id: generate_id(),
                    tool_id: tool.tool_id.clone(),
                    success: result.is_some(),
                    output: result.map(|r| serde_json::json!({ "result": r })),
                    error: if result.is_none() { Some("Invalid expression".to_string()) } else { None },
                    execution_time: Duration::from_millis(10),
                    retries: 0,
                    timestamp: DateTime<Utc>::now(),
                }
            }
            _ => {
                ToolResult {
                    request_id: generate_id(),
                    tool_id: tool.tool_id.clone(),
                    success: true,
                    output: Some(serde_json::json!({ "status": "executed" })),
                    error: None,
                    execution_time: Duration::from_millis(50),
                    retries: 0,
                    timestamp: DateTime<Utc>::now(),
                }
            }
        }
    }

    fn evaluate_expression(&self, expr: &str) -> Option<f64> {
        // Very simple evaluator
        let expr = expr.replace(" ", "");
        if let Ok(num) = expr.parse::<f64>() {
            return Some(num);
        }

        // Basic operations
        if let Some(pos) = expr.find('+') {
            let (a, b) = (&expr[..pos], &expr[pos + 1..]);
            return self.evaluate_expression(a).zip(self.evaluate_expression(b)).map(|(x, y)| x + y);
        }

        if let Some(pos) = expr.find('-') {
            let (a, b) = (&expr[..pos], &expr[pos + 1..]);
            return self.evaluate_expression(a).zip(self.evaluate_expression(b)).map(|(x, y)| x - y);
        }

        if let Some(pos) = expr.find('*') {
            let (a, b) = (&expr[..pos], &expr[pos + 1..]);
            return self.evaluate_expression(a).zip(self.evaluate_expression(b)).map(|(x, y)| x * y);
        }

        if let Some(pos) = expr.find('/') {
            let (a, b) = (&expr[..pos], &expr[pos + 1..]);
            return self.evaluate_expression(a).zip(self.evaluate_expression(b))
                .and_then(|(x, y)| if y != 0.0 { Some(x / y) } else { None });
        }

        None
    }

    fn validate_parameters(&self, params: &HashMap<String, ParameterValue>, schema: &Schema) -> Option<String> {
        // Check required parameters
        for required in &schema.required {
            if !params.contains_key(required) {
                return Some(format!("Missing required parameter: {}", required));
            }
        }

        // Validate types (simplified)
        for (name, param) in params {
            if let Some(field) = schema.fields.iter().find(|f| f.name == *name) {
                let valid = match field.field_type {
                    FieldType::String => param.value.is_string(),
                    FieldType::Integer => param.value.is_i64() || param.value.is_u64(),
                    FieldType::Float => param.value.is_number(),
                    FieldType::Boolean => param.value.is_boolean(),
                    FieldType::Array => param.value.is_array(),
                    FieldType::Object => param.value.is_object(),
                };

                if !valid {
                    return Some(format!("Invalid type for parameter '{}'", name));
                }
            }
        }

        None
    }

    fn update_stats(&mut self, result: &ToolResult, tool_id: &str) {
        self.execution_stats.total_executions += 1;

        if result.success {
            self.execution_stats.successful_executions += 1;
        } else {
            self.execution_stats.failed_executions += 1;
        }

        // Update most used
        if let Some((_, count)) = self.execution_stats.most_used_tools.iter_mut().find(|(id, _)| *id == tool_id) {
            *count += 1;
        } else {
            self.execution_stats.most_used_tools.push((tool_id.to_string(), 1));
        }

        // Sort by count
        self.execution_stats.most_used_tools.sort_by(|a, b| b.1.cmp(&a.1));
        self.execution_stats.most_used_tools.truncate(10);
    }

    /// Get execution history
    pub fn get_history(&self, limit: usize) -> Vec<&ToolExecution> {
        self.execution_history.iter().rev().take(limit).collect()
    }

    /// Get tool statistics
    pub fn get_stats(&self) -> &ToolStats {
        &self.execution_stats
    }
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("tool_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tool_execution() {
        let mut engine = ToolUseEngine::new();

        let request = ToolRequest {
            request_id: "test_1".to_string(),
            tool_id: "calculator".to_string(),
            parameters: HashMap::new(),
            timeout: Duration::from_secs(10),
            retry_on_failure: false,
            max_retries: 0,
        };

        let result = engine.execute_tool(request).await;
        assert!(result.success);
    }

    #[test]
    fn test_tool_registration() {
        let mut engine = ToolUseEngine::new();

        let tool = engine.get_tool("calculator");
        assert!(tool.is_some());
        assert_eq!(tool.unwrap().name, "Calculator");
    }
}
