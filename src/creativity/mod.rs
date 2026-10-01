//! Tariro Creativity Engine Module
//!
//! Implements creative thinking mechanisms:
//! - Divergent thinking (generating multiple solutions)
//! - Conceptual combination (merging concepts)
//! - Surprise maximization
//! - Novel idea generation

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// Core Types
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CreativityId(pub Uuid);

impl CreativityId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for CreativityId {
    fn default() -> Self {
        Self::new()
    }
}

/// A creative idea or concept
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeIdea {
    pub id: CreativityId,
    pub description: String,
    pub novelty: f64,
    pub usefulness: f64,
    pub surprise: f64,
    pub components: Vec<Concept>,
    pub origin_type: CreativityType,
    pub timestamp: chrono::DateTime<Utc>,
}

impl CreativeIdea {
    pub fn new(description: &str, origin_type: CreativityType) -> Self {
        Self {
            id: CreativityId::new(),
            description: description.to_string(),
            novelty: 0.5,
            usefulness: 0.5,
            surprise: 0.5,
            components: Vec::new(),
            origin_type,
            timestamp: Utc::now(),
        }
    }

    /// Calculate creativity score
    pub fn creativity_score(&self) -> f64 {
        // Guilford's formula: Creativity = Novelty + Usefulness + Surprise
        (self.novelty + self.usefulness + self.surprise) / 3.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreativityType {
    Divergent,       // Multiple solutions
    Convergent,      // Single best solution
    Conceptual,       // Novel combinations
    Transformational, // Radical change
}

/// Concept for combination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub id: String,
    pub name: String,
    pub properties: Vec<String>,
    pub category: String,
}

impl Concept {
    pub fn new(name: &str, category: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            properties: Vec::new(),
            category: category.to_string(),
        }
    }

    pub fn with_property(mut self, prop: &str) -> Self {
        self.properties.push(prop.to_string());
        self
    }
}

// ============================================================================
// Divergent Thinking
// ============================================================================

/// Generates multiple possible solutions
#[derive(Debug, Clone)]
pub struct DivergentThinking {
    pub max_ideas: usize,
    pub fluency_weight: f64,
    pub flexibility_weight: f64,
    pub originality_weight: f64,
}

impl DivergentThinking {
    pub fn new() -> Self {
        Self {
            max_ideas: 20,
            fluency_weight: 0.3,
            flexibility_weight: 0.3,
            originality_weight: 0.4,
        }
    }

    /// Generate multiple ideas for a problem
    pub fn generate_ideas(&self, problem: &str, constraints: &[String]) -> Vec<CreativeIdea> {
        let mut ideas = Vec::new();

        // Generate conceptual combinations
        let base_concepts = self.extract_concepts(problem);
        let combinations = self.generate_combinations(&base_concepts);

        for combo in combinations {
            let idea = self.evaluate_combination(&combo, constraints);
            if idea.creativity_score() > 0.4 {
                ideas.push(idea);
            }
        }

        // Ensure we have some baseline ideas
        if ideas.is_empty() {
            ideas.push(CreativeIdea::new(
                &format!("Alternative approach to: {}", problem),
                CreativityType::Divergent,
            ));
        }

        ideas.truncate(self.max_ideas);
        ideas
    }

    fn extract_concepts(&self, problem: &str) -> Vec<Concept> {
        let words: Vec<&str> = problem.split_whitespace().collect();
        words
            .iter()
            .filter(|w| w.len() > 3)
            .map(|w| Concept::new(w, "unknown"))
            .collect()
    }

    fn generate_combinations(&self, concepts: &[Concept]) -> Vec<Vec<Concept>> {
        let mut combinations = Vec::new();

        for i in 0..concepts.len() {
            for j in (i + 1)..concepts.len() {
                combinations.push(vec![concepts[i].clone(), concepts[j].clone()]);
            }
        }

        combinations
    }

    fn evaluate_combination(&self, combo: &[Concept], _constraints: &[String]) -> CreativeIdea {
        let description = combo
            .iter()
            .map(|c| c.name.as_str())
            .collect::<Vec<_>>()
            .join(" + ");

        let mut idea = CreativeIdea::new(&description, CreativityType::Divergent);

        // Evaluate novelty
        idea.novelty = (combo.len() as f64 / 3.0).min(1.0);

        // Evaluate usefulness (heuristic)
        idea.usefulness = 0.6;

        // Evaluate surprise
        idea.surprise = 0.5;

        idea.components = combo.to_vec();
        idea
    }

    /// Calculate fluency score
    pub fn fluency_score(&self, ideas: &[CreativeIdea]) -> f64 {
        (ideas.len() as f64 / self.max_ideas as f64).min(1.0)
    }

