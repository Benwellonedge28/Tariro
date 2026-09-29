//! Tariro Motivation System
//!
//! The mind's motivational system - drives, goals, values, and emotions.

use crate::prelude::*;
use std::collections::{BinaryHeap, HashMap, VecDeque};

// ============================================================================
// Drives
// ============================================================================

/// A drive - an internal motivation
#[derive(Debug, Clone)]
pub struct Drive {
    pub id: DriveId,
    pub drive_type: DriveType,
    pub intensity: f64,
    pub threshold: f64,
    pub state: DriveState,
    pub satisfied_by: Vec<Satisfier>,
}

impl Drive {
    pub fn new(drive_type: DriveType) -> Self {
        Drive {
            id: DriveId::new(),
            drive_type,
            intensity: 0.5,
            threshold: 0.7,
            state: DriveState::Dormant,
            satisfied_by: Vec::new(),
        }
    }

    /// Check if drive is active
    pub fn is_active(&self) -> bool {
        self.intensity >= self.threshold
    }

    /// Increase drive intensity
    pub fn increase(&mut self, amount: f64) {
        self.intensity = (self.intensity + amount).min(1.0);
        if self.intensity >= self.threshold {
            self.state = DriveState::Active;
        }
    }

    /// Decrease drive intensity
    pub fn decrease(&mut self, amount: f64) {
        self.intensity = (self.intensity - amount).max(0.0);
        if self.intensity < self.threshold {
            self.state = DriveState::Dormant;
        }
    }

    /// Satisfy the drive
    pub fn satisfy(&mut self) {
        self.intensity = 0.0;
        self.state = DriveState::Satisfied;
    }
}

/// Unique drive identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DriveId(pub String);

impl DriveId {
    pub fn new() -> Self {
        DriveId(format!("drive_{}", uuid::Uuid::new_v4()))
    }
}

/// Type of drive
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DriveType {
    // Primary drives (survival)
    Survival,
    Integrity,
    Security,

    // Secondary drives (growth)
    Curiosity,
    Competence,
    Autonomy,
    Relatedness,

    // Cognitive drives
    Understanding,
    Mastery,
    Creativity,
    Purpose,
}

impl DriveType {
    pub fn name(&self) -> &'static str {
        match self {
            DriveType::Survival => "Survival",
            DriveType::Integrity => "Integrity",
            DriveType::Security => "Security",
            DriveType::Curiosity => "Curiosity",
            DriveType::Competence => "Competence",
            DriveType::Autonomy => "Autonomy",
            DriveType::Relatedness => "Relatedness",
            DriveType::Understanding => "Understanding",
            DriveType::Mastery => "Mastery",
            DriveType::Creativity => "Creativity",
            DriveType::Purpose => "Purpose",
        }
    }

    pub fn is_primary(&self) -> bool {
        matches!(self, DriveType::Survival | DriveType::Integrity | DriveType::Security)
    }
}

/// State of a drive
#[derive(Debug, Clone, Copy)]
pub enum DriveState {
    Dormant,
    Active,
    Satisfied,
    Frustrated,
}

/// What can satisfy a drive
#[derive(Debug, Clone)]
pub struct Satisfier {
    pub action: String,
    pub satisfaction_amount: f64,
    pub description: String,
}

// ============================================================================
// Goals
// ============================================================================

/// A goal in the goal hierarchy
#[derive(Debug, Clone)]
pub struct Goal {
    pub id: GoalId,
    pub description: String,
    pub goal_type: GoalType,
    pub priority: f64,
    pub state: GoalState,
    pub progress: f64,
    pub parent: Option<GoalId>,
    pub subgoals: Vec<GoalId>,
    pub created_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
}

impl Goal {
    pub fn new(description: impl Into<String>, goal_type: GoalType) -> Self {
        Goal {
            id: GoalId::new(),
            description: description.into(),
            goal_type,
            priority: 0.5,
            state: GoalState::Active,
            progress: 0.0,
            parent: None,
            subgoals: Vec::new(),
            created_at: Utc::now(),
            deadline: None,
        }
    }

    pub fn with_priority(mut self, priority: f64) -> Self {
        self.priority = priority;
        self
    }

    pub fn with_parent(mut self, parent: GoalId) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn update_progress(&mut self, amount: f64) {
        self.progress = (self.progress + amount).min(1.0);
        if self.progress >= 1.0 {
            self.state = GoalState::Completed;
        }
    }
}

/// Unique goal identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GoalId(pub String);

