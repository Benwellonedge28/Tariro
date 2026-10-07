//! # Tariro Unified Orchestrator
//!
//! The central orchestrator that coordinates all TUMC compiler modules
//! with AGI cognitive systems.

use crate::unified::{UnifiedConfig, SystemStatus, ComponentStatus, SystemMetrics, IntentResult, ProcessingMetrics, Intent, IntentType, UnifiedResult};
use crate::unified::cognitive_core::CognitiveCore;
use crate::unified::compiler_integration::CompilerIntegration;
use crate::unified::unified_memory::UnifiedMemory;
use crate::tumc_compiler::optimization::OptLevel;
use crate::tumc_compiler::ir::Module;
use crate::tumc_compiler::agi_integration::AgiInsight;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Instant, Duration};

/// Main orchestrator for TUMC-AGI unified system
pub struct TariroUnifiedOrchestrator {
    config: UnifiedConfig,

    /// Cognitive processing core
    cognitive_core: Option<CognitiveCore>,

    /// Compiler integration layer
    compiler: CompilerIntegration,

    /// Unified memory system
    memory: Arc<RwLock<UnifiedMemory>>,

    /// System status
    status: SystemStatus,

    /// Start time for uptime calculation
    start_time: Instant,

    /// Component health status
    component_health: HashMap<String, bool>,
}

impl TariroUnifiedOrchestrator {
    /// Create a new orchestrator
    pub fn new(config: UnifiedConfig) -> Self {
        let start_time = Instant::now();
        let mut components = HashMap::new();

        components.insert("cognitive_core".to_string(), ComponentStatus {
            name: "Cognitive Core".to_string(),
            initialized: config.enable_cognitive_compiler || config.enable_agi_reasoning,
            healthy: true,
            error: None,
        });

        components.insert("compiler".to_string(), ComponentStatus {
            name: "Compiler Integration".to_string(),
            initialized: true,
            healthy: true,
            error: None,
        });

        components.insert("memory".to_string(), ComponentStatus {
            name: "Unified Memory".to_string(),
            initialized: true,
            healthy: true,
            error: None,
        });

        let cognitive_core = if config.enable_cognitive_compiler || config.enable_agi_reasoning {
            Some(CognitiveCore::new(config.clone()))
        } else {
            None
        };

        Self {
            config,
            cognitive_core,
            compiler: CompilerIntegration::new(),
            memory: Arc::new(RwLock::new(UnifiedMemory::new(1024))),
            status: SystemStatus {
                version: "1.0.13".to_string(),
                uptime_mts: 0,
                components,
                metrics: SystemMetrics {
                    compilations_completed: 0,
                    insights_generated: 0,
                    optimizations_applied: 0,
                    memory_usage_mb: 0,
                    active_agents: 1,
                },
            },
            start_time,
            component_health: HashMap::new(),
        }
    }

    /// Initialize all components
    pub fn initialize(&mut self) -> Result<(), String> {
        println!("Initializing TUMC-AGI Unified System v{}", self.status.version);

        // Initialize cognitive core
        if let Some(ref mut cc) = self.cognitive_core {
            cc.initialize()?;
            self.component_health.insert("cognitive_core".to_string(), true);
        }

        // Initialize compiler
        self.compiler.initialize()?;
        self.component_health.insert("compiler".to_string(), true);

        // Initialize memory
        {
            let mut mem = self.memory.write().map_err(|e| e.to_string())?;
            mem.initialize();
        }
        self.component_health.insert("memory".to_string(), true);

        println!("All components initialized successfully");
        Ok(())
    }

    /// Process an intent through the unified system (legacy string-based method)
    pub fn process_intent(&mut self, intent: &str, target: &str) -> IntentResult {
        let start = Instant::now();
        let mut errors = Vec::new();
        let mut insights = Vec::new();
        let mut output = String::new();

        // Step 1: Cognitive preprocessing
        if let Some(ref mut cc) = self.cognitive_core {
            let cog_result = cc.process_intent(intent);
            if !cog_result.success {
                errors.push(cog_result.error.clone());
            }
            insights.extend(cog_result.insights);
        }

        // Step 2: Compile with cognitive augmentation
        let compile_result = self.compiler.compile_with_insights(intent, target, &insights);

        if compile_result.success {
            output = compile_result.output;
        } else {
            errors.extend(compile_result.errors);
        }

        // Step 3: Post-processing and memory update
        {
            let mut mem = match self.memory.write() {
                Ok(m) => m,
                Err(_) => return IntentResult {
                    success: false,
                    output: String::new(),
                    insights,
                    metrics: ProcessingMetrics {
                        compilation_time_ms: start.elapsed().as_millis() as u64,
                        reasoning_steps: insights.len(),
                        optimizations_applied: 0,
                        confidence: 0.0,
                    },
                    errors: vec!["Memory lock error".to_string()],
                },
            };
            mem.store_intent(intent, &output, &insights);
        }

        // Update metrics
        self.status.metrics.compilations_completed += 1;
        self.status.metrics.insights_generated += insights.len() as u64;
        self.status.metrics.optimizations_applied += compile_result.optimizations_applied;

        IntentResult {
            success: errors.is_empty(),
            output,
            insights,
            metrics: ProcessingMetrics {
                compilation_time_ms: start.elapsed().as_millis() as u64,
                reasoning_steps: insights.len(),
                optimizations_applied: compile_result.optimizations_applied,
                confidence: calculate_confidence(&insights),
            },
            errors,
        }
    }

