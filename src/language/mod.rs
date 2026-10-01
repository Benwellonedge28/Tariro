//! Tariro Language Understanding & Generation Module
//!
//! Implements symbolic natural language processing without token prediction.
//! Includes semantic parsing, dialogue management, and rule-based generation.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// Core Types
// ============================================================================

/// Unique identifier for language constructs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LanguageId(pub Uuid);

impl LanguageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for LanguageId {
    fn default() -> Self {
        Self::new()
    }
}

/// Semantic representation of meaning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeaningRepresentation {
    pub id: LanguageId,
    pub predicate: Predicate,
    pub arguments: Vec<MeaningArgument>,
    pub modality: SemanticModality,
    pub tense: SemanticTense,
    pub polarity: bool, // true = positive, false = negative
}

impl MeaningRepresentation {
    pub fn new(predicate: Predicate) -> Self {
        Self {
            id: LanguageId::new(),
            predicate,
            arguments: Vec::new(),
            modality: SemanticModality::Declarative,
            tense: SemanticTense::Present,
            polarity: true,
        }
    }
}

/// Predicate in semantic representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Predicate {
    pub name: String,
    pub arity: usize,
    pub type_constraints: Vec<String>,
}

impl Predicate {
    pub fn new(name: &str, arity: usize) -> Self {
        Self {
            name: name.to_string(),
            arity,
            type_constraints: Vec::new(),
        }
    }
}

/// Argument in semantic representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeaningArgument {
    Individual(String),         // e.g., "John"
    Variable(String),          // e.g., "X"
    Quantified(Quantifier, Box<MeaningRepresentation>), // e.g., "every X"
    Event(String),             // e.g., "event_1"
}

impl MeaningArgument {
    pub fn to_string(&self) -> String {
        match self {
            MeaningArgument::Individual(name) => name.clone(),
            MeaningArgument::Variable(var) => var.clone(),
            MeaningArgument::Quantified(q, mr) => format!("{:?} {}", q, mr.predicate.name),
            MeaningArgument::Event(e) => e.clone(),
        }
    }
}

/// Quantifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Quantifier {
    Universal,  // "every", "all"
    Existential, // "some", "a"
    None,       // "no"
    Most,
    Few,
}

/// Semantic modality
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticModality {
    Declarative,
    Interrogative,
    Imperative,
    Conditional,
    Optative,  // wishes
}

/// Semantic tense
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SemanticTense {
    Past,
    Present,
    Future,
    Conditional, // "would"
}

// ============================================================================
// Semantic Parser
// ============================================================================

/// Semantic parser - converts surface text to logical form
#[derive(Debug, Clone)]
pub struct SemanticParser {
    lexicon: Arc<RwLock<Lexicon>>,
    grammar: Arc<RwLock<SemanticGrammar>>,
    stats: LanguageStats,
}

impl SemanticParser {
    pub fn new() -> Self {
        let mut parser = Self {
            lexicon: Arc::new(RwLock::new(Lexicon::new())),
            grammar: Arc::new(RwLock::new(SemanticGrammar::new())),
            stats: LanguageStats::default(),
        };
        parser.initialize_default_lexicon();
        parser
    }