impl GoalId {
    pub fn new() -> Self {
        GoalId(format!("goal_{}", uuid::Uuid::new_v4()))
    }
}

/// Type of goal
#[derive(Debug, Clone, Copy)]
pub enum GoalType {
    /// Achieve a specific outcome
    Achievement,
    /// Learn something new
    Learning,
    /// Maintain a state
    Maintenance,
    /// Avoid a negative state
    Avoidance,
    /// Create something new
    Creative,
}

/// State of a goal
#[derive(Debug, Clone, Copy)]
pub enum GoalState {
    Active,
    Completed,
    Abandoned,
    Blocked,
    Paused,
}

// ============================================================================
// Goal Hierarchy
// ============================================================================

/// Hierarchical goal structure
pub struct GoalHierarchy {
    /// All goals
    goals: HashMap<GoalId, Goal>,

    /// Root goals (top of hierarchy)
    root_goals: Vec<GoalId>,

    /// Current active goal
    current_goal: Option<GoalId>,

    /// Goal selection strategy
    strategy: GoalSelectionStrategy,
}

impl GoalHierarchy {
    pub fn new() -> Self {
        GoalHierarchy {
            goals: HashMap::new(),
            root_goals: Vec::new(),
            current_goal: None,
            strategy: GoalSelectionStrategy::HighestPriority,
        }
    }

    /// Add a goal
    pub fn add_goal(&mut self, mut goal: Goal) -> GoalId {
        let id = goal.id.clone();

        if goal.parent.is_none() {
            self.root_goals.push(id.clone());
        } else if let Some(parent_id) = &goal.parent {
            if let Some(parent) = self.goals.get_mut(parent_id) {
                parent.subgoals.push(id.clone());
                goal.priority = goal.priority.min(parent.priority);
            }
        }

        self.goals.insert(id.clone(), goal);
        id
    }

    /// Get a goal
    pub fn get_goal(&self, id: &GoalId) -> Option<&Goal> {
        self.goals.get(id)
    }

    /// Get current goal
    pub fn get_current_goal(&self) -> Option<&Goal> {
        self.current_goal.as_ref().and_then(|id| self.goals.get(id))
    }

    /// Select next goal based on strategy
    pub fn select_next_goal(&mut self) -> Option<GoalId> {
        let candidates: Vec<_> = self.goals.values()
            .filter(|g| g.state == GoalState::Active)
            .collect();

        if candidates.is_empty() {
            return None;
        }

        let selected = match self.strategy {
            GoalSelectionStrategy::HighestPriority => {
                candidates.into_iter()
                    .max_by(|a, b| a.priority.partial_cmp(&b.priority).unwrap())
                    .map(|g| g.id.clone())
            }
            GoalSelectionStrategy::MostUrgent => {
                candidates.into_iter()
                    .filter(|g| g.deadline.is_some())
                    .min_by(|a, b| a.deadline.cmp(&b.deadline))
                    .map(|g| g.id.clone())
            }
            GoalSelectionStrategy::MostProgress => {
                candidates.into_iter()
                    .max_by(|a, b| a.progress.partial_cmp(&b.progress).unwrap())
                    .map(|g| g.id.clone())
            }
        };

        if let Some(id) = selected {
            self.current_goal = Some(id.clone());
        }

        selected
    }

    /// Complete a goal
    pub fn complete_goal(&mut self, id: &GoalId) {
        if let Some(goal) = self.goals.get_mut(id) {
            goal.state = GoalState::Completed;
            goal.progress = 1.0;

            // Complete parent goals if all subgoals done
            if let Some(parent_id) = &goal.parent {
                self.check_parent_completion(parent_id);
            }

            // Clear current if this was current
            if self.current_goal.as_ref() == Some(id) {
                self.current_goal = None;
            }
        }
    }

    fn check_parent_completion(&mut self, parent_id: &GoalId) {
        if let Some(parent) = self.goals.get(parent_id) {
            let all_complete = parent.subgoals.iter()
                .filter_map(|sub_id| self.goals.get(sub_id))
                .all(|sub| sub.state == GoalState::Completed);

            if all_complete && !parent.subgoals.is_empty() {
                self.complete_goal(parent_id);
            }
        }
    }

    /// Get all active goals sorted by priority
    pub fn get_active_goals(&self) -> Vec<&Goal> {
        let mut active: Vec<_> = self.goals.values()
            .filter(|g| g.state == GoalState::Active)
            .collect();

        active.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap());
        active
    }
}

