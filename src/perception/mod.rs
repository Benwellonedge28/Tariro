//! Tariro Perception Module
//!
//! Handles sensory input processing, transforming raw data into
//! structured mental representations for cognitive processing.
//!
//! This module implements the perception layer of Tariro's cognitive architecture,
//! converting text input into meaningful perceptual experiences.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for perceptions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PerceptionId(pub Uuid);

impl PerceptionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for PerceptionId {
    fn default() -> Self {
        Self::new()
    }
}

/// Perceptual modality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Modality {
    /// Text input
    Text,
    /// Structured data
    Structured,
    /// System events
    System,
}

impl Modality {
    pub fn as_str(&self) -> &'static str {
        match self {
            Modality::Text => "text",
            Modality::Structured => "structured",
            Modality::System => "system",
        }
    }
}

/// Raw perceptual input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceptualInput {
    pub id: PerceptionId,
    pub modality: Modality,
    pub raw_content: String,
    pub timestamp: chrono::DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub confidence: f64,
}

impl PerceptualInput {
    pub fn new(modality: Modality, content: String) -> Self {
        Self {
            id: PerceptionId::new(),
            modality,
            raw_content: content,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            confidence: 1.0,
        }
    }

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_confidence(mut self, confidence: f64) -> Self {
        self.confidence = confidence.clamp(0.0, 1.0);
        self
    }
}

/// Processed perceptual content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedPerception {
    pub id: PerceptionId,
    pub modality: Modality,
    pub timestamp: chrono::DateTime<Utc>,
    pub tokens: Vec<Token>,
    pub entities: Vec<PerceivedEntity>,
    pub relations: Vec<PerceivedRelation>,
    pub intent: Option<Intent>,
    pub sentiment: Option<Sentiment>,
    pub confidence: f64,
    pub processing_info: ProcessingInfo,
}

/// Token representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub text: String,
    pub position: usize,
    pub token_type: TokenType,
    pub lemmatized: Option<String>,
    pub attributes: HashMap<String, String>,
}

impl Token {
    pub fn new(text: &str, position: usize) -> Self {
        Self {
            text: text.to_string(),
            position,
            token_type: TokenType::Word,
            lemmatized: None,
            attributes: HashMap::new(),
        }
    }
}

/// Token types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Word,
    Number,
    Punctuation,
    Symbol,
    Whitespace,
    Unknown,
}

/// Entity extracted from perception
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceivedEntity {
    pub name: String,
    pub entity_type: EntityType,
    pub start_pos: usize,
    pub end_pos: usize,
    pub confidence: f64,
    pub properties: HashMap<String, String>,
}

impl PerceivedEntity {
    pub fn new(name: &str, entity_type: EntityType, start: usize, end: usize) -> Self {
        Self {
            name: name.to_string(),
            entity_type,
            start_pos: start,
            end_pos: end,
            confidence: 0.8,
            properties: HashMap::new(),
        }
    }
}

/// Entity types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    /// Person name
    Person,
    /// Organization
    Organization,
    /// Location
    Location,
    /// Date/time
    DateTime,
    /// Number/quantity
    Number,
    /// Concept
    Concept,
    /// Action verb
    Action,
    /// Object
    Object,
    /// Unknown
    Unknown,
}

/// Relation between entities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerceivedRelation {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub confidence: f64,
}

/// Intent classification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub category: IntentCategory,
    pub confidence: f64,
    pub parameters: HashMap<String, String>,
}

impl Intent {
    pub fn new(category: IntentCategory, confidence: f64) -> Self {
        Self {
            category,
            confidence,
            parameters: HashMap::new(),
        }
    }
}

/// Intent categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentCategory {
    Question,
    Statement,
    Command,
    Greeting,
    Farewell,
    Request,
    Assertion,
    Agreement,
    Disagreement,
    Clarification,
    Unknown,
}