    fn initialize_default_lexicon(&mut self) {
        // Common verbs
        let verbs = vec![
            ("is", "be", 1),           // is(X)
            ("are", "be", 1),
            ("was", "be", 1),
            ("loves", "love", 2),      // love(X, Y)
            ("loved", "love", 2),
            ("hits", "hit", 2),        // hit(X, Y)
            ("saw", "see", 2),
            ("sees", "see", 2),
            ("gives", "give", 3),      // give(X, Y, Z)
            ("wants", "want", 2),       // want(X, Y)
            ("knows", "know", 2),      // know(X, Y)
            ("thinks", "think", 2),     // think(X, Y)
            ("believes", "believe", 2),
            ("says", "say", 2),
            ("walks", "walk", 1),
            ("runs", "run", 1),
            ("eats", "eat", 2),
            ("drinks", "drink", 2),
            ("sleeps", "sleep", 1),
        ];

        // Common nouns
        let nouns = vec![
            ("human", "human", "thing"),
            ("man", "man", "human"),
            ("woman", "woman", "human"),
            ("dog", "dog", "thing"),
            ("cat", "cat", "thing"),
            ("book", "book", "thing"),
            ("tree", "tree", "thing"),
            ("water", "water", "substance"),
            ("food", "food", "thing"),
            ("house", "house", "thing"),
        ];

        // Common adjectives
        let adjectives = vec![
            ("big", "big"),
            ("small", "small"),
            ("red", "color", "red"),
            ("happy", "happy"),
            ("sad", "sad"),
            ("tall", "tall"),
            ("short", "short"),
        ];

        // Common determiners
        let determiners = vec![
            ("a", Quantifier::Existential),
            ("an", Quantifier::Existential),
            ("the", Quantifier::Existential),
            ("every", Quantifier::Universal),
            ("all", Quantifier::Universal),
            ("some", Quantifier::Existential),
            ("no", Quantifier::None),
            ("most", Quantifier::Most),
            ("few", Quantifier::Few),
        ];

        let mut lexicon = Lexicon::new();

        for (surface, lemma, arity) in verbs {
            lexicon.add_verb(surface, lemma, arity);
        }

        for (surface, lemma, hypernym) in nouns {
            lexicon.add_noun(surface, lemma, hypernym);
        }

        for adj_data in adjectives {
            match adj_data {
                (surf, lemma) => lexicon.add_adjective(surf, lemma, None),
                (surf, lemma, sense) => lexicon.add_adjective(surf, lemma, Some(sense)),
            }
        }

        for (surface, quant) in determiners {
            lexicon.add_determiner(surface, quant);
        }

        // Store in shared state
        std::mem::swap(&mut *self.lexicon.write().await, &mut lexicon);
    }

    /// Parse text into semantic representation
    pub async fn parse(&mut self, text: &str) -> ParseResult {
        self.stats.utterances_parsed += 1;

        let tokens = self.tokenize(text);
        let pos_tags = self.pos_tag(&tokens);
        let constituents = self.constituent_parse(&tokens, &pos_tags);
        let meaning = self.extract_meaning(&constituents, &pos_tags).await;

        ParseResult {
            original_text: text.to_string(),
            tokens: tokens.clone(),
            pos_tags,
            constituents,
            meaning: meaning.clone(),
            logical_form: self.to_logical_form(&meaning),
            confidence: 0.85,
        }
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|s| s.trim_matches(|c: char| c.is_ascii_punctuation()).to_lowercase())
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn pos_tag(&self, tokens: &[String]) -> Vec<PartOfSpeech> {
        tokens
            .iter()
            .map(|t| self.guess_pos(t))
            .collect()
    }

    fn guess_pos(&self, word: &str) -> PartOfSpeech {
        let w = word.to_lowercase();

        // Determiners
        if ["a", "an", "the", "every", "some", "no", "all", "most", "few"].contains(&w.as_str()) {
            return PartOfSpeech::Determiner;
        }

        // Pronouns
        if ["i", "you", "he", "she", "it", "we", "they", "me", "him", "her", "us", "them"]
            .contains(&w.as_str())
        {
            return PartOfSpeech::Pronoun;
        }

        // Prepositions
        if ["in", "on", "at", "by", "with", "for", "to", "from", "of", "about"]
            .contains(&w.as_str())
        {
            return PartOfSpeech::Preposition;
        }

        // Question words
        if ["who", "what", "where", "when", "why", "how"].contains(&w.as_str()) {
            return PartOfSpeech::QuestionWord;
        }

        // Common verb endings
        if w.ends_with("ing") || w.ends_with("ed") || w.ends_with("s") && w.len() > 3 {
            return PartOfSpeech::Verb;
        }

        // Common adjective endings
        if w.ends_with("ful") || w.ends_with("less") || w.ends_with("ous") {
            return PartOfSpeech::Adjective;
        }

        // Numbers
        if w.parse::<f64>().is_ok() {
            return PartOfSpeech::Number;
        }

        // Capitalized = likely proper noun
        if word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            return PartOfSpeech::ProperNoun;
        }