    /// Process an Intent struct through the unified system (new unified API)
    pub fn process_unified_intent(&mut self, intent: Intent) -> Result<UnifiedResult, String> {
        let start = Instant::now();
        let mut errors = Vec::new();
        let mut insights = Vec::new();
        let mut output = String::new();

        // Extract target from context if available
        let target = intent.context.get("target")
            .map(|s| s.as_str())
            .unwrap_or("wasm");

        // Step 1: Cognitive preprocessing based on intent type
        if let Some(ref mut cc) = self.cognitive_core {
            match intent.intent_type {
                IntentType::Compile | IntentType::Optimize => {
                    let cog_result = cc.process_intent(&intent.description);
                    if !cog_result.success {
                        errors.push(cog_result.error.clone());
                    }
                    insights.extend(cog_result.insights);
                }
                IntentType::Reason | IntentType::Understand => {
                    let cog_result = cc.process_intent(&intent.description);
                    if !cog_result.success {
                        errors.push(cog_result.error.clone());
                    }
                    insights.extend(cog_result.insights);
                }
                _ => {
                    // For other intent types, do basic processing
                    let cog_result = cc.process_intent(&intent.description);
                    if !cog_result.success {
                        errors.push(cog_result.error.clone());
                    }
                    insights.extend(cog_result.insights);
                }
            }
        }

        // Step 2: Compile with cognitive augmentation based on intent type
        let optimizations_applied = match intent.intent_type {
            IntentType::Compile | IntentType::Optimize => {
                let compile_result = self.compiler.compile_with_insights(&intent.description, target, &insights);
                if compile_result.success {
                    output = compile_result.output;
                } else {
                    errors.extend(compile_result.errors);
                }
                compile_result.optimizations_applied
            }
            IntentType::Analyze => {
                // Analyze code
                let compile_result = self.compiler.compile_with_insights(&intent.description, target, &insights);
                if compile_result.success {
                    output = format!("Analysis complete:\n{}", compile_result.output);
                } else {
                    errors.extend(compile_result.errors);
                }
                0
            }
            IntentType::Reason | IntentType::Understand => {
                // Generate reasoning output
                output = format!("Reasoning result for: {}", intent.description);
                insights.iter().for_each(|i| {
                    output.push_str(&format!("\n- {} (confidence: {:.2})", i.description, i.confidence));
                });
                0
            }
            IntentType::Learn => {
                output = format!("Learning from: {}", intent.description);
                0
            }
            IntentType::Create | IntentType::Generate => {
                let spec = &intent.description;
                output = self.compiler.generate_self_modifying_code(spec);
                0
            }
            _ => {
                output = format!("Processed intent: {:?} - {}", intent.intent_type, intent.description);
                0
            }
        };

        // Step 3: Post-processing and memory update
        {
            let mut mem = match self.memory.write() {
                Ok(m) => m,
                Err(_) => {
                    errors.push("Memory lock error".to_string());
                    return Err("Memory lock error".to_string());
                }
            };
            mem.store_intent(&intent.description, &output, &insights);
        }

        // Update metrics
        self.status.metrics.compilations_completed += 1;
        self.status.metrics.insights_generated += insights.len() as u64;
        self.status.metrics.optimizations_applied += optimizations_applied;

        let metrics = ProcessingMetrics {
            compilation_time_ms: start.elapsed().as_millis() as u64,
            reasoning_steps: insights.len(),
            optimizations_applied,
            confidence: calculate_confidence(&insights),
        };

        if errors.is_empty() {
            Ok(UnifiedResult {
                success: true,
                output,
                insights,
                metrics,
                errors: Vec::new(),
            })
        } else {
            Ok(UnifiedResult {
                success: false,
                output,
                insights,
                metrics,
                errors,
            })
        }
    }

    /// Compile a module with AGI augmentation
    pub fn compile_module(&mut self, mut module: Module, level: OptLevel) -> Module {
        // Apply cognitive optimizations if enabled
        if let Some(ref mut cc) = self.cognitive_core {
            module = cc.optimize_with_insights(module);
        }

        // Compile with TUMC
        self.compiler.optimize_module(&mut module, level);
        module
    }

    /// Generate code with self-modification
    pub fn generate_self_modifying(&self, spec: &str) -> String {
        self.compiler.generate_self_modifying_code(spec)
    }

    /// Get system status
    pub fn get_status(&self) -> SystemStatus {
        let mut status = self.status.clone();
        status.uptime_mts = self.start_time.elapsed().as_millis() as u128;

        // Update component health
        for (name, healthy) in &self.component_health {
            if let Some(comp) = status.components.get_mut(name) {
                comp.healthy = *healthy;
            }
        }

        status
    }

    /// Get AGI insights from the system
    pub fn get_insights(&self, limit: usize) -> Vec<AgiInsight> {
        if let Some(ref cc) = self.cognitive_core {
            cc.get_recent_insights(limit)
        } else {
            Vec::new()
        }
    }

    /// Shutdown the system gracefully
    pub fn shutdown(&mut self) {
        println!("Shutting down TUMC-AGI Unified System...");

        // Finalize cognitive core
        if let Some(ref mut cc) = self.cognitive_core {
            cc.finalize();
        }

        // Finalize compiler
        self.compiler.finalize();

        // Finalize memory
        if let Ok(mut mem) = self.memory.write() {
            mem.finalize();
        }

        println!("Shutdown complete");
    }
}

/// Calculate confidence from insights
fn calculate_confidence(insights: &[AgiInsight]) -> f64 {
    if insights.is_empty() {
        return 0.0;
    }
    insights.iter().map(|i| i.confidence).sum::<f64>() / insights.len() as f64
}

/// Compilation result from compiler integration
pub struct CompilationResult {
    pub success: bool,
    pub output: String,
    pub optimizations_applied: usize,
    pub errors: Vec<String>,
}

impl Default for TariroUnifiedOrchestrator {
    fn default() -> Self {
        Self::new(UnifiedConfig::default())
    }
}