impl IntentCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            IntentCategory::Question => "question",
            IntentCategory::Statement => "statement",
            IntentCategory::Command => "command",
            IntentCategory::Greeting => "greeting",
            IntentCategory::Farewell => "farewell",
            IntentCategory::Request => "request",
            IntentCategory::Assertion => "assertion",
            IntentCategory::Agreement => "agreement",
            IntentCategory::Disagreement => "disagreement",
            IntentCategory::Clarification => "clarification",
            IntentCategory::Unknown => "unknown",
        }
    }
}

/// Sentiment analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sentiment {
    pub valence: f64,      // -1.0 (negative) to 1.0 (positive)
    pub arousal: f64,      // 0.0 (calm) to 1.0 (excited)
    pub dominance: f64,    // 0.0 (submissive) to 1.0 (dominant)
    pub emotion: Option<EmotionCategory>,
}

impl Default for Sentiment {
    fn default() -> Self {
        Self {
            valence: 0.0,
            arousal: 0.0,
            dominance: 0.5,
            emotion: None,
        }
    }
}

/// Emotion categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmotionCategory {
    Joy,
    Sadness,
    Anger,
    Fear,
    Surprise,
    Disgust,
    Trust,
    Anticipation,
    Neutral,
}

/// Processing metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProcessingInfo {
    pub processing_time_ms: u64,
    pub pipeline_stages: Vec<String>,
    pub warnings: Vec<String>,
}

// ============================================================================
// Text Perception
// ============================================================================

/// Text tokenizer
pub struct Tokenizer;

impl Tokenizer {
    /// Simple whitespace + punctuation tokenizer
    pub fn tokenize(text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut position = 0;

        let mut chars = text.char_indices().peekable();

        while let Some((start, ch)) = chars.next() {
            let token_text = if ch.is_whitespace() {
                // Collect whitespace
                let mut end = start;
                while let Some(&(idx, c)) = chars.peek() {
                    if c.is_whitespace() {
                        chars.next();
                        end = idx + c.len_utf8();
                    } else {
                        break;
                    }
                }
                TokenType::Whitespace
            } else if ch.is_alphabetic() {
                // Collect word
                let mut end = start + ch.len_utf8();
                while let Some(&(idx, c)) = chars.peek() {
                    if c.is_alphanumeric() || c == '\'' || c == '-' {
                        chars.next();
                        end = idx + c.len_utf8();
                    } else {
                        break;
                    }
                }
                TokenType::Word
            } else if ch.is_numeric() {
                // Collect number
                let mut end = start + ch.len_utf8();
                while let Some(&(idx, c)) = chars.peek() {
                    if c.is_numeric() || c == '.' || c == ',' {
                        chars.next();
                        end = idx + c.len_utf8();
                    } else {
                        break;
                    }
                }
                TokenType::Number
            } else if ch.is_ascii_punctuation() {
                TokenType::Punctuation
            } else {
                TokenType::Symbol
            };

            let token_str = &text[start..end];
            if token_str.chars().any(|c| !c.is_whitespace()) {
                tokens.push(Token::new(token_str, position));
            }
            position += 1;
        }

        tokens
    }

    /// Lemmatize a token (simple rule-based)
    pub fn lemmatize(token: &Token) -> Option<String> {
        let word = token.text.to_lowercase();

        // Simple suffix stripping
        let lemmas = [
            ("ing", ""),
            ("ed", ""),
            ("ies", "y"),
            ("es", ""),
            ("s", ""),
        ];

        for (suffix, replacement) in lemmas {
            if word.ends_with(suffix) && word.len() > suffix.len() + 2 {
                let mut lemma = word[..word.len() - suffix.len()].to_string();
                lemma.push_str(replacement);
                return Some(lemma);
            }
        }

        None
    }
}

/// Entity extractor
pub struct EntityExtractor;

