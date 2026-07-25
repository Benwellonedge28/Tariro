// src/executive_function.rs

//! Executive Function for Tariro AGI
//! Central coordinator that integrates all AGI subsystems for unified goal-directed behavior

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;

/// The central executive function coordinating all AGI subsystems
pub struct ExecutiveFunction {
    pub goal_manager: GoalManager,
    pub attention_controller: AttentionController,
    pub behavior_selector: BehaviorSelector,
    pub metacognitive_monitor: MetacognitiveMonitor,
    pub resource_allocator: ResourceAllocator,
    pub state_manager: GlobalStateManager,
    pub integration_bus: IntegrationBus,
}

impl ExecutiveFunction {
    pub fn new() -> Self {
        ExecutiveFunction {
            goal_manager: GoalManager::new(),
            attention_controller: AttentionController::new(),
            behavior_selector: BehaviorSelector::new(),
            metacognitive_monitor: MetacognitiveMonitor::new(),
            resource_allocator: ResourceAllocator::new(),
            state_manager: GlobalStateManager::new(),
            integration_bus: IntegrationBus::new(),
        }
    }

    /// Main processing cycle - called every tick
    pub fn process_cycle(&mut self, perception_input: &PerceptionInput, context: &ProcessingContext) -> ExecutiveDecision {
        println!("[Executive] Starting processing cycle");

        // Step 1: Update global state from perception
        self.state_manager.update_from_perception(perception_input);

        // Step 2: Update goal priorities based on new input
        self.goal_manager.update_priorities(&self.state_manager.current_state);

        // Step 3: Attention selection - what to focus on
        let focus_items = self.attention_controller.select_focus(
            &self.state_manager.current_state,
            &self.goal_manager.active_goals
        );

        // Step 4: Metacognitive monitoring
        self.metacognitive_monitor.check_processing(self);

        // Step 5: Resource allocation
        let resources = self.resource_allocator.allocate(
            &focus_items,
            &self.state_manager.current_state
        );

        // Step 6: Behavior selection
        let selected_behavior = self.behavior_selector.select(
            &self.goal_manager.highest_priority_goal(),
            &focus_items,
            &resources,
            context
        );

        // Step 7: Decision output
        let decision = ExecutiveDecision {
            selected_behavior,
            focus_of_attention: focus_items,
            resource_allocation: resources,
            goal_updates: self.goal_manager.pending_updates.clone(),
            metacognitive_flags: self.metacognitive_monitor.current_flags.clone(),
            confidence: self.calculate_confidence(),
        };

        // Step 8: Update state
        self.state_manager.record_decision(&decision);

        println!("[Executive] Cycle complete. Decision confidence: {:.2}", decision.confidence);
        decision
    }

    fn calculate_confidence(&self) -> f64 {
        let processing_quality = 1.0 - self.metacognitive_monitor.processing_errors;
        let resource_availability = self.resource_allocator.available_resources();
        let goal_clarity = self.goal_manager.clarity_of_goals();

        (processing_quality + resource_availability + goal_clarity) / 3.0
    }

    /// Handle goal from user or internal generation
    pub fn receive_goal(&mut self, goal: &str, priority: f64, goal_type: GoalCategory) {
        println!("[Executive] Received goal: {} (priority: {:.2})", goal, priority);
        self.goal_manager.add_goal(goal, priority, goal_type);
    }

    /// Integrate with external systems via message bus
    pub fn handle_message(&mut self, message: &SystemMessage) -> Option<ExecutiveDecision> {
        match message {
            SystemMessage::PerceptionUpdate(input) => {
                Some(self.process_cycle(input, &ProcessingContext::default()))
            }
            SystemMessage::NewGoal(goal) => {
                self.receive_goal(&goal.goal_text, goal.priority, goal.goal_type);
                None
            }
            SystemMessage::ResourceChange(change) => {
                self.resource_allocator.apply_change(change);
                None
            }
            SystemMessage::ExternalInterrupt(interrupt) => {
                self.handle_interrupt(interrupt)
            }
        }
    }

