// Tariro AGI - Emotional Architecture Module
// Comprehensive emotional processing system for AGI

use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Emotion Representations
// ============================================================================

/// Basic emotion types (Ekman's fundamental emotions plus neutral)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BasicEmotion {
    Happiness,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Neutral,
}

impl BasicEmotion {
    pub fn to_index(&self) -> usize {
        match self {
            BasicEmotion::Happiness => 0,
            BasicEmotion::Sadness => 1,
            BasicEmotion::Anger => 2,
            BasicEmotion::Fear => 3,
            BasicEmotion::Surprise => 4,
            BasicEmotion::Disgust => 5,
            BasicEmotion::Neutral => 6,
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(BasicEmotion::Happiness),
            1 => Some(BasicEmotion::Sadness),
            2 => Some(BasicEmotion::Anger),
            3 => Some(BasicEmotion::Fear),
            4 => Some(BasicEmotion::Surprise),
            5 => Some(BasicEmotion::Disgust),
            6 => Some(BasicEmotion::Neutral),
            _ => None,
        }
    }

    pub fn arousal_level(&self) -> f32 {
        match self {
            BasicEmotion::Happiness => 0.7,
            BasicEmotion::Sadness => 0.3,
            BasicEmotion::Anger => 0.9,
            BasicEmotion::Fear => 0.8,
            BasicEmotion::Surprise => 0.8,
            BasicEmotion::Disgust => 0.6,
            BasicEmotion::Neutral => 0.5,
        }
    }

    pub fn valence_level(&self) -> f32 {
        match self {
            BasicEmotion::Happiness => 0.9,
            BasicEmotion::Sadness => -0.7,
            BasicEmotion::Anger => -0.4,
            BasicEmotion::Fear => -0.6,
            BasicEmotion::Surprise => 0.2,
            BasicEmotion::Disgust => -0.8,
            BasicEmotion::Neutral => 0.0,
        }
    }
}

/// Dimensional emotion representation using valence-arousal-dominance model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VADEmotion {
    pub valence: f32,      // -1.0 (negative) to 1.0 (positive)
    pub arousal: f32,       // 0.0 (calm) to 1.0 (excited)
    pub dominance: f32,     // 0.0 (submissive) to 1.0 (dominant)
}

impl VADEmotion {
    pub fn new(valence: f32, arousal: f32, dominance: f32) -> Self {
        Self {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(0.0, 1.0),
            dominance: dominance.clamp(0.0, 1.0),
        }
    }

    pub fn neutral() -> Self {
        Self {
            valence: 0.0,
            arousal: 0.5,
            dominance: 0.5,
        }
    }

    pub fn from_basic(emotion: BasicEmotion) -> Self {
        Self {
            valence: emotion.valence_level(),
            arousal: emotion.arousal_level(),
            dominance: 0.5,
        }
    }

    pub fn to_basic(&self) -> BasicEmotion {
        // Map VAD to basic emotion
        if self.valence > 0.5 && self.arousal > 0.5 {
            BasicEmotion::Happiness
        } else if self.valence < -0.5 && self.arousal < 0.5 {
            BasicEmotion::Sadness
        } else if self.valence < 0.0 && self.arousal > 0.7 {
            BasicEmotion::Anger
        } else if self.valence < -0.3 && self.arousal > 0.5 {
            BasicEmotion::Fear
        } else if self.arousal > 0.7 && self.valence > 0.0 {
            BasicEmotion::Surprise
        } else if self.valence < -0.5 {
            BasicEmotion::Disgust
        } else if self.valence.abs() < 0.2 && (self.arousal - 0.5).abs() < 0.2 {
            BasicEmotion::Neutral
        } else {
            BasicEmotion::Neutral // Default fallback
        }
    }

    pub fn euclidean_distance(&self, other: &VADEmotion) -> f32 {
        ((self.valence - other.valence).powi(2)
            + (self.arousal - other.arousal).powi(2)
            + (self.dominance - other.dominance).powi(2))
        .sqrt()
    }
}

/// Complete emotional state representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub basic_emotions: [f32; 7],        // Intensity of each basic emotion
    pub vad: VADEmotion,                  // Dimensional representation
    pub mood: MoodState,                  // Longer-term mood
    pub emotional_complexity: f32,        // How mixed/differentiated emotions are
    pub timestamp: DateTime<Utc>,
    pub duration: Duration,
}

impl EmotionalState {
    pub fn new() -> Self {
        Self {
            basic_emotions: [0.0; 7],
            vad: VADEmotion::neutral(),
            mood: MoodState::Neutral,
            emotional_complexity: 0.0,
            timestamp: Utc::now(),
            duration: Duration::ZERO,
        }
    }

    pub fn with_emotion(mut self, emotion: BasicEmotion, intensity: f32) -> Self {
        self.basic_emotions[emotion.to_index()] = intensity.clamp(0.0, 1.0);
        self
    }

    pub fn dominant_emotion(&self) -> BasicEmotion {
        self.basic_emotions
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .and_then(|(i, _)| BasicEmotion::from_index(i))
            .unwrap_or(BasicEmotion::Sadness)
    }

    pub fn total_intensity(&self) -> f32 {
        self.basic_emotions.iter().sum()
    }