impl EntityExtractor {
    /// Extract entities from tokens (rule-based)
    pub fn extract(tokens: &[Token], text: &str) -> Vec<PerceivedEntity> {
        let mut entities = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            let token = &tokens[i];

            if token.token_type == TokenType::Word {
                let word = token.text.to_lowercase();

                // Capitalized words at sentence start or after punctuation = Person/Location
                if token.text.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    let entity_type = if word == "i" {
                        EntityType::Person
                    } else if Self::is_location(&word) {
                        EntityType::Location
                    } else if Self::is_organization(&word) {
                        EntityType::Organization
                    } else {
                        EntityType::Unknown
                    };

                    // Try to get full entity span
                    let mut end_pos = i;
                    while end_pos + 1 < tokens.len()
                        && tokens[end_pos + 1].text.chars().next().map(|c| c.is_uppercase()).unwrap_or(false)
                        && ![".", "!", "?", ",", ";", ":", "-"].contains(&tokens[end_pos + 1].text.as_str())
                    {
                        end_pos += 1;
                    }

                    if end_pos > i || entity_type != EntityType::Unknown {
                        let start = tokens[i].position;
                        let end = tokens[end_pos].position + tokens[end_pos].text.len();
                        let name = &text[start..end];

                        entities.push(PerceivedEntity::new(
                            name,
                            entity_type,
                            start,
                            end,
                        ));
                    }
                }

                // Numbers with units
                if i > 0 && tokens[i - 1].token_type == TokenType::Number {
                    let entity = PerceivedEntity::new(
                        &format!("{} {}", tokens[i - 1].text, token.text),
                        EntityType::Number,
                        tokens[i - 1].position,
                        token.position + token.text.len(),
                    );
                    entities.push(entity);
                }

                // Common action verbs
                if Self::is_action(&word) {
                    entities.push(PerceivedEntity::new(
                        &token.text,
                        EntityType::Action,
                        token.position,
                        token.position + token.text.len(),
                    ));
                }

                // Common concepts
                if Self::is_concept(&word) {
                    entities.push(PerceivedEntity::new(
                        &token.text,
                        EntityType::Concept,
                        token.position,
                        token.position + token.text.len(),
                    ));
                }
            }

            i += 1;
        }

        entities
    }

    fn is_location(word: &str) -> bool {
        let locations = [
            "city", "country", "state", "province", "town", "village",
            "street", "road", "avenue", "building", "room", "floor",
            "north", "south", "east", "west", "center", "park",
        ];
        locations.contains(&word)
    }

    fn is_organization(word: &str) -> bool {
        let orgs = [
            "company", "inc", "corp", "llc", "ltd", "university",
            "college", "school", "hospital", "government", "agency",
        ];
        orgs.contains(&word)
    }

    fn is_action(word: &str) -> bool {
        let actions = [
            "run", "walk", "eat", "drink", "sleep", "think", "know",
            "want", "need", "create", "destroy", "find", "give",
            "take", "make", "do", "say", "see", "hear", "feel",
        ];
        actions.contains(&word)
    }

    fn is_concept(word: &str) -> bool {
        let concepts = [
            "love", "hate", "fear", "hope", "idea", "theory",
            "knowledge", "wisdom", "truth", "beauty", "justice",
            "freedom", "right", "wrong", "good", "evil",
        ];
        concepts.contains(&word)
    }
}

/// Relation extractor
pub struct RelationExtractor;