        PartOfSpeech::CommonNoun
    }

    fn constituent_parse(&self, tokens: &[String], pos_tags: &[PartOfSpeech]) -> Vec<Constituent> {
        let mut constituents = Vec::new();
        let mut i = 0;

        while i < tokens.len() {
            match pos_tags[i] {
                PartOfSpeech::Determiner => {
                    let start = i;
                    while i < tokens.len()
                        && matches!(
                            pos_tags[i],
                            PartOfSpeech::Determiner | PartOfSpeech::Adjective
                        )
                    {
                        i += 1;
                    }
                    if i < tokens.len() && matches!(pos_tags[i], PartOfSpeech::CommonNoun) {
                        constituents.push(Constituent::NP {
                            words: tokens[start..=i].to_vec(),
                            head: i,
                        });
                        i += 1;
                    }
                }
                PartOfSpeech::Pronoun => {
                    constituents.push(Constituent::NP {
                        words: vec![tokens[i].clone()],
                        head: i,
                    });
                    i += 1;
                }
                PartOfSpeech::ProperNoun => {
                    constituents.push(Constituent::NP {
                        words: vec![tokens[i].clone()],
                        head: i,
                    });
                    i += 1;
                }
                PartOfSpeech::Verb => {
                    constituents.push(Constituent::VP {
                        words: vec![tokens[i].clone()],
                        head: i,
                    });
                    i += 1;
                }
                PartOfSpeech::Preposition => {
                    constituents.push(Constituent::PP {
                        words: vec![tokens[i].clone()],
                        head: i,
                    });
                    i += 1;
                }
                _ => i += 1,
            }
        }

        constituents
    }

    async fn extract_meaning(
        &self,
        constituents: &[Constituent],
        pos_tags: &[PartOfSpeech],
    ) -> Vec<MeaningRepresentation> {
        let mut meanings = Vec::new();
        let lexicon = self.lexicon.read().await;

        // Find verbs and build predicate-argument structures
        for (i, tag) in pos_tags.iter().enumerate() {
            if matches!(tag, PartOfSpeech::Verb) {
                if let Some(verb_info) = lexicon.get_verb(&pos_tags[i].to_string()) {
                    let pred = Predicate::new(&verb_info.lemma, verb_info.arity);

                    let mut meaning = MeaningRepresentation::new(pred);

                    // Try to find arguments based on surrounding NPs
                    for (j, const_) in constituents.iter().enumerate() {
                        if let Constituent::NP { words, .. } = const_ {
                            if j < i {
                                meaning.arguments.push(MeaningArgument::Individual(
                                    words.first().cloned().unwrap_or_default(),
                                ));
                            }
                        }
                    }

                    // Assume subject is first argument, object is second
                    meanings.push(meaning);
                }
            }
        }

        // If no verbs found, create simple proposition
        if meanings.is_empty() {
            let pred = Predicate::new("proposition", 0);
            let mut meaning = MeaningRepresentation::new(pred);
            for const_ in constituents {
                if let Constituent::NP { words, .. } = const_ {
                    meaning.arguments.push(MeaningArgument::Individual(
                        words.first().cloned().unwrap_or_default(),
                    ));
                }
            }
            meanings.push(meaning);
        }

        meanings
    }

    fn to_logical_form(&self, meanings: &[MeaningRepresentation]) -> String {
        meanings
            .iter()
            .map(|m| {
                let args = m
                    .arguments
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", m.predicate.name, args)
            })
            .collect::<Vec<_>>()
            .join(" & ")
    }

    pub fn stats(&self) -> &LanguageStats {
        &self.stats
    }
}

impl Default for SemanticParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Part of speech tags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartOfSpeech {
    Determiner,
    Pronoun,
    CommonNoun,
    ProperNoun,
    Verb,
    Adjective,
    Adverb,
    Preposition,
    Conjunction,
    Number,
    QuestionWord,
    Punctuation,
    Other,
}

/// Constituent structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Constituent {
    NP { words: Vec<String>, head: usize },
    VP { words: Vec<String>, head: usize },
    PP { words: Vec<String>, head: usize },
    S { clauses: Vec<Box<Constituent>> },
}

/// Parse result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseResult {
    pub original_text: String,
    pub tokens: Vec<String>,
    pub pos_tags: Vec<PartOfSpeech>,
    pub constituents: Vec<Constituent>,
    pub meaning: Vec<MeaningRepresentation>,
    pub logical_form: String,
    pub confidence: f64,
}

// ============================================================================
// Lexicon
// ============================================================================