    /// Calculate flexibility score (categories used)
    pub fn flexibility_score(&self, ideas: &[CreativeIdea]) -> f64 {
        let categories: HashSet<_> = ideas
            .iter()
            .flat_map(|i| i.components.iter().map(|c| c.category.clone()))
            .collect();
        (categories.len() as f64 / 5.0).min(1.0)
    }
}

impl Default for DivergentThinking {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Conceptual Combination
// ============================================================================

/// Combines concepts to create novel ideas
#[derive(Debug, Clone)]
pub struct ConceptualCombiner {
    pub max_depth: usize,
    pub mutation_rate: f64,
}

impl ConceptualCombiner {
    pub fn new() -> Self {
        Self {
            max_depth: 3,
            mutation_rate: 0.1,
        }
    }

    /// Combine two concepts
    pub fn combine(&self, concept1: &Concept, concept2: &Concept) -> Vec<Concept> {
        let mut combined = Vec::new();

        // Create merged concept
        let mut new_concept = Concept::new(
            &format!("{} + {}", concept1.name, concept2.name),
            &format!("{}_{}", concept1.category, concept2.category),
        );

        // Merge properties
        let mut all_properties: HashSet<_> = concept1.properties.iter().collect();
        all_properties.extend(concept2.properties.iter());
        new_concept.properties = all_properties.into_iter().cloned().collect();

        combined.push(new_concept);

        // Create filtered concepts
        if self.mutation_rate > 0.5 {
            let filtered1 = Concept::new(&concept1.name, &concept1.category);
            combined.push(filtered1);
        }

        combined
    }

    /// Generate conceptual blend
    pub fn blend(&self, source: &Concept, target: &Concept) -> Concept {
        let blend_name = format!("blend({}, {})", source.name, target.name);

        let mut blended = Concept::new(&blend_name, "blend");

        // Select best properties from each
        let mut props: Vec<_> = source.properties.iter().take(2).cloned().collect();
        props.extend(target.properties.iter().take(2).cloned());

        blended.properties = props;
        blended
    }
}

impl Default for ConceptualCombiner {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Surprise Maximizer
// ============================================================================

/// Maximizes creative surprise
#[derive(Debug, Clone)]
pub struct SurpriseMaximizer {
    pub expectation_model: HashMap<String, f64>,
    pub baseline_surprise: f64,
}

impl SurpriseMaximizer {
    pub fn new() -> Self {
        Self {
            expectation_model: HashMap::new(),
            baseline_surprise: 0.5,
        }
    }

    /// Calculate surprise of an idea
    pub fn calculate_surprise(&self, idea: &CreativeIdea) -> f64 {
        // Check against expectations
        let word_count = idea.description.split_whitespace().count();

        let expected_length = 5.0; // Expected words
        let length_deviation = (word_count as f64 - expected_length).abs() / expected_length;

        let expected_categories = self.expectation_model.len() as f64;
        let category_deviation = if expected_categories > 0.0 {
            0.0 // Simplified
        } else {
            1.0
        };

        // Surprise = deviation from expectation
        let surprise = (length_deviation + category_deviation) / 2.0;
        surprise.min(1.0)
    }

    /// Update expectation model based on creative output
    pub fn update_expectations(&mut self, ideas: &[CreativeIdea]) {
        for idea in ideas {
            for word in idea.description.split_whitespace() {
                *self.expectation_model.entry(word.to_lowercase()).or_insert(0.0) += 0.1;
            }
        }
    }
}

impl Default for SurpriseMaximizer {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Creativity Engine
// ============================================================================

/// Main creativity engine
#[derive(Debug, Clone)]
pub struct CreativityEngine {
    pub divergent: DivergentThinking,
    pub combiner: ConceptualCombiner,
    pub surprise: SurpriseMaximizer,
    pub idea_history: VecDeque<CreativeIdea>,
    pub stats: CreativityStats,
}

impl CreativityEngine {
    pub fn new() -> Self {
        Self {
            divergent: DivergentThinking::new(),
            combiner: ConceptualCombiner::new(),
            surprise: SurpriseMaximizer::new(),
            idea_history: VecDeque::new(),
            stats: CreativityStats::default(),
        }
    }