impl RelationExtractor {
    /// Extract subject-predicate-object relations
    pub fn extract(tokens: &[Token], entities: &[PerceivedEntity], text: &str) -> Vec<PerceivedRelation> {
        let mut relations = Vec::new();

        // Common predicate patterns
        let predicate_patterns = [
            ("is", "a", "IsA"),
            ("is", "the", "IsA"),
            ("has", "a", "HasProperty"),
            ("has", "the", "HasProperty"),
            ("can", "", "CanDo"),
            ("does", "", "Does"),
            ("does not", "", "NotDoes"),
            ("is not", "", "IsNot"),
            ("causes", "", "Causes"),
            ("leads to", "", "LeadsTo"),
            ("results in", "", "ResultsIn"),
            ("enables", "", "Enables"),
            ("requires", "", "Requires"),
            ("part of", "", "PartOf"),
            ("located in", "", "LocatedIn"),
            ("belongs to", "", "BelongsTo"),
        ];

        // For each entity pair, check for relations
        for i in 0..entities.len() {
            for j in (i + 1)..entities.len() {
                let e1 = &entities[i];
                let e2 = &entities[j];

                // Get text between entities
                if e2.start_pos > e1.end_pos {
                    let between = &text[e1.end_pos..e2.start_pos].to_lowercase();

                    for (pred_start, pred_end, rel_type) in predicate_patterns {
                        if between.contains(pred_start) && (pred_end.is_empty() || between.contains(pred_end)) {
                            relations.push(PerceivedRelation {
                                subject: e1.name.clone(),
                                predicate: rel_type.to_string(),
                                object: e2.name.clone(),
                                confidence: 0.7,
                            });
                            break;
                        }
                    }
                }
            }
        }

        relations
    }
}

/// Intent classifier
pub struct IntentClassifier;

impl IntentClassifier {
    /// Classify intent from tokens and text
    pub fn classify(tokens: &[Token], text: &str) -> Intent {
        let text_lower = text.to_lowercase();
        let first_token = tokens.first().map(|t| t.text.to_lowercase()).unwrap_or_default();

        // Question detection
        if text.ends_with('?') || first_token == "what" || first_token == "who"
            || first_token == "where" || first_token == "when" || first_token == "why"
            || first_token == "how" || first_token == "is" || first_token == "are"
            || first_token == "do" || first_token == "does" || first_token == "can"
        {
            return Intent::new(IntentCategory::Question, 0.9);
        }

        // Command detection
        if first_token == "do" || first_token == "make" || first_token == "create"
            || first_token == "delete" || first_token == "remove" || first_token == "add"
            || first_token == "go" || first_token == "run" || first_token == "stop"
            || first_token == "start" || first_token == "finish" || first_token == "complete"
        {
            return Intent::new(IntentCategory::Command, 0.85);
        }

        // Greeting detection
        if first_token == "hello" || first_token == "hi" || first_token == "hey"
            || first_token == "greetings" || text_lower.contains("good morning")
            || text_lower.contains("good afternoon") || text_lower.contains("good evening")
        {
            return Intent::new(IntentCategory::Greeting, 0.95);
        }

        // Farewell detection
        if first_token == "goodbye" || first_token == "bye" || first_token == "farewell"
            || text_lower.contains("see you") || text_lower.contains("talk later")
        {
            return Intent::new(IntentCategory::Farewell, 0.9);
        }

        // Request detection
        if text_lower.contains("please") || text_lower.contains("could you")
            || text_lower.contains("would you") || text_lower.contains("can you")
        {
            return Intent::new(IntentCategory::Request, 0.8);
        }

        // Agreement/Disagreement
        if text_lower.contains("yes") || text_lower.contains("agree")
            || text_lower.contains("correct") || text_lower.contains("right")
        {
            return Intent::new(IntentCategory::Agreement, 0.8);
        }

        if text_lower.contains("no") || text_lower.contains("disagree")
            || text_lower.contains("wrong") || text_lower.contains("incorrect")
        {
            return Intent::new(IntentCategory::Disagreement, 0.8);
        }

        // Default to statement
        Intent::new(IntentCategory::Statement, 0.6)
    }
}

/// Sentiment analyzer
pub struct SentimentAnalyzer;

