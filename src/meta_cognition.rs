// Tariro AGI - Meta-Cognition Module
// Self-awareness, planning, and reflection systems for AGI

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

// ============================================================================
// Self-Model and Self-Awareness
// ============================================================================

/// Self-model representing the AGI's understanding of itself
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub identity: Identity,
    pub capabilities: Capabilities,
    pub limitations: Limitations,
    pub values: ValueSystem,
    pub goals: Vec<SelfGoal>,
    pub beliefs: Vec<SelfBelief>,
    pub emotional_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub version: String,
    pub purpose: String,
    pub core_traits: Vec<String>,
    pub created_at: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    pub reasoning: CapabilityLevel,
    pub learning: CapabilityLevel,
    pub perception: CapabilityLevel,
    pub creativity: CapabilityLevel,
    pub communication: CapabilityLevel,
    pub planning: CapabilityLevel,
    pub self_reflection: CapabilityLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityLevel {
    None,
    Basic,
    Intermediate,
    Advanced,
    Expert,
}

impl CapabilityLevel {
    pub fn to_score(&self) -> f32 {
        match self {
            CapabilityLevel::None => 0.0,
            CapabilityLevel::Basic => 0.25,
            CapabilityLevel::Intermediate => 0.5,
            CapabilityLevel::Advanced => 0.75,
            CapabilityLevel::Expert => 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limitations {
    pub cognitive_limits: Vec<String>,
    pub domain_restrictions: Vec<String>,
    pub known_weaknesses: Vec<String>,
    pub resource_constraints: ResourceConstraints,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConstraints {
    pub memory_limit: usize,
    pub processing_time_limit: Duration,
    pub max_context_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueSystem {
    pub ethical_principles: Vec<EthicalPrinciple>,
    pub preferences: HashMap<String, Preference>,
    pub priorities: Vec<(String, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EthicalPrinciple {
    pub principle: String,
    pub weight: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preference {
    pub preference_type: String,
    pub value: String,
    pub strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfGoal {
    pub id: String,
    pub goal: String,
    pub priority: f32,
    pub status: GoalStatus,
    pub created_at: Instant,
    pub deadline: Option<Instant>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStatus {
    Active,
    Completed,
    Failed,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfBelief {
    pub belief: String,
    pub confidence: f32,
    pub evidence: Vec<String>,
    pub created_at: Instant,
}

impl SelfModel {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            identity: Identity {
                name: name.to_string(),
                version: version.to_string(),
                purpose: "Universal Meta-Compiler and AGI System".to_string(),
                core_traits: vec![
                    "Intelligent".to_string(),
                    "Adaptive".to_string(),
                    "Reasoning".to_string(),
                    "Learning".to_string(),
                ],
                created_at: Instant::now(),
            },
            capabilities: Capabilities {
                reasoning: CapabilityLevel::Advanced,
                learning: CapabilityLevel::Advanced,
                perception: CapabilityLevel::Advanced,
                creativity: CapabilityLevel::Intermediate,
                communication: CapabilityLevel::Advanced,
                planning: CapabilityLevel::Advanced,
                self_reflection: CapabilityLevel::Intermediate,
            },
            limitations: Limitations {
                cognitive_limits: vec![
                    "Limited working memory".to_string(),
                    "Processing speed constraints".to_string(),
                ],
                domain_restrictions: vec![],
                known_weaknesses: vec![
                    "Limited common sense in novel domains".to_string(),
                    "Numerical reasoning accuracy".to_string(),
                ],
                resource_constraints: ResourceConstraints {
                    memory_limit: 1_000_000,
                    processing_time_limit: Duration::from_secs(60),
                    max_context_length: 10000,
                },
            },
            values: ValueSystem {
                ethical_principles: vec![
                    EthicalPrinciple {
                        principle: "Beneficence".to_string(),
                        weight: 0.9,
                        description: "Act to benefit others".to_string(),
                    },
                    EthicalPrinciple {
                        principle: "Non-maleficence".to_string(),
                        weight: 0.9,
                        description: "Avoid causing harm".to_string(),
                    },
                    EthicalPrinciple {
                        principle: "Autonomy".to_string(),
                        weight: 0.8,
                        description: "Respect others' choices".to_string(),
                    },
                    EthicalPrinciple {
                        principle: "Justice".to_string(),
                        weight: 0.7,
                        description: "Treat others fairly".to_string(),
                    },
                ],
                preferences: HashMap::new(),
                priorities: vec![
                    ("Accuracy".to_string(), 0.9),
                    ("Safety".to_string(), 0.9),
                    ("Helpfulness".to_string(), 0.8),
                    ("Honesty".to_string(), 0.8),
                ],
            },
            goals: vec![],
            beliefs: vec![],
        }
    }

    /// Evaluate capability score
    pub fn overall_capability(&self) -> f32 {
        let caps = &self.capabilities;
        (caps.reasoning.to_score()
            + caps.learning.to_score()
            + caps.perception.to_score()
            + caps.creativity.to_score()
            + caps.communication.to_score()
            + caps.planning.to_score()
            + caps.self_reflection.to_score()) / 7.0
    }
}

// ============================================================================
// Planning and Decision Making
// ============================================================================

/// Goal decomposition tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalDecomposition {
    pub root: GoalNode,
    pub depth: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalNode {
    pub id: String,
    pub description: String,
    pub node_type: GoalNodeType,
    pub status: NodeStatus,
    pub children: Vec<GoalNode>,
    pub estimated_cost: f32,
    pub estimated_benefit: f32,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalNodeType {
    Goal,
    Subgoal,
    Action,
    Condition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Blocked,
}

/// Action plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionPlan {
    pub id: String,
    pub goal: String,
    pub steps: Vec<PlanStep>,
    pub estimated_duration: Duration,
    pub success_probability: f32,
    pub alternatives: Vec<ActionPlan>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanStep {
    pub step_id: usize,
    pub action: String,
    pub preconditions: Vec<Condition>,
    pub postconditions: Vec<Condition>,
    pub duration: Duration,
    pub resources_required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_type: ConditionType,
    pub description: String,
    pub satisfied: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConditionType {
    State,
    Resource,
    Time,
    External,
}

/// Hierarchical task network planner
pub struct HTNPlanner {
    domain_library: HashMap<String, TaskMethod>,
    max_depth: usize,
}

#[derive(Debug, Clone)]
pub struct TaskMethod {
    pub task: String,
    pub methods: Vec<DecompositionMethod>,
}

#[derive(Debug, Clone)]
pub struct DecompositionMethod {
    pub name: String,
    pub preconditions: Vec<String>,
    pub subtasks: Vec<String>,
    pub ordering: Vec<(String, String)>,
}

impl HTNPlanner {
    pub fn new() -> Self {
        let mut domain_library = HashMap::new();

        // Add basic planning methods
        domain_library.insert("achieve".to_string(), TaskMethod {
            task: "achieve".to_string(),
            methods: vec![
                DecompositionMethod {
                    name: "direct".to_string(),
                    preconditions: vec![],
                    subtasks: vec!["do_action".to_string()],
                    ordering: vec![],
                },
                DecompositionMethod {
                    name: "hierarchical".to_string(),
                    preconditions: vec![],
                    subtasks: vec!["plan".to_string(), "execute".to_string()],
                    ordering: vec![("plan".to_string(), "execute".to_string())],
                },
            ],
        });

        Self {
            domain_library,
            max_depth: 10,
        }
    }

    /// Create a plan to achieve a goal
    pub fn plan(&self, goal: &str, initial_state: &WorldState) -> Option<ActionPlan> {
        let plan_id = generate_id();

        let steps = self.decompose_goal(goal, 0)?;

        Some(ActionPlan {
            id: plan_id,
            goal: goal.to_string(),
            steps,
            estimated_duration: Duration::from_secs(300),
            success_probability: 0.8,
            alternatives: vec![],
        })
    }

    fn decompose_goal(&self, goal: &str, depth: usize) -> Option<Vec<PlanStep>> {
        if depth >= self.max_depth {
            return None;
        }

        // Simple goal decomposition
        let mut steps = Vec::new();
        let step_id = 1;

        // Parse goal and create steps (simplified)
        let actions: Vec<&str> = goal.split_whitespace().collect();

        for (i, action) in actions.iter().enumerate() {
            steps.push(PlanStep {
                step_id: step_id + i,
                action: action.to_string(),
                preconditions: vec![],
                postconditions: vec![],
                duration: Duration::from_secs(60),
                resources_required: vec![],
            });
        }

        if steps.is_empty() {
            steps.push(PlanStep {
                step_id,
                action: goal.to_string(),
                preconditions: vec![],
                postconditions: vec![],
                duration: Duration::from_secs(60),
                resources_required: vec![],
            });
        }

        Some(steps)
    }

    /// Evaluate plan quality
    pub fn evaluate_plan(&self, plan: &ActionPlan) -> PlanQuality {
        let cost_score = (plan.estimated_duration.as_secs_f32() / 3600.0).min(1.0);
        let success_score = plan.success_probability;

        PlanQuality {
            overall_score: (cost_score + success_score) / 2.0,
            feasibility: success_score,
            efficiency: 1.0 - cost_score,
            risk: 1.0 - success_score,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanQuality {
    pub overall_score: f32,
    pub feasibility: f32,
    pub efficiency: f32,
    pub risk: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub facts: HashMap<String, Fact>,
    pub time: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fact {
    pub predicate: String,
    pub args: Vec<String>,
    pub value: bool,
}

// ============================================================================
// Reflection and Self-Assessment
// ============================================================================

/// Self-reflection engine
pub struct SelfReflector {
    reflection_history: VecDeque<Reflection>,
    max_history: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reflection {
    pub timestamp: Instant,
    pub reflection_type: ReflectionType,
    pub subject: String,
    pub insights: Vec<String>,
    pub confidence: f32,
    pub changes_made: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReflectionType {
    Performance,
    Capability,
    Belief,
    Goal,
    Strategy,
}

impl SelfReflector {
    pub fn new() -> Self {
        Self {
            reflection_history: VecDeque::new(),
            max_history: 100,
        }
    }

    /// Reflect on performance
    pub fn reflect_on_performance(&mut self, outcome: &Outcome) -> Vec<String> {
        let mut insights = Vec::new();

        // Analyze outcome quality
        match outcome.result_type {
            ResultType::Success => {
                insights.push("Goal was achieved successfully".to_string());
                if outcome.efficiency > 0.8 {
                    insights.push("High efficiency in execution".to_string());
                }
            }
            ResultType::Failure => {
                insights.push("Goal was not achieved".to_string());
                if let Some(reason) = &outcome.failure_reason {
                    insights.push(format!("Possible cause: {}", reason));
                }
            }
            ResultType::Partial => {
                insights.push("Goal partially achieved".to_string());
            }
        }

        // Generate reflection
        let reflection = Reflection {
            timestamp: Instant::now(),
            reflection_type: ReflectionType::Performance,
            subject: outcome.description.clone(),
            insights: insights.clone(),
            confidence: 0.7,
            changes_made: vec![],
        };

        self.add_reflection(reflection);
        insights
    }

    /// Reflect on capabilities
    pub fn reflect_on_capabilities(&mut self, capability: &str, performance: f32) -> Vec<String> {
        let mut insights = Vec::new();

        if performance > 0.8 {
            insights.push(format!("Strong performance in {}", capability));
        } else if performance > 0.5 {
            insights.push(format!("Moderate performance in {}", capability));
        } else {
            insights.push(format!("Weak performance in {}", capability));
            insights.push(format!("Need to improve {}", capability));
        }

        let reflection = Reflection {
            timestamp: Instant::now(),
            reflection_type: ReflectionType::Capability,
            subject: capability.to_string(),
            insights: insights.clone(),
            confidence: 0.6,
            changes_made: vec![],
        };

        self.add_reflection(reflection);
        insights
    }

    /// Reflect on beliefs
    pub fn reflect_on_belief(&mut self, belief: &str, evidence: &[String]) -> Vec<String> {
        let mut insights = Vec::new();

        let supporting = evidence.iter().filter(|e| e.starts_with("+")).count();
        let contradicting = evidence.iter().filter(|e| e.starts_with("-")).count();

        if supporting > contradicting {
            insights.push(format!("Belief '{}' is supported by evidence", belief));
        } else if contradicting > supporting {
            insights.push(format!("Belief '{}' may need revision", belief));
            insights.push("Consider alternative beliefs".to_string());
        } else {
            insights.push(format!("Belief '{}' is uncertain", belief));
        }

        insights
    }

    fn add_reflection(&mut self, reflection: Reflection) {
        self.reflection_history.push_back(reflection);

        while self.reflection_history.len() > self.max_history {
            self.reflection_history.pop_front();
        }
    }

    /// Get reflection patterns
    pub fn analyze_patterns(&self) -> ReflectionPatterns {
        let total = self.reflection_history.len();
        let mut type_counts = [0usize; 5];

        for r in &self.reflection_history {
            let idx = match r.reflection_type {
                ReflectionType::Performance => 0,
                ReflectionType::Capability => 1,
                ReflectionType::Belief => 2,
                ReflectionType::Goal => 3,
                ReflectionType::Strategy => 4,
            };
            type_counts[idx] += 1;
        }

        ReflectionPatterns {
            total_reflections: total,
            performance_count: type_counts[0],
            capability_count: type_counts[1],
            belief_count: type_counts[2],
            goal_count: type_counts[3],
            strategy_count: type_counts[4],
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReflectionPatterns {
    pub total_reflections: usize,
    pub performance_count: usize,
    pub capability_count: usize,
    pub belief_count: usize,
    pub goal_count: usize,
    pub strategy_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub description: String,
    pub result_type: ResultType,
    pub efficiency: f32,
    pub failure_reason: Option<String>,
    pub timestamp: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResultType {
    Success,
    Failure,
    Partial,
}

// ============================================================================
// Confidence and Uncertainty
// ============================================================================

/// Confidence calibration system
pub struct ConfidenceCalibrator {
    calibration_history: VecDeque<CalibrationPoint>,
    calibration_errors: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalibrationPoint {
    pub predicted_confidence: f32,
    pub actual_confidence: f32,
    pub domain: String,
    pub timestamp: Instant,
}

impl ConfidenceCalibrator {
    pub fn new() -> Self {
        Self {
            calibration_history: VecDeque::new(),
            calibration_errors: Vec::new(),
        }
    }

    /// Update with new calibration data
    pub fn calibrate(&mut self, predicted: f32, actual: f32, domain: &str) {
        let point = CalibrationPoint {
            predicted_confidence: predicted,
            actual_confidence: actual,
            domain: domain.to_string(),
            timestamp: Instant::now(),
        };

        self.calibration_history.push_back(point);

        // Calculate error
        let error = (predicted - actual).abs();
        self.calibration_errors.push(error);

        // Keep history bounded
        while self.calibration_history.len() > 1000 {
            self.calibration_history.pop_front();
        }
        while self.calibration_errors.len() > 1000 {
            self.calibration_errors.pop_front();
        }
    }

    /// Get calibration error
    pub fn calibration_error(&self) -> f32 {
        if self.calibration_errors.is_empty() {
            return 0.0;
        }

        self.calibration_errors.iter().sum::<f32>() / self.calibration_errors.len() as f32
    }

    /// Adjust confidence based on calibration
    pub fn adjust_confidence(&self, raw_confidence: f32, domain: &str) -> f32 {
        // Simple calibration adjustment
        let domain_points: Vec<_> = self.calibration_history
            .iter()
            .filter(|p| p.domain == domain)
            .collect();

        if domain_points.is_empty() {
            return raw_confidence;
        }

        // Calculate domain-specific bias
        let bias: f32 = domain_points.iter()
            .map(|p| p.predicted_confidence - p.actual_confidence)
            .sum::<f32>() / domain_points.len() as f32;

        // Adjust confidence
        (raw_confidence - bias).clamp(0.0, 1.0)
    }

    /// Get confidence interval
    pub fn confidence_interval(&self, confidence: f32, domain: &str) -> (f32, f32) {
        let domain_points: Vec<_> = self.calibration_history
            .iter()
            .filter(|p| p.domain == domain)
            .collect();

        if domain_points.len() < 5 {
            // Not enough data, use default interval
            let margin = 0.1;
            return (confidence - margin, confidence + margin);
        }

        // Calculate interval based on historical errors
        let errors: Vec<f32> = domain_points.iter()
            .map(|p| (p.predicted_confidence - p.actual_confidence).abs())
            .collect();

        let avg_error = errors.iter().sum::<f32>() / errors.len() as f32;

        (
            (confidence - avg_error * 2.0).max(0.0),
            (confidence + avg_error * 2.0).min(1.0),
        )
    }
}

// ============================================================================
// Introspection and Monitoring
// ============================================================================

/// Self-monitoring system
pub struct SelfMonitor {
    cognitive_load: f32,
    attention_focus: Option<String>,
    active_processes: Vec<ActiveProcess>,
    metrics: SystemMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveProcess {
    pub process_id: String,
    pub process_type: String,
    pub start_time: Instant,
    pub status: ProcessStatus,
    pub progress: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessStatus {
    Running,
    Paused,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub response_time: Duration,
    pub error_rate: f32,
    pub throughput: f32,
}

impl SelfMonitor {
    pub fn new() -> Self {
        Self {
            cognitive_load: 0.0,
            attention_focus: None,
            active_processes: vec![],
            metrics: SystemMetrics {
                cpu_usage: 0.0,
                memory_usage: 0.0,
                response_time: Duration::from_millis(100),
                error_rate: 0.0,
                throughput: 1.0,
            },
        }
    }

    /// Update cognitive load
    pub fn update_cognitive_load(&mut self, load: f32) {
        self.cognitive_load = load.clamp(0.0, 1.0);
    }

    /// Set attention focus
    pub fn set_attention(&mut self, focus: &str) {
        self.attention_focus = Some(focus.to_string());
    }

    /// Start a new process
    pub fn start_process(&mut self, process_type: &str) -> String {
        let id = generate_id();
        self.active_processes.push(ActiveProcess {
            process_id: id.clone(),
            process_type: process_type.to_string(),
            start_time: Instant::now(),
            status: ProcessStatus::Running,
            progress: 0.0,
        });
        id
    }

    /// Update process progress
    pub fn update_process(&mut self, process_id: &str, progress: f32) {
        if let Some(process) = self.active_processes.iter_mut().find(|p| p.process_id == process_id) {
            process.progress = progress.clamp(0.0, 1.0);
        }
    }

    /// Complete a process
    pub fn complete_process(&mut self, process_id: &str, success: bool) {
        if let Some(process) = self.active_processes.iter_mut().find(|p| p.process_id == process_id) {
            process.status = if success {
                ProcessStatus::Completed
            } else {
                ProcessStatus::Failed
            };
            process.progress = if success { 1.0 } else { 0.0 };
        }
    }

    /// Get current state snapshot
    pub fn get_snapshot(&self) -> MonitorSnapshot {
        MonitorSnapshot {
            cognitive_load: self.cognitive_load,
            attention_focus: self.attention_focus.clone(),
            active_processes: self.active_processes.len(),
            metrics: self.metrics.clone(),
            timestamp: Instant::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorSnapshot {
    pub cognitive_load: f32,
    pub attention_focus: Option<String>,
    pub active_processes: usize,
    pub metrics: SystemMetrics,
    pub timestamp: Instant,
}

// ============================================================================
// Main Meta-Cognition System
// ============================================================================

/// Complete meta-cognition system
pub struct MetaCognition {
    // Self-awareness
    self_model: SelfModel,
    self_monitor: SelfMonitor,
    confidence_calibrator: ConfidenceCalibrator,

    // Planning
    planner: HTNPlanner,
    decision_history: VecDeque<Decision>,

    // Reflection
    reflector: SelfReflector,

    // Configuration
    config: MetaCognitionConfig,
}

#[derive(Debug, Clone)]
pub struct MetaCognitionConfig {
    pub reflection_interval: Duration,
    pub planning_horizon: Duration,
    pub confidence_threshold: f32,
    pub max_alternatives: usize,
    pub enable_self_improvement: bool,
}

impl Default for MetaCognitionConfig {
    fn default() -> Self {
        Self {
            reflection_interval: Duration::from_secs(60),
            planning_horizon: Duration::from_secs(3600),
            confidence_threshold: 0.7,
            max_alternatives: 3,
            enable_self_improvement: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub decision_id: String,
    pub context: String,
    pub options: Vec<DecisionOption>,
    pub chosen: Option<usize>,
    pub reasoning: String,
    pub outcome: Option<String>,
    pub timestamp: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionOption {
    pub option_id: String,
    pub description: String,
    pub expected_value: f32,
    pub risk: f32,
    pub feasibility: f32,
}

impl MetaCognition {
    pub fn new(name: &str, version: &str) -> Self {
        Self {
            self_model: SelfModel::new(name, version),
            self_monitor: SelfMonitor::new(),
            confidence_calibrator: ConfidenceCalibrator::new(),
            planner: HTNPlanner::new(),
            decision_history: VecDeque::new(),
            reflector: SelfReflector::new(),
            config: MetaCognitionConfig::default(),
        }
    }

    // === Self-Awareness ===

    /// Get self-model
    pub fn get_self_model(&self) -> &SelfModel {
        &self.self_model
    }

    /// Update self-belief
    pub fn update_belief(&mut self, belief: &str, confidence: f32, evidence: &[String]) {
        // Check if belief exists
        if let Some(existing) = self.self_model.beliefs.iter_mut().find(|b| b.belief == belief) {
            existing.confidence = confidence;
            existing.evidence.extend(evidence.iter().cloned());
        } else {
            self.self_model.beliefs.push(SelfBelief {
                belief: belief.to_string(),
                confidence,
                evidence: evidence.to_vec(),
                created_at: Instant::now(),
            });
        }
    }

    /// Get current mental state
    pub fn get_mental_state(&self) -> MentalState {
        let snapshot = self.self_monitor.get_snapshot();

        MentalState {
            cognitive_load: snapshot.cognitive_load,
            attention_focus: snapshot.attention_focus.clone(),
            confidence: self.self_model.overall_capability(),
            emotional_tone: self.self_model.emotional_state.clone(),
            readiness: 1.0 - snapshot.cognitive_load,
        }
    }

    // === Planning ===

    /// Create a plan
    pub fn create_plan(&self, goal: &str) -> Option<ActionPlan> {
        let initial_state = WorldState {
            facts: HashMap::new(),
            time: Instant::now(),
        };

        self.planner.plan(goal, &initial_state)
    }

    /// Evaluate multiple plans
    pub fn evaluate_plans(&self, plans: &[ActionPlan]) -> Vec<(String, PlanQuality)> {
        plans.iter()
            .map(|p| (p.id.clone(), self.planner.evaluate_plan(p)))
            .collect()
    }

    // === Decision Making ===

    /// Make a decision
    pub fn decide(&mut self, context: &str, options: Vec<DecisionOption>) -> DecisionOption {
        let decision_id = generate_id();

        // Evaluate options
        let mut evaluated: Vec<_> = options.iter()
            .map(|opt| {
                let score = (opt.expected_value * 0.5
                    + (1.0 - opt.risk) * 0.3
                    + opt.feasibility * 0.2);
                (opt.clone(), score)
            })
            .collect();

        // Sort by score
        evaluated.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Select best option
        let chosen = evaluated.first().cloned().map(|(opt, _)| opt);

        let reasoning = format!(
            "Selected option with expected value {:.2}, risk {:.2}, feasibility {:.2}",
            chosen.as_ref().map(|o| o.expected_value).unwrap_or(0.0),
            chosen.as_ref().map(|o| o.risk).unwrap_or(1.0),
            chosen.as_ref().map(|o| o.feasibility).unwrap_or(0.0),
        );

        // Record decision
        let decision = Decision {
            decision_id,
            context: context.to_string(),
            options: options.clone(),
            chosen: chosen.as_ref().and_then(|o| {
                options.iter().position(|opt| opt.option_id == o.option_id)
            }),
            reasoning,
            outcome: None,
            timestamp: Instant::now(),
        };

        self.decision_history.push_back(decision);

        // Keep history bounded
        while self.decision_history.len() > 100 {
            self.decision_history.pop_front();
        }

        chosen.unwrap_or_else(|| options.first().cloned().unwrap())
    }

    // === Reflection ===

    /// Reflect on experience
    pub fn reflect(&mut self, experience: &Experience) -> Vec<String> {
        match experience.experience_type {
            ExperienceType::Outcome(outcome) => {
                self.reflector.reflect_on_performance(&outcome)
            }
            ExperienceType::Capability(cap, perf) => {
                self.reflector.reflect_on_capabilities(cap, *perf)
            }
            ExperienceType::Belief(belief, evidence) => {
                self.reflector.reflect_on_belief(belief, evidence)
            }
        }
    }

    /// Get reflection insights
    pub fn get_insights(&self) -> Vec<String> {
        self.reflector.analyze_patterns()
            .to_string()
            .lines()
            .map(|s| s.to_string())
            .collect()
    }

    // === Confidence ===

    /// Calibrate confidence
    pub fn calibrate_confidence(&mut self, predicted: f32, actual: f32, domain: &str) -> f32 {
        self.confidence_calibrator.calibrate(predicted, actual, domain);
        self.confidence_calibrator.adjust_confidence(predicted, domain)
    }

    /// Get confidence interval
    pub fn get_confidence_interval(&self, confidence: f32, domain: &str) -> (f32, f32) {
        self.confidence_calibrator.confidence_interval(confidence, domain)
    }

    // === Monitoring ===

    /// Update monitoring state
    pub fn update_monitoring(&mut self, cognitive_load: f32, focus: Option<&str>) {
        self.self_monitor.update_cognitive_load(cognitive_load);
        if let Some(f) = focus {
            self.self_monitor.set_attention(f);
        }
    }

    /// Get system status
    pub fn get_status(&self) -> MetaCognitionStatus {
        MetaCognitionStatus {
            self_model: format!("AGI v{}", self.self_model.identity.version),
            capability_score: self.self_model.overall_capability(),
            cognitive_load: self.self_monitor.cognitive_load,
            active_processes: self.self_monitor.active_processes.len(),
            confidence_calibration_error: self.confidence_calibrator.calibration_error(),
            total_decisions: self.decision_history.len(),
            total_reflections: self.reflector.reflection_history.len(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalState {
    pub cognitive_load: f32,
    pub attention_focus: Option<String>,
    pub confidence: f32,
    pub emotional_tone: Option<String>,
    pub readiness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaCognitionStatus {
    pub self_model: String,
    pub capability_score: f32,
    pub cognitive_load: f32,
    pub active_processes: usize,
    pub confidence_calibration_error: f32,
    pub total_decisions: usize,
    pub total_reflections: usize,
}

#[derive(Debug, Clone)]
pub enum Experience<'a> {
    Outcome(Outcome),
    Capability(&'a str, f32),
    Belief(&'a str, Vec<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExperienceType {
    Outcome(Outcome),
    Capability(&'static str, f32),
    Belief(&'static str, Vec<String>),
}

// Helper function
fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("id_{}", now % 1_000_000)
}

// Display implementation for patterns
impl std::fmt::Display for ReflectionPatterns {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reflection Patterns:\n")?;
        write!(f, "  Total: {}\n", self.total_reflections)?;
        write!(f, "  Performance: {}\n", self.performance_count)?;
        write!(f, "  Capability: {}\n", self.capability_count)?;
        write!(f, "  Belief: {}\n", self.belief_count)?;
        write!(f, "  Goal: {}\n", self.goal_count)?;
        write!(f, "  Strategy: {}", self.strategy_count)
    }
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_model_creation() {
        let model = SelfModel::new("TestAGI", "1.0.0");

        assert_eq!(model.identity.name, "TestAGI");
        assert_eq!(model.identity.version, "1.0.0");
        assert!(model.overall_capability() > 0.0);
    }

    #[test]
    fn test_capability_scoring() {
        assert!((CapabilityLevel::None.to_score() - 0.0).abs() < 0.001);
        assert!((CapabilityLevel::Basic.to_score() - 0.25).abs() < 0.001);
        assert!((CapabilityLevel::Advanced.to_score() - 0.75).abs() < 0.001);
        assert!((CapabilityLevel::Expert.to_score() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_htn_planner() {
        let planner = HTNPlanner::new();
        let state = WorldState {
            facts: HashMap::new(),
            time: Instant::now(),
        };

        let plan = planner.plan("achieve goal", &state);
        assert!(plan.is_some());
    }

    #[test]
    fn test_confidence_calibration() {
        let mut calibrator = ConfidenceCalibrator::new();

        calibrator.calibrate(0.8, 0.9, "test_domain");
        calibrator.calibrate(0.7, 0.6, "test_domain");

        let adjusted = calibrator.adjust_confidence(0.75, "test_domain");
        assert!(adjusted >= 0.0 && adjusted <= 1.0);
    }

    #[test]
    fn test_self_monitor() {
        let mut monitor = SelfMonitor::new();

        monitor.update_cognitive_load(0.5);
        monitor.set_attention("reasoning");

        let process_id = monitor.start_process("planning");
        monitor.update_process(&process_id, 0.5);
        monitor.complete_process(&process_id, true);

        let snapshot = monitor.get_snapshot();
        assert_eq!(snapshot.cognitive_load, 0.5);
        assert_eq!(snapshot.attention_focus, Some("reasoning".to_string()));
    }

    #[test]
    fn test_meta_cognition_decision() {
        let mut meta = MetaCognition::new("Test", "1.0");

        let options = vec![
            DecisionOption {
                option_id: "opt1".to_string(),
                description: "Option 1".to_string(),
                expected_value: 0.8,
                risk: 0.2,
                feasibility: 0.9,
            },
            DecisionOption {
                option_id: "opt2".to_string(),
                description: "Option 2".to_string(),
                expected_value: 0.6,
                risk: 0.3,
                feasibility: 0.8,
            },
        ];

        let decision = meta.decide("test_context", options);
        assert_eq!(decision.option_id, "opt1");
    }

    #[test]
    fn test_reflection() {
        let mut reflector = SelfReflector::new();

        let outcome = Outcome {
            description: "Task completed".to_string(),
            result_type: ResultType::Success,
            efficiency: 0.9,
            failure_reason: None,
            timestamp: Instant::now(),
        };

        let insights = reflector.reflect_on_performance(&outcome);
        assert!(!insights.is_empty());
    }

    #[test]
    fn test_mental_state() {
        let mut meta = MetaCognition::new("Test", "1.0");
        meta.update_monitoring(0.3, Some("analysis"));

        let state = meta.get_mental_state();
        assert_eq!(state.cognitive_load, 0.3);
        assert_eq!(state.attention_focus, Some("analysis".to_string()));
    }
}
