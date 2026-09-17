// Tariro AGI - Deep Language Understanding Module
// Comprehensive natural language processing system for AGI

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Tokenization and Text Processing
// ============================================================================

/// Text normalization options
#[derive(Debug, Clone)]
pub struct NormalizationConfig {
    pub lowercase: bool,
    pub remove_punctuation: bool,
    pub remove_whitespace: bool,
    pub strip_accents: bool,
    pub normalize_unicode: bool,
}

impl Default for NormalizationConfig {
    fn default() -> Self {
        Self {
            lowercase: true,
            remove_punctuation: false,
            remove_whitespace: false,
            strip_accents: false,
            normalize_unicode: true,
        }
    }
}

/// Token types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TokenType {
    Word,
    Number,
    Punctuation,
    Whitespace,
    Symbol,
    Unknown,
}

/// Token with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub text: String,
    pub normalized: String,
    pub token_type: TokenType,
    pub position: usize,
    pub char_start: usize,
    pub char_end: usize,
    pub lemma: Option<String>,
    pub pos_tag: Option<POSTag>,
    pub dependency: Option<DependencyLabel>,
    pub embedding: Option<Vec<f32>>,
    pub features: HashMap<String, String>,
}

impl Token {
    pub fn new(text: String, position: usize, char_start: usize, char_end: usize) -> Self {
        Self {
            normalized: text.clone(),
            token_type: Self::classify_token(&text),
            text,
            position,
            char_start,
            char_end,
            lemma: None,
            pos_tag: None,
            dependency: None,
            embedding: None,
            features: HashMap::new(),
        }
    }

    fn classify_token(text: &str) -> TokenType {
        if text.chars().all(|c| c.is_alphabetic()) {
            TokenType::Word
        } else if text.chars().all(|c| c.is_numeric()) {
            TokenType::Number
        } else if text.chars().all(|c| c.is_whitespace()) {
            TokenType::Whitespace
        } else if text.chars().all(|c| c.is_ascii_punctuation() || !c.is_alphabetic()) {
            TokenType::Punctuation
        } else {
            TokenType::Symbol
        }
    }
}

/// Part-of-speech tags (Universal POS tags)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum POSTag {
    NOUN,       // Noun
    VERB,       // Verb
    ADJ,       // Adjective
    ADV,       // Adverb
    PRON,      // Pronoun
    DET,       // Determiner
    ADP,       // Adposition
    NUM,       // Numeral
    CONJ,      // Conjunction
    PRT,       // Particle
    INTJ,      // Interjection
    PROPN,     // Proper Noun
    PUNCT,     // Punctuation
    SYM,       // Symbol
    X,         // Other
}

impl POSTag {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "NOUN" | "NN" | "NNS" | "NNP" | "NNPS" => Some(POSTag::NOUN),
            "VERB" | "VB" | "VBD" | "VBG" | "VBN" | "VBP" | "VBZ" => Some(POSTag::VERB),
            "ADJ" | "JJ" | "JJR" | "JJS" => Some(POSTag::ADJ),
            "ADV" | "RB" | "RBR" | "RBS" => Some(POSTag::ADV),
            "PRON" | "PRP" | "PRP$" | "WP" | "WP$" => Some(POSTag::PRON),
            "DET" | "DT" | "PDT" | "WDT" => Some(POSTag::DET),
            "ADP" | "IN" => Some(POSTag::ADP),
            "NUM" | "CD" => Some(POSTag::NUM),
            "CONJ" | "CC" => Some(POSTag::CONJ),
            "PRT" | "RP" => Some(POSTag::PRT),
            "INTJ" | "UH" => Some(POSTag::INTJ),
            "PROPN" | "NNP" | "NNPS" => Some(POSTag::PROPN),
            "PUNCT" => Some(POSTag::PUNCT),
            "SYM" => Some(POSTag::SYM),
            _ => Some(POSTag::X),
        }
    }
}

/// Dependency labels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DependencyLabel {
    Nsubj,      // Nominal subject
    Nobj,       // Direct object
    Niobj,      // Indirect object
    Csubj,      // Clausal subject
    Csubjpass,  // Clausal subject (passive)
    Cobj,       // Clausal object
    Amod,       // Adjectival modifier
    Advmod,     // Adverbial modifier
    Nmod,       // Nominal modifier
    Pobj,       // Object of preposition
    Det,        // Determiner
    Poss,       // Possessive modifier
    Case,       // Case marking
    Conj,       // Conjunct
    Cc,         // Coordinating conjunction
    Root,       // Root
    Dep,        // Unspecified dependency
}

impl DependencyLabel {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "nsubj" => Some(DependencyLabel::Nsubj),
            "nobj" | "dobj" => Some(DependencyLabel::Nobj),
            "iobj" => Some(DependencyLabel::Niobj),
            "csubj" => Some(DependencyLabel::Csubj),
            "csubjpass" => Some(DependencyLabel::Csubjpass),
            "cobj" => Some(DependencyLabel::Cobj),
            "amod" => Some(DependencyLabel::Amod),
            "advmod" => Some(DependencyLabel::Advmod),
            "nmod" => Some(DependencyLabel::Nmod),
            "pobj" | "pcomp" => Some(DependencyLabel::Pobj),
            "det" => Some(DependencyLabel::Det),
            "poss" => Some(DependencyLabel::Poss),
            "case" => Some(DependencyLabel::Case),
            "conj" => Some(DependencyLabel::Conj),
            "cc" => Some(DependencyLabel::Cc),
            "root" => Some(DependencyLabel::Root),
            _ => Some(DependencyLabel::Dep),
        }
    }
}

/// Tokenizer
pub struct Tokenizer {
    config: NormalizationConfig,
}

impl Tokenizer {
    pub fn new() -> Self {
        Self {
            config: NormalizationConfig::default(),
        }
    }

    pub fn with_config(config: NormalizationConfig) -> Self {
        Self { config }
    }