impl SentimentAnalyzer {
    /// Analyze sentiment from text
    pub fn analyze(text: &str) -> Sentiment {
        let text_lower = text.to_lowercase();

        // Positive words
        let positive_words = [
            "good", "great", "excellent", "wonderful", "amazing", "fantastic",
            "love", "like", "happy", "joy", "beautiful", "nice", "best",
            "perfect", "awesome", "brilliant", "superb", "positive", "yes",
        ];

        // Negative words
        let negative_words = [
            "bad", "terrible", "awful", "horrible", "hate", "dislike",
            "sad", "angry", "ugly", "worst", "poor", "fail", "wrong",
            "negative", "no", "never", "nothing", "nobody",
        ];

        // Intensifiers
        let intensifiers = ["very", "really", "extremely", "absolutely", "completely", "totally"];

        // Negators
        let negators = ["not", "never", "no", "don't", "doesn't", "didn't", "won't", "wouldn't"];

        let mut valence_score = 0.0;
        let mut word_count = 0;
        let mut negated = false;

        for word in text_lower.split_whitespace() {
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());

            if negators.contains(&clean_word) {
                negated = true;
                continue;
            }

            let intensity = if intensifiers.contains(&clean_word) { 1.5 } else { 1.0 };

            if positive_words.contains(&clean_word) {
                let score = 0.5 * intensity;
                valence_score += if negated { -score } else { score };
                word_count += 1;
                negated = false;
            } else if negative_words.contains(&clean_word) {
                let score = -0.5 * intensity;
                valence_score += if negated { -score } else { score };
                word_count += 1;
                negated = false;
            }
        }

        let valence = if word_count > 0 {
            (valence_score / word_count as f64).clamp(-1.0, 1.0)
        } else {
            0.0
        };

        // Arousal based on punctuation and caps
        let exclamation_count = text.chars().filter(|&c| c == '!').count();
        let question_count = text.chars().filter(|&c| c == '?').count();
        let caps_ratio = text.chars().filter(|c| c.is_uppercase()).count() as f64
            / text.len().max(1) as f64;

        let arousal = ((exclamation_count * 0.1) + (question_count * 0.05) + (caps_ratio * 0.3))
            .min(1.0);

        // Dominance based on confidence indicators
        let dominance = if text_lower.contains("should") || text_lower.contains("must")
            || text_lower.contains("need to") || text_lower.contains("have to")
        {
            0.7
        } else if text_lower.contains("might") || text_lower.contains("could")
            || text_lower.contains("perhaps") || text_lower.contains("maybe")
        {
            0.4
        } else {
            0.5
        };

        // Detect emotion
        let emotion = Self::detect_emotion(valence, arousal);

        Sentiment {
            valence,
            arousal,
            dominance,
            emotion,
        }
    }

    fn detect_emotion(valence: f64, arousal: f64) -> Option<EmotionCategory> {
        if valence > 0.3 && arousal > 0.5 {
            Some(EmotionCategory::Joy)
        } else if valence < -0.3 && arousal > 0.5 {
            Some(EmotionCategory::Anger)
        } else if valence < -0.3 && arousal < 0.5 {
            Some(EmotionCategory::Sadness)
        } else if valence.abs() < 0.2 && arousal > 0.6 {
            Some(EmotionCategory::Fear)
        } else if valence > 0.3 && arousal < 0.4 {
            Some(EmotionCategory::Trust)
        } else if valence < -0.3 && arousal > 0.3 {
            Some(EmotionCategory::Disgust)
        } else if valence > 0.0 && arousal > 0.3 {
            Some(EmotionCategory::Anticipation)
        } else if valence.abs() < 0.2 && arousal < 0.3 {
            Some(EmotionCategory::Neutral)
        } else {
            None
        }
    }
}

// ============================================================================
// Perception Engine
// ============================================================================

/// Main perception engine
#[derive(Debug, Clone)]
pub struct PerceptionEngine {
    config: PerceptionConfig,
    stats: PerceptionStats,
}

impl PerceptionEngine {
    pub fn new() -> Self {
        Self {
            config: PerceptionConfig::default(),
            stats: PerceptionStats::default(),
        }
    }

    pub fn with_config(config: PerceptionConfig) -> Self {
        Self {
            config,
            stats: PerceptionStats::default(),
        }
    }