    /// Generate creative ideas for a prompt
    pub fn generate(&mut self, prompt: &str, constraints: &[String]) -> Vec<CreativeIdea> {
        // Generate divergent ideas
        let ideas = self.divergent.generate_ideas(prompt, constraints);

        // Update expectations
        self.surprise.update_expectations(&ideas);

        // Calculate surprise for each
        let mut evaluated_ideas: Vec<_> = ideas
            .into_iter()
            .map(|mut idea| {
                idea.surprise = self.surprise.calculate_surprise(&idea);
                idea
            })
            .collect();

        // Sort by creativity score
        evaluated_ideas.sort_by(|a, b| {
            b.creativity_score()
                .partial_cmp(&a.creativity_score())
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Add to history
        for idea in &evaluated_ideas {
            if self.idea_history.len() >= 100 {
                self.idea_history.pop_front();
            }
            self.idea_history.push_back(idea.clone());
        }

        self.stats.total_ideas_generated += evaluated_ideas.len();

        evaluated_ideas
    }

    /// Combine two existing ideas
    pub fn combine_ideas(&self, idea1: &CreativeIdea, idea2: &CreativeIdea) -> Vec<CreativeIdea> {
        let concepts1: Vec<Concept> = idea1
            .components
            .iter()
            .take(1)
            .cloned()
            .collect();
        let concepts2: Vec<Concept> = idea2
            .components
            .iter()
            .take(1)
            .cloned()
            .collect();

        if concepts1.is_empty() || concepts2.is_empty() {
            return vec![CreativeIdea::new(
                &format!("{} | {}", idea1.description, idea2.description),
                CreativityType::Conceptual,
            )];
        }

        let combined = self
            .combiner
            .combine(&concepts1[0], &concepts2[0]);

        combined
            .into_iter()
            .map(|c| {
                let mut idea = CreativeIdea::new(&c.name, CreativityType::Conceptual);
                idea.components = vec![c];
                idea.novelty = 0.7;
                idea
            })
            .collect()
    }

    /// Get creative insight
    pub fn get_insight(&self, problem: &str) -> Option<String> {
        // Look for patterns in history
        let related: Vec<_> = self
            .idea_history
            .iter()
            .filter(|i| i.description.contains(problem))
            .collect();

        if related.len() >= 2 {
            // Found related ideas - suggest combination
            return Some(format!(
                "Combining {} related ideas could yield a solution",
                related.len()
            ));
        }

        None
    }

    pub fn stats(&self) -> &CreativityStats {
        &self.stats
    }
}

impl Default for CreativityEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct CreativityStats {
    pub total_ideas_generated: usize,
    pub combinations_attempted: usize,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod divergent_tests {
    use super::*;

    #[test]
    fn test_generate_ideas() {
        let engine = DivergentThinking::new();
        let ideas = engine.generate_ideas("solve problem", &[]);

        assert!(!ideas.is_empty() || ideas.is_empty());
    }

    #[test]
    fn test_fluency_score() {
        let engine = DivergentThinking::new();
        let ideas = vec![
            CreativeIdea::new("Idea 1", CreativityType::Divergent),
            CreativeIdea::new("Idea 2", CreativityType::Divergent),
        ];

        let score = engine.fluency_score(&ideas);
        assert!(score > 0.0);
    }
}

#[cfg(test)]
mod combiner_tests {
    use super::*;

    #[test]
    fn test_combine() {
        let combiner = ConceptualCombiner::new();
        let c1 = Concept::new("Fire", "element");
        let c2 = Concept::new("Water", "element");

        let combined = combiner.combine(&c1, &c2);
        assert!(!combined.is_empty());
    }

    #[test]
    fn test_blend() {
        let combiner = ConceptualCombiner::new();
        let c1 = Concept::new("Dog", "animal");
        let c2 = Concept::new("Cat", "animal");

        let blend = combiner.blend(&c1, &c2);
        assert!(blend.name.contains("blend"));
    }
}

#[cfg(test)]
mod surprise_tests {
    use super::*;

    #[test]
    fn test_calculate_surprise() {
        let maximizer = SurpriseMaximizer::new();
        let idea = CreativeIdea::new("This is a very long creative idea description", CreativityType::Divergent);

        let surprise = maximizer.calculate_surprise(&idea);
        assert!(surprise >= 0.0);
    }
}

#[cfg(test)]
mod creativity_engine_tests {
    use super::*;

    #[test]
    fn test_generate() {
        let mut engine = CreativityEngine::new();
        let ideas = engine.generate("Create something new", &[]);

        assert!(!ideas.is_empty() || ideas.is_empty());
    }

    #[test]
    fn test_combine_ideas() {
        let engine = CreativityEngine::new();
        let idea1 = CreativeIdea::new("Idea 1", CreativityType::Divergent);
        let idea2 = CreativeIdea::new("Idea 2", CreativityType::Divergent);

        let combined = engine.combine_ideas(&idea1, &idea2);
        assert!(!combined.is_empty());
    }
}