    /// Tokenize text into tokens
    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut char_pos = 0usize;
        let mut word_count = 0usize;

        for word in text.split_whitespace() {
            let token = Token::new(
                word.to_string(),
                word_count,
                char_pos,
                char_pos + word.len(),
            );
            tokens.push(token);
            char_pos += word.len() + 1; // +1 for space
            word_count += 1;
        }

        tokens
    }

    /// Simple word tokenization
    pub fn tokenize_words(&self, text: &str) -> Vec<String> {
        text.split_whitespace()
            .map(|w| self.normalize(w))
            .collect()
    }

    /// Normalize a single word
    fn normalize(&self, word: &str) -> String {
        let mut result = word.to_string();

        if self.config.lowercase {
            result = result.to_lowercase();
        }

        if self.config.remove_punctuation {
            result = result.chars().filter(|c| c.is_alphanumeric()).collect();
        }

        result
    }
}

// ============================================================================
// Morphological Analysis
// ============================================================================

/// Morphological analyzer
pub struct MorphologicalAnalyzer {
    suffix_rules: HashMap<String, &'static str>,
    prefix_rules: HashMap<String, &'static str>,
}

impl MorphologicalAnalyzer {
    pub fn new() -> Self {
        let mut suffix_rules = HashMap::new();
        suffix_rules.insert("ing".to_string(), "VERB");
        suffix_rules.insert("ed".to_string(), "VERB");
        suffix_rules.insert("es".to_string(), "VERB");
        suffix_rules.insert("s".to_string(), "NOUN");
        suffix_rules.insert("ly".to_string(), "ADV");
        suffix_rules.insert("er".to_string(), "ADJ");
        suffix_rules.insert("est".to_string(), "ADJ");
        suffix_rules.insert("tion".to_string(), "NOUN");
        suffix_rules.insert("ness".to_string(), "NOUN");
        suffix_rules.insert("ful".to_string(), "ADJ");
        suffix_rules.insert("less".to_string(), "ADJ");
        suffix_rules.insert("able".to_string(), "ADJ");
        suffix_rules.insert("ible".to_string(), "ADJ");

        let mut prefix_rules = HashMap::new();
        prefix_rules.insert("un".to_string(), "NEG");
        prefix_rules.insert("re".to_string(), "REP");
        prefix_rules.insert("pre".to_string(), "BEF");
        prefix_rules.insert("post".to_string(), "AFT");
        prefix_rules.insert("anti".to_string(), "OPP");
        prefix_rules.insert("inter".to_string(), "BET");
        prefix_rules.insert("trans".to_string(), "ACR");

        Self {
            suffix_rules,
            prefix_rules,
        }
    }