/// Word lexicon
#[derive(Debug, Clone)]
pub struct Lexicon {
    pub verbs: HashMap<String, VerbEntry>,
    pub nouns: HashMap<String, NounEntry>,
    pub adjectives: HashMap<String, AdjectiveEntry>,
    pub determiners: HashMap<String, Quantifier>,
}

impl Lexicon {
    pub fn new() -> Self {
        Self {
            verbs: HashMap::new(),
            nouns: HashMap::new(),
            adjectives: HashMap::new(),
            determiners: HashMap::new(),
        }
    }

    pub fn add_verb(&mut self, surface: &str, lemma: &str, arity: usize) {
        self.verbs.insert(
            surface.to_lowercase(),
            VerbEntry {
                lemma: lemma.to_string(),
                arity,
                subcategorization: vec![],
            },
        );
    }

    pub fn add_noun(&mut self, surface: &str, lemma: &str, hypernym: &str) {
        self.nouns.insert(
            surface.to_lowercase(),
            NounEntry {
                lemma: lemma.to_string(),
                hypernym: hypernym.to_string(),
                count: 1,
            },
        );
    }

    pub fn add_adjective(&mut self, surface: &str, lemma: &str, sense: Option<&str>) {
        self.adjectives.insert(
            surface.to_lowercase(),
            AdjectiveEntry {
                lemma: lemma.to_string(),
                sense: sense.map(|s| s.to_string()),
            },
        );
    }

    pub fn add_determiner(&mut self, surface: &str, quant: Quantifier) {
        self.determiners.insert(surface.to_lowercase(), quant);
    }

    pub fn get_verb(&self, surface: &str) -> Option<&VerbEntry> {
        self.verbs.get(&surface.to_lowercase())
    }

    pub fn get_noun(&self, surface: &str) -> Option<&NounEntry> {
        self.nouns.get(&surface.to_lowercase())
    }
}

#[derive(Debug, Clone)]
pub struct VerbEntry {
    pub lemma: String,
    pub arity: usize,
    pub subcategorization: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NounEntry {
    pub lemma: String,
    pub hypernym: String,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct AdjectiveEntry {
    pub lemma: String,
    pub sense: Option<String>,
}

/// Semantic grammar
#[derive(Debug, Clone)]
pub struct SemanticGrammar {
    pub rules: Vec<GrammarRule>,
}

impl SemanticGrammar {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
}

// ============================================================================
// Dialogue Manager
// ============================================================================

/// Dialogue state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueState {
    pub id: DialogueId,
    pub turn: usize,
    pub intent: Option<Intent>,
    pub slots: HashMap<String, SlotValue>,
    pub history: Vec<DialogueTurn>,
    pub context: HashMap<String, String>,
}

impl DialogueState {
    pub fn new() -> Self {
        Self {
            id: DialogueId::new(),
            turn: 0,
            intent: None,
            slots: HashMap::new(),
            history: Vec::new(),
            context: HashMap::new(),
        }
    }

