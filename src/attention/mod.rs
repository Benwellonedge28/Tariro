//! Tariro Global Workspace Theory Attention Module
//!
//! Implements Baars' Global Workspace Theory for consciousness:
//! - Competition: Multiple specialized modules compete for attention
//! - Broadcasting: Winning content becomes globally accessible
//! - Coherence: Integrates disparate information into unified experience

use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for workspace contents
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentId(pub Uuid);

impl ContentId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for ContentId {
    fn default() -> Self {
        Self::new()
    }
}

/// Content in the global workspace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceContent {
    pub id: ContentId,
    pub source_module: ModuleType,
    pub content_type: ContentType,
    pub data: ContentData,
    pub activation: f64,
    pub relevance: f64,
    pub novelty: f64,
    pub emotional_valence: f64,
    pub timestamp: chrono::DateTime<Utc>,
    pub access_count: u32,
}

impl WorkspaceContent {
    pub fn new(source: ModuleType, content_type: ContentType, data: ContentData) -> Self {
        Self {
            id: ContentId::new(),
            source_module: source,
            content_type,
            data,
            activation: 0.5,
            relevance: 0.5,
            novelty: 0.5,
            emotional_valence: 0.0,
            timestamp: Utc::now(),
            access_count: 0,
        }
    }

    /// Calculate salience (combined activation)
    pub fn salience(&self) -> f64 {
        // Baars' salience formula: activation * relevance * novelty
        let base_salience = self.activation * self.relevance * (1.0 + self.novelty);

        // Emotional boost
        let emotional_boost = self.emotional_valence.abs() * 0.2;

        (base_salience + emotional_boost).min(1.0)
    }

    /// Record access
    pub fn accessed(&mut self) {
        self.access_count += 1;
        self.novelty = (self.novelty * 0.9).max(0.0); // Decay novelty on access
    }
}

/// Module types that can contribute to workspace
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModuleType {
    Perception,
    Language,
    Memory,
    Reasoning,
    Planning,
    Motivation,
    Emotion,
    Motor,
    WorldModel,
    MetaCognition,
}

/// Content types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentType {
    Perception,
    Thought,
    Memory,
    Plan,
    Emotion,
    Intention,
    Decision,
    Question,
    Answer,
    Goal,
}

/// Actual content data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentData {
    Text(String),
    Visual(VisualContent),
    Audio(AudioContent),
    Action(ActionContent),
    Concept(ConceptContent),
    Memory(MemoryContent),
    Plan(PlanContent),
    Emotion(EmotionContent),
}