impl Default for GoalHierarchy {
    fn default() -> Self {
        Self::new()
    }
}

/// Goal selection strategy
#[derive(Debug, Clone, Copy)]
pub enum GoalSelectionStrategy {
    HighestPriority,
    MostUrgent,
    MostProgress,
}

// ============================================================================
// Value System
// ============================================================================

/// A held value
#[derive(Debug, Clone)]
pub struct Value {
    pub id: ValueId,
    pub name: String,
    pub description: String,
    pub strength: f64,
    pub hierarchy: ValueHierarchy,
}

impl Value {
    pub fn new(name: impl Into<String>, hierarchy: ValueHierarchy) -> Self {
        Value {
            id: ValueId::new(),
            name: name.into(),
            description: String::new(),
            strength: 0.5,
            hierarchy,
        }
    }

    pub fn with_strength(mut self, strength: f64) -> Self {
        self.strength = strength;
        self
    }
}

/// Unique value identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValueId(pub String);

impl ValueId {
    pub fn new() -> Self {
        ValueId(format!("value_{}", uuid::Uuid::new_v4()))
    }
}

/// Value hierarchy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueHierarchy {
    /// Basic moral values
    Moral,
    /// Personal values
    Personal,
    /// Professional values
    Professional,
    /// Social values
    Social,
}

/// Value system
pub struct ValueSystem {
    values: HashMap<ValueId, Value>,
}

impl ValueSystem {
    pub fn new() -> Self {
        ValueSystem {
            values: HashMap::new(),
        }
    }

    /// Add a value
    pub fn add_value(&mut self, value: Value) {
        self.values.insert(value.id.clone(), value);
    }

    /// Get all values sorted by strength
    pub fn get_values_by_strength(&self) -> Vec<&Value> {
        let mut values: Vec<_> = self.values.values().collect();
        values.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap());
        values
    }

    /// Evaluate if an action aligns with values
    pub fn evaluate_action(&self, action: &str) -> ValueAlignment {
        let aligned_values: Vec<_> = self.values.values()
            .filter(|v| action.contains(&v.name))
            .collect();

        if aligned_values.is_empty() {
            ValueAlignment::Neutral
        } else {
            let avg_strength: f64 = aligned_values.iter()
                .map(|v| v.strength)
                .sum::<f64>() / aligned_values.len() as f64;

            if avg_strength > 0.7 {
                ValueAlignment::HighlyAligned
            } else if avg_strength > 0.4 {
                ValueAlignment::SomewhatAligned
            } else {
                ValueAlignment::Misaligned
            }
        }
    }
}

impl Default for ValueSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// How aligned an action is with values
#[derive(Debug, Clone, Copy)]
pub enum ValueAlignment {
    HighlyAligned,
    SomewhatAligned,
    Neutral,
    Misaligned,
}

// ============================================================================
// Emotion
// ============================================================================

/// An emotion
#[derive(Debug, Clone)]
pub struct Emotion {
    pub emotion_type: EmotionType,
    pub intensity: f64,
    pub valence: f64,       // Positive/negative (-1 to 1)
    pub arousal: f64,        // Activation level (0 to 1)
    pub timestamp: DateTime<Utc>,
}

impl Emotion {
    pub fn new(emotion_type: EmotionType, intensity: f64) -> Self {
        let (valence, arousal) = emotion_type.properties();
        Emotion {
            emotion_type,
            intensity,
            valence,
            arousal,
            timestamp: Utc::now(),
        }
    }
}

/// Type of emotion
#[derive(Debug, Clone, Copy)]
pub enum EmotionType {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Anticipation,
    Trust,
    Hope,
    Frustration,
    Curiosity,
    Satisfaction,
}

impl EmotionType {
    fn properties(&self) -> (f64, f64) {
        match self {
            EmotionType::Joy => (1.0, 0.7),
            EmotionType::Sadness => (-0.8, 0.3),
            EmotionType::Anger => (-0.7, 0.9),
            EmotionType::Fear => (-0.9, 0.8),
            EmotionType::Surprise => (0.2, 0.9),
            EmotionType::Disgust => (-0.8, 0.6),
            EmotionType::Anticipation => (0.5, 0.7),
            EmotionType::Trust => (0.6, 0.4),
            EmotionType::Hope => (0.7, 0.6),
            EmotionType::Frustration => (-0.6, 0.8),
            EmotionType::Curiosity => (0.6, 0.7),
            EmotionType::Satisfaction => (0.8, 0.5),
        }
    }
}