    /// Process text input
    pub fn process_text(&mut self, text: String) -> ProcessedPerception {
        let start_time = std::time::Instant::now();

        // Tokenize
        let tokens = Tokenizer::tokenize(&text);

        // Lemmatize
        let mut lemmatized_tokens = tokens.clone();
        for token in &mut lemmatized_tokens {
            token.lemmatized = Tokenizer::lemmatize(token);
        }

        // Extract entities
        let entities = EntityExtractor::extract(&lemmatized_tokens, &text);

        // Extract relations
        let relations = RelationExtractor::extract(&lemmatized_tokens, &entities, &text);

        // Classify intent
        let intent = IntentClassifier::classify(&lemmatized_tokens, &text);

        // Analyze sentiment
        let sentiment = SentimentAnalyzer::analyze(&text);

        let processing_time = start_time.elapsed().as_millis() as u64;

        self.stats.inputs_processed += 1;
        self.stats.total_processing_time_ms += processing_time;

        ProcessedPerception {
            id: PerceptionId::new(),
            modality: Modality::Text,
            timestamp: Utc::now(),
            tokens: lemmatized_tokens,
            entities,
            relations,
            intent: Some(intent),
            sentiment: Some(sentiment),
            confidence: 0.8,
            processing_info: ProcessingInfo {
                processing_time_ms: processing_time,
                pipeline_stages: vec![
                    "tokenization".to_string(),
                    "lemmatization".to_string(),
                    "entity_extraction".to_string(),
                    "relation_extraction".to_string(),
                    "intent_classification".to_string(),
                    "sentiment_analysis".to_string(),
                ],
                warnings: Vec::new(),
            },
        }
    }

    /// Process structured input
    pub fn process_structured(&mut self, data: serde_json::Value) -> ProcessedPerception {
        let start_time = std::time::Instant::now();

        let text = data.to_string();
        let mut tokens = Tokenizer::tokenize(&text);
        let entities = EntityExtractor::extract(&tokens, &text);

        let processing_time = start_time.elapsed().as_millis() as u64;

        self.stats.inputs_processed += 1;
        self.stats.total_processing_time_ms += processing_time;

        ProcessedPerception {
            id: PerceptionId::new(),
            modality: Modality::Structured,
            timestamp: Utc::now(),
            tokens,
            entities,
            relations: Vec::new(),
            intent: None,
            sentiment: None,
            confidence: 0.9,
            processing_info: ProcessingInfo {
                processing_time_ms: processing_time,
                pipeline_stages: vec!["structured_parse".to_string()],
                warnings: Vec::new(),
            },
        }
    }

    /// Get statistics
    pub fn stats(&self) -> &PerceptionStats {
        &self.stats
    }

    /// Get average processing time
    pub fn avg_processing_time_ms(&self) -> f64 {
        if self.stats.inputs_processed == 0 {
            0.0
        } else {
            self.stats.total_processing_time_ms as f64 / self.stats.inputs_processed as f64
        }
    }
}

impl Default for PerceptionEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Perception configuration
#[derive(Debug, Clone)]
pub struct PerceptionConfig {
    pub enable_lemmatization: bool,
    pub enable_entity_extraction: bool,
    pub enable_relation_extraction: bool,
    pub enable_sentiment_analysis: bool,
    pub max_tokens: usize,
}

impl Default for PerceptionConfig {
    fn default() -> Self {
        Self {
            enable_lemmatization: true,
            enable_entity_extraction: true,
            enable_relation_extraction: true,
            enable_sentiment_analysis: true,
            max_tokens: 1000,
        }
    }
}

/// Perception statistics
#[derive(Debug, Clone, Default)]
pub struct PerceptionStats {
    pub inputs_processed: u64,
    pub total_processing_time_ms: u64,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tokenizer_tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = Tokenizer::tokenize("Hello world");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "Hello");
        assert_eq!(tokens[1].text, "world");
    }

    #[test]
    fn test_tokenize_with_punctuation() {
        let tokens = Tokenizer::tokenize("Hello, world!");
        assert!(tokens.iter().any(|t| t.text == "Hello"));
        assert!(tokens.iter().any(|t| t.text == ","));
        assert!(tokens.iter().any(|t| t.text == "world"));
        assert!(tokens.iter().any(|t| t.text == "!"));
    }

    #[test]
    fn test_tokenize_numbers() {
        let tokens = Tokenizer::tokenize("The price is 100 dollars");
        assert!(tokens.iter().any(|t| t.text == "100"));
    }
}