    fn handle_interrupt(&mut self, interrupt: &Interrupt) -> Option<ExecutiveDecision> {
        if interrupt.criticality > 0.8 {
            // High priority interrupt - preempt current processing
            self.state_manager.enter_interrupt_mode(interrupt);
            Some(self.generate_interrupt_response(interrupt))
        } else {
            // Queue for later processing
            self.state_manager.queue_interrupt(interrupt);
            None
        }
    }

    fn generate_interrupt_response(&self, interrupt: &Interrupt) -> ExecutiveDecision {
        ExecutiveDecision {
            selected_behavior: Behavior {
                behavior_type: interrupt.response_type.clone(),
                parameters: interrupt.parameters.clone(),
                expected_outcome: interrupt.expected_effect.clone(),
            },
            focus_of_attention: vec![interrupt.focus.clone()],
            resource_allocation: ResourceAllocation::maximum(),
            goal_updates: Vec::new(),
            metacognitive_flags: vec![MetacognitiveFlag {
                flag_type: "interrupt_handled".to_string(),
                severity: interrupt.criticality,
            }],
            confidence: 0.95,
        }
    }
}

/// Goal management subsystem
pub struct GoalManager {
    pub active_goals: Vec<Goal>,
    pub completed_goals: Vec<CompletedGoal>,
    pub failed_goals: Vec<FailedGoal>,
    pub pending_updates: Vec<GoalUpdate>,
    pub goal_hierarchy: GoalHierarchy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: String,
    pub goal_text: String,
    pub goal_type: GoalCategory,
    pub priority: f64,
    pub state: GoalState,
    pub subgoals: Vec<String>,
    pub deadline: Option<DateTime<Utc>>,
    pub progress: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalCategory {
    Survival,
    Task,
    Learning,
    Social,
    Maintenance,
    Creative,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalState {
    Active,
    Suspended,
    Completed,
    Failed,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedGoal {
    pub goal_id: String,
    pub completion_time: DateTime<Utc>,
    pub outcome: String,
    pub lessons_learned: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedGoal {
    pub goal_id: String,
    pub failure_time: DateTime<Utc>,
    pub reason: String,
    pub recovery_possible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalUpdate {
    pub goal_id: String,
    pub update_type: GoalUpdateType,
    pub new_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GoalUpdateType {
    PriorityChange,
    ProgressUpdate,
    StateChange,
    SubgoalAddition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalHierarchy {
    pub root_goals: Vec<String>,
    pub hierarchy_levels: u32,
}

impl GoalManager {
    pub fn new() -> Self {
        GoalManager {
            active_goals: Vec::new(),
            completed_goals: Vec::new(),
            failed_goals: Vec::new(),
            pending_updates: Vec::new(),
            goal_hierarchy: GoalHierarchy {
                root_goals: Vec::new(),
                hierarchy_levels: 0,
            },
        }
    }

    pub fn add_goal(&mut self, goal_text: &str, priority: f64, goal_type: GoalCategory) {
        let goal = Goal {
            id: format!("goal_{}", Utc::now().timestamp_nanos()),
            goal_text: goal_text.to_string(),
            goal_type,
            priority,
            state: GoalState::Active,
            subgoals: Vec::new(),
            deadline: None,
            progress: 0.0,
            created_at: Utc::now(),
        };

        self.active_goals.push(goal);
        self.sort_by_priority();
    }

    pub fn update_priorities(&mut self, _state: &GlobalState) {
        // Update priorities based on state changes, deadlines, etc.
        for goal in &mut self.active_goals {
            // Time pressure increases priority
            if let Some(deadline) = goal.deadline {
                let time_remaining = (deadline - Utc::now()).num_seconds() as f64;
                if time_remaining < 3600.0 {
                    // Less than 1 hour
                    goal.priority = (goal.priority * 1.2).min(1.0);
                }
            }
        }
        self.sort_by_priority();
    }

    pub fn highest_priority_goal(&self) -> Option<&Goal> {
        self.active_goals.iter().max_by(|a, b| {
            a.priority.partial_cmp(&b.priority).unwrap()
        })
    }

    pub fn clarity_of_goals(&self) -> f64 {
        if self.active_goals.is_empty() {
            return 0.0;
        }
        // Higher clarity when fewer, clearer goals
        let goal_count_factor = 1.0 - (self.active_goals.len() as f64 / 10.0).min(0.8);
        let priority_spread = if self.active_goals.len() > 1 {
            let priorities: Vec<f64> = self.active_goals.iter().map(|g| g.priority).collect();
            let max = priorities.iter().cloned().fold(0.0f64, f64::max);
            let min = priorities.iter().cloned().fold(1.0f64, f64::min);
            max - min
        } else {
            1.0
        };

        (goal_count_factor + priority_spread) / 2.0
    }

    fn sort_by_priority(&mut self) {
        self.active_goals.sort_by(|a, b| {
            b.priority.partial_cmp(&a.priority).unwrap()
        });
    }

    pub fn complete_goal(&mut self, goal_id: &str, outcome: &str) {
        if let Some(pos) = self.active_goals.iter().position(|g| g.id == goal_id) {
            let goal = self.active_goals.remove(pos);
            self.completed_goals.push(CompletedGoal {
                goal_id: goal.id,
                completion_time: Utc::now(),
                outcome: outcome.to_string(),
                lessons_learned: Vec::new(),
            });
        }
    }
}

/// Attention controller - manages what information is processed
pub struct AttentionController {
    pub current_focus: Vec<FocusItem>,
    pub attention_history: Vec<AttentionSnapshot>,
    pub saliency_threshold: f64,
    pub novelty_bias: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusItem {
    pub item_id: String,
    pub saliency: f64,
    pub novelty: f64,
    pub relevance: f64,
    pub source: AttentionSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttentionSource {
    Perception,
    Memory,
    Goal,
    Social,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionSnapshot {
    pub timestamp: DateTime<Utc>,
    pub focus_items: Vec<String>,
    pub switches: u32,
}

impl AttentionController {
    pub fn new() -> Self {
        AttentionController {
            current_focus: Vec::new(),
            attention_history: Vec::new(),
            saliency_threshold: 0.3,
            novelty_bias: 0.2,
        }
    }

    pub fn select_focus(&mut self, state: &GlobalState, goals: &[Goal]) -> Vec<String> {
        let mut candidates = Vec::new();

        // Add perception-based candidates
        for item in &state.perceptual saliency {
            let score = self.calculate_saliency(item);
            if score >= self.saliency_threshold {
                candidates.push(FocusItem {
                    item_id: item.item_id.clone(),
                    saliency: score,
                    novelty: item.novelty,
                    relevance: self.calculate_relevance(&item.item_id, goals),
                    source: AttentionSource::Perception,
                });
            }
        }

        // Add goal-based candidates
        for goal in goals {
            if goal.priority > 0.5 {
                candidates.push(FocusItem {
                    item_id: goal.id.clone(),
                    saliency: goal.priority,
                    novelty: 0.5,
                    relevance: 1.0,
                    source: AttentionSource::Goal,
                });
            }
        }

        // Sort by weighted score
        candidates.sort_by(|a, b| {
            let score_a = a.saliency * 0.4 + a.relevance * 0.4 + a.novelty * self.novelty_bias;
            let score_b = b.saliency * 0.4 + b.relevance * 0.4 + b.novelty * self.novelty_bias;
            score_b.partial_cmp(&score_a).unwrap()
        });

        // Take top N items
        let selected: Vec<String> = candidates.into_iter()
            .take(5)
            .map(|c| c.item_id)
            .collect();

        self.current_focus = candidates.into_iter().take(5).collect();

        println!("[Attention] Selected {} focus items", selected.len());
        selected
    }

    fn calculate_saliency(&self, item: &PerceptualSaliency) -> f64 {
        item.intensity * 0.3 + item.novelty * 0.3 + item.emotional * 0.2 + item.reward * 0.2
    }

    fn calculate_relevance(&self, item_id: &str, goals: &[Goal]) -> f64 {
        // Simplified relevance calculation
        let mut relevance = 0.5;
        for goal in goals {
            if goal.goal_text.contains(item_id) || item_id.contains(&goal.id) {
                relevance += goal.priority * 0.5;
            }
        }
        relevance.min(1.0)
    }
}

/// Behavior selector - chooses which action to execute
pub struct BehaviorSelector {
    pub behavior_library: HashMap<String, BehaviorTemplate>,
    pub recent_selections: VecDeque<BehaviorSelection>,
    pub exploration_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Behavior {
    pub behavior_type: String,
    pub parameters: HashMap<String, String>,
    pub expected_outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorTemplate {
    pub name: String,
    pub category: BehaviorCategory,
    pub preconditions: Vec<String>,
    pub effects: Vec<String>,
    pub utility_range: (f64, f64),
    pub resource_requirements: ResourceRequirement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BehaviorCategory {
    Perception,
    Reasoning,
    Action,
    Communication,
    Learning,
    Reflection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRequirement {
    pub computation: f64,
    pub memory: f64,
    pub time: f64,
    pub energy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorSelection {
    pub behavior: String,
    pub utility: f64,
    pub timestamp: DateTime<Utc>,
}

impl BehaviorSelector {
    pub fn new() -> Self {
        BehaviorSelector {
            behavior_library: Self::default_library(),
            recent_selections: VecDeque::new(),
            exploration_rate: 0.2,
        }
    }

    pub fn select(
        &self,
        goal: Option<&Goal>,
        focus_items: &[String],
        resources: &ResourceAllocation,
        context: &ProcessingContext
    ) -> Behavior {
        let mut candidates = Vec::new();

        // Score each behavior
        for (name, template) in &self.behavior_library {
            let utility = self.score_behavior(template, goal, focus_items, resources, context);
            candidates.push((name.clone(), utility));
        }

        // Sort by utility
        candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Either exploit (best) or explore (random)
        if rand_simple() < self.exploration_rate {
            if let Some((name, _)) = candidates.iter().choose(&mut rand::thread_rng()) {
                return self.instantiate_behavior(name, focus_items, goal);
            }
        }

        if let Some((name, _)) = candidates.first() {
            self.instantiate_behavior(name, focus_items, goal)
        } else {
            Behavior {
                behavior_type: "idle".to_string(),
                parameters: HashMap::new(),
                expected_outcome: "maintain_state".to_string(),
            }
        }
    }

    fn score_behavior(
        &self,
        template: &BehaviorTemplate,
        goal: Option<&Goal>,
        focus_items: &[String],
        resources: &ResourceAllocation,
        context: &ProcessingContext
    ) -> f64 {
        let mut score = 0.0;

        // Goal alignment
        if let Some(goal) = goal {
            for effect in &template.effects {
                if goal.goal_text.contains(effect) || effect.contains(&goal.goal_text) {
                    score += goal.priority * 0.5;
                }
            }
        }

        // Focus alignment
        let focus_overlap = focus_items.iter()
            .filter(|item| template.preconditions.iter().any(|p| p.contains(item)))
            .count() as f64;
        score += focus_overlap / (template.preconditions.len().max(1) as f64) * 0.3;

        // Resource availability
        let resource_fit = resources.fits(&template.resource_requirements);
        score *= resource_fit;

        // Context appropriateness
        if context.task_type == template.category.as_str() {
            score *= 1.2;
        }

        (score + template.utility_range.0) / 2.0
    }

    fn instantiate_behavior(&self, name: &str, focus_items: &[String], goal: Option<&Goal>) -> Behavior {
        let mut params = HashMap::new();
        params.insert("focus".to_string(), focus_items.join(","));
        if let Some(g) = goal {
            params.insert("goal".to_string(), g.goal_text.clone());
        }

        Behavior {
            behavior_type: name.to_string(),
            parameters: params,
            expected_outcome: format!("achieve_{}", name),
        }
    }

    fn default_library() -> HashMap<String, BehaviorTemplate> {
        let mut library = HashMap::new();

        library.insert("perceive".to_string(), BehaviorTemplate {
            name: "perceive".to_string(),
            category: BehaviorCategory::Perception,
            preconditions: vec!["sensory_input".to_string()],
            effects: vec!["understanding".to_string(), "context".to_string()],
            utility_range: (0.3, 0.8),
            resource_requirements: ResourceRequirement {
                computation: 0.2,
                memory: 0.1,
                time: 0.1,
                energy: 0.1,
            },
        });

        library.insert("reason".to_string(), BehaviorTemplate {
            name: "reason".to_string(),
            category: BehaviorCategory::Reasoning,
            preconditions: vec!["information".to_string()],
            effects: vec!["insight".to_string(), "decision".to_string()],
            utility_range: (0.4, 0.9),
            resource_requirements: ResourceRequirement {
                computation: 0.5,
                memory: 0.3,
                time: 0.3,
                energy: 0.2,
            },
        });

        library.insert("act".to_string(), BehaviorTemplate {
            name: "act".to_string(),
            category: BehaviorCategory::Action,
            preconditions: vec!["decision".to_string(), "capability".to_string()],
            effects: vec!["state_change".to_string()],
            utility_range: (0.5, 1.0),
            resource_requirements: ResourceRequirement {
                computation: 0.3,
                memory: 0.1,
                time: 0.2,
                energy: 0.4,
            },
        });

        library
    }
}

impl BehaviorCategory {
    pub fn as_str(&self) -> &str {
        match self {
            BehaviorCategory::Perception => "perception",
            BehaviorCategory::Reasoning => "reasoning",
            BehaviorCategory::Action => "action",
            BehaviorCategory::Communication => "communication",
            BehaviorCategory::Learning => "learning",
            BehaviorCategory::Reflection => "reflection",
        }
    }
}

/// Metacognitive monitor - watches the thinking process
pub struct MetacognitiveMonitor {
    pub processing_errors: f64,
    pub confidence_tracking: Vec<f64>,
    pub cognitive_load: f64,
    pub current_flags: Vec<MetacognitiveFlag>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetacognitiveFlag {
    pub flag_type: String,
    pub severity: f64,
}

impl MetacognitiveMonitor {
    pub fn new() -> Self {
        MetacognitiveMonitor {
            processing_errors: 0.0,
            confidence_tracking: Vec::new(),
            cognitive_load: 0.5,
            current_flags: Vec::new(),
        }
    }

    pub fn check_processing(&mut self, _executive: &ExecutiveFunction) {
        // Monitor for problems
        self.current_flags.clear();

        // Check processing errors
        if self.processing_errors > 0.3 {
            self.current_flags.push(MetacognitiveFlag {
                flag_type: "high_error_rate".to_string(),
                severity: self.processing_errors,
            });
        }

        // Check cognitive load
        if self.cognitive_load > 0.8 {
            self.current_flags.push(MetacognitiveFlag {
                flag_type: "high_cognitive_load".to_string(),
                severity: self.cognitive_load,
            });
        }
    }
}

/// Resource allocator - manages computational resources
pub struct ResourceAllocator {
    pub available: ResourcePool,
    pub allocated: ResourceAllocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePool {
    pub computation: f64,
    pub memory: f64,
    pub time: f64,
    pub energy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAllocation {
    pub computation: f64,
    pub memory: f64,
    pub time: f64,
    pub energy: f64,
}

impl ResourceAllocation {
    pub fn fits(&self, requirement: &ResourceRequirement) -> f64 {
        let comp_fit = (self.computation / requirement.computation).min(1.0);
        let mem_fit = (self.memory / requirement.memory).min(1.0);
        let time_fit = (self.time / requirement.time).min(1.0);
        let energy_fit = (self.energy / requirement.energy).min(1.0);

        (comp_fit + mem_fit + time_fit + energy_fit) / 4.0
    }

    pub fn maximum() -> Self {
        ResourceAllocation {
            computation: 1.0,
            memory: 1.0,
            time: 1.0,
            energy: 1.0,
        }
    }
}

impl ResourceAllocator {
    pub fn new() -> Self {
        ResourceAllocator {
            available: ResourcePool {
                computation: 1.0,
                memory: 1.0,
                time: 1.0,
                energy: 1.0,
            },
            allocated: ResourceAllocation {
                computation: 0.0,
                memory: 0.0,
                time: 0.0,
                energy: 0.0,
            },
        }
    }

    pub fn available_resources(&self) -> f64 {
        let comp_avail = 1.0 - self.allocated.computation;
        let mem_avail = 1.0 - self.allocated.memory;
        let time_avail = 1.0 - self.allocated.time;
        let energy_avail = 1.0 - self.allocated.energy;

        (comp_avail + mem_avail + time_avail + energy_avail) / 4.0
    }

    pub fn allocate(&mut self, focus_items: &[String], _state: &GlobalState) -> ResourceAllocation {
        // Allocate based on focus and current demands
        let focus_factor = (focus_items.len() as f64 / 5.0).min(1.0);

        self.allocated = ResourceAllocation {
            computation: 0.3 * focus_factor + 0.2,
            memory: 0.2 * focus_factor + 0.1,
            time: 0.4 * focus_factor + 0.2,
            energy: 0.3 * focus_factor + 0.1,
        };

        self.allocated.clone()
    }

    pub fn apply_change(&mut self, change: &ResourceChange) {
        match change.change_type.as_str() {
            "increase" => {
                self.available.computation += change.amount * 0.1;
            }
            "decrease" => {
                self.available.computation = (self.available.computation - change.amount * 0.1).max(0.1);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceChange {
    pub change_type: String,
    pub resource_type: String,
    pub amount: f64,
}

/// Global state manager
pub struct GlobalStateManager {
    pub current_state: GlobalState,
    pub state_history: Vec<GlobalState>,
    pub interrupt_queue: Vec<Interrupt>,
    pub mode: ProcessingMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalState {
    pub timestamp: DateTime<Utc>,
    pub perceptual_saliency: Vec<PerceptualSaliency>,
    pub active_memory_items: Vec<String>,
    pub current_goals: Vec<String>,
    pub emotional_state: EmotionalState,
    pub energy_level: f64,
    pub arousal_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptualSaliency {
    pub item_id: String,
    pub intensity: f64,
    pub novelty: f64,
    pub emotional: f64,
    pub reward: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub valence: f64,
    pub arousal: f64,
    pub dominance: f64,
    pub primary_emotion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingMode {
    Normal,
    Interrupt,
    Sleep,
    Dreaming,
}

impl GlobalStateManager {
    pub fn new() -> Self {
        GlobalStateManager {
            current_state: GlobalState {
                timestamp: Utc::now(),
                perceptual_saliency: Vec::new(),
                active_memory_items: Vec::new(),
                current_goals: Vec::new(),
                emotional_state: EmotionalState {
                    valence: 0.5,
                    arousal: 0.5,
                    dominance: 0.5,
                    primary_emotion: "neutral".to_string(),
                },
                energy_level: 1.0,
                arousal_level: 0.5,
            },
            state_history: Vec::new(),
            interrupt_queue: Vec::new(),
            mode: ProcessingMode::Normal,
        }
    }

    pub fn update_from_perception(&mut self, input: &PerceptionInput) {
        self.current_state.perceptual_saliency = input.saliency_items.clone();
        self.current_state.timestamp = Utc::now();
    }

    pub fn record_decision(&mut self, decision: &ExecutiveDecision) {
        self.state_history.push(self.current_state.clone());
        if self.state_history.len() > 100 {
            self.state_history.remove(0);
        }
    }

    pub fn enter_interrupt_mode(&mut self, interrupt: &Interrupt) {
        self.mode = ProcessingMode::Interrupt;
        println!("[StateManager] Entered interrupt mode: {:?}", interrupt.interrupt_type);
    }

    pub fn queue_interrupt(&mut self, interrupt: &Interrupt) {
        self.interrupt_queue.push(interrupt.clone());
    }
}

/// Integration bus - message passing between subsystems
pub struct IntegrationBus {
    pub message_queue: VecDeque<BusMessage>,
    pub subscriptions: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusMessage {
    pub from: String,
    pub to: String,
    pub message_type: String,
    pub payload: String,
    pub timestamp: DateTime<Utc>,
}

impl IntegrationBus {
    pub fn new() -> Self {
        IntegrationBus {
            message_queue: VecDeque::new(),
            subscriptions: HashMap::new(),
        }
    }

    pub fn send(&mut self, from: &str, to: &str, msg_type: &str, payload: &str) {
        self.message_queue.push_back(BusMessage {
            from: from.to_string(),
            to: to.to_string(),
            message_type: msg_type.to_string(),
            payload: payload.to_string(),
            timestamp: Utc::now(),
        });
    }
}

// Supporting types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptionInput {
    pub saliency_items: Vec<PerceptualSaliency>,
    pub raw_sensory: HashMap<String, String>,
}

impl Default for PerceptionInput {
    fn default() -> Self {
        PerceptionInput {
            saliency_items: Vec::new(),
            raw_sensory: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingContext {
    pub task_type: String,
    pub urgency: f64,
    pub social_context: Option<String>,
}

impl Default for ProcessingContext {
    fn default() -> Self {
        ProcessingContext {
            task_type: "general".to_string(),
            urgency: 0.5,
            social_context: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemMessage {
    PerceptionUpdate(PerceptionInput),
    NewGoal(NewGoalMessage),
    ResourceChange(ResourceChange),
    ExternalInterrupt(Interrupt),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGoalMessage {
    pub goal_text: String,
    pub priority: f64,
    pub goal_type: GoalCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interrupt {
    pub interrupt_type: String,
    pub source: String,
    pub criticality: f64,
    pub focus: String,
    pub response_type: String,
    pub parameters: HashMap<String, String>,
    pub expected_effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutiveDecision {
    pub selected_behavior: Behavior,
    pub focus_of_attention: Vec<String>,
    pub resource_allocation: ResourceAllocation,
    pub goal_updates: Vec<GoalUpdate>,
    pub metacognitive_flags: Vec<MetacognitiveFlag>,
    pub confidence: f64,
}

impl Default for ExecutiveFunction {
    fn default() -> Self {
        Self::new()
    }
}

fn rand_simple() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos as f64 / u32::MAX as f64
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

trait ChooseRandom {
    fn choose<R: Rng>(&self, rng: &mut R) -> Option<&Self::Item>;
}

impl<T> ChooseRandom for [T] {
    type Item = T;

    fn choose<R: Rng>(&self, rng: &mut R) -> Option<&Self::Item> {
        if self.is_empty() {
            None
        } else {
            Some(&self[rng.gen_range(0..self.len())])
        }
    }
}

use rand::Rng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executive_creation() {
        let ef = ExecutiveFunction::new();
        assert_eq!(ef.goal_manager.active_goals.len(), 0);
    }

    #[test]
    fn test_goal_management() {
        let mut ef = ExecutiveFunction::new();
        ef.receive_goal("Test goal", 0.8, GoalCategory::Task);
        assert_eq!(ef.goal_manager.active_goals.len(), 1);
    }

    #[test]
    fn test_processing_cycle() {
        let mut ef = ExecutiveFunction::new();
        ef.receive_goal("Important task", 0.9, GoalCategory::Task);
        let input = PerceptionInput::default();
        let context = ProcessingContext::default();
        let decision = ef.process_cycle(&input, &context);
        assert!(decision.confidence >= 0.0);
    }
}