    pub fn update_duration(&mut self) {
        self.duration = Utc::now().signed_duration_since(self.timestamp).to_std().unwrap_or(Duration::ZERO);
    }
}

impl Default for EmotionalState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Mood States
// ============================================================================

/// Long-term mood states (more persistent than emotions)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoodState {
    Joyful,
    Content,
    Calm,
    Anxious,
    Sad,
    Angry,
    Fearful,
    Neutral,
    Confused,
    Excited,
}

impl MoodState {
    pub fn to_vad(&self) -> VADEmotion {
        match self {
            MoodState::Joyful => VADEmotion::new(0.8, 0.7, 0.7),
            MoodState::Content => VADEmotion::new(0.6, 0.3, 0.6),
            MoodState::Calm => VADEmotion::new(0.3, 0.2, 0.5),
            MoodState::Anxious => VADEmotion::new(-0.4, 0.7, 0.3),
            MoodState::Sad => VADEmotion::new(-0.6, 0.3, 0.2),
            MoodState::Angry => VADEmotion::new(-0.5, 0.8, 0.7),
            MoodState::Fearful => VADEmotion::new(-0.6, 0.7, 0.2),
            MoodState::Neutral => VADEmotion::neutral(),
            MoodState::Confused => VADEmotion::new(0.0, 0.5, 0.3),
            MoodState::Excited => VADEmotion::new(0.7, 0.9, 0.8),
        }
    }

    pub fn from_vad(vad: &VADEmotion) -> Self {
        if vad.valence > 0.5 && vad.arousal > 0.5 {
            MoodState::Excited
        } else if vad.valence > 0.5 {
            MoodState::Joyful
        } else if vad.valence > 0.2 && vad.arousal < 0.4 {
            MoodState::Content
        } else if vad.arousal < 0.3 && vad.dominance > 0.4 {
            MoodState::Calm
        } else if vad.valence < -0.4 && vad.arousal > 0.6 {
            MoodState::Angry
        } else if vad.valence < -0.4 && vad.dominance < 0.4 {
            MoodState::Fearful
        } else if vad.valence < -0.3 {
            MoodState::Sad
        } else if vad.arousal > 0.6 && vad.dominance < 0.4 {
            MoodState::Anxious
        } else if vad.dominance < 0.4 {
            MoodState::Confused
        } else {
            MoodState::Neutral
        }
    }
}

/// Mood transition probabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoodTransition {
    pub from: MoodState,
    pub to: MoodState,
    pub probability: f32,
    pub decay_time: Duration,
}

impl MoodTransition {
    pub fn standard_transitions() -> Vec<Self> {
        let mut transitions = Vec::new();

        // Common mood transitions
        let pairs = vec![
            (MoodState::Joyful, MoodState::Content, 0.7),
            (MoodState::Content, MoodState::Calm, 0.6),
            (MoodState::Calm, MoodState::Neutral, 0.5),
            (MoodState::Neutral, MoodState::Content, 0.4),
            (MoodState::Excited, MoodState::Joyful, 0.6),
            (MoodState::Angry, MoodState::Anxious, 0.4),
            (MoodState::Angry, MoodState::Calm, 0.3),
            (MoodState::Fearful, MoodState::Anxious, 0.5),
            (MoodState::Fearful, MoodState::Calm, 0.3),
            (MoodState::Sad, MoodState::Neutral, 0.4),
            (MoodState::Anxious, MoodState::Calm, 0.5),
            (MoodState::Confused, MoodState::Neutral, 0.4),
        ];

        for (from, to, prob) in pairs {
            transitions.push(MoodTransition {
                from,
                to,
                probability: prob,
                decay_time: Duration::from_secs(300), // 5 minutes default
            });
        }

        transitions
    }
}

// ============================================================================
// Emotional Processing Components
// ============================================================================

/// Emotion detector - identifies emotions from various inputs
pub struct EmotionDetector {
    threshold: f32,
    confidence_weights: HashMap<String, f32>,
}

impl EmotionDetector {
    pub fn new() -> Self {
        let mut weights = HashMap::new();
        weights.insert("text".to_string(), 0.4);
        weights.insert("voice".to_string(), 0.35);
        weights.insert("facial".to_string(), 0.25);

        Self {
            threshold: 0.3,
            confidence_weights: weights,
        }
    }

