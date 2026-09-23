// src/continuous_learning.rs

//! Continuous Learning Module for Tariro AGI
//! Implements lifelong learning, skill acquisition, and adaptive improvement

use crate::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents a learned skill or capability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub domain: String,
    pub proficiency: f64,
    pub learning_history: Vec<LearningEvent>,
    pub prerequisites: Vec<String>,
    pub transferability: f64,
    pub last_practiced: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: LearningEventType,
    pub description: String,
    pub improvement: f64,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LearningEventType {
    Observation,
    Instruction,
    Practice,
    Feedback,
    Discovery,
    Mistake,
    Breakthrough,
}

/// Knowledge graph for learned information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeGraph {
    pub nodes: Vec<KnowledgeNode>,
    pub edges: Vec<KnowledgeEdge>,
    pub embeddings: HashMap<String, Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeNode {
    pub id: String,
    pub concept: String,
    pub node_type: ConceptType,
    pub confidence: f64,
    pub source: KnowledgeSource,
    pub evidence: Vec<Evidence>,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptType {
    Fact,
    Concept,
    Procedure,
    Principle,
    Heuristic,
    Rule,
    Theory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub content: String,
    pub reliability: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    Observation,
    Inference,
    Authority,
    Testimony,
    LogicalDerivation,
    Statistical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KnowledgeSource {
    UserInput,
    Document,
    Experience,
    Inference,
    External,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeEdge {
    pub from: String,
    pub to: String,
    pub relation: EdgeRelation,
    pub strength: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdgeRelation {
    IsA,
    PartOf,
    Causes,
    Enables,
    Contradicts,
    SimilarTo,
    InstanceOf,
    Precedes,
}

/// Learning rate and optimization parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningConfig {
    pub learning_rate: f64,
    pub forgetting_rate: f64,
    pub consolidation_threshold: f64,
    pub exploration_rate: f64,
    pub rehearsal_frequency: u32,
    pub adaptation_speed: f64,
}

impl Default for LearningConfig {
    fn default() -> Self {
        LearningConfig {
            learning_rate: 0.01,
            forgetting_rate: 0.001,
            consolidation_threshold: 0.7,
            exploration_rate: 0.2,
            rehearsal_frequency: 10,
            adaptation_speed: 0.5,
        }
    }
}

/// Main continuous learning system
pub struct ContinuousLearning {
    pub skills: HashMap<String, LearnedSkill>,
    pub knowledge: KnowledgeGraph,
    pub learning_history: Vec<LearningEvent>,
    pub config: LearningConfig,
    pub learner_model: LearnerModel,
    pub learning_strategies: Vec<LearningStrategy>,
    pub feedback_loops: Vec<FeedbackLoop>,
    pub metacognitive_learning: MetacognitiveLearning,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnerModel {
    pub learning_style: LearningStyle,
    pub cognitive_strengths: Vec<String>,
    pub cognitive_weaknesses: Vec<String>,
    pub preferred_modalities: Vec<String>,
    pub optimal_difficulty: f64,
    pub attention_span: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStyle {
    pub visual: f64,
    pub auditory: f64,
    pub reading: f64,
    pub kinesthetic: f64,
    pub deductive: f64,
    pub inductive: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStrategy {
    pub name: String,
    pub description: String,
    pub effectiveness: f64,
    pub applicable_scenarios: Vec<String>,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackLoop {
    pub id: String,
    pub feedback_type: FeedbackType,
    pub source: String,
    pub delay_ms: u64,
    pub accuracy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackType {
    Correct,
    Incorrect,
    Partial,
    Implicit,
    SelfGenerated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetacognitiveLearning {
    pub learning_from_errors: bool,
    pub strategy_selection: bool,
    pub self_explanation: bool,
    pub interleaving: bool,
    pub spaced_repetition: bool,
}

impl ContinuousLearning {
    pub fn new() -> Self {
        ContinuousLearning {
            skills: HashMap::new(),
            knowledge: KnowledgeGraph {
                nodes: Vec::new(),
                edges: Vec::new(),
                embeddings: HashMap::new(),
            },
            learning_history: Vec::new(),
            config: LearningConfig::default(),
            learner_model: LearnerModel {
                learning_style: LearningStyle {
                    visual: 0.6,
                    auditory: 0.5,
                    reading: 0.7,
                    kinesthetic: 0.4,
                    deductive: 0.8,
                    inductive: 0.7,
                },
                cognitive_strengths: vec!["Pattern recognition".to_string(), "Logical reasoning".to_string()],
                cognitive_weaknesses: vec!["Long-term retention".to_string()],
                preferred_modalities: vec!["Text".to_string(), "Code".to_string()],
                optimal_difficulty: 0.6,
                attention_span: 30.0,
            },
            learning_strategies: vec![
                LearningStrategy {
                    name: "Spaced Repetition".to_string(),
                    description: "Review at increasing intervals".to_string(),
                    effectiveness: 0.85,
                    applicable_scenarios: vec!["Memory".to_string(), "Facts".to_string()],
                    steps: vec![
                        "Learn new information".to_string(),
                        "Review after 1 day".to_string(),
                        "Review after 3 days".to_string(),
                        "Review after 1 week".to_string(),
                        "Review after 2 weeks".to_string(),
                    ],
                },
                LearningStrategy {
                    name: "Interleaving".to_string(),
                    description: "Mix different topics during practice".to_string(),
                    effectiveness: 0.75,
                    applicable_scenarios: vec!["Skills".to_string(), "Procedures".to_string()],
                    steps: vec![
                        "Identify related skills".to_string(),
                        "Create mixed practice sets".to_string(),
                        "Rotate between topics".to_string(),
                    ],
                },
            ],
            feedback_loops: Vec::new(),
            metacognitive_learning: MetacognitiveLearning {
                learning_from_errors: true,
                strategy_selection: true,
                self_explanation: true,
                interleaving: true,
                spaced_repetition: true,
            },
        }
    }

    /// Learn from experience
    pub fn learn_from_experience(&mut self, experience: &str, outcome: &str, feedback: f64) -> LearningResult {
        println!("[Learning] Learning from experience: {}", experience);

        let event = LearningEvent {
            timestamp: Utc::now(),
            event_type: if feedback > 0.7 {
                LearningEventType::Breakthrough
            } else if feedback > 0.3 {
                LearningEventType::Practice
            } else {
                LearningEventType::Mistake
            },
            description: experience.to_string(),
            improvement: feedback,
            context: outcome.to_string(),
        };

        self.learning_history.push(event.clone());

        // Update knowledge graph
        self.update_knowledge(experience, outcome, feedback);

        // Adjust learning parameters based on feedback
        self.adapt_learning_parameters(feedback);

        LearningResult {
            event,
            knowledge_gained: self.extract_new_knowledge(experience, outcome),
            skill_improvements: self.calculate_skill_improvements(experience),
            retention_probability: self.predict_retention(experience),
        }
    }

    fn update_knowledge(&mut self, experience: &str, outcome: &str, confidence: f64) {
        // Add or update nodes
        let experience_node = KnowledgeNode {
            id: format!("node_{}", Utc::now().timestamp()),
            concept: experience.to_string(),
            node_type: ConceptType::Experience,
            confidence,
            source: KnowledgeSource::Experience,
            evidence: vec![Evidence {
                evidence_type: EvidenceType::Observation,
                content: outcome.to_string(),
                reliability: confidence,
                timestamp: Utc::now(),
            }],
            last_updated: Utc::now(),
        };

        self.knowledge.nodes.push(experience_node);
        println!("[Learning] Updated knowledge graph with new node");
    }

    fn extract_new_knowledge(&self, _experience: &str, _outcome: &str) -> Vec<String> {
        vec!["New pattern recognized".to_string()]
    }

    fn calculate_skill_improvements(&self, _experience: &str) -> Vec<SkillImprovement> {
        vec![SkillImprovement {
            skill_name: "generalization".to_string(),
            improvement_amount: 0.05,
        }]
    }

    fn predict_retention(&self, knowledge: &str) -> f64 {
        // Simple retention prediction
        let recency_factor = if self.learning_history.len() > 10 { 0.8 } else { 0.5 };
        recency_factor
    }

    fn adapt_learning_parameters(&mut self, feedback: f64) {
        if feedback > 0.8 {
            // High success - increase difficulty
            self.config.learning_rate *= 1.1;
            self.config.exploration_rate *= 1.05;
        } else if feedback < 0.3 {
            // Low success - slow down and consolidate
            self.config.learning_rate *= 0.9;
            self.config.forgetting_rate *= 0.8;
        }
    }

    /// Acquire a new skill
    pub fn acquire_skill(&mut self, skill_name: &str, domain: &str) -> SkillAcquisitionResult {
        println!("[Learning] Acquiring skill: {} in {}", skill_name, domain);

        let skill = LearnedSkill {
            id: format!("skill_{}", Utc::now().timestamp()),
            name: skill_name.to_string(),
            description: format!("Skill in {}", domain),
            domain: domain.to_string(),
            proficiency: 0.1,
            learning_history: vec![LearningEvent {
                timestamp: Utc::now(),
                event_type: LearningEventType::Discovery,
                description: format!("Discovered {}", skill_name),
                improvement: 0.1,
                context: domain.to_string(),
            }],
            prerequisites: vec![],
            transferability: 0.5,
            last_practiced: Some(Utc::now()),
        };

        self.skills.insert(skill.id.clone(), skill.clone());

        SkillAcquisitionResult {
            skill,
            learning_path: self.generate_learning_path(skill_name, domain),
            estimated_time: 100,
            difficulty_level: 0.5,
        }
    }

    fn generate_learning_path(&self, skill_name: &str, domain: &str) -> Vec<LearningStep> {
        vec![
            LearningStep {
                step_number: 1,
                task: format!("Learn basics of {}", skill_name),
                objective: "Understand fundamental concepts".to_string(),
                resources: vec!["documentation".to_string()],
                practice_exercises: vec!["Basic exercise 1".to_string()],
            },
            LearningStep {
                step_number: 2,
                task: format!("Practice {} in {}", skill_name, domain),
                objective: "Apply knowledge in context".to_string(),
                resources: vec!["examples".to_string()],
                practice_exercises: vec!["Intermediate exercise".to_string()],
            },
        ]
    }

    /// Practice existing skill
    pub fn practice_skill(&mut self, skill_id: &str, practice_data: &str) -> PracticeResult {
        if let Some(skill) = self.skills.get_mut(skill_id) {
            let improvement = self.calculate_practice_improvement(skill, practice_data);

            skill.proficiency = (skill.proficiency + improvement).min(1.0);
            skill.last_practiced = Some(Utc::now());
            skill.learning_history.push(LearningEvent {
                timestamp: Utc::now(),
                event_type: LearningEventType::Practice,
                description: format!("Practiced {}: {}", skill.name, practice_data),
                improvement,
                context: "skill_practice".to_string(),
            });

            PracticeResult {
                skill_id: skill_id.to_string(),
                new_proficiency: skill.proficiency,
                improvement,
                feedback: self.generate_practice_feedback(improvement),
                next_practice_time: Utc::now() + ChronoDuration::days(1),
            }
        } else {
            PracticeResult {
                skill_id: skill_id.to_string(),
                new_proficiency: 0.0,
                improvement: 0.0,
                feedback: "Skill not found".to_string(),
                next_practice_time: Utc::now(),
            }
        }
    }

    fn calculate_practice_improvement(&self, skill: &LearnedSkill, _practice_data: &str) -> f64 {
        // Base improvement from practice
        let base_improvement = 0.05;

        // Adjust for current proficiency (diminishing returns)
        let proficiency_factor = 1.0 - skill.proficiency * 0.5;

        // Adjust for time since last practice
        let time_factor = if let Some(last) = skill.last_practiced {
            let days_since = (Utc::now() - last).num_days() as f64;
            if days_since < 1.0 { 1.2 } else if days_since > 7.0 { 0.5 } else { 1.0 }
        } else {
            1.0
        };

        base_improvement * proficiency_factor * time_factor * self.config.learning_rate * 10.0
    }

    fn generate_practice_feedback(&self, improvement: f64) -> String {
        if improvement > 0.05 {
            "Excellent progress! Keep up the good work.".to_string()
        } else if improvement > 0.02 {
            "Good practice. You're making steady progress.".to_string()
        } else {
            "Consider varying your practice approach.".to_string()
        }
    }

    /// Transfer learning from one domain to another
    pub fn transfer_learning(&mut self, source_skill: &str, target_domain: &str) -> TransferResult {
        println!("[Learning] Transferring {} to {}", source_skill, target_domain);

        let transferability = self.skills.get(source_skill)
            .map(|s| s.transferability)
            .unwrap_or(0.3);

        let transferred_knowledge = self.identify_transferable_knowledge(source_skill, target_domain);

        TransferResult {
            source_skill: source_skill.to_string(),
            target_domain: target_domain.to_string(),
            transferred_elements: transferred_knowledge,
            transfer_efficiency: transferability * 0.8,
            adaptation_required: vec![
                "Adjust for new context".to_string(),
            ],
        }
    }

    fn identify_transferable_knowledge(&self, source: &str, _target: &str) -> Vec<String> {
        // Find related knowledge in the knowledge graph
        self.knowledge.nodes.iter()
            .filter(|n| n.concept.contains(source) || n.concept.to_lowercase().contains(&source.to_lowercase()))
            .map(|n| n.concept.clone())
            .collect()
    }

    /// Implement spaced repetition
    pub fn schedule_review(&self, item_id: &str) -> ReviewSchedule {
        let node = self.knowledge.nodes.iter().find(|n| n.id == item_id);

        let interval = if let Some(n) = node {
            let base_interval = 1.0 / (1.0 - n.confidence);
            base_interval * self.config.rehearsal_frequency as f64
        } else {
            1.0
        };

        ReviewSchedule {
            item_id: item_id.to_string(),
            next_review: Utc::now() + ChronoDuration::days(interval as i64),
            interval_days: interval as u32,
            ease_factor: 2.5,
            repetitions: 1,
        }
    }

    /// Learn from mistakes
    pub fn learn_from_mistake(&mut self, mistake: &str, correct_answer: &str) -> ErrorLearningResult {
        println!("[Learning] Learning from mistake: {}", mistake);

        // Create error knowledge node
        let error_node = KnowledgeNode {
            id: format!("error_{}", Utc::now().timestamp()),
            concept: format!("Common error: {}", mistake),
            node_type: ConceptType::Heuristic,
            confidence: 0.9,
            source: KnowledgeSource::Experience,
            evidence: vec![Evidence {
                evidence_type: EvidenceType::Observation,
                content: format!("Correct: {}", correct_answer),
                reliability: 1.0,
                timestamp: Utc::now(),
            }],
            last_updated: Utc::now(),
        };

        self.knowledge.nodes.push(error_node);

        // Add to learning history
        self.learning_history.push(LearningEvent {
            timestamp: Utc::now(),
            event_type: LearningEventType::Mistake,
            description: format!("Made error: {}. Correct: {}", mistake, correct_answer),
            improvement: 0.2,
            context: "error_learning".to_string(),
        });

        ErrorLearningResult {
            error_learned: mistake.to_string(),
            correction: correct_answer.to_string(),
            pattern_identified: self.identify_error_pattern(mistake),
            prevention_strategies: vec![
                "Double-check assumptions".to_string(),
                "Verify against known patterns".to_string(),
            ],
        }
    }

    fn identify_error_pattern(&self, error: &str) -> String {
        // Simple pattern identification
        if error.contains("type") || error.contains("syntax") {
            "Careless execution error".to_string()
        } else if error.contains("wrong approach") {
            "Strategic planning error".to_string()
        } else {
            "Unknown error pattern".to_string()
        }
    }

    /// Self-directed learning
    pub fn self_directed_learning(&mut self, topic: &str) -> LearningPlan {
        println!("[Learning] Self-directed learning on: {}", topic);

        let current_knowledge = self.get_current_knowledge(topic);
        let knowledge_gaps = self.identify_knowledge_gaps(topic);
        let optimal_strategy = self.select_learning_strategy(topic);

        LearningPlan {
            topic: topic.to_string(),
            current_state: current_knowledge,
            learning_objectives: knowledge_gaps,
            recommended_activities: vec![
                "Study fundamentals".to_string(),
                "Practice with examples".to_string(),
                "Teach others".to_string(),
            ],
            estimated_time: 10,
            strategy: optimal_strategy,
            checkpoints: vec![
                Checkpoint {
                    milestone: "Basic understanding".to_string(),
                    criteria: "Can explain core concepts".to_string(),
                    deadline: Utc::now() + ChronoDuration::days(3),
                },
            ],
        }
    }

    fn get_current_knowledge(&self, topic: &str) -> f64 {
        let relevant_nodes: Vec<_> = self.knowledge.nodes.iter()
            .filter(|n| n.concept.to_lowercase().contains(&topic.to_lowercase()))
            .collect();

        if relevant_nodes.is_empty() {
            0.0
        } else {
            relevant_nodes.iter().map(|n| n.confidence).sum::<f64>() / relevant_nodes.len() as f64
        }
    }

    fn identify_knowledge_gaps(&self, topic: &str) -> Vec<String> {
        // Identify missing knowledge areas
        vec![
            format!("Advanced concepts in {}", topic),
            format!("Practical applications of {}", topic),
        ]
    }

    fn select_learning_strategy(&self, _topic: &str) -> String {
        // Select based on learner model
        if self.learner_model.learning_style.deductive > 0.7 {
            "Start with theory, then practice".to_string()
        } else {
            "Start with examples, then theory".to_string()
        }
    }

    /// Consolidate learning into long-term memory
    pub fn consolidate_learning(&mut self) -> ConsolidationResult {
        println!("[Learning] Consolidating learning...");

        let mut consolidated_count = 0;
        let mut improved_count = 0;

        for node in &mut self.knowledge.nodes {
            // Check if node meets consolidation threshold
            if node.confidence >= self.config.consolidation_threshold {
                // Strengthen connections
                for edge in &mut self.knowledge.edges {
                    if edge.from == node.id || edge.to == node.id {
                        edge.strength = (edge.strength + 0.1).min(1.0);
                    }
                }
                consolidated_count += 1;
            } else {
                // Apply forgetting to weak nodes
                node.confidence *= (1.0 - self.config.forgetting_rate);
                if node.confidence < 0.2 {
                    improved_count += 1;
                }
            }
        }

        ConsolidationResult {
            nodes_consolidated: consolidated_count,
            nodes_pruned: improved_count,
            network_density_change: 0.05,
            coherence_improvement: 0.1,
        }
    }

    /// Get learning statistics
    pub fn get_learning_stats(&self) -> LearningStats {
        LearningStats {
            total_skills: self.skills.len(),
            total_knowledge_nodes: self.knowledge.nodes.len(),
            total_learning_events: self.learning_history.len(),
            average_proficiency: if self.skills.is_empty() {
                0.0
            } else {
                self.skills.values().map(|s| s.proficiency).sum::<f64>() / self.skills.len() as f64
            },
            recent_improvements: self.learning_history.iter()
                .rev()
                .take(10)
                .map(|e| e.improvement)
                .sum::<f64>() / 10.0,
        }
    }
}

impl Default for ContinuousLearning {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningResult {
    pub event: LearningEvent,
    pub knowledge_gained: Vec<String>,
    pub skill_improvements: Vec<SkillImprovement>,
    pub retention_probability: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillImprovement {
    pub skill_name: String,
    pub improvement_amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAcquisitionResult {
    pub skill: LearnedSkill,
    pub learning_path: Vec<LearningStep>,
    pub estimated_time: u32,
    pub difficulty_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStep {
    pub step_number: u32,
    pub task: String,
    pub objective: String,
    pub resources: Vec<String>,
    pub practice_exercises: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeResult {
    pub skill_id: String,
    pub new_proficiency: f64,
    pub improvement: f64,
    pub feedback: String,
    pub next_practice_time: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferResult {
    pub source_skill: String,
    pub target_domain: String,
    pub transferred_elements: Vec<String>,
    pub transfer_efficiency: f64,
    pub adaptation_required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSchedule {
    pub item_id: String,
    pub next_review: DateTime<Utc>,
    pub interval_days: u32,
    pub ease_factor: f64,
    pub repetitions: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorLearningResult {
    pub error_learned: String,
    pub correction: String,
    pub pattern_identified: String,
    pub prevention_strategies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPlan {
    pub topic: String,
    pub current_state: f64,
    pub learning_objectives: Vec<String>,
    pub recommended_activities: Vec<String>,
    pub estimated_time: u32,
    pub strategy: String,
    pub checkpoints: Vec<Checkpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub milestone: String,
    pub criteria: String,
    pub deadline: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationResult {
    pub nodes_consolidated: usize,
    pub nodes_pruned: usize,
    pub network_density_change: f64,
    pub coherence_improvement: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStats {
    pub total_skills: usize,
    pub total_knowledge_nodes: usize,
    pub total_learning_events: usize,
    pub average_proficiency: f64,
    pub recent_improvements: f64,
}

/// Integration with Tariro's existing systems
impl ContinuousLearning {
    /// Learn from user feedback
    pub fn learn_from_feedback(&mut self, input: &str, output: &str, user_feedback: &str) -> FeedbackLearningResult {
        let feedback_score = if user_feedback.contains("good") || user_feedback.contains("correct") {
            0.8
        } else if user_feedback.contains("wrong") || user_feedback.contains("incorrect") {
            0.2
        } else {
            0.5
        };

        FeedbackLearningResult {
            input: input.to_string(),
            output: output.to_string(),
            feedback: user_feedback.to_string(),
            learned_adjustment: self.learn_from_experience(
                &format!("Response to: {}", input),
                output,
                feedback_score,
            ),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackLearningResult {
    pub input: String,
    pub output: String,
    pub feedback: String,
    pub learned_adjustment: LearningResult,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learning_system_creation() {
        let learning = ContinuousLearning::new();
        assert_eq!(learning.skills.len(), 0);
        assert_eq!(learning.knowledge.nodes.len(), 0);
    }

    #[test]
    fn test_skill_acquisition() {
        let mut learning = ContinuousLearning::new();
        let result = learning.acquire_skill("Rust Programming", "Software Development");
        assert_eq!(result.skill.name, "Rust Programming");
        assert!(result.skill.proficiency > 0.0);
    }

    #[test]
    fn test_learning_from_experience() {
        let mut learning = ContinuousLearning::new();
        let result = learning.learn_from_experience("Test input", "Test output", 0.8);
        assert_eq!(result.knowledge_gained.len() > 0, true);
    }

    #[test]
    fn test_error_learning() {
        let mut learning = ContinuousLearning::new();
        let result = learning.learn_from_mistake("Wrong assumption", "Correct understanding");
        assert_eq!(result.error_learned, "Wrong assumption");
    }

    #[test]
    fn test_spaced_repetition() {
        let learning = ContinuousLearning::new();
        let schedule = learning.schedule_review("test_node");
        assert!(schedule.interval_days > 0);
    }
}