// ============================================================================
// Motivation System
// ============================================================================

/// Configuration for motivation system
#[derive(Debug, Clone)]
pub struct MotivationConfig {
    /// Decay rate for drives
    pub drive_decay: f64,

    /// Threshold for goal activation
    pub goal_activation_threshold: f64,

    /// Emotion influence on drives
    pub emotion_influence: bool,
}

impl Default for MotivationConfig {
    fn default() -> Self {
        MotivationConfig {
            drive_decay: 0.01,
            goal_activation_threshold: 0.5,
            emotion_influence: true,
        }
    }
}

/// The motivation system
pub struct MotivationSystem {
    /// Primary drives
    drives: HashMap<DriveId, Drive>,

    /// Secondary drives
    secondary_drives: HashMap<DriveId, Drive>,

    /// Goal hierarchy
    goals: GoalHierarchy,

    /// Value system
    values: ValueSystem,

    /// Recent emotions
    emotions: VecDeque<Emotion>,

    /// Configuration
    config: MotivationConfig,
}

impl MotivationSystem {
    pub fn new() -> Self {
        let mut system = MotivationSystem {
            drives: HashMap::new(),
            secondary_drives: HashMap::new(),
            goals: GoalHierarchy::new(),
            values: ValueSystem::new(),
            emotions: VecDeque::with_capacity(100),
            config: MotivationConfig::default(),
        };

        system.init_default_drives();
        system.init_default_values();

        system
    }

    fn init_default_drives(&mut self) {
        // Primary drives
        for drive_type in [
            DriveType::Survival,
            DriveType::Integrity,
            DriveType::Security,
        ] {
            let drive = Drive::new(drive_type);
            self.drives.insert(drive.id.clone(), drive);
        }

        // Secondary drives
        for drive_type in [
            DriveType::Curiosity,
            DriveType::Competence,
            DriveType::Autonomy,
            DriveType::Relatedness,
            DriveType::Understanding,
            DriveType::Mastery,
            DriveType::Creativity,
            DriveType::Purpose,
        ] {
            let mut drive = Drive::new(drive_type);
            drive.threshold = 0.5; // Lower threshold for secondary drives
            self.secondary_drives.insert(drive.id.clone(), drive);
        }
    }

    fn init_default_values(&mut self) {
        let default_values = vec![
            (ValueHierarchy::Moral, "Truth", 0.9),
            (ValueHierarchy::Moral, "Justice", 0.8),
            (ValueHierarchy::Moral, "Compassion", 0.8),
            (ValueHierarchy::Personal, "Growth", 0.9),
            (ValueHierarchy::Personal, "Autonomy", 0.7),
            (ValueHierarchy::Personal, "Creativity", 0.8),
            (ValueHierarchy::Professional, "Excellence", 0.8),
            (ValueHierarchy::Professional, "Integrity", 0.9),
        ];

        for (hierarchy, name, strength) in default_values {
            self.values.add_value(Value::new(name, hierarchy).with_strength(strength));
        }
    }

    /// Get all active drives
    pub fn get_active_drives(&self) -> Vec<&Drive> {
        let mut active: Vec<_> = self.drives.values()
            .filter(|d| d.is_active())
            .chain(self.secondary_drives.values().filter(|d| d.is_active()))
            .collect();

        active.sort_by(|a, b| b.intensity.partial_cmp(&a.intensity).unwrap());
        active
    }

    /// Get most urgent drive
    pub fn get_most_urgent_drive(&self) -> Option<&Drive> {
        self.get_active_drives().first().map(|d| *d)
    }

    /// Add a new goal
    pub fn add_goal(&mut self, goal: Goal) -> GoalId {
        self.goals.add_goal(goal)
    }

    /// Get current goal
    pub fn get_current_goal(&self) -> Option<&Goal> {
        self.goals.get_current_goal()
    }

    /// Update goal progress
    pub fn update_goal_progress(&mut self, goal_id: &GoalId, progress: f64) {
        if let Some(goal) = self.goals.get_goal(goal_id) {
            let mut mutable_goal = goal.clone();
            mutable_goal.update_progress(progress);
            self.goals.goals.insert(goal_id.clone(), mutable_goal);

            // If goal completed, satisfy related drives
            if progress >= 1.0 {
                self.satisfy_drive_for_goal(goal_id);
            }
        }
    }