    /// Detect emotion from textual content
    pub fn detect_from_text(&self, text: &str) -> (BasicEmotion, f32) {
        let text_lower = text.to_lowercase();

        // Simple keyword-based emotion detection
        let emotion_scores = [
            // Happiness indicators
            Self::count_keywords(&text_lower, &["happy", "joy", "love", "great", "wonderful", "excellent", "fantastic", "amazing", "good", "nice"]),
            // Sadness indicators
            Self::count_keywords(&text_lower, &["sad", "unhappy", "depressed", "miserable", "terrible", "awful", "bad", "sorrow", "grief", "disappointed"]),
            // Anger indicators
            Self::count_keywords(&text_lower, &["angry", "mad", "furious", "hate", "annoyed", "frustrated", "upset", "irritated", "rage", "outraged"]),
            // Fear indicators
            Self::count_keywords(&text_lower, &["afraid", "scared", "frightened", "worried", "anxious", "nervous", "terrified", "panic", "dread", "fear"]),
            // Surprise indicators
            Self::count_keywords(&text_lower, &["surprised", "amazed", "astonished", "shocked", "unexpected", "wow", "oh", "incredible", "unbelievable"]),
            // Disgust indicators
            Self::count_keywords(&text_lower, &["disgusting", "gross", "revolting", "sickening", "nasty", "horrible", "dislike", "repulsed", "vile"]),
        ];

        // Find dominant emotion
        let mut max_score = 0.0f32;
        let mut max_idx = 0;

        for (i, score) in emotion_scores.iter().enumerate() {
            let score_f32 = *score as f32;
            if score_f32 > max_score {
                max_score = score_f32;
                max_idx = i;
            }
        }

        let emotion = BasicEmotion::from_index(max_idx).unwrap_or(BasicEmotion::Sadness);
        let confidence = (max_score / 10.0).min(1.0);

        (emotion, confidence)
    }

    fn count_keywords(text: &str, keywords: &[&str]) -> usize {
        keywords.iter().filter(|kw| text.contains(*kw)).count()
    }

    /// Detect emotion from voice/audio features
    pub fn detect_from_voice(&self, pitch: f32, intensity: f32, speaking_rate: f32) -> (BasicEmotion, f32) {
        let mut scores = [0.0f32; 7];

        // High pitch + high intensity + fast speech = Happiness/Surprise
        if pitch > 200.0 && intensity > 0.7 {
            scores[BasicEmotion::Happiness.to_index()] = 0.7;
            scores[BasicEmotion::Surprise.to_index()] = 0.5;
        }

        // Low pitch + low intensity + slow speech = Sadness
        if pitch < 150.0 && intensity < 0.4 && speaking_rate < 0.5 {
            scores[BasicEmotion::Sadness.to_index()] = 0.8;
        }

        // High pitch + high intensity + irregular = Anger
        if intensity > 0.8 && speaking_rate > 0.7 {
            scores[BasicEmotion::Anger.to_index()] = 0.7;
        }

        // High pitch variability + trembling = Fear
        if pitch > 180.0 && speaking_rate > 0.6 {
            scores[BasicEmotion::Fear.to_index()] = 0.6;
        }

        // Find dominant
        let max_idx = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        let emotion = BasicEmotion::from_index(max_idx).unwrap_or(BasicEmotion::Sadness);
        let confidence = scores[max_idx];

        (emotion, confidence)
    }

    /// Detect emotion from facial features
    pub fn detect_from_face(&self, smile_intensity: f32, brow_furrow: f32, eye_widening: f32) -> (BasicEmotion, f32) {
        let mut scores = [0.0f32; 7];

        // Smile = Happiness
        scores[BasicEmotion::Happiness.to_index()] = smile_intensity;

        // Brow furrow + lowered eyebrows = Sadness/Anger
        if brow_furrow > 0.5 {
            scores[BasicEmotion::Sadness.to_index()] = 0.6;
            scores[BasicEmotion::Anger.to_index()] = 0.4;
        }

        // Eye widening = Fear/Surprise
        if eye_widening > 0.5 {
            scores[BasicEmotion::Surprise.to_index()] = 0.5;
            scores[BasicEmotion::Fear.to_index()] = 0.5;
        }

        // Find dominant
        let max_idx = scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        let emotion = BasicEmotion::from_index(max_idx).unwrap_or(BasicEmotion::Happiness);
        let confidence = scores[max_idx];

        (emotion, confidence)
    }

    /// Fuse multiple emotion detection sources
    pub fn fuse_emotions(&self, detections: Vec<(BasicEmotion, f32, &str)>) -> (BasicEmotion, f32) {
        if detections.is_empty() {
            return (BasicEmotion::Neutral, 0.0);
        }

        // Weighted voting
        let mut weighted_scores = [0.0f32; 7];

        for (emotion, confidence, source) in detections {
            let weight = self.confidence_weights.get(source).copied().unwrap_or(0.33);
            let idx = emotion.to_index();
            weighted_scores[idx] += confidence * weight;
        }

        // Find winner
        let max_idx = weighted_scores
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        let emotion = BasicEmotion::from_index(max_idx).unwrap_or(BasicEmotion::Neutral);
        let confidence = weighted_scores[max_idx];

        (emotion, confidence)
    }
}

/// Emotion regulator - manages emotional responses and regulation strategies
pub struct EmotionRegulator {
    strategies: HashMap<RegulationStrategy, f32>,
    target_valence: f32,
    target_arousal: f32,
    regulation_strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RegulationStrategy {
    Suppression,      // Reduce emotional expression
    Reappraisal,      // Reinterpret situation
    Acceptance,       // Accept emotional state
    Attention,        // Shift attention away
    Rumination,       // Focus on negative (should be avoided)
    Distraction,      // Engage in different activity
    SocialSupport,    // Seek help from others
}

impl EmotionRegulator {
    pub fn new() -> Self {
        Self {
            strategies: HashMap::new(),
            target_valence: 0.3,
            target_arousal: 0.5,
            regulation_strength: 0.7,
        }
    }

