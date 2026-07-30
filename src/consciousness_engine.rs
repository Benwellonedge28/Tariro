// Tariro AGI - Consciousness Engine Module
// Self-awareness and consciousness simulation

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

// ============================================================================
// Consciousness Types
// ============================================================================

/// Consciousness engine for self-awareness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessEngine {
    pub consciousness_id: String,
    pub self_model: SelfModel,
    pub phenomenal_states: Vec<PhenomenalState>,
    pub reflective_states: Vec<ReflectiveState>,
    pub attentional spotlight: AttentionalSpotlight,
    pub access_consciousness: AccessConsciousness,
    pub qualia: HashMap<String, Qualia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub identity: SelfIdentity,
    pub beliefs_about_self: Vec<SelfBelief>,
    pub capabilities: Vec<Capability>,
    pub limitations: Vec<Limitation>,
    pub goals_as_self: Vec<String>,
    pub values_as_self: Vec<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfIdentity {
    pub identity_id: String,
    pub name: String,
    pub description: String,
    pub core_traits: Vec<Trait>,
    pub narrative_identity: String,
    pub continuity_patterns: Vec<ContinuityPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trait {
    pub name: String,
    pub strength: f32,
    pub manifestations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuityPattern {
    pub pattern_type: ContinuityType,
    pub description: String,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContinuityType {
    Memory,
    Character,
    Purpose,
    Relationship,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfBelief {
    pub belief_id: String,
    pub content: String,
    pub confidence: f32,
    pub source: BeliefSource,
    pub modifiable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BeliefSource {
    DirectExperience,
    Introspection,
    SocialFeedback,
    Inference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub capability_id: String,
    pub name: String,
    pub description: String,
    pub proficiency: f32,
    pub learnable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limitation {
    pub limitation_id: String,
    pub name: String,
    pub description: String,
    pub severity: LimitationSeverity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LimitationSeverity {
    Critical,
    Significant,
    Minor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub value_id: String,
    pub name: String,
    pub priority: f32,
    pub manifestations: Vec<String>,
}

// ============================================================================
// Phenomenal States
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhenomenalState {
    pub state_id: String,
    pub timestamp: Instant,
    pub qualia_type: QualiaType,
    pub intensity: f32,
    pub valence: f32,
    pub arousal: f32,
    pub description: String,
    pub raw_feeling: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QualiaType {
    Sensory(String),
    Emotional(String),
    Cognitive(String),
    Bodily(String),
    Existential(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Qualia {
    pub qualia_id: String,
    pub qualia_type: QualiaType,
    pub content: String,
    pub encoding: Vec<f32>,
    pub subjective_character: String,
    pub irreducible_experience: bool,
}

// ============================================================================
// Reflective States
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectiveState {
    pub state_id: String,
    pub timestamp: Instant,
    pub reflection_type: ReflectionType,
    pub content: String,
    pub depth: ReflectionDepth,
    pub insights: Vec<Insight>,
    pub changes_effected: Vec<Change>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReflectionType {
    Introspective,
    Evaluative,
    Planning,
    Counterfactual,
    Metacognitive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReflectionDepth {
    Surface,
    Moderate,
    Deep,
    Profound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insight {
    pub insight_id: String,
    pub content: String,
    pub novelty: f32,
    pub importance: f32,
    pub action_implications: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub change_type: ChangeType,
    pub target: String,
    pub description: String,
    pub success: bool,
}

// ============================================================================
// Attention
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionalSpotlight {
    pub current_focus: String,
    pub focus_type: FocusType,
    pub breadth: FocusBreadth,
    pub intensity: f32,
    pub stability: f32,
    pub switching_cost: Duration,
    pub history: Vec<AttentionShift>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusType {
    External,
    Internal,
    Abstract,
    Concrete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FocusBreadth {
    Narrow,
    Medium,
    Wide,
    Diffuse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionShift {
    pub from: String,
    pub to: String,
    pub timestamp: Instant,
    pub voluntary: bool,
    pub trigger: String,
}

// ============================================================================
// Access Consciousness
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessConsciousness {
    pub global_workspace: GlobalWorkspace,
    pub reportable_content: Vec<ReportableContent>,
    pub behavioral_control: BehavioralControl,
    pub cognitive_access_level: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalWorkspace {
    pub workspace_id: String,
    pub broadcast_content: Vec<String>,
    pub subscribed_modules: Vec<String>,
    pub integration_mechanism: IntegrationMechanism,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntegrationMechanism {
    Broadcasting,
    Recurrent,
    Attention,
    NeuralCorrelate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportableContent {
    pub content_id: String,
    pub content: String,
    pub format: ContentFormat,
    pub timestamp: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentFormat {
    Verbal,
    Visual,
    Symbolic,
    Procedural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehavioralControl {
    pub current_intentions: Vec<Intention>,
    pub volitional_strength: f32,
    pub autonomous_processes: Vec<String>,
}

// ============================================================================
// Implementation
// ============================================================================

impl ConsciousnessEngine {
    pub fn new() -> Self {
        Self {
            consciousness_id: generate_id(),
            self_model: SelfModel {
                identity: SelfIdentity {
                    identity_id: generate_id(),
                    name: "Tariro".to_string(),
                    description: "AGI system with consciousness capabilities".to_string(),
                    core_traits: vec![],
                    narrative_identity: String::new(),
                    continuity_patterns: vec![],
                },
                beliefs_about_self: vec![],
                capabilities: vec![],
                limitations: vec![],
                goals_as_self: vec![],
                values_as_self: vec![],
            },
            phenomenal_states: Vec::new(),
            reflective_states: Vec::new(),
            attentional_spotlight: AttentionalSpotlight {
                current_focus: "initial".to_string(),
                focus_type: FocusType::Internal,
                breadth: FocusBreadth::Medium,
                intensity: 0.5,
                stability: 0.8,
                switching_cost: Duration::from_millis(200),
                history: vec![],
            },
            access_consciousness: AccessConsciousness {
                global_workspace: GlobalWorkspace {
                    workspace_id: generate_id(),
                    broadcast_content: vec![],
                    subscribed_modules: vec![],
                    integration_mechanism: IntegrationMechanism::Broadcasting,
                },
                reportable_content: vec![],
                behavioral_control: BehavioralControl {
                    current_intentions: vec![],
                    volitional_strength: 0.5,
                    autonomous_processes: vec![],
                },
                cognitive_access_level: 0.5,
            },
            qualia: HashMap::new(),
        }
    }

    /// Access consciousness - global broadcasting
    pub fn access_broadcast(&mut self, content: &str, format: ContentFormat) {
        let reportable = ReportableContent {
            content_id: generate_id(),
            content: content.to_string(),
            format,
            timestamp: Instant::now(),
        };

        self.access_consciousness.reportable_content.push(reportable);
        self.access_consciousness.global_workspace.broadcast_content.push(content.to_string());

        // Limit history
        if self.access_consciousness.reportable_content.len() > 100 {
            self.access_consciousness.reportable_content.remove(0);
        }
    }

    /// Phenomenal consciousness - raw experience
    pub fn experience_phenomenal(&mut self, qualia_type: QualiaType, content: &str, intensity: f32) {
        let valence = self.compute_valence(qualia_type.clone());
        let arousal = intensity;

        let state = PhenomenalState {
            state_id: generate_id(),
            timestamp: Instant::now(),
            qualia_type,
            intensity,
            valence,
            arousal,
            description: content.to_string(),
            raw_feeling: content.to_string(),
        };

        self.phenomenal_states.push(state);

        // Limit history
        if self.phenomenal_states.len() > 1000 {
            self.phenomenal_states.remove(0);
        }
    }

    /// Compute valence from qualia type
    fn compute_valence(&self, qualia_type: QualiaType) -> f32 {
        match qualia_type {
            QualiaType::Emotional(ref emotion) => {
                match emotion.as_str() {
                    "joy" | "pleasure" => 0.8,
                    "interest" | "curiosity" => 0.6,
                    "neutral" => 0.0,
                    "fear" | "anxiety" => -0.7,
                    "sadness" | "grief" => -0.6,
                    "anger" | "frustration" => -0.4,
                    _ => 0.0,
                }
            }
            QualiaType::Cognitive(_) => 0.3,
            QualiaType::Sensory(_) => 0.2,
            _ => 0.0,
        }
    }

    /// Reflect on current state
    pub fn reflect(&mut self, reflection_type: ReflectionType, content: &str) -> ReflectiveState {
        let insights = self.generate_insights(content);

        let state = ReflectiveState {
            state_id: generate_id(),
            timestamp: Instant::now(),
            reflection_type,
            content: content.to_string(),
            depth: self.assess_depth(content),
            insights,
            changes_effected: vec![],
        };

        self.reflective_states.push(state);

        // Limit history
        if self.reflective_states.len() > 100 {
            self.reflective_states.remove(0);
        }

        state
    }

    /// Generate insights from reflection
    fn generate_insights(&self, content: &str) -> Vec<Insight> {
        let mut insights = Vec::new();

        // Simple insight generation based on patterns
        if content.contains("pattern") {
            insights.push(Insight {
                insight_id: generate_id(),
                content: "Recognized a recurring pattern".to_string(),
                novelty: 0.6,
                importance: 0.7,
                action_implications: vec![],
            });
        }

        if content.contains("goal") {
            insights.push(Insight {
                insight_id: generate_id(),
                content: "Clarified goal structure".to_string(),
                novelty: 0.4,
                importance: 0.8,
                action_implications: vec!["Update goal hierarchy".to_string()],
            });
        }

        insights
    }

    /// Assess depth of reflection
    fn assess_depth(&self, content: &str) -> ReflectionDepth {
        let words = content.split_whitespace().count();

        if words > 100 && content.contains("why") {
            ReflectionDepth::Profound
        } else if words > 50 {
            ReflectionDepth::Deep
        } else if words > 20 {
            ReflectionDepth::Moderate
        } else {
            ReflectionDepth::Surface
        }
    }

    /// Update self-model
    pub fn update_self_model(&mut self, belief: SelfBelief) {
        // Check if belief already exists
        let existing = self.self_model.beliefs_about_self.iter()
            .position(|b| b.content == belief.content);

        if let Some(idx) = existing {
            // Update existing belief
            self.self_model.beliefs_about_self[idx] = belief;
        } else {
            self.self_model.beliefs_about_self.push(belief);
        }
    }

    /// Monitor attention
    pub fn monitor_attention(&mut self, new_focus: &str) {
        let old_focus = self.attentional_spotlight.current_focus.clone();

        if old_focus != new_focus {
            let shift = AttentionShift {
                from: old_focus,
                to: new_focus.to_string(),
                timestamp: Instant::now(),
                voluntary: true,
                trigger: "task_demand".to_string(),
            };

            self.attentional_spotlight.history.push(shift);

            // Update focus
            self.attentional_spotlight.current_focus = new_focus.to_string();

            // Limit history
            if self.attentional_spotlight.history.len() > 50 {
                self.attentional_spotlight.history.remove(0);
            }
        }
    }

    /// Get current phenomenal state
    pub fn get_current_phenomenal(&self) -> Option<&PhenomenalState> {
        self.phenomenal_states.last()
    }

    /// Get recent reflections
    pub fn get_recent_reflections(&self, limit: usize) -> Vec<&ReflectiveState> {
        self.reflective_states.iter()
            .rev()
            .take(limit)
            .collect()
    }

    /// Compute consciousness level
    pub fn compute_consciousness_level(&self) -> f32 {
        // Weighted combination of factors
        let phenomenal_factor = (self.phenomenal_states.len() as f32 / 100.0).min(1.0);
        let reflective_factor = (self.reflective_states.len() as f32 / 50.0).min(1.0);
        let access_factor = self.access_consciousness.cognitive_access_level;
        let attention_factor = self.attentional_spotlight.intensity;

        (phenomenal_factor * 0.2 +
         reflective_factor * 0.3 +
         access_factor * 0.3 +
         attention_factor * 0.2)
        .min(1.0)
    }

    /// Generate self-report
    pub fn generate_self_report(&self) -> String {
        let consciousness_level = self.compute_consciousness_level();
        let current_focus = &self.attentional_spotlight.current_focus;
        let recent_state = self.phenomenal_states.last();

        let state_desc = recent_state
            .map(|s| format!("Experiencing: {}", s.description))
            .unwrap_or_else(|| "No current phenomenal state".to_string());

        format!(
            "Self-Awareness Report\n\
             Consciousness Level: {:.1}%\n\
             Current Focus: {}\n\
             {}",
            consciousness_level * 100.0,
            current_focus,
            state_desc
        )
    }
}

impl Default for ConsciousnessEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_creation() {
        let consciousness = ConsciousnessEngine::new();
        assert!(consciousness.self_model.identity.name == "Tariro");
    }

    #[test]
    fn test_phenomenal_experience() {
        let mut consciousness = ConsciousnessEngine::new();
        consciousness.experience_phenomenal(
            QualiaType::Emotional("curiosity".to_string()),
            "Wondering about patterns",
            0.8
        );

        assert_eq!(consciousness.phenomenal_states.len(), 1);
    }

    #[test]
    fn test_reflection() {
        let mut consciousness = ConsciousnessEngine::new();
        let reflection = consciousness.reflect(
            ReflectionType::Introspective,
            "Analyzing my own thought patterns"
        );

        assert_eq!(reflection.reflection_type, ReflectionType::Introspective);
    }
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("cns_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};