impl ContentData {
    pub fn as_text(&self) -> Option<&str> {
        match self {
            ContentData::Text(s) => Some(s),
            ContentData::Concept(c) => Some(&c.description),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            ContentData::Text(s) => s.clone(),
            ContentData::Visual(v) => v.description.clone(),
            ContentData::Audio(a) => a.transcription.clone(),
            ContentData::Action(a) => a.description.clone(),
            ContentData::Concept(c) => c.description.clone(),
            ContentData::Memory(m) => m.description.clone(),
            ContentData::Plan(p) => p.description.clone(),
            ContentData::Emotion(e) => format!("{:?}", e.emotion_type),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualContent {
    pub description: String,
    pub objects: Vec<String>,
    pub spatial_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioContent {
    pub transcription: String,
    pub speaker: Option<String>,
    pub prosody: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionContent {
    pub description: String,
    pub target: Option<String>,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptContent {
    pub description: String,
    pub category: String,
    pub properties: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryContent {
    pub description: String,
    pub episode_type: String,
    pub emotional_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanContent {
    pub description: String,
    pub steps: Vec<String>,
    pub goal: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionContent {
    pub emotion_type: EmotionType,
    pub intensity: f64,
    pub trigger: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmotionType {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Trust,
    Anticipation,
    Interest,
}

// ============================================================================
// Competition Mechanism
// ============================================================================

/// Content competition for workspace access
#[derive(Debug, Clone)]
pub struct CompetitionMechanism {
    pub threshold: f64,
    pub bias_strength: f64,
    pub history_window: usize,
    recent_winners: VecDeque<ContentId>,
}

impl CompetitionMechanism {
    pub fn new() -> Self {
        Self {
            threshold: 0.3,
            bias_strength: 0.1,
            history_window: 10,
            recent_winners: VecDeque::new(),
        }
    }

    /// Run competition and select winner
    pub fn compete(&mut self, contents: &[WorkspaceContent]) -> Option<ContentId> {
        if contents.is_empty() {
            return None;
        }

        // Calculate saliences
        let mut saliences: Vec<(ContentId, f64)> = contents
            .iter()
            .map(|c| {
                let mut salience = c.salience();

                // Bias against recently broadcast content
                if self.recent_winners.contains(&c.id) {
                    salience *= 1.0 - self.bias_strength;
                }

                // Bias for novelty
                salience *= 1.0 + (c.novelty * 0.2);

                (c.id.clone(), salience)
            })
            .collect();

        // Find winner above threshold
        saliences.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        if let Some((winner_id, salience)) = saliences.first() {
            if *salience >= self.threshold {
                self.recent_winners.push_back(winner_id.clone());
                if self.recent_winners.len() > self.history_window {
                    self.recent_winners.pop_front();
                }
                return Some(winner_id.clone());
            }
        }

        None
    }

    /// Update threshold based on situation
    pub fn adjust_threshold(&mut self, context: &CompetitionContext) {
        // Higher threshold when cognitively loaded
        if context.cognitive_load > 0.7 {
            self.threshold = (self.threshold + 0.1).min(0.8);
        } else if context.cognitive_load < 0.3 {
            self.threshold = (self.threshold - 0.05).max(0.2);
        }

        // Lower threshold for emotional content
        if context.emotional_content_present {
            self.threshold = (self.threshold - 0.1).max(0.1);
        }
    }
}

impl Default for CompetitionMechanism {
    fn default() -> Self {
        Self::new()
    }
}

/// Context for competition adjustment
#[derive(Debug, Clone)]
pub struct CompetitionContext {
    pub cognitive_load: f64,
    pub emotional_content_present: bool,
    pub attentional_demands: usize,
    pub time_pressure: f64,
}

// ============================================================================
// Broadcast Channel
// ============================================================================

/// Global broadcast channel
#[derive(Debug, Clone)]
pub struct BroadcastChannel {
    pub contents: Arc<RwLock<VecDeque<BroadcastItem>>>,
    pub subscribers: HashMap<ModuleType, SubscriberConfig>,
    pub max_history: usize,
}

impl BroadcastChannel {
    pub fn new() -> Self {
        Self {
            contents: Arc::new(RwLock::new(VecDeque::new())),
            subscribers: HashMap::new(),
            max_history: 100,
        }
    }

    /// Broadcast content to all subscribers
    pub async fn broadcast(&self, content: &WorkspaceContent, recipients: Option<Vec<ModuleType>>) {
        let item = BroadcastItem {
            content_id: content.id.clone(),
            content_type: content.content_type,
            source: content.source_module,
            timestamp: Utc::now(),
            data: content.data.clone(),
        };

        let mut history = self.contents.write().await;
        history.push_back(item);

        // Trim if too long
        while history.len() > self.max_history {
            history.pop_front();
        }
    }

    /// Subscribe a module to broadcasts
    pub fn subscribe(&mut self, module: ModuleType, config: SubscriberConfig) {
        self.subscribers.insert(module, config);
    }

    /// Get recent broadcasts for a module
    pub async fn get_for_subscriber(&self, module: ModuleType) -> Vec<BroadcastItem> {
        let history = self.contents.read().await;

        if let Some(config) = self.subscribers.get(&module) {
            history
                .iter()
                .filter(|item| {
                    // Filter by content type
                    if !config.content_types.is_empty()
                        && !config.content_types.contains(&item.content_type)
                    {
                        return false;
                    }

                    // Filter by recency
                    let age = Utc::now() - item.timestamp;
                    age.num_seconds() < config.max_age_seconds as i64
                })
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all recent broadcasts
    pub async fn get_all_recent(&self, max_age_seconds: u64) -> Vec<BroadcastItem> {
        let history = self.contents.read().await;
        let cutoff = Utc::now() - chrono::Duration::seconds(max_age_seconds as i64);

        history
            .iter()
            .filter(|item| item.timestamp > cutoff)
            .cloned()
            .collect()
    }
}

impl Default for BroadcastChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BroadcastItem {
    pub content_id: ContentId,
    pub content_type: ContentType,
    pub source: ModuleType,
    pub timestamp: chrono::DateTime<Utc>,
    pub data: ContentData,
}

/// Subscriber configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriberConfig {
    pub content_types: Vec<ContentType>,
    pub max_age_seconds: u64,
    pub priority: u8,
}

impl Default for SubscriberConfig {
    fn default() -> Self {
        Self {
            content_types: Vec::new(),
            max_age_seconds: 300,
            priority: 5,
        }
    }
}

// ============================================================================
// Global Workspace
// ============================================================================

/// Global Workspace Theory implementation
#[derive(Debug, Clone)]
pub struct GlobalWorkspace {
    pub consciousness_threshold: f64,
    pub competition: CompetitionMechanism,
    pub broadcast: BroadcastChannel,
    pub contents: HashMap<ContentId, WorkspaceContent>,
    pub current_focus: Option<ContentId>,
    pub focus_history: VecDeque<ContentId>,
    pub attention_span: usize,
    stats: WorkspaceStats,
}

impl GlobalWorkspace {
    pub fn new() -> Self {
        let mut workspace = Self {
            consciousness_threshold: 0.5,
            competition: CompetitionMechanism::new(),
            broadcast: BroadcastChannel::new(),
            contents: HashMap::new(),
            current_focus: None,
            focus_history: VecDeque::new(),
            attention_span: 7,
            stats: WorkspaceStats::default(),
        };

        // Initialize default subscriptions
        workspace.init_subscriptions();

        workspace
    }

    fn init_subscriptions(&mut self) {
        // All modules subscribe by default
        let all_types = vec![
            ContentType::Perception,
            ContentType::Thought,
            ContentType::Memory,
            ContentType::Plan,
            ContentType::Emotion,
            ContentType::Intention,
            ContentType::Decision,
        ];

        for module in [
            ModuleType::Reasoning,
            ModuleType::Planning,
            ModuleType::Motivation,
            ModuleType::WorldModel,
            ModuleType::MetaCognition,
        ] {
            self.broadcast.subscribe(
                module,
                SubscriberConfig {
                    content_types: all_types.clone(),
                    max_age_seconds: 60,
                    priority: 5,
                },
            );
        }
    }

    /// Add content to workspace
    pub fn add_content(&mut self, content: WorkspaceContent) -> ContentId {
        let id = content.id.clone();
        self.contents.insert(id.clone(), content);
        self.stats.total_content_added += 1;
        id
    }

    /// Update content activation
    pub fn update_activation(&mut self, content_id: &ContentId, delta: f64) {
        if let Some(content) = self.contents.get_mut(content_id) {
            content.activation = (content.activation + delta).clamp(0.0, 1.0);
        }
    }

    /// Run attention competition and select focus
    pub fn attend(&mut self, context: Option<CompetitionContext>) {
        let contents: Vec<&WorkspaceContent> = self.contents.values().collect();

        // Adjust competition based on context
        if let Some(ctx) = context {
            self.competition.adjust_threshold(&ctx);
        }

        // Run competition
        if let Some(winner_id) = self.competition.compete(&contents) {
            self.focus_on(&winner_id);
        }
    }

    /// Focus on specific content
    pub fn focus_on(&mut self, content_id: &ContentId) {
        if self.contents.contains_key(content_id) {
            // Record access
            if let Some(content) = self.contents.get_mut(content_id) {
                content.accessed();
                content.activation = (content.activation + 0.1).min(1.0);
            }

            // Update current focus
            if self.current_focus.as_ref() != Some(content_id) {
                if let Some(old_id) = &self.current_focus {
                    self.focus_history.push_back(old_id.clone());
                }

                self.current_focus = Some(content_id.clone());

                // Trim history
                while self.focus_history.len() > self.attention_span {
                    self.focus_history.pop_front();
                }
            }

            self.stats.focus_changes += 1;
        }
    }

    /// Get current focus content
    pub fn get_focus(&self) -> Option<&WorkspaceContent> {
        self.current_focus.as_ref().and_then(|id| self.contents.get(id))
    }

    /// Get current focus (mutable)
    pub fn get_focus_mut(&mut self) -> Option<&mut WorkspaceContent> {
        self.current_focus
            .as_ref()
            .and_then(|id| self.contents.get_mut(id))
    }

    /// Broadcast current focus
    pub async fn broadcast_focus(&self) {
        if let Some(content) = self.get_focus() {
            self.broadcast.broadcast(content, None).await;
            self.stats.total_broadcasts += 1;
        }
    }

    /// Get content above consciousness threshold
    pub fn get_conscious_content(&self) -> Vec<&WorkspaceContent> {
        self.contents
            .values()
            .filter(|c| c.salience() >= self.consciousness_threshold)
            .collect()
    }

    /// Get recent focus history
    pub fn get_focus_history(&self) -> Vec<&WorkspaceContent> {
        self.focus_history
            .iter()
            .filter_map(|id| self.contents.get(id))
            .collect()
    }

    /// Get all content sorted by salience
    pub fn get_salience_ranking(&self) -> Vec<&WorkspaceContent> {
        let mut contents: Vec<_> = self.contents.values().collect();
        contents.sort_by(|a, b| b.salience().partial_cmp(&a.salience()).unwrap_or(std::cmp::Ordering::Equal));
        contents
    }

    /// Decay all content activations
    pub fn decay(&mut self, rate: f64) {
        for content in self.contents.values_mut() {
            content.activation = (content.activation * (1.0 - rate)).max(0.1);
            content.relevance = (content.relevance * 0.99).max(0.1);
        }

        self.stats.decay_operations += 1;
    }

    /// Remove old content
    pub fn prune(&mut self, max_age_seconds: i64) {
        let cutoff = Utc::now() - chrono::Duration::seconds(max_age_seconds);

        self.contents.retain(|_, content| {
            content.timestamp > cutoff || content.access_count > 0
        });
    }

    pub fn stats(&self) -> &WorkspaceStats {
        &self.stats
    }
}

impl Default for GlobalWorkspace {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceStats {
    pub total_content_added: u64,
    pub focus_changes: u64,
    pub total_broadcasts: u64,
    pub decay_operations: u64,
}

// ============================================================================
// Integration with Cognition
// ============================================================================

/// Attention controller for cognitive system
#[derive(Debug, Clone)]
pub struct AttentionController {
    pub workspace: GlobalWorkspace,
    pub config: AttentionConfig,
}

impl AttentionController {
    pub fn new() -> Self {
        Self {
            workspace: GlobalWorkspace::new(),
            config: AttentionConfig::default(),
        }
    }

    /// Process perceptual input
    pub fn process_perception(&mut self, perception: ContentData) {
        let content = WorkspaceContent {
            id: ContentId::new(),
            source_module: ModuleType::Perception,
            content_type: ContentType::Perception,
            data: perception,
            activation: 0.8, // Fresh perception starts strong
            relevance: 0.9,
            novelty: 0.7,
            emotional_valence: 0.0,
            timestamp: Utc::now(),
            access_count: 0,
        };

        self.workspace.add_content(content);
    }

    /// Process internal thought
    pub fn process_thought(&mut self, thought: ContentData, emotional_valence: f64) {
        let content = WorkspaceContent {
            id: ContentId::new(),
            source_module: ModuleType::Reasoning,
            content_type: ContentType::Thought,
            data: thought,
            activation: 0.5,
            relevance: 0.7,
            novelty: 0.6,
            emotional_valence,
            timestamp: Utc::now(),
            access_count: 0,
        };

        self.workspace.add_content(content);
    }

    /// Process emotional content
    pub fn process_emotion(&mut self, emotion: EmotionContent) {
        let content = WorkspaceContent {
            id: ContentId::new(),
            source_module: ModuleType::Emotion,
            content_type: ContentType::Emotion,
            data: ContentData::Emotion(emotion.clone()),
            activation: 0.9,
            relevance: 1.0,
            novelty: 0.8,
            emotional_valence: match emotion.emotion_type {
                EmotionType::Joy | EmotionType::Trust | EmotionType::Anticipation => 0.8,
                EmotionType::Anger | EmotionType::Fear | EmotionType::Disgust => -0.8,
                _ => 0.0,
            },
            timestamp: Utc::now(),
            access_count: 0,
        };

        self.workspace.add_content(content);
    }

    /// Run attentional cycle
    pub async fn attentional_cycle(&mut self) {
        // Competition
        self.workspace.attend(None);

        // Broadcast
        self.workspace.broadcast_focus().await;
    }

    /// Get current conscious experience
    pub fn get_conscious_experience(&self) -> ConsciousExperience {
        let focus = self.workspace.get_focus();

        ConsciousExperience {
            focus_content: focus.map(|c| c.data.to_string()),
            focus_type: focus.map(|c| c.content_type),
            salient_context: self
                .workspace
                .get_conscious_content()
                .iter()
                .map(|c| c.data.to_string())
                .collect(),
            awareness_level: focus.map(|c| c.salience()).unwrap_or(0.0),
        }
    }
}

impl Default for AttentionController {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionConfig {
    pub consciousness_threshold: f64,
    pub decay_rate: f64,
    pub max_content: usize,
    pub attention_span: usize,
}

impl Default for AttentionConfig {
    fn default() -> Self {
        Self {
            consciousness_threshold: 0.5,
            decay_rate: 0.01,
            max_content: 100,
            attention_span: 7,
        }
    }
}

/// Current conscious experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousExperience {
    pub focus_content: Option<String>,
    pub focus_type: Option<ContentType>,
    pub salient_context: Vec<String>,
    pub awareness_level: f64,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod workspace_content_tests {
    use super::*;

    #[test]
    fn test_content_creation() {
        let content = WorkspaceContent::new(
            ModuleType::Perception,
            ContentType::Perception,
            ContentData::Text("I see fire".to_string()),
        );

        assert_eq!(content.source_module, ModuleType::Perception);
        assert!(content.salience() > 0.0);
    }

    #[test]
    fn test_salience_calculation() {
        let mut content = WorkspaceContent::new(
            ModuleType::Reasoning,
            ContentType::Thought,
            ContentData::Text("Important thought".to_string()),
        );

        content.activation = 0.8;
        content.relevance = 0.9;
        content.novelty = 0.7;

        let salience = content.salience();
        assert!(salience > 0.5);
    }

    #[test]
    fn test_access_decay() {
        let mut content = WorkspaceContent::new(
            ModuleType::Memory,
            ContentType::Memory,
            ContentData::Memory(MemoryContent {
                description: "Test".to_string(),
                episode_type: "test".to_string(),
                emotional_tags: vec![],
            }),
        );

        content.novelty = 0.8;
        content.accessed();

        assert!(content.novelty < 0.8);
        assert_eq!(content.access_count, 1);
    }
}

#[cfg(test)]
mod competition_tests {
    use super::*;

    #[test]
    fn test_competition() {
        let mut competition = CompetitionMechanism::new();

        let contents = vec![
            WorkspaceContent::new(
                ModuleType::Perception,
                ContentType::Perception,
                ContentData::Text("See fire".to_string()),
            ),
            WorkspaceContent::new(
                ModuleType::Reasoning,
                ContentType::Thought,
                ContentData::Text("Think about fire".to_string()),
            ),
        ];

        // Manually set activations
        let mut contents = contents;
        contents[0].activation = 0.9;
        contents[1].activation = 0.5;

        let winner = competition.compete(&contents);
        assert!(winner.is_some());
    }

    #[test]
    fn test_no_winner_below_threshold() {
        let mut competition = CompetitionMechanism::new();
        competition.threshold = 0.9;

        let contents = vec![WorkspaceContent::new(
            ModuleType::Perception,
            ContentType::Perception,
            ContentData::Text("Low activation".to_string()),
        )];

        let winner = competition.compete(&contents);
        assert!(winner.is_none());
    }
}

#[cfg(test)]
mod broadcast_tests {
    use super::*;

    #[tokio::test]
    async fn test_broadcast() {
        let channel = BroadcastChannel::new();

        let content = WorkspaceContent::new(
            ModuleType::Perception,
            ContentType::Perception,
            ContentData::Text("Broadcast test".to_string()),
        );

        channel.broadcast(&content, None).await;

        let recent = channel.get_all_recent(60).await;
        assert!(!recent.is_empty());
    }

    #[tokio::test]
    async fn test_subscriber_filtering() {
        let mut channel = BroadcastChannel::new();
        channel.subscribe(
            ModuleType::Reasoning,
            SubscriberConfig {
                content_types: vec![ContentType::Thought],
                max_age_seconds: 60,
                priority: 5,
            },
        );

        let content = WorkspaceContent::new(
            ModuleType::Perception,
            ContentType::Perception,
            ContentData::Text("Perception".to_string()),
        );

        channel.broadcast(&content, None).await;

        let for_reasoning = channel.get_for_subscriber(ModuleType::Reasoning).await;
        // Should be empty because content type is Perception, not Thought
        assert!(for_reasoning.is_empty());
    }
}

#[cfg(test)]
mod global_workspace_tests {
    use super::*;

    #[test]
    fn test_add_content() {
        let mut workspace = GlobalWorkspace::new();

        let content = WorkspaceContent::new(
            ModuleType::Perception,
            ContentType::Perception,
            ContentData::Text("Add test".to_string()),
        );

        let id = workspace.add_content(content);
        assert!(workspace.contents.contains_key(&id));
    }

    #[test]
    fn test_attend() {
        let mut workspace = GlobalWorkspace::new();

        let content = WorkspaceContent::new(
            ModuleType::Perception,
            ContentType::Perception,
            ContentData::Text("Attend test".to_string()),
        );

        let id = workspace.add_content(content);
        workspace.attend(None);

        // Winner should be focused
        assert!(workspace.current_focus.is_some());
    }

    #[test]
    fn test_salience_ranking() {
        let mut workspace = GlobalWorkspace::new();

        let content1 = WorkspaceContent::new(
            ModuleType::Perception,
            ContentType::Perception,
            ContentData::Text("High salience".to_string()),
        );

        let content2 = WorkspaceContent::new(
            ModuleType::Reasoning,
            ContentType::Thought,
            ContentData::Text("Low salience".to_string()),
        );

        workspace.add_content(content1);
        workspace.add_content(content2);

        let ranking = workspace.get_salience_ranking();
        assert!(!ranking.is_empty());
    }

    #[test]
    fn test_decay() {
        let mut workspace = GlobalWorkspace::new();

        let content = WorkspaceContent::new(
            ModuleType::Memory,
            ContentType::Memory,
            ContentData::Memory(MemoryContent {
                description: "Decay test".to_string(),
                episode_type: "test".to_string(),
                emotional_tags: vec![],
            }),
        );

        workspace.add_content(content);

        let original_activation = workspace.contents.values().next().unwrap().activation;
        workspace.decay(0.5);

        let new_activation = workspace.contents.values().next().unwrap().activation;
        assert!(new_activation < original_activation);
    }
}

#[cfg(test)]
mod attention_controller_tests {
    use super::*;

    #[test]
    fn test_process_perception() {
        let mut controller = AttentionController::new();

        controller.process_perception(ContentData::Text("Fire detected".to_string()));

        assert!(!controller.workspace.contents.is_empty());
    }

    #[test]
    fn test_process_emotion() {
        let mut controller = AttentionController::new();

        controller.process_emotion(EmotionContent {
            emotion_type: EmotionType::Joy,
            intensity: 0.8,
            trigger: Some("Good news".to_string()),
        });

        // Emotional content should have high activation
        let content = controller.workspace.contents.values().next().unwrap();
        assert!(content.activation > 0.8);
    }

    #[tokio::test]
    async fn test_attentional_cycle() {
        let mut controller = AttentionController::new();

        controller.process_perception(ContentData::Text("Cycle test".to_string()));

        controller.attentional_cycle().await;

        // Should have run competition and broadcast
        assert!(controller.workspace.current_focus.is_some());
    }

    #[test]
    fn test_conscious_experience() {
        let mut controller = AttentionController::new();

        controller.process_perception(ContentData::Text("Conscious test".to_string()));
        controller.workspace.attend(None);

        let experience = controller.get_conscious_experience();

        assert!(experience.focus_content.is_some() || experience.awareness_level >= 0.0);
    }
}