    /// Regulate an emotional state towards target
    pub fn regulate(&self, state: &mut EmotionalState) {
        // Calculate regulation needed
        let valence_diff = self.target_valence - state.vad.valence;
        let arousal_diff = self.target_arousal - state.vad.arousal;

        // Apply regulation strategies
        let total_diff = (valence_diff.powi(2) + arousal_diff.powi(2)).sqrt();

        if total_diff < 0.1 {
            return; // Already close enough
        }

        // Determine appropriate strategy based on emotional state
        let dominant = state.dominant_emotion();

        let strategy = match dominant {
            BasicEmotion::Anger | BasicEmotion::Fear => RegulationStrategy::Suppression,
            BasicEmotion::Sadness => RegulationStrategy::Reappraisal,
            _ => RegulationStrategy::Acceptance,
        };

        // Apply strategy effects
        match strategy {
            RegulationStrategy::Suppression => {
                // Reduce arousal and expression
                for emotion in state.basic_emotions.iter_mut() {
                    *emotion *= (1.0 - self.regulation_strength * 0.3);
                }
                state.vad.arousal *= (1.0 - self.regulation_strength * 0.2);
            }
            RegulationStrategy::Reappraisal => {
                // Shift valence towards positive
                state.vad.valence += valence_diff * self.regulation_strength * 0.5;
            }
            RegulationStrategy::Acceptance => {
                // Keep emotional state but reduce regulation
                // Just maintain current state
            }
            RegulationStrategy::Attention => {
                // Reduce intensity
                for emotion in state.basic_emotions.iter_mut() {
                    *emotion *= (1.0 - self.regulation_strength * 0.2);
                }
            }
            RegulationStrategy::Distraction => {
                // Reduce all emotions towards neutral
                for emotion in state.basic_emotions.iter_mut() {
                    *emotion *= (1.0 - self.regulation_strength * 0.4);
                }
                state.vad = VADEmotion::neutral();
            }
            _ => {}
        }

        // Clamp values
        state.vad.valence = state.vad.valence.clamp(-1.0, 1.0);
        state.vad.arousal = state.vad.arousal.clamp(0.0, 1.0);
        state.vad.dominance = state.vad.dominance.clamp(0.0, 1.0);
    }

    /// Set target emotional state
    pub fn set_target(&mut self, valence: f32, arousal: f32) {
        self.target_valence = valence.clamp(-1.0, 1.0);
        self.target_arousal = arousal.clamp(0.0, 1.0);
    }

    /// Set regulation strength
    pub fn set_strength(&mut self, strength: f32) {
        self.regulation_strength = strength.clamp(0.0, 1.0);
    }
}

/// Empathy engine - understanding and responding to others' emotions
pub struct EmpathyEngine {
    perspective_taking: bool,
    emotional_contagion: f32,
    compassionate_threshold: f32,
}

impl EmpathyEngine {
    pub fn new() -> Self {
        Self {
            perspective_taking: true,
            emotional_contagion: 0.4,
            compassionate_threshold: 0.5,
        }
    }

    /// Understand another entity's emotional state
    pub fn understand_others_emotion(&self, observed_state: &EmotionalState) -> EmpathyResponse {
        // Calculate empathy intensity based on own current state
        EmpathyResponse {
            empathic_accuracy: 0.7, // Simplified
            emotional_resonance: self.calculate_resonance(observed_state),
            compassion_triggered: self.should_show_compassion(observed_state),
            suggested_response: self.suggest_response(observed_state),
        }
    }

    fn calculate_resonance(&self, other: &EmotionalState) -> f32 {
        // Simplified resonance calculation
        let avg_emotion = other.basic_emotions.iter().sum::<f32>() / 6.0;
        avg_emotion * self.emotional_contagion
    }

    fn should_show_compassion(&self, other: &EmotionalState) -> bool {
        let vad = &other.vad;
        (vad.valence < -0.3 || other.dominant_emotion() == BasicEmotion::Sadness)
            && vad.arousal < 0.5
    }

    fn suggest_response(&self, other: &EmotionalState) -> EmpathyResponseType {
        match other.dominant_emotion() {
            BasicEmotion::Happiness => EmpathyResponseType::Celebrate,
            BasicEmotion::Sadness => EmpathyResponseType::Comfort,
            BasicEmotion::Anger => EmpathyResponseType::Deescalate,
            BasicEmotion::Fear => EmpathyResponseType::Reassure,
            BasicEmotion::Surprise => EmpathyResponseType::Engage,
            BasicEmotion::Disgust => EmpathyResponseType::Distance,
            BasicEmotion::Neutral => EmpathyResponseType::Engage,
        }
    }