    pub fn update(&mut self, turn: DialogueTurn) {
        self.turn += 1;
        self.history.push(turn);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DialogueId(pub Uuid);

impl DialogueId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueTurn {
    pub speaker: Speaker,
    pub utterance: String,
    pub parse: Option<ParseResult>,
    pub intent: Option<Intent>,
    pub slots: HashMap<String, SlotValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Speaker {
    User,
    System,
}

/// Intent classification result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub name: IntentType,
    pub confidence: f64,
    pub slots: HashMap<String, SlotValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IntentType {
    Greeting,
    Farewell,
    Question,
    Answer,
    Request,
    Statement,
    Acknowledgment,
    Apology,
    Complaint,
    Confirmation,
    Negation,
    Help,
    None,
}

/// Slot value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SlotValue {
    String(String),
    Number(f64),
    Entity(String),
    DateTime(String),
    None,
}

/// Dialogue manager
#[derive(Debug, Clone)]
pub struct DialogueManager {
    pub state: DialogueState,
    pub response_templates: HashMap<IntentType, Vec<ResponseTemplate>>,
}

impl DialogueManager {
    pub fn new() -> Self {
        let mut manager = Self {
            state: DialogueState::new(),
            response_templates: HashMap::new(),
        };
        manager.initialize_templates();
        manager
    }

    fn initialize_templates(&mut self) {
        // Greeting templates
        self.response_templates.insert(
            IntentType::Greeting,
            vec![
                ResponseTemplate {
                    text: "Hello! How can I help you today?".to_string(),
                    condition: None,
                },
                ResponseTemplate {
                    text: "Hi there! What would you like to know?".to_string(),
                    condition: None,
                },
            ],
        );

        // Farewell templates
        self.response_templates.insert(
            IntentType::Farewell,
            vec![ResponseTemplate {
                text: "Goodbye! Have a great day!".to_string(),
                condition: None,
            }],
        );

        // Help templates
        self.response_templates.insert(
            IntentType::Help,
            vec![ResponseTemplate {
                text: "I can help you with various tasks. Just ask!".to_string(),
                condition: None,
            }],
        );

        // Question templates
        self.response_templates.insert(
            IntentType::Question,
            vec![ResponseTemplate {
                text: "That's an interesting question. Let me think...".to_string(),
                condition: None,
            }],
        );
    }

    pub fn process(&mut self, utterance: &str, parser: &mut SemanticParser) -> DialogueResponse {
        self.state.turn += 1;

        // Parse utterance
        let parse_result = parser.parse(utterance).await;
        let intent = self.classify_intent(&parse_result);
        let slots = self.extract_slots(&parse_result);

        let turn = DialogueTurn {
            speaker: Speaker::User,
            utterance: utterance.to_string(),
            parse: Some(parse_result),
            intent: Some(intent.clone()),
            slots: slots.clone(),
        };

        self.state.update(turn);

        // Generate response
        let response_text = self.generate_response(&intent);

        DialogueResponse {
            text: response_text,
            intent: intent.clone(),
            slots,
        }
    }

    fn classify_intent(&self, parse: &ParseResult) -> Intent {
        let text_lower = parse.original_text.to_lowercase();
        let tokens = &parse.tokens;

        // Question detection
        if text_lower.contains('?')
            || tokens.first().map(|t| {
                ["who", "what", "where", "when", "why", "how", "is", "are", "do", "does", "can"]
                    .contains(&t.as_str())
            }) == Some(true)
        {
            return Intent {
                name: IntentType::Question,
                confidence: 0.9,
                slots: HashMap::new(),
            };
        }

        // Greeting detection
        if ["hello", "hi", "hey", "greetings"]
            .iter()
            .any(|g| text_lower.starts_with(g))
        {
            return Intent {
                name: IntentType::Greeting,
                confidence: 0.95,
                slots: HashMap::new(),
            };
        }

        // Farewell detection
        if ["goodbye", "bye", "farewell"]
            .iter()
            .any(|g| text_lower.starts_with(g))
        {
            return Intent {
                name: IntentType::Farewell,
                confidence: 0.95,
                slots: HashMap::new(),
            };
        }

        // Help detection
        if text_lower.contains("help")
            || text_lower.contains("what can you do")
        {
            return Intent {
                name: IntentType::Help,
                confidence: 0.9,
                slots: HashMap::new(),
            };
        }

        // Default to statement
        Intent {
            name: IntentType::Statement,
            confidence: 0.6,
            slots: HashMap::new(),
        }
    }

    fn extract_slots(&self, parse: &ParseResult) -> HashMap<String, SlotValue> {
        let mut slots = HashMap::new();

        // Extract named entities
        for const_ in &parse.constituents {
            if let Constituent::NP { words, .. } = const_ {
                if let Some(first) = words.first() {
                    // Capitalized words are likely entities
                    if first.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        slots.insert(
                            "entity".to_string(),
                            SlotValue::Entity(first.clone()),
                        );
                    }
                }
            }
        }

        // Extract question word
        for tag in &parse.pos_tags {
            if matches!(tag, PartOfSpeech::QuestionWord) {
                slots.insert("question_type".to_string(), SlotValue::String("wh".to_string()));
            }
        }

        slots
    }

    fn generate_response(&self, intent: &Intent) -> String {
        if let Some(templates) = self.response_templates.get(&intent.name) {
            if let Some(template) = templates.first() {
                return template.text.clone();
            }
        }

        // Default responses
        match intent.name {
            IntentType::Question => "I'm processing your question...".to_string(),
            IntentType::Statement => "I understand.".to_string(),
            IntentType::Request => "I'll try to help with that.".to_string(),
            IntentType::Acknowledgment => "Thank you for letting me know.".to_string(),
            _ => "I'm here to help.".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseTemplate {
    pub text: String,
    pub condition: Option<String>,
}

/// Dialogue response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueResponse {
    pub text: String,
    pub intent: Intent,
    pub slots: HashMap<String, SlotValue>,
}

// ============================================================================
// Text Generation
// ============================================================================

/// Rule-based text generator
#[derive(Debug, Clone)]
pub struct TextGenerator {
    pub grammar: RealizationGrammar,
    pub lexicon: Arc<RwLock<Lexicon>>,
}

impl TextGenerator {
    pub fn new() -> Self {
        Self {
            grammar: RealizationGrammar::new(),
            lexicon: Arc::new(RwLock::new(Lexicon::new())),
        }
    }

    /// Generate surface text from semantic representation
    pub async fn generate(&self, meaning: &MeaningRepresentation) -> String {
        let pred = &meaning.predicate.name;
        let args = &meaning.arguments;

        // Simple verb-centered generation
        let subject = args
            .first()
            .map(|a| match a {
                MeaningArgument::Individual(name) => name.clone(),
                MeaningArgument::Variable(var) => var.clone(),
                _ => "something".to_string(),
            })
            .unwrap_or_else(|| "it".to_string());

        let object = args.get(1).map(|a| match a {
            MeaningArgument::Individual(name) => name.clone(),
            MeaningArgument::Variable(var) => var.clone(),
            _ => "something".to_string(),
        });

        let verb = match pred.as_str() {
            "be" => "is",
            "love" => "loves",
            "hit" => "hits",
            "see" => "sees",
            "give" => "gives",
            "want" => "wants",
            "know" => "knows",
            "think" => "thinks",
            _ => pred.as_str(),
        };

        if args.len() >= 2 {
            format!("{} {}", subject, verb)
        } else {
            format!("{} {}", verb, subject)
        }
    }

    /// Generate full sentence from logical form
    pub async fn realize(&self, logical_form: &str) -> String {
        // Parse simple logical form like "love(John, Mary)"
        if let Some((pred, args_str)) = logical_form.trim().split_once('(') {
            let args: Vec<String> = args_str
                .trim_end_matches(')')
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

            let subject = args.first().map(|s| s.as_str()).unwrap_or("something");
            let object = args.get(1).map(|s| s.as_str());

            let verb = match pred {
                "be" => "is",
                "love" => "loves",
                "hit" => "hits",
                "see" => "sees",
                "give" => "gives",
                "want" => "wants",
                "know" => "knows",
                "think" => "thinks",
                "see" => "sees",
                _ => pred,
            };

            match object {
                Some(obj) => format!("{} {} {}.", self.capitalize(subject), verb, obj),
                None => format!("{} {}.", self.capitalize(subject), verb),
            }
        } else {
            self.capitalize(logical_form)
        }
    }

    fn capitalize(&self, s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

impl Default for TextGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Grammar for surface realization
#[derive(Debug, Clone)]
pub struct RealizationGrammar {
    pub rules: HashMap<String, Vec<String>>,
}

impl RealizationGrammar {
    pub fn new() -> Self {
        let mut grammar = Self {
            rules: HashMap::new(),
        };

        // Simple phrase structure rules
        grammar.rules.insert("S".to_string(), vec!["NP VP".to_string()]);
        grammar.rules.insert("NP".to_string(), vec!["Det N".to_string(), "N".to_string()]);
        grammar.rules.insert("VP".to_string(), vec!["V".to_string(), "V NP".to_string()]);

        grammar
    }
}

// ============================================================================
// Language Statistics
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct LanguageStats {
    pub utterances_parsed: u64,
    pub meanings_extracted: u64,
    pub dialogues_managed: u64,
    pub texts_generated: u64,
}

impl LanguageStats {
    pub fn increment_parsed(&mut self) {
        self.utterances_parsed += 1;
    }

    pub fn increment_generated(&mut self) {
        self.texts_generated += 1;
    }
}

// ============================================================================
// Unified Language System
// ============================================================================

/// Unified language understanding and generation system
#[derive(Debug, Clone)]
pub struct LanguageSystem {
    pub parser: SemanticParser,
    pub dialogue_manager: DialogueManager,
    pub generator: TextGenerator,
    pub stats: LanguageStats,
}

impl LanguageSystem {
    pub fn new() -> Self {
        Self {
            parser: SemanticParser::new(),
            dialogue_manager: DialogueManager::new(),
            generator: TextGenerator::new(),
            stats: LanguageStats::default(),
        }
    }

    /// Process user input
    pub async fn understand(&mut self, text: &str) -> ParseResult {
        self.stats.increment_parsed();
        self.parser.parse(text).await
    }

    /// Generate response
    pub async fn generate(&self, meaning: &MeaningRepresentation) -> String {
        self.stats.increment_generated();
        self.generator.generate(meaning).await
    }

    /// Full dialogue turn
    pub async fn dialogue_turn(&mut self, utterance: &str) -> DialogueResponse {
        self.stats.dialogues_managed += 1;
        self.dialogue_manager.process(utterance, &mut self.parser).await
    }

    /// Realize logical form as text
    pub async fn realize(&self, logical_form: &str) -> String {
        self.generator.realize(logical_form).await
    }
}

impl Default for LanguageSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod semantic_parser_tests {
    use super::*;

    #[tokio::test]
    async fn test_parse_simple_sentence() {
        let mut parser = SemanticParser::new();
        let result = parser.parse("John loves Mary").await;

        assert!(!result.tokens.is_empty());
        assert!(result.meaning.len() > 0);
    }

    #[tokio::test]
    async fn test_pos_tagging() {
        let parser = SemanticParser::new();
        let tags = parser.pos_tag(&["John".to_string(), "loves".to_string(), "Mary".to_string()]);

        assert_eq!(tags[0], PartOfSpeech::ProperNoun);
        assert_eq!(tags[1], PartOfSpeech::Verb);
        assert_eq!(tags[2], PartOfSpeech::ProperNoun);
    }

    #[tokio::test]
    async fn test_logical_form() {
        let mut parser = SemanticParser::new();
        let result = parser.parse("Dogs bark").await;

        assert!(!result.logical_form.is_empty());
    }
}

#[cfg(test)]
mod dialogue_manager_tests {
    use super::*;

    #[tokio::test]
    async fn test_greeting_classification() {
        let mut manager = DialogueManager::new();
        let response = manager.process("Hello!", &mut SemanticParser::new()).await;

        assert_eq!(response.intent.name, IntentType::Greeting);
    }

    #[tokio::test]
    async fn test_question_classification() {
        let mut manager = DialogueManager::new();
        let response = manager.process("What is the answer?", &mut SemanticParser::new()).await;

        assert_eq!(response.intent.name, IntentType::Question);
    }

    #[tokio::test]
    async fn test_farewell_classification() {
        let mut manager = DialogueManager::new();
        let response = manager.process("Goodbye!", &mut SemanticParser::new()).await;

        assert_eq!(response.intent.name, IntentType::Farewell);
    }
}

#[cfg(test)]
mod text_generator_tests {
    use super::*;

    #[tokio::test]
    async fn test_simple_generation() {
        let generator = TextGenerator::new();
        let meaning = MeaningRepresentation::new(Predicate::new("loves", 2));

        let text = generator.generate(&meaning).await;
        assert!(!text.is_empty());
    }

    #[tokio::test]
    async fn test_realization() {
        let generator = TextGenerator::new();
        let text = generator.realize("love(John, Mary)").await;

        assert!(text.contains("John"));
        assert!(text.contains("loves"));
        assert!(text.contains("Mary"));
    }
}

#[cfg(test)]
mod language_system_tests {
    use super::*;

    #[tokio::test]
    async fn test_understand() {
        let mut system = LanguageSystem::new();
        let result = system.understand("Hello there").await;

        assert!(!result.tokens.is_empty());
    }

    #[tokio::test]
    async fn test_dialogue() {
        let mut system = LanguageSystem::new();
        let response = system.dialogue_turn("Hi!").await;

        assert!(!response.text.is_empty());
    }

    #[tokio::test]
    async fn test_realize() {
        let system = LanguageSystem::new();
        let text = system.realize("is(X, happy)").await;

        assert!(text.starts_with(|c: char| c.is_uppercase()));
    }
}