#[cfg(test)]
mod entity_extractor_tests {
    use super::*;

    #[test]
    fn test_extract_capitalized() {
        let tokens = Tokenizer::tokenize("John lives in Paris");
        let entities = EntityExtractor::extract(&tokens, "John lives in Paris");

        // Should extract "John" and "Paris"
        assert!(entities.iter().any(|e| e.name == "John"));
    }

    #[test]
    fn test_extract_concepts() {
        let tokens = Tokenizer::tokenize("I love truth and justice");
        let entities = EntityExtractor::extract(&tokens, "I love truth and justice");

        // Should extract concept words
        assert!(entities.iter().any(|e| e.entity_type == EntityType::Concept));
    }
}

#[cfg(test)]
mod intent_classifier_tests {
    use super::*;

    #[test]
    fn test_classify_question() {
        let tokens = Tokenizer::tokenize("What is the answer?");
        let intent = IntentClassifier::classify(&tokens, "What is the answer?");

        assert_eq!(intent.category, IntentCategory::Question);
    }

    #[test]
    fn test_classify_command() {
        let tokens = Tokenizer::tokenize("Please do the task");
        let intent = IntentClassifier::classify(&tokens, "Please do the task");

        assert_eq!(intent.category, IntentCategory::Command);
    }

    #[test]
    fn test_classify_greeting() {
        let tokens = Tokenizer::tokenize("Hello there");
        let intent = IntentClassifier::classify(&tokens, "Hello there");

        assert_eq!(intent.category, IntentCategory::Greeting);
    }

    #[test]
    fn test_classify_farewell() {
        let tokens = Tokenizer::tokenize("Goodbye friend");
        let intent = IntentClassifier::classify(&tokens, "Goodbye friend");

        assert_eq!(intent.category, IntentCategory::Farewell);
    }
}

#[cfg(test)]
mod sentiment_analyzer_tests {
    use super::*;

    #[test]
    fn test_positive_sentiment() {
        let sentiment = SentimentAnalyzer::analyze("This is great and wonderful!");
        assert!(sentiment.valence > 0.0);
        assert!(sentiment.emotion.is_some());
    }

    #[test]
    fn test_negative_sentiment() {
        let sentiment = SentimentAnalyzer::analyze("This is terrible and awful");
        assert!(sentiment.valence < 0.0);
    }

    #[test]
    fn test_neutral_sentiment() {
        let sentiment = SentimentAnalyzer::analyze("The meeting is at 3pm");
        assert!(sentiment.valence.abs() < 0.3);
    }

    #[test]
    fn test_arousal_exclamation() {
        let normal = SentimentAnalyzer::analyze("This is good");
        let excited = SentimentAnalyzer::analyze("This is good!!!");

        assert!(excited.arousal > normal.arousal);
    }
}

#[cfg(test)]
mod perception_engine_tests {
    use super::*;

    #[test]
    fn test_process_text() {
        let mut engine = PerceptionEngine::new();

        let perception = engine.process_text("Hello, how are you?".to_string());

        assert_eq!(perception.modality, Modality::Text);
        assert!(!perception.tokens.is_empty());
        assert!(perception.intent.is_some());
        assert!(perception.sentiment.is_some());
    }

    #[test]
    fn test_processing_stats() {
        let mut engine = PerceptionEngine::new();

        engine.process_text("Test".to_string());
        engine.process_text("Another test".to_string());

        let stats = engine.stats();
        assert_eq!(stats.inputs_processed, 2);
        assert!(engine.avg_processing_time_ms() > 0.0);
    }
}