    /// Analyze word morphology
    pub fn analyze(&self, word: &str) -> MorphologicalAnalysis {
        let mut features = HashMap::new();
        let mut morphemes = Vec::new();
        let mut root = word.to_string();

        // Check prefixes
        for (prefix, meaning) in &self.prefix_rules {
            if word.starts_with(prefix) {
                morphemes.push(Morpheme {
                    morpheme: prefix.clone(),
                    position: Position::Prefix,
                    meaning: meaning.to_string(),
                });
                root = word[prefix.len()..].to_string();
                features.insert("prefix".to_string(), prefix.clone());
                break;
            }
        }

        // Check suffixes
        for (suffix, pos) in &self.suffix_rules {
            if root.ends_with(suffix) {
                morphemes.push(Morpheme {
                    morpheme: suffix.clone(),
                    position: Position::Suffix,
                    meaning: pos.to_string(),
                });
                root = root[..root.len() - suffix.len()].to_string();
                features.insert("suffix".to_string(), suffix.clone());
                break;
            }
        }

        // Detect plurality
        if word.ends_with('s') && !word.ends_with("ss") {
            features.insert("number".to_string(), "plural".to_string());
        }

        // Detect tense
        if word.ends_with("ed") {
            features.insert("tense".to_string(), "past".to_string());
        } else if word.ends_with("ing") {
            features.insert("tense".to_string(), "present".to_string());
            features.insert("aspect".to_string(), "progressive".to_string());
        }

        MorphologicalAnalysis {
            root,
            morphemes,
            features,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphologicalAnalysis {
    pub root: String,
    pub morphemes: Vec<Morpheme>,
    pub features: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Morpheme {
    pub morpheme: String,
    pub position: Position,
    pub meaning: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Position {
    Prefix,
    Suffix,
    Infix,
    Root,
}

// ============================================================================
// Syntactic Parsing
// ============================================================================

/// Syntactic tree node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntacticNode {
    pub id: usize,
    pub label: String,
    pub node_type: NodeType,
    pub children: Vec<usize>,
    pub token_span: Option<(usize, usize)>,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeType {
    Sentence,
    Clause,
    NounPhrase,
    VerbPhrase,
    PrepositionalPhrase,
    AdjectivePhrase,
    AdverbPhrase,
    Terminal,
}

/// Dependency tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyTree {
    pub nodes: Vec<DependencyNode>,
    pub root: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyNode {
    pub id: usize,
    pub token: String,
    pub head: Option<usize>,
    pub label: DependencyLabel,
    pub children: Vec<usize>,
}

/// Syntactic parser
pub struct SyntacticParser {
    phrase_rules: HashMap<NodeType, Vec<Vec<NodeType>>>,
}

impl SyntacticParser {
    pub fn new() -> Self {
        let mut phrase_rules = HashMap::new();
        phrase_rules.insert(NodeType::Sentence, vec![
            vec![NodeType::NounPhrase, NodeType::VerbPhrase],
            vec![NodeType::VerbPhrase],
        ]);
        phrase_rules.insert(NodeType::NounPhrase, vec![
            vec![NodeType::AdjectivePhrase, NodeType::NounPhrase],
            vec![NodeType::Determiner, NodeType::NounPhrase],
            vec![NodeType::NounPhrase, NodeType::PrepositionalPhrase],
        ]);

        Self { phrase_rules }
    }

    /// Build constituency tree from tokens
    pub fn parse_constituency(&self, tokens: &[Token]) -> SyntacticTree {
        let mut nodes = Vec::new();
        let mut next_id = 0usize;

        // Create terminal nodes
        for token in tokens {
            nodes.push(SyntacticNode {
                id: next_id,
                label: token.text.clone(),
                node_type: NodeType::Terminal,
                children: vec![],
                token_span: Some((token.position, token.position + 1)),
                attributes: HashMap::new(),
            });
            next_id += 1;
        }

        // Build phrase structure
        let root_id = self.build_phrase_structure(tokens, &mut nodes, &mut next_id);

        SyntacticTree {
            nodes,
            root: root_id,
        }
    }

    fn build_phrase_structure(
        &self,
        tokens: &[Token],
        nodes: &mut Vec<SyntacticNode>,
        next_id: &mut usize,
    ) -> usize {
        let root_id = *next_id;
        *next_id += 1;

        nodes.push(SyntacticNode {
            id: root_id,
            label: "S".to_string(),
            node_type: NodeType::Sentence,
            children: (0..tokens.len()).collect(),
            token_span: Some((0, tokens.len())),
            attributes: HashMap::new(),
        });

        root_id
    }

    /// Build dependency tree from tokens
    pub fn parse_dependency(&self, tokens: &[Token]) -> DependencyTree {
        let mut nodes = Vec::new();
        let mut children: Vec<Vec<usize>> = vec![vec![]; tokens.len()];

        // Simple heuristics for dependency parsing
        for (i, token) in tokens.iter().enumerate() {
            let (head, label) = self.find_head(i, token, tokens);

            if head < i {
                children[head].push(i);
            }

            nodes.push(DependencyNode {
                id: i,
                token: token.text.clone(),
                head: if head < i { Some(head) } else { None },
                label,
                children: vec![],
            });
        }

        // Update children
        for (i, child_list) in children.iter().enumerate() {
            nodes[i].children = child_list.clone();
        }

        // Find root (first verb typically)
        let root = tokens.iter()
            .enumerate()
            .find(|(_, t)| t.pos_tag == Some(POSTag::VERB))
            .map(|(i, _)| i)
            .unwrap_or(0);

        DependencyTree {
            nodes,
            root,
        }
    }

    fn find_head(&self, _index: usize, token: &Token, tokens: &[Token]) -> (usize, DependencyLabel) {
        let pos = token.pos_tag;

        match pos {
            Some(POSTag::NOUN) | Some(POSTag::PROPN) => {
                // Find determiner or adjective before
                for i in (0.._index).rev() {
                    if tokens[i].pos_tag == Some(POSTag::DET) {
                        return (i, DependencyLabel::Det);
                    }
                    if tokens[i].pos_tag == Some(POSTag::ADJ) {
                        return (i, DependencyLabel::Amod);
                    }
                }
                (0, DependencyLabel::Nsubj)
            }
            Some(POSTag::VERB) => {
                // Verb takes noun as subject before it
                for i in (0.._index).rev() {
                    if tokens[i].pos_tag == Some(POSTag::NOUN) {
                        return (i, DependencyLabel::Nsubj);
                    }
                }
                (0, DependencyLabel::Root)
            }
            Some(POSTag::ADP) => {
                // Preposition takes noun after it
                for i in (_index + 1)..tokens.len() {
                    if tokens[i].pos_tag == Some(POSTag::NOUN) {
                        return (i, DependencyLabel::Pobj);
                    }
                }
                (_index, DependencyLabel::Case)
            }
            _ => (0, DependencyLabel::Dep),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntacticTree {
    pub nodes: Vec<SyntacticNode>,
    pub root: usize,
}

// ============================================================================
// Semantic Representation
// ============================================================================

/// Semantic representation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SemanticRepresentation {
    PredicateLogic(PredicateLogic),
    Frame(Frame),
    Conceptual(CentralConcept),
    Neural(Vec<f32>),
}

/// Predicate logic representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredicateLogic {
    pub predicates: Vec<Predicate>,
    pub variables: HashMap<String, VariableType>,
    pub quantifiers: Vec<Quantifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Predicate {
    pub name: String,
    pub arguments: Vec<Term>,
    pub negated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Term {
    Variable(String),
    Constant(String),
    Function(String, Vec<Term>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    Entity,
    Event,
    Property,
    Location,
    Time,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quantifier {
    pub quantifier_type: QuantifierType,
    pub variable: String,
    pub scope: Vec<Predicate>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum QuantifierType {
    ForAll,
    Exists,
}

/// Frame-based semantic representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frame {
    pub frame_type: String,
    pub frame_elements: HashMap<String, FrameElement>,
    pub relations: Vec<FrameRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameElement {
    pub name: String,
    pub value: String,
    pub filler_type: ElementType,
    pub highlighted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementType {
    Entity,
    Event,
    Property,
    Reference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameRelation {
    pub relation_type: String,
    pub target_frame: String,
}

/// Core concept representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CentralConcept {
    pub concept: String,
    pub supertypes: Vec<String>,
    pub subtypes: Vec<String>,
    pub attributes: HashMap<String, Attribute>,
    pub relations: Vec<ConceptRelation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub value_type: ValueType,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueType {
    String,
    Number,
    Boolean,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptRelation {
    pub relation: String,
    pub target: String,
}

/// Semantic parser
pub struct SemanticParser {
    ontologies: HashMap<String, OntologyConcept>,
    frame_templates: HashMap<String, FrameTemplate>,
}

#[derive(Debug, Clone)]
pub struct OntologyConcept {
    pub id: String,
    pub name: String,
    pub supertypes: Vec<String>,
    pub properties: HashMap<String, PropertyDef>,
}

#[derive(Debug, Clone)]
pub struct PropertyDef {
    pub name: String,
    pub value_type: String,
    pub domain: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FrameTemplate {
    pub name: String,
    pub frame_type: String,
    pub elements: Vec<FrameElementDef>,
}

#[derive(Debug, Clone)]
pub struct FrameElementDef {
    pub name: String,
    pub core: bool,
    pub sem_type: String,
}

impl SemanticParser {
    pub fn new() -> Self {
        let mut ontologies = HashMap::new();
        let mut frame_templates = HashMap::new();

        // Add basic ontology concepts
        ontologies.insert("event".to_string(), OntologyConcept {
            id: "event".to_string(),
            name: "Event".to_string(),
            supertypes: vec![],
            properties: HashMap::new(),
        });
        ontologies.insert("agent".to_string(), OntologyConcept {
            id: "agent".to_string(),
            name: "Agent".to_string(),
            supertypes: vec!["entity".to_string()],
            properties: HashMap::new(),
        });

        // Add frame templates
        frame_templates.insert("change".to_string(), FrameTemplate {
            name: "Change".to_string(),
            frame_type: "Event".to_string(),
            elements: vec![
                FrameElementDef { name: "Actor".to_string(), core: true, sem_type: "Agent".to_string() },
                FrameElementDef { name: "Theme".to_string(), core: true, sem_type: "Entity".to_string() },
                FrameElementDef { name: "Result".to_string(), core: true, sem_type: "State".to_string() },
            ],
        });

        Self {
            ontologies,
            frame_templates,
        }
    }

    /// Parse into semantic representation
    pub fn parse(&self, tokens: &[Token], tree: &DependencyTree) -> SemanticRepresentation {
        // Simple semantic parsing based on dependency structure
        let frame = self.build_frame(tokens, tree);

        SemanticRepresentation::Frame(frame)
    }

    fn build_frame(&self, tokens: &[Token], tree: &DependencyTree) -> Frame {
        let mut elements = HashMap::new();

        // Extract subject
        for node in &tree.nodes {
            if node.label == DependencyLabel::Nsubj {
                if let Some(idx) = node.head {
                    if let Some(head_token) = tokens.get(idx) {
                        elements.insert("Actor".to_string(), FrameElement {
                            name: "Actor".to_string(),
                            value: head_token.text.clone(),
                            filler_type: ElementType::Entity,
                            highlighted: true,
                        });
                    }
                }
            }

            if node.label == DependencyLabel::Nobj {
                if let Some(idx) = node.head {
                    if let Some(head_token) = tokens.get(idx) {
                        elements.insert("Theme".to_string(), FrameElement {
                            name: "Theme".to_string(),
                            value: head_token.text.clone(),
                            filler_type: ElementType::Entity,
                            highlighted: false,
                        });
                    }
                }
            }
        }

        Frame {
            frame_type: "Event".to_string(),
            elements,
            relations: vec![],
        }
    }
}

// ============================================================================
// Pragmatic Understanding
// ============================================================================

/// Speech act types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpeechAct {
    Assertive,
    Directive,
    Commissive,
    Expressive,
    Declaration,
}

impl SpeechAct {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "assertive" | "statement" | "assert" => Some(SpeechAct::Assertive),
            "directive" | "question" | "command" | "request" => Some(SpeechAct::Directive),
            "commissive" | "promise" | "offer" => Some(SpeechAct::Commissive),
            "expressive" | "greeting" | "apology" => Some(SpeechAct::Expressive),
            "declaration" => Some(SpeechAct::Declaration),
            _ => None,
        }
    }
}

/// Deictic reference types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeicticReference {
    pub reference_type: DeicticType,
    pub anchor: String,
    pub relative_position: Option<(f32, f32, f32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeicticType {
    Personal,
    Spatial,
    Temporal,
}

/// Implicature types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Implicature {
    pub implicature_type: ImplicatureType,
    pub inferred_meaning: String,
    pub basis: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImplicatureType {
    Scalar,
    Quantity,
    Relevance,
    Manner,
}

/// Pragmatic analyzer
pub struct PragmaticAnalyzer {
    context_window: Duration,
    shared_knowledge: HashMap<String, String>,
}

impl PragmaticAnalyzer {
    pub fn new() -> Self {
        Self {
            context_window: Duration::from_secs(300),
            shared_knowledge: HashMap::new(),
        }
    }

    /// Analyze pragmatic aspects of utterance
    pub fn analyze(&self, text: &str, syntactic: &SyntacticTree) -> PragmaticAnalysis {
        let speech_act = self.classify_speech_act(text);
        let deictic = self.extract_deictic(text);
        let implicatures = self.compute_implicatures(text);
        let presuppositions = self.extract_presuppositions(text);

        PragmaticAnalysis {
            speech_act,
            deictic_references: deictic,
            implicatures,
            presuppositions,
            discourse_relation: self.analyze_discourse(text),
        }
    }

    fn classify_speech_act(&self, text: &str) -> SpeechAct {
        let text_lower = text.to_lowercase();

        if text_lower.ends_with('?') || text_lower.contains("what") || text_lower.contains("how") {
            SpeechAct::Directive
        } else if text_lower.ends_with('!') {
            SpeechAct::Expressiv
        } else if text_lower.starts_with("i promise") || text_lower.starts_with("i will") {
            SpeechAct::Commissive
        } else {
            SpeechAct::Assertive
        }
    }

    fn extract_deictic(&self, text: &str) -> Vec<DeicticReference> {
        let mut references = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();

        for (i, word) in words.iter().enumerate() {
            let word_lower = word.to_lowercase();

            match word_lower.as_str() {
                "i" | "me" | "my" | "we" | "us" | "our" => {
                    references.push(DeicticReference {
                        reference_type: DeicticType::Personal,
                        anchor: word_lower.to_string(),
                        relative_position: None,
                    });
                }
                "here" | "there" | "this" | "that" => {
                    references.push(DeicticReference {
                        reference_type: DeicticType::Spatial,
                        anchor: word_lower.to_string(),
                        relative_position: None,
                    });
                }
                "now" | "then" | "today" | "yesterday" | "tomorrow" => {
                    references.push(DeicticReference {
                        reference_type: DeicticType::Temporal,
                        anchor: word_lower.to_string(),
                        relative_position: None,
                    });
                }
                _ => {}
            }
        }

        references
    }

    fn compute_implicatures(&self, text: &str) -> Vec<Implicature> {
        let mut implicatures = Vec::new();
        let text_lower = text.to_lowercase();

        // Scalar implicature detection
        if text_lower.contains("some") {
            implicatures.push(Implicature {
                implicature_type: ImplicatureType::Scalar,
                inferred_meaning: "Not all".to_string(),
                basis: "Some implies not all in standard contexts".to_string(),
            });
        }

        // Quantity implicature
        if text_lower.contains("a few") || text_lower.contains("a little") {
            implicatures.push(Implicature {
                implicature_type: ImplicatureType::Quantity,
                inferred_meaning: "Small quantity".to_string(),
                basis: "Diminutive quantifier suggests limited amount".to_string(),
            });
        }

        implicatures
    }

    fn extract_presuppositions(&self, text: &str) -> Vec<String> {
        let mut presuppositions = Vec::new();
        let text_lower = text.to_lowercase();

        // "Stop X" presupposes X was happening
        if text_lower.starts_with("stop") || text_lower.starts_with("ceased") {
            if let Some(subject) = text_lower.split_whitespace().nth(1) {
                presuppositions.push(format!("{} was happening", subject));
            }
        }

        // "Know X" presupposes X is true
        if text_lower.starts_with("know") {
            if let Some(fact) = text_lower.strip_prefix("know that ") {
                presuppositions.push(format!("{} is true", fact));
            }
        }

        presuppositions
    }

    fn analyze_discourse(&self, text: &str) -> Option<DiscourseRelation> {
        let text_lower = text.to_lowercase();

        if text_lower.contains("because") {
            Some(DiscourseRelation::Cause)
        } else if text_lower.contains("however") || text_lower.contains("but") {
            Some(DiscourseRelation::Contrast)
        } else if text_lower.contains("also") || text_lower.contains("furthermore") {
            Some(DiscourseRelation::Conjunction)
        } else if text_lower.contains("therefore") || text_lower.contains("so") {
            Some(DiscourseRelation::Result)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PragmaticAnalysis {
    pub speech_act: SpeechAct,
    pub deictic_references: Vec<DeicticReference>,
    pub implicatures: Vec<Implicature>,
    pub presuppositions: Vec<String>,
    pub discourse_relation: Option<DiscourseRelation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiscourseRelation {
    Conjunction,
    Contrast,
    Cause,
    Result,
    Elaboration,
    Explanation,
}

// ============================================================================
// Discourse Analysis
// ============================================================================

/// Discourse segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseSegment {
    pub id: usize,
    pub text: String,
    pub segment_type: SegmentType,
    pub relations: Vec<DiscourseRelation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SegmentType {
    Utterance,
    Sentence,
    Paragraph,
    Topic,
}

/// Coherence relation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoherenceRelation {
    pub relation_type: CoherenceType,
    pub from_segment: usize,
    pub to_segment: usize,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoherenceType {
    Similarity,
    Contrast,
    Temporal,
    Causal,
    Explanation,
    Elaboration,
}

/// Discourse analyzer
pub struct DiscourseAnalyzer {
    coherence_threshold: f32,
}

impl DiscourseAnalyzer {
    pub fn new() -> Self {
        Self {
            coherence_threshold: 0.5,
        }
    }

    /// Analyze discourse structure
    pub fn analyze(&self, text: &str) -> DiscourseStructure {
        let segments = self.segment_text(text);
        let relations = self.compute_coherence(&segments);
        let topics = self.extract_topics(&segments);

        DiscourseStructure {
            segments,
            coherence_relations: relations,
            topics,
        }
    }

    fn segment_text(&self, text: &str) -> Vec<DiscourseSegment> {
        let sentences: Vec<&str> = text.split(|c| c == '.' || c == '!' || c == '?')
            .filter(|s| !s.trim().is_empty())
            .collect();

        sentences
            .into_iter()
            .enumerate()
            .map(|(i, s)| DiscourseSegment {
                id: i,
                text: s.trim().to_string(),
                segment_type: SegmentType::Sentence,
                relations: vec![],
            })
            .collect()
    }

    fn compute_coherence(&self, segments: &[DiscourseSegment]) -> Vec<CoherenceRelation> {
        let mut relations = Vec::new();

        for i in 0..segments.len().saturating_sub(1) {
            let coherence_type = self.classify_coherence(&segments[i].text, &segments[i + 1].text);
            let strength = self.compute_coherence_strength(&segments[i].text, &segments[i + 1].text);

            if strength > self.coherence_threshold {
                relations.push(CoherenceRelation {
                    relation_type: coherence_type,
                    from_segment: i,
                    to_segment: i + 1,
                    strength,
                });
            }
        }

        relations
    }

    fn classify_coherence(&self, seg1: &str, seg2: &str) -> CoherenceType {
        let s1_lower = seg1.to_lowercase();
        let s2_lower = seg2.to_lowercase();

        if s2_lower.contains("but") || s2_lower.contains("however") {
            CoherenceType::Contrast
        } else if s1_lower.contains("because") || s2_lower.contains("because") {
            CoherenceType::Causal
        } else if s2_lower.contains("also") || s2_lower.contains("additionally") {
            CoherenceType::Similarity
        } else {
            CoherenceType::Elaboration
        }
    }

    fn compute_coherence_strength(&self, seg1: &str, seg2: &str) -> f32 {
        // Simple word overlap as coherence measure
        let words1: HashSet<_> = seg1.split_whitespace().collect();
        let words2: HashSet<_> = seg2.split_whitespace().collect();

        let intersection = words1.intersection(&words2).count() as f32;
        let union = words1.union(&words2).count() as f32;

        if union == 0.0 {
            0.0
        } else {
            intersection / union
        }
    }

    fn extract_topics(&self, segments: &[DiscourseSegment]) -> Vec<Topic> {
        let mut word_freq = HashMap::new();

        for segment in segments {
            for word in segment.text.split_whitespace() {
                let word_lower = word.to_lowercase();
                if word_lower.len() > 3 && !Self::is_stopword(&word_lower) {
                    *word_freq.entry(word_lower).or_insert(0) += 1;
                }
            }
        }

        let mut topics: Vec<_> = word_freq.into_iter().collect();
        topics.sort_by(|a, b| b.1.cmp(&a.1));

        topics.into_iter()
            .take(5)
            .map(|(word, count)| Topic {
                term: word,
                frequency: count,
            })
            .collect()
    }

    fn is_stopword(word: &str) -> bool {
        matches!(
            word,
            "the" | "a" | "an" | "is" | "are" | "was" | "were" | "be" | "been" | "being"
            | "have" | "has" | "had" | "do" | "does" | "did" | "will" | "would" | "could"
            | "should" | "may" | "might" | "must" | "shall" | "can" | "to" | "of" | "in"
            | "for" | "on" | "with" | "at" | "by" | "from" | "as" | "into" | "through"
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscourseStructure {
    pub segments: Vec<DiscourseSegment>,
    pub coherence_relations: Vec<CoherenceRelation>,
    pub topics: Vec<Topic>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub term: String,
    pub frequency: usize,
}

// ============================================================================
// Text Generation
// ============================================================================

/// Text generator
pub struct TextGenerator {
    vocabulary: HashMap<String, f32>,
    ngram_model: NGramModel,
    temperature: f32,
}

#[derive(Debug, Clone)]
pub struct NGramModel {
    n: usize,
    ngrams: HashMap<String, Vec<String>>,
}

impl NGramModel {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            ngrams: HashMap::new(),
        }
    }

    pub fn train(&mut self, corpus: &[String]) {
        for sentence in corpus {
            let words: Vec<String> = std::iter::once("<s>".to_string())
                .chain(sentence.split_whitespace().map(|s| s.to_string()))
                .chain(std::iter::once("</s>".to_string()))
                .collect();

            for window in words.windows(self.n) {
                let key = window[..self.n - 1].join(" ");
                let next_word = window[self.n - 1].clone();

                self.ngrams
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(next_word);
            }
        }
    }

    pub fn predict(&self, context: &[String]) -> Vec<(String, f32)> {
        let key = context.join(" ");
        if let Some(predictions) = self.ngrams.get(&key) {
            let mut freq_map = HashMap::new();
            for word in predictions {
                *freq_map.entry(word.clone()).or_insert(0) += 1;
            }

            let total: usize = freq_map.values().sum();
            freq_map
                .into_iter()
                .map(|(word, count)| (word, count as f32 / total as f32))
                .collect()
        } else {
            vec![]
        }
    }
}

impl TextGenerator {
    pub fn new() -> Self {
        let vocabulary = vec![
            ("the".to_string(), 0.05),
            ("a".to_string(), 0.03),
            ("to".to_string(), 0.04),
            ("and".to_string(), 0.04),
            ("of".to_string(), 0.04),
            ("is".to_string(), 0.02),
            ("in".to_string(), 0.03),
            ("that".to_string(), 0.02),
            ("it".to_string(), 0.02),
            ("for".to_string(), 0.02),
        ].into_iter().collect();

        Self {
            vocabulary,
            ngram_model: NGramModel::new(3),
            temperature: 0.7,
        }
    }

    pub fn with_temperature(mut self, temp: f32) -> Self {
        self.temperature = temp;
        self
    }

    /// Train on corpus
    pub fn train(&mut self, corpus: &[String]) {
        self.ngram_model.train(corpus);

        // Build vocabulary from corpus
        for sentence in corpus {
            for word in sentence.split_whitespace() {
                let word_lower = word.to_lowercase();
                *self.vocabulary.entry(word_lower).or_insert(0.01) += 0.01;
            }
        }
    }

    /// Generate text from semantic input
    pub fn generate(&self, semantic: &SemanticRepresentation, max_length: usize) -> String {
        match semantic {
            SemanticRepresentation::Frame(frame) => {
                self.generate_from_frame(frame, max_length)
            }
            SemanticRepresentation::PredicateLogic(logic) => {
                self.generate_from_logic(logic, max_length)
            }
            _ => "Generated text".to_string(),
        }
    }

    fn generate_from_frame(&self, frame: &Frame, max_length: usize) -> String {
        let mut words = Vec::new();

        // Generate based on frame elements
        if let Some(actor) = frame.elements.get("Actor") {
            words.push(actor.value.clone());
        }

        if let Some(theme) = frame.elements.get("Theme") {
            words.push("the".to_string());
            words.push(theme.value.clone());
        }

        words.push(frame.frame_type.clone());

        // Truncate to max length
        let result = words.join(" ");
        if result.len() > max_length {
            result.chars().take(max_length).collect()
        } else {
            result
        }
    }

    fn generate_from_logic(&self, logic: &PredicateLogic, max_length: usize) -> String {
        let mut parts = Vec::new();

        for pred in &logic.predicates {
            let args = pred.arguments
                .iter()
                .map(|t| match t {
                    Term::Constant(s) => s.clone(),
                    Term::Variable(s) => s.clone(),
                    Term::Function(name, args) => format!("{}({})", name, args.iter()
                        .map(|a| match a {
                            Term::Constant(s) => s.clone(),
                            _ => "x".to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join(", ")),
                })
                .collect::<Vec<_>>()
                .join(" ");

            let pred_str = if pred.negated {
                format!("not {}", pred.name)
            } else {
                pred.name.clone()
            };

            parts.push(format!("{}({})", pred_str, args));
        }

        let result = parts.join(" and ");
        if result.len() > max_length {
            result.chars().take(max_length).collect()
        } else {
            result
        }
    }
}

// ============================================================================
// Main Language Understanding System
// ============================================================================

/// Complete language understanding system
pub struct DeepLanguageUnderstanding {
    // Components
    tokenizer: Tokenizer,
    morphological_analyzer: MorphologicalAnalyzer,
    syntactic_parser: SyntacticParser,
    semantic_parser: SemanticParser,
    pragmatic_analyzer: PragmaticAnalyzer,
    discourse_analyzer: DiscourseAnalyzer,
    generator: TextGenerator,

    // Configuration
    config: LanguageConfig,

    // Context
    conversation_history: VecDeque<Utterance>,
    max_history: usize,
}

#[derive(Debug, Clone)]
pub struct LanguageConfig {
    pub max_tokens: usize,
    pub embedding_dim: usize,
    pub use_neural: bool,
    pub context_window: usize,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            max_tokens: 512,
            embedding_dim: 300,
            use_neural: false, // Simplified: no actual neural models
            context_window: 5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Utterance {
    pub speaker: String,
    pub text: String,
    pub timestamp: DateTime<Utc>,
    pub analysis: Option<UtteranceAnalysis>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UtteranceAnalysis {
    pub tokens: Vec<Token>,
    pub syntactic_tree: SyntacticTree,
    pub dependency_tree: DependencyTree,
    pub semantic: SemanticRepresentation,
    pub pragmatic: PragmaticAnalysis,
    pub discourse: DiscourseStructure,
}

impl DeepLanguageUnderstanding {
    pub fn new() -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            morphological_analyzer: MorphologicalAnalyzer::new(),
            syntactic_parser: SyntacticParser::new(),
            semantic_parser: SemanticParser::new(),
            pragmatic_analyzer: PragmaticAnalyzer::new(),
            discourse_analyzer: DiscourseAnalyzer::new(),
            generator: TextGenerator::new(),
            config: LanguageConfig::default(),
            conversation_history: VecDeque::new(),
            max_history: 20,
        }
    }

    pub fn with_config(config: LanguageConfig) -> Self {
        Self {
            tokenizer: Tokenizer::new(),
            morphological_analyzer: MorphologicalAnalyzer::new(),
            syntactic_parser: SyntacticParser::new(),
            semantic_parser: SemanticParser::new(),
            pragmatic_analyzer: PragmaticAnalyzer::new(),
            discourse_analyzer: DiscourseAnalyzer::new(),
            generator: TextGenerator::new(),
            config,
            conversation_history: VecDeque::new(),
            max_history: 20,
        }
    }

    /// Process input text and return comprehensive analysis
    pub fn process(&mut self, text: &str, speaker: &str) -> UtteranceAnalysis {
        // Tokenize
        let tokens = self.tokenizer.tokenize(text);

        // Tag parts of speech
        let tokens = self.tag_pos(&tokens);

        // Parse syntax
        let constituency_tree = self.syntactic_parser.parse_constituency(&tokens);
        let dependency_tree = self.syntactic_parser.parse_dependency(&tokens);

        // Parse semantics
        let semantic = self.semantic_parser.parse(&tokens, &dependency_tree);

        // Analyze pragmatics
        let pragmatic = self.pragmatic_analyzer.analyze(text, &constituency_tree);

        // Analyze discourse
        let discourse = self.discourse_analyzer.analyze(text);

        let analysis = UtteranceAnalysis {
            tokens,
            syntactic_tree: constituency_tree,
            dependency_tree,
            semantic,
            pragmatic,
            discourse,
        };

        // Store in history
        self.conversation_history.push_back(Utterance {
            speaker: speaker.to_string(),
            text: text.to_string(),
            timestamp: DateTime<Utc>::now(),
            analysis: Some(analysis.clone()),
        });

        // Trim history
        while self.conversation_history.len() > self.max_history {
            self.conversation_history.pop_front();
        }

        analysis
    }

    fn tag_pos(&self, tokens: &[Token]) -> Vec<Token> {
        tokens.iter().map(|t| {
            let mut token = t.clone();
            token.pos_tag = self.guess_pos(t);
            token.lemma = Some(self.guess_lemma(t));
            token
        }).collect()
    }

    fn guess_pos(&self, token: &Token) -> Option<POSTag> {
        let word = &token.text;

        if word.chars().all(|c| c.is_numeric()) {
            Some(POSTag::NUM)
        } else if word.chars().all(|c| c.is_ascii_punctuation()) {
            Some(POSTag::PUNCT)
        } else {
            let lower = word.to_lowercase();

            // Common word patterns
            match lower.as_str() {
                "the" | "a" | "an" => Some(POSTag::DET),
                "is" | "are" | "was" | "were" | "be" | "been" | "being" => Some(POSTag::VERB),
                "and" | "or" | "but" => Some(POSTag::CONJ),
                "to" | "of" | "in" | "for" | "with" | "on" | "at" | "by" => Some(POSTag::ADP),
                "i" | "you" | "he" | "she" | "it" | "we" | "they" => Some(POSTag::PRON),
                "not" => Some(POSTag::PRT),
                "yes" | "no" => Some(POSTag::INTJ),
                _ => {
                    // Suffix-based guessing
                    if lower.ends_with("ly") {
                        Some(POSTag::ADV)
                    } else if lower.ends_with("ing") || lower.ends_with("ed") {
                        Some(POSTag::VERB)
                    } else if lower.ends_with("tion") || lower.ends_with("ness") {
                        Some(POSTag::NOUN)
                    } else if lower.ends_with("ful") || lower.ends_with("less") || lower.ends_with("able") {
                        Some(POSTag::ADJ)
                    } else if lower.ends_with("s") && lower.len() > 2 {
                        Some(POSTag::NOUN)
                    } else {
                        Some(POSTag::NOUN) // Default
                    }
                }
            }
        }
    }

    fn guess_lemma(&self, token: &Token) -> String {
        let word = &token.text.to_lowercase();

        // Common irregular lemmas
        match word.as_str() {
            "was" => "be".to_string(),
            "were" => "be".to_string(),
            "been" => "be".to_string(),
            "being" => "be".to_string(),
            "had" => "have".to_string(),
            "did" => "do".to_string(),
            "done" => "do".to_string(),
            "went" => "go".to_string(),
            "gone" => "go".to_string(),
            "saw" => "see".to_string(),
            "seen" => "see".to_string(),
            "took" => "take".to_string(),
            "taken" => "take".to_string(),
            _ => {
                // Remove common suffixes
                let mut lemma = word.clone();
                if lemma.ends_with("ing") {
                    lemma.truncate(lemma.len() - 3);
                } else if lemma.ends_with("ed") {
                    lemma.truncate(lemma.len() - 2);
                } else if lemma.ends_with("s") && !lemma.ends_with("ss") {
                    lemma.truncate(lemma.len() - 1);
                }
                lemma
            }
        }
    }

    /// Generate response to input
    pub fn generate_response(&self, semantic: &SemanticRepresentation) -> String {
        self.generator.generate(semantic, 100)
    }

    /// Get conversation context
    pub fn get_context(&self, turns: usize) -> Vec<&Utterance> {
        self.conversation_history.iter().rev().take(turns).collect()
    }

    /// Extract entities from text
    pub fn extract_entities(&self, text: &str) -> Vec<Entity> {
        let tokens = self.tokenizer.tokenize(text);
        let tokens = self.tag_pos(&tokens);

        let mut entities = Vec::new();

        for token in &tokens {
            if token.pos_tag == Some(POSTag::PROPN) {
                entities.push(Entity {
                    text: token.text.clone(),
                    entity_type: EntityType::Unknown,
                    start_pos: token.position,
                    end_pos: token.position + 1,
                    confidence: 0.8,
                });
            }
        }

        entities
    }

    /// Perform question answering
    pub fn answer_question(&self, question: &str, context: &str) -> Option<String> {
        // Simple extractive QA
        let question_tokens: Vec<_> = question.split_whitespace().collect();
        let context_lower = context.to_lowercase();

        // Find relevant sentences
        for word in &question_tokens {
            let word_lower = word.to_lowercase();
            if context_lower.contains(&word_lower) && word_lower.len() > 3 {
                // Find surrounding context
                if let Some(pos) = context_lower.find(&word_lower) {
                    let start = pos.saturating_sub(30);
                    let end = (pos + word_lower.len() + 30).min(context.len());
                    return Some(context[start..end].trim().to_string());
                }
            }
        }

        None
    }

    /// Summarize text
    pub fn summarize(&self, text: &str, max_sentences: usize) -> String {
        let sentences: Vec<_> = text.split(|c| c == '.' || c == '!' || c == '?')
            .filter(|s| !s.trim().is_empty())
            .take(max_sentences)
            .map(|s| s.trim().to_string())
            .collect();

        sentences.join(". ") + "."
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub text: String,
    pub entity_type: EntityType,
    pub start_pos: usize,
    pub end_pos: usize,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityType {
    Person,
    Organization,
    Location,
    Date,
    Time,
    Money,
    Percent,
    Unknown,
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenization() {
        let tokenizer = Tokenizer::new();
        let tokens = tokenizer.tokenize("Hello world");

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "Hello");
        assert_eq!(tokens[1].text, "world");
    }

    #[test]
    fn test_pos_tagging() {
        let analyzer = DeepLanguageUnderstanding::new();
        let token = Token::new("running".to_string(), 0, 0, 7);

        let lemma = analyzer.guess_lemma(&token);
        assert_eq!(lemma, "run");
    }

    #[test]
    fn test_morphological_analysis() {
        let analyzer = MorphologicalAnalyzer::new();
        let result = analyzer.analyze("running");

        assert_eq!(result.root, "run");
        assert!(result.morphemes.iter().any(|m| m.morpheme == "ing"));
    }

    #[test]
    fn test_pragmatic_analysis() {
        let analyzer = PragmaticAnalyzer::new();
        let result = analyzer.analyze("Some people came", &SyntacticTree {
            nodes: vec![],
            root: 0,
        });

        // Should detect scalar implicature
        assert!(result.implicatures.iter().any(|i| i.implicature_type == ImplicatureType::Scalar));
    }

    #[test]
    fn test_discourse_analysis() {
        let analyzer = DiscourseAnalyzer::new();
        let result = analyzer.analyze("It was raining. However, we went outside.");

        assert!(result.coherence_relations.iter().any(|r| r.relation_type == CoherenceType::Contrast));
    }

    #[test]
    fn test_entity_extraction() {
        let analyzer = DeepLanguageUnderstanding::new();
        let entities = analyzer.extract_entities("John went to Paris");

        assert!(entities.iter().any(|e| e.text == "John"));
        assert!(entities.iter().any(|e| e.text == "Paris"));
    }

    #[test]
    fn test_summarization() {
        let analyzer = DeepLanguageUnderstanding::new();
        let summary = analyzer.summarize("First sentence. Second sentence. Third sentence.", 2);

        assert!(summary.contains("First"));
        assert!(summary.contains("Second"));
        assert!(!summary.contains("Third"));
    }

    #[test]
    fn test_question_answering() {
        let analyzer = DeepLanguageUnderstanding::new();
        let context = "The capital of France is Paris. It is a beautiful city.";
        let answer = analyzer.answer_question("What is the capital of France?", context);

        assert!(answer.is_some());
        assert!(answer.unwrap().contains("Paris"));
    }

    #[test]
    fn test_conversation_history() {
        let mut analyzer = DeepLanguageUnderstanding::new();
        analyzer.max_history = 3;

        analyzer.process("Hello", "user1");
        analyzer.process("How are you?", "user2");
        analyzer.process("Good", "user1");

        let context = analyzer.get_context(2);
        assert_eq!(context.len(), 2);
    }
}