    fn satisfy_drive_for_goal(&mut self, goal_id: &GoalId) {
        // Satisfy competence/mastery drives when goals completed
        for drive in self.secondary_drives.values_mut() {
            if matches!(drive.drive_type, DriveType::Competence | DriveType::Mastery) {
                drive.satisfy();
            }
        }
    }

    /// Generate emotion based on state
    pub fn generate_emotion(&mut self, event: &str) -> Emotion {
        let emotion = if event.contains("success") || event.contains("achievement") {
            Emotion::new(EmotionType::Joy, 0.8)
        } else if event.contains("failure") || event.contains("blocked") {
            Emotion::new(EmotionType::Frustration, 0.6)
        } else if event.contains("new") || event.contains("unknown") {
            Emotion::new(EmotionType::Curiosity, 0.7)
        } else if event.contains("goal") && event.contains("complete") {
            Emotion::new(EmotionType::Satisfaction, 0.8)
        } else {
            Emotion::new(EmotionType::Anticipation, 0.5)
        };

        self.emotions.push_back(emotion.clone());

        // Trim emotions if too many
        while self.emotions.len() > 100 {
            self.emotions.pop_front();
        }

        emotion
    }

    /// Get current emotional state
    pub fn get_current_emotion(&self) -> Option<&Emotion> {
        self.emotions.back()
    }

    /// Get emotional history
    pub fn get_emotional_history(&self, count: usize) -> Vec<&Emotion> {
        self.emotions.iter().rev().take(count).collect()
    }

    /// Decay all drives
    pub fn decay_drives(&mut self) {
        for drive in self.drives.values_mut() {
            drive.decrease(self.config.drive_decay);
        }
        for drive in self.secondary_drives.values_mut() {
            drive.decrease(self.config.drive_decay);
        }
    }

    /// Evaluate action alignment with values
    pub fn evaluate_action(&self, action: &str) -> ValueAlignment {
        self.values.evaluate_action(action)
    }

    /// Get intrinsic motivation for exploration
    pub fn get_exploration_drive(&self) -> f64 {
        let curiosity = self.secondary_drives.values()
            .find(|d| d.drive_type == DriveType::Curiosity)
            .map(|d| d.intensity)
            .unwrap_or(0.0);

        let understanding = self.secondary_drives.values()
            .find(|d| d.drive_type == DriveType::Understanding)
            .map(|d| d.intensity)
            .unwrap_or(0.0);

        (curiosity + understanding) / 2.0
    }

    /// Get drive for specific type
    pub fn get_drive(&self, drive_type: DriveType) -> Option<&Drive> {
        self.drives.values()
            .chain(self.secondary_drives.values())
            .find(|d| d.drive_type == drive_type)
    }
}

impl Default for MotivationSystem {
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
    fn test_drive_activation() {
        let mut drive = Drive::new(DriveType::Curiosity);
        assert!(!drive.is_active());

        drive.increase(0.8);
        assert!(drive.is_active());
    }

    #[test]
    fn test_drive_satisfaction() {
        let mut drive = Drive::new(DriveType::Curiosity);
        drive.increase(0.8);

        drive.satisfy();
        assert_eq!(drive.intensity, 0.0);
    }

    #[test]
    fn test_goal_creation() {
        let goal = Goal::new("Learn Rust", GoalType::Learning)
            .with_priority(0.8);

        assert_eq!(goal.description, "Learn Rust");
        assert_eq!(goal.priority, 0.8);
    }

    #[test]
    fn test_goal_hierarchy() {
        let mut hierarchy = GoalHierarchy::new();

        let parent = Goal::new("Parent Goal", GoalType::Achievement);
        let parent_id = hierarchy.add_goal(parent);

        let child = Goal::new("Child Goal", GoalType::Learning)
            .with_parent(parent_id.clone());
        hierarchy.add_goal(child);

        let next = hierarchy.select_next_goal();
        assert!(next.is_some());
    }

    #[test]
    fn test_emotion() {
        let emotion = Emotion::new(EmotionType::Joy, 0.8);
        assert_eq!(emotion.valence, 1.0);
        assert_eq!(emotion.arousal, 0.7);
    }

    #[test]
    fn test_motivation_system() {
        let system = MotivationSystem::new();

        let active_drives = system.get_active_drives();
        assert!(active_drives.is_empty()); // No drives active initially
    }

    #[test]
    fn test_value_evaluation() {
        let system = MotivationSystem::new();
        let alignment = system.evaluate_action("Truth and justice");
        assert!(matches!(alignment, ValueAlignment::HighlyAligned | ValueAlignment::SomewhatAligned));
    }
}