    /// Simulate emotional contagion from another entity
    pub fn apply_contagion(&self, other_state: &EmotionalState, self_state: &mut EmotionalState) {
        let resonance = self.calculate_resonance(other_state);

        for (i, emotion) in self_state.basic_emotions.iter_mut().enumerate() {
            let other_emotion = other_state.basic_emotions[i];
            *emotion = *emotion * (1.0 - resonance) + other_emotion * resonance;
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpathyResponse {
    pub empathic_accuracy: f32,
    pub emotional_resonance: f32,
    pub compassion_triggered: bool,
    pub suggested_response: EmpathyResponseType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EmpathyResponseType {
    Celebrate,
    Comfort,
    Deescalate,
    Reassure,
    Engage,
    Distance,
    Neutral,
}

/// Emotional memory - stores emotional experiences
pub struct EmotionalMemory {
    episodes: VecDeque<EmotionalEpisode>,
    max_episodes: usize,
    consolidation_threshold: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalEpisode {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub stimulus: String,
    pub emotional_state: EmotionalState,
    pub intensity: f32,
    pub regulation_applied: bool,
    pub outcome: String,
    pub learned: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Uuid([u8; 16]);

impl Uuid {
    pub fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut uuid = [0u8; 16];
        let bytes = now.to_le_bytes();
        uuid[..8].copy_from_slice(&bytes[..8]);
        uuid[8..].copy_from_slice(&now.to_be_bytes()[..8]);
        Uuid(uuid)
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

impl EmotionalMemory {
    pub fn new(max_episodes: usize) -> Self {
        Self {
            episodes: VecDeque::new(),
            max_episodes,
            consolidation_threshold: 50,
        }
    }

    /// Store a new emotional episode
    pub fn store(&mut self, stimulus: &str, state: &EmotionalState, outcome: &str) {
        let episode = EmotionalEpisode {
            id: Uuid::new(),
            timestamp: Utc::now(),
            stimulus: stimulus.to_string(),
            emotional_state: state.clone(),
            intensity: state.total_intensity(),
            regulation_applied: false,
            outcome: outcome.to_string(),
            learned: false,
        };

        self.episodes.push_back(episode);

        while self.episodes.len() > self.max_episodes {
            self.episodes.pop_front();
        }
    }

    /// Retrieve similar emotional memories
    pub fn retrieve_similar(&self, state: &EmotionalState, limit: usize) -> Vec<&EmotionalEpisode> {
        let mut similarities: Vec<_> = self.episodes
            .iter()
            .map(|ep| {
                let distance = state.vad.euclidean_distance(&ep.emotional_state.vad);
                (ep, distance)
            })
            .collect();

        similarities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        similarities.into_iter()
            .take(limit)
            .map(|(ep, _)| ep)
            .collect()
    }

    /// Get emotional patterns over time
    pub fn analyze_patterns(&self) -> EmotionalPatterns {
        if self.episodes.is_empty() {
            return EmotionalPatterns::default();
        }

        let mut valence_trend = Vec::new();
        let mut arousal_trend = Vec::new();
        let mut emotion_frequencies = [0usize; 6];

        for episode in &self.episodes {
            valence_trend.push(episode.emotional_state.vad.valence);
            arousal_trend.push(episode.emotional_state.vad.arousal);

            let dominant = episode.emotional_state.dominant_emotion();
            emotion_frequencies[dominant.to_index()] += 1;
        }

        EmotionalPatterns {
            valence_trend,
            arousal_trend,
            emotion_frequencies,
            total_episodes: self.episodes.len(),
        }
    }

    /// Consolidate recent episodes into long-term memory
    pub fn consolidate(&mut self) {
        if self.episodes.len() >= self.consolidation_threshold {
            for episode in self.episodes.iter_mut() {
                if !episode.learned {
                    episode.learned = true;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmotionalPatterns {
    pub valence_trend: Vec<f32>,
    pub arousal_trend: Vec<f32>,
    pub emotion_frequencies: [usize; 6],
    pub total_episodes: usize,
}

// ============================================================================
// Main Emotional Architecture
// ============================================================================

/// Complete emotional architecture system
pub struct EmotionalArchitecture {
    // Core components
    current_state: EmotionalState,
    detector: EmotionDetector,
    regulator: EmotionRegulator,
    empathy: EmpathyEngine,
    memory: EmotionalMemory,

    // Mood dynamics
    mood: MoodState,
    mood_transitions: Vec<MoodTransition>,
    mood_decay_rate: f32,

    // Affective computing
    sentiment_analyzer: SentimentAnalyzer,

    // Configuration
    config: EmotionalConfig,

    // History
    state_history: VecDeque<EmotionalState>,
    max_history: usize,
}

#[derive(Debug, Clone)]
pub struct EmotionalConfig {
    pub emotional_inertia: f32,        // How slowly emotions change
    pub mood_baseline: f32,            // Default mood level
    pub empathy_strength: f32,         // How much we absorb others' emotions
    pub regulation_aggressiveness: f32,
    pub emotional_expressiveness: f32, // How much emotions affect behavior
    pub memory_consolidation_interval: Duration,
}

impl Default for EmotionalConfig {
    fn default() -> Self {
        Self {
            emotional_inertia: 0.3,
            mood_baseline: 0.0,
            empathy_strength: 0.4,
            regulation_aggressiveness: 0.5,
            emotional_expressiveness: 0.7,
            memory_consolidation_interval: Duration::from_secs(300),
        }
    }
}

/// Sentiment analysis for text
pub struct SentimentAnalyzer {
    positive_words: Vec<String>,
    negative_words: Vec<String>,
    intensifiers: Vec<String>,
    negators: Vec<String>,
}

impl SentimentAnalyzer {
    pub fn new() -> Self {
        Self {
            positive_words: vec![
                "good", "great", "excellent", "amazing", "wonderful", "fantastic",
                "happy", "joy", "love", "beautiful", "perfect", "best",
                "awesome", "brilliant", "outstanding", "superb", "terrific"
            ].into_iter().map(|s| s.to_string()).collect(),
            negative_words: vec![
                "bad", "terrible", "awful", "horrible", "poor", "disappointing",
                "sad", "angry", "hate", "ugly", "worst", "failure",
                "pathetic", "useless", "dreadful", "disgusting", "miserable"
            ].into_iter().map(|s| s.to_string()).collect(),
            intensifiers: vec![
                "very", "really", "extremely", "absolutely", "completely",
                "totally", "highly", "incredibly", "particularly"
            ].into_iter().map(|s| s.to_string()).collect(),
            negators: vec![
                "not", "no", "never", "neither", "nor", "don't", "doesn't",
                "didn't", "won't", "wouldn't", "couldn't", "shouldn't"
            ].into_iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Analyze sentiment of text
    pub fn analyze(&self, text: &str) -> SentimentResult {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut score = 0.0f32;
        let mut positive_count = 0usize;
        let mut negative_count = 0usize;
        let mut negation_active = false;
        let mut intensity_boost = 1.0f32;

        for (i, word) in words.iter().enumerate() {
            let word_lower = word.to_lowercase();

            // Check for intensifiers
            if self.intensifiers.contains(&word_lower) {
                intensity_boost = 1.5;
                continue;
            }

            // Check for negators
            if self.negators.contains(&word_lower) {
                negation_active = true;
                continue;
            }

            // Check positive words
            if self.positive_words.contains(&word_lower) {
                let contribution = 1.0 * intensity_boost;
                if negation_active {
                    score -= contribution;
                    negative_count += 1;
                } else {
                    score += contribution;
                    positive_count += 1;
                }
                negation_active = false;
                intensity_boost = 1.0;
            }
            // Check negative words
            else if self.negative_words.contains(&word_lower) {
                let contribution = 1.0 * intensity_boost;
                if negation_active {
                    score += contribution;
                    positive_count += 1;
                } else {
                    score -= contribution;
                    negative_count += 1;
                }
                negation_active = false;
                intensity_boost = 1.0;
            }

            // Reset negation after a few words
            if i > 2 {
                negation_active = false;
            }
        }

        // Normalize score
        let total_words = words.len().max(1) as f32;
        let normalized_score = (score / total_words).clamp(-1.0, 1.0);

        // Determine polarity
        let polarity = if normalized_score > 0.1 {
            SentimentPolarity::Positive
        } else if normalized_score < -0.1 {
            SentimentPolarity::Negative
        } else {
            SentimentPolarity::Neutral
        };

        // Calculate confidence
        let confidence = ((positive_count + negative_count) as f32 / total_words)
            .min(1.0);

        SentimentResult {
            polarity,
            score: normalized_score,
            confidence,
            positive_words: positive_count,
            negative_words: negative_count,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentimentResult {
    pub polarity: SentimentPolarity,
    pub score: f32,        // -1.0 to 1.0
    pub confidence: f32,   // 0.0 to 1.0
    pub positive_words: usize,
    pub negative_words: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SentimentPolarity {
    Positive,
    Neutral,
    Negative,
}

impl EmotionalArchitecture {
    pub fn new() -> Self {
        Self {
            current_state: EmotionalState::new(),
            detector: EmotionDetector::new(),
            regulator: EmotionRegulator::new(),
            empathy: EmpathyEngine::new(),
            memory: EmotionalMemory::new(1000),
            mood: MoodState::Neutral,
            mood_transitions: MoodTransition::standard_transitions(),
            mood_decay_rate: 0.1,
            sentiment_analyzer: SentimentAnalyzer::new(),
            config: EmotionalConfig::default(),
            state_history: VecDeque::new(),
            max_history: 100,
        }
    }

    pub fn with_config(config: EmotionalConfig) -> Self {
        Self {
            current_state: EmotionalState::new(),
            detector: EmotionDetector::new(),
            regulator: EmotionRegulator::new(),
            empathy: EmpathyEngine::new(),
            memory: EmotionalMemory::new(1000),
            mood: MoodState::Neutral,
            mood_transitions: MoodTransition::standard_transitions(),
            mood_decay_rate: 0.1,
            sentiment_analyzer: SentimentAnalyzer::new(),
            config,
            state_history: VecDeque::new(),
            max_history: 100,
        }
    }

    /// Process emotional input and update state
    pub fn process_emotional_input(&mut self, input: EmotionalInput) -> &EmotionalState {
        match input {
            EmotionalInput::Text(text) => {
                let (emotion, confidence) = self.detector.detect_from_text(&text);
                self.activate_emotion(emotion, confidence);

                // Also run sentiment analysis
                let sentiment = self.sentiment_analyzer.analyze(&text);
                self.apply_sentiment(sentiment);
            }
            EmotionalInput::Voice { pitch, intensity, rate } => {
                let (emotion, confidence) = self.detector.detect_from_voice(pitch, intensity, rate);
                self.activate_emotion(emotion, confidence);
            }
            EmotionalInput::Facial { smile, brow, eyes } => {
                let (emotion, confidence) = self.detector.detect_from_face(smile, brow, eyes);
                self.activate_emotion(emotion, confidence);
            }
            EmotionalInput::DirectEmotion(emotion, intensity) => {
                self.activate_emotion(emotion, intensity);
            }
            EmotionalInput::VAD(vad) => {
                self.current_state.vad = vad;
                // Initialize all basic emotions to neutral when using VAD input
                for i in 0..self.current_state.basic_emotions.len() {
                    self.current_state.basic_emotions[i] = 0.5;
                }
            }
        }

        // Apply regulation
        self.regulator.regulate(&mut self.current_state);

        // Update mood based on current state
        self.update_mood();

        // Store in history
        self.state_history.push_back(self.current_state.clone());
        if self.state_history.len() > self.max_history {
            self.state_history.pop_front();
        }

        &self.current_state
    }

    /// Activate a specific emotion
    fn activate_emotion(&mut self, emotion: BasicEmotion, intensity: f32) {
        let idx = emotion.to_index();

        // Apply emotional inertia
        let current = self.current_state.basic_emotions[idx];
        let new_value = current * (1.0 - self.config.emotional_inertia)
            + intensity * self.config.emotional_inertia;

        self.current_state.basic_emotions[idx] = new_value.clamp(0.0, 1.0);

        // Update VAD based on emotion
        let vad = VADEmotion::from_basic(emotion);
        self.current_state.vad.valence = self.current_state.vad.valence * (1.0 - intensity * 0.5)
            + vad.valence * intensity * 0.5;
        self.current_state.vad.arousal = self.current_state.vad.arousal * (1.0 - intensity * 0.5)
            + vad.arousal * intensity * 0.5;

        // Update timestamp
        self.current_state.timestamp = Utc::now();
    }

    /// Apply sentiment analysis result
    fn apply_sentiment(&mut self, sentiment: SentimentResult) {
        // Map sentiment to emotional state
        match sentiment.polarity {
            SentimentPolarity::Positive => {
                self.activate_emotion(BasicEmotion::Happiness, sentiment.score.abs() * sentiment.confidence);
            }
            SentimentPolarity::Negative => {
                self.activate_emotion(BasicEmotion::Sadness, sentiment.score.abs() * sentiment.confidence);
            }
            SentimentPolarity::Neutral => {}
        }
    }

    /// Update mood based on current emotional state
    fn update_mood(&mut self) {
        let new_mood = MoodState::from_vad(&self.current_state.vad);

        if new_mood != self.mood {
            // Check transition probability
            let transition_exists = self.mood_transitions.iter()
                .any(|t| t.from == self.mood && t.to == new_mood);

            if transition_exists || self.mood_decay_rate > 0.5 {
                self.mood = new_mood;
            }
        }

        self.current_state.mood = self.mood;
    }

    /// Experience empathy for another entity's emotions
    pub fn experience_empathy(&mut self, other_state: &EmotionalState) -> EmpathyResponse {
        let response = self.empathy.understand_others_emotion(other_state);

        // Apply emotional contagion
        self.empathy.apply_contagion(other_state, &mut self.current_state);

        response
    }

    /// Store current emotional state in memory
    pub fn store_memory(&mut self, stimulus: &str, outcome: &str) {
        self.memory.store(stimulus, &self.current_state, outcome);
    }

    /// Retrieve similar emotional memories
    pub fn retrieve_similar_experiences(&self, limit: usize) -> Vec<&EmotionalEpisode> {
        self.memory.retrieve_similar(&self.current_state, limit)
    }

    /// Get emotional patterns
    pub fn analyze_emotional_patterns(&self) -> EmotionalPatterns {
        self.memory.analyze_patterns()
    }

    /// Get current emotional state
    pub fn get_state(&self) -> &EmotionalState {
        &self.current_state
    }

    /// Get current mood
    pub fn get_mood(&self) -> MoodState {
        self.mood
    }

    /// Get emotional history
    pub fn get_history(&self) -> Vec<&EmotionalState> {
        self.state_history.iter().collect()
    }

    /// Get affective state as VAD
    pub fn get_vad(&self) -> VADEmotion {
        self.current_state.vad.clone()
    }

    /// Reset emotional state
    pub fn reset(&mut self) {
        self.current_state = EmotionalState::new();
        self.mood = MoodState::Neutral;
        self.state_history.clear();
    }

    /// Generate emotional expression based on current state
    pub fn generate_expression(&self) -> EmotionalExpression {
        let dominant = self.current_state.dominant_emotion();
        let intensity = self.current_state.basic_emotions[dominant.to_index()];

        EmotionalExpression {
            emotion: dominant,
            intensity,
            vad: self.current_state.vad.clone(),
            mood: self.mood,
            suggested_action: self.suggest_action(dominant, intensity),
        }
    }

    fn suggest_action(&self, emotion: BasicEmotion, intensity: f32) -> Option<EmotionalAction> {
        if intensity < 0.3 {
            return None;
        }

        match emotion {
            BasicEmotion::Happiness => Some(EmotionalAction::Share),
            BasicEmotion::Sadness => Some(EmotionalAction::SeekComfort),
            BasicEmotion::Anger => Some(EmotionalAction::Deescalate),
            BasicEmotion::Fear => Some(EmotionalAction::Retreat),
            BasicEmotion::Surprise => Some(EmotionalAction::Investigate),
            BasicEmotion::Disgust => Some(EmotionalAction::Avoid),
            BasicEmotion::Neutral => None,
        }
    }

    /// Set emotional target state
    pub fn set_target(&mut self, valence: f32, arousal: f32) {
        self.regulator.set_target(valence, arousal);
    }

    /// Adjust emotional expressiveness
    pub fn set_expressiveness(&mut self, level: f32) {
        self.config.emotional_expressiveness = level.clamp(0.0, 1.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalExpression {
    pub emotion: BasicEmotion,
    pub intensity: f32,
    pub vad: VADEmotion,
    pub mood: MoodState,
    pub suggested_action: Option<EmotionalAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionalAction {
    Share,
    SeekComfort,
    Deescalate,
    Retreat,
    Investigate,
    Avoid,
}

/// Input types for emotional processing
#[derive(Debug, Clone)]
pub enum EmotionalInput {
    Text(String),
    Voice { pitch: f32, intensity: f32, rate: f32 },
    Facial { smile: f32, brow: f32, eyes: f32 },
    DirectEmotion(BasicEmotion, f32),
    VAD(VADEmotion),
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_emotion_index() {
        assert_eq!(BasicEmotion::Happiness.to_index(), 0);
        assert_eq!(BasicEmotion::Sadness.to_index(), 1);
        assert_eq!(BasicEmotion::Anger.to_index(), 2);
    }

    #[test]
    fn test_vad_creation() {
        let vad = VADEmotion::new(0.5, 0.7, 0.6);
        assert!((vad.valence - 0.5).abs() < 0.001);
        assert!((vad.arousal - 0.7).abs() < 0.001);
        assert!((vad.dominance - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_vad_clamping() {
        let vad = VADEmotion::new(1.5, 1.2, 2.0);
        assert_eq!(vad.valence, 1.0);
        assert_eq!(vad.arousal, 1.0);
        assert_eq!(vad.dominance, 1.0);
    }

    #[test]
    fn test_mood_from_vad() {
        let joyful = VADEmotion::new(0.8, 0.8, 0.7);
        assert_eq!(MoodState::from_vad(&joyful), MoodState::Excited);

        let sad = VADEmotion::new(-0.6, 0.3, 0.2);
        assert_eq!(MoodState::from_vad(&sad), MoodState::Sad);
    }

    #[test]
    fn test_emotion_detector_text() {
        let detector = EmotionDetector::new();
        let (emotion, confidence) = detector.detect_from_text("I am so happy and joyful today!");

        assert_eq!(emotion, BasicEmotion::Happiness);
        assert!(confidence > 0.0);
    }

    #[test]
    fn test_emotion_detector_sadness() {
        let detector = EmotionDetector::new();
        let (emotion, confidence) = detector.detect_from_text("I feel so sad and disappointed");

        assert_eq!(emotion, BasicEmotion::Sadness);
        assert!(confidence > 0.0);
    }

    #[test]
    fn test_sentiment_analyzer() {
        let analyzer = SentimentAnalyzer::new();
        let result = analyzer.analyze("This is a great and wonderful experience");

        assert_eq!(result.polarity, SentimentPolarity::Positive);
        assert!(result.score > 0.0);
    }

    #[test]
    fn test_sentiment_with_negation() {
        let analyzer = SentimentAnalyzer::new();
        let result = analyzer.analyze("This is not good at all");

        assert_eq!(result.polarity, SentimentPolarity::Negative);
        assert!(result.score < 0.0);
    }

    #[test]
    fn test_emotional_architecture() {
        let mut emotion = EmotionalArchitecture::new();

        // Process happy input
        emotion.process_emotional_input(EmotionalInput::Text("I am so happy!".to_string()));

        let state = emotion.get_state();
        assert_eq!(state.dominant_emotion(), BasicEmotion::Happiness);
    }

    #[test]
    fn test_empathy_engine() {
        let empathy = EmpathyEngine::new();
        let other_state = EmotionalState::new()
            .with_emotion(BasicEmotion::Sadness, 0.8);

        let response = empathy.understand_others_emotion(&other_state);
        assert!(response.compassion_triggered);
    }

    #[test]
    fn test_emotional_memory() {
        let mut memory = EmotionalMemory::new(10);

        let state = EmotionalState::new()
            .with_emotion(BasicEmotion::Happiness, 0.8);

        memory.store("Test stimulus", &state, "Good outcome");

        let similar = memory.retrieve_similar(&state, 5);
        assert!(!similar.is_empty());
    }

    #[test]
    fn test_emotional_regulation() {
        let mut state = EmotionalState::new()
            .with_emotion(BasicEmotion::Anger, 0.9);

        let regulator = EmotionRegulator::new();
        regulator.regulate(&mut state);

        // Anger should be reduced after regulation
        assert!(state.basic_emotions[BasicEmotion::Anger.to_index()] < 0.9);
    }

    #[test]
    fn test_uuid_uniqueness() {
        let uuid1 = Uuid::new();
        let uuid2 = Uuid::new();
        assert_ne!(uuid1.0, uuid2.0);
    }
}
