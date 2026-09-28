// Tariro AGI - Creativity Engine Module
// Analogical reasoning and creative concept generation

use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

// ============================================================================
// Creativity Types
// ============================================================================

/// Creativity engine for analogical reasoning and novel concept generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativityEngine {
    pub engine_id: String,
    pub analogy_graph: AnalogyGraph,
    pub concept_spaces: Vec<ConceptSpace>,
    pub creative_products: Vec<CreativeProduct>,
    pub divergent_thinking: DivergentThinking,
    pub convergent_thinking: ConvergentThinking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogyGraph {
    pub source_concepts: Vec<Concept>,
    pub target_concepts: Vec<Concept>,
    pub mappings: Vec<Mapping>,
    pub constraints: Vec<MappingConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Concept {
    pub concept_id: String,
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub relations: Vec<Relation>,
    pub abstractions: Vec<String>,
    pub instances: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    pub value: AttributeValue,
    pub importance: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    Numeric(f32),
    Categorical(String),
    Boolean(bool),
    Structured(Vec<(String, String)>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub relation_type: String,
    pub target: String,
    pub strength: f32,
    pub bidirectional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mapping {
    pub mapping_id: String,
    pub source_element: String,
    pub target_element: String,
    pub mapping_type: MappingType,
    pub confidence: f32,
    pub justification: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MappingType {
    Literal,
    Systematic,
    Metaphorical,
    Structural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MappingConstraint {
    pub constraint_type: ConstraintType,
    pub source_constraint: String,
    pub target_constraint: String,
    pub preserved: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    Relational,
    Attribute,
    Structural,
}

// ============================================================================
// Concept Spaces
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptSpace {
    pub space_id: String,
    pub space_type: SpaceType,
    pub dimensions: Vec<Dimension>,
    pub concepts: Vec<Concept>,
    pub similarity_cache: HashMap<(String, String), f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimension {
    pub name: String,
    pub dim_type: DimensionType,
    pub range: (f32, f32),
    pub weight: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DimensionType {
    Continuous,
    Categorical,
    Ordinal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpaceType {
    Semantic,
    Associative,
    Conceptual,
    Metaphorical,
}

// ============================================================================
// Creative Products
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeProduct {
    pub product_id: String,
    pub product_type: ProductType,
    pub description: String,
    pub novelty: f32,
    pub value: f32,
    pub inspiration_sources: Vec<String>,
    pub generation_process: GenerationProcess,
    pub evaluation: Option<Evaluation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductType {
    Concept,
    Solution,
    Metaphor,
    Story,
    Art,
    Method,
    Theory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationProcess {
    pub process_type: ProcessType,
    pub steps: Vec<ProcessStep>,
    pub techniques_used: Vec<CreativityTechnique>,
    pub incubation_period: Option<Duration>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProcessType {
    Combinational,
    Exploratory,
    Transformational,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStep {
    pub step_id: String,
    pub description: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreativityTechnique {
    Brainstorming,
    LateralThinking,
    AnalogicalReasoning,
    RandomInput,
    SCAMPER,
    MindMapping,
    MorphologicalAnalysis,
    Synectics,
    ConstraintRelaxation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evaluation {
    pub novelty_score: f32,
    pub value_score: f32,
    pub feasibility_score: f32,
    pub overall_score: f32,
    pub evaluator_notes: String,
}

// ============================================================================
// Divergent and Convergent Thinking
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DivergentThinking {
    pub fluency: usize,
    pub flexibility: usize,
    pub originality: f32,
    pub elaboration: usize,
    pub ideas_generated: Vec<Idea>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Idea {
    pub idea_id: String,
    pub description: String,
    pub category: String,
    pub novelty: f32,
    pub feasibility: f32,
    pub elaboration_level: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergentThinking {
    pub criteria: Vec<EvaluationCriterion>,
    pub ranked_ideas: Vec<RankedIdea>,
    pub best_ideas: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCriterion {
    pub criterion_id: String,
    pub name: String,
    pub weight: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankedIdea {
    pub idea_id: String,
    pub score: f32,
    pub rank: usize,
    pub scores_by_criterion: HashMap<String, f32>,
}

use std::time::Duration;

// ============================================================================
// Implementation
// ============================================================================

impl CreativityEngine {
    pub fn new() -> Self {
        Self {
            engine_id: generate_id(),
            analogy_graph: AnalogyGraph {
                source_concepts: vec![],
                target_concepts: vec![],
                mappings: vec![],
                constraints: vec![],
            },
            concept_spaces: vec![],
            creative_products: vec![],
            divergent_thinking: DivergentThinking {
                fluency: 0,
                flexibility: 0,
                originality: 0.0,
                elaboration: 0,
                ideas_generated: vec![],
            },
            convergent_thinking: ConvergentThinking {
                criteria: vec![],
                ranked_ideas: vec![],
                best_ideas: vec![],
            },
        }
    }

    /// Find analogical mappings between source and target
    pub fn find_analogies(
        &self,
        source: &Concept,
        target: &Concept,
    ) -> Vec<Analogy> {
        let mut analogies = Vec::new();

        // Find attribute mappings
        for src_attr in &source.attributes {
            for tgt_attr in &target.attributes {
                let similarity = self.attribute_similarity(src_attr, tgt_attr);

                if similarity > 0.5 {
                    analogies.push(Analogy {
                        analogy_id: generate_id(),
                        source_element: src_attr.name.clone(),
                        target_element: tgt_attr.name.clone(),
                        similarity,
                        mapping_type: MappingType::Literal,
                        inference: self.generate_inference(src_attr, tgt_attr),
                    });
                }
            }
        }

        // Find relational mappings
        for src_rel in &source.relations {
            for tgt_rel in &target.relations {
                if src_rel.relation_type == tgt_rel.relation_type {
                    analogies.push(Analogy {
                        analogy_id: generate_id(),
                        source_element: src_rel.target.clone(),
                        target_element: tgt_rel.target.clone(),
                        similarity: src_rel.strength.min(tgt_rel.strength),
                        mapping_type: MappingType::Structural,
                        inference: format!(
                            "{} relates to {} similarly to {} relating to {}",
                            source.name, src_rel.target, target.name, tgt_rel.target
                        ),
                    });
                }
            }
        }

        analogies
    }

    /// Compute attribute similarity
    fn attribute_similarity(&self, a: &Attribute, b: &Attribute) -> f32 {
        if a.name != b.name {
            return 0.0;
        }

        match (&a.value, &b.value) {
            (AttributeValue::Numeric(na), AttributeValue::Numeric(nb)) => {
                let diff = (na - nb).abs();
                (1.0 - diff.min(1.0))
            }
            (AttributeValue::Categorical(ca), AttributeValue::Categorical(cb)) => {
                if ca == cb { 1.0 } else { 0.0 }
            }
            (AttributeValue::Boolean(ba), AttributeValue::Boolean(bb)) => {
                if ba == bb { 1.0 } else { 0.0 }
            }
            _ => 0.5,
        }
    }

    /// Generate inference from analogy
    fn generate_inference(&self, src_attr: &Attribute, tgt_attr: &Attribute) -> String {
        format!(
            "If {} is {}, then analogous attribute {} might have similar properties",
            src_attr.name, format!("{:?}", src_attr.value), tgt_attr.name
        )
    }

    /// Generate creative concepts using combination
    pub fn combinational_creativity(
        &self,
        concepts: &[&Concept],
        combination_strategy: CombinationStrategy,
    ) -> Vec<CreativeProduct> {
        let mut products = Vec::new();

        match combination_strategy {
            CombinationStrategy::Random => {
                // Randomly combine attributes from different concepts
                for concept in concepts {
                    for attr in &concept.attributes {
                        products.push(CreativeProduct {
                            product_id: generate_id(),
                            product_type: ProductType::Concept,
                            description: format!("Novel concept based on {}", attr.name),
                            novelty: 0.7,
                            value: 0.6,
                            inspiration_sources: vec![concept.name.clone()],
                            generation_process: GenerationProcess {
                                process_type: ProcessType::Combinational,
                                steps: vec![],
                                techniques_used: vec![CreativityTechnique::RandomInput],
                                incubation_period: None,
                            },
                            evaluation: None,
                        });
                    }
                }
            }
            CombinationStrategy::Systematic => {
                // Systematically combine all attributes
                let attributes: Vec<_> = concepts.iter()
                    .flat_map(|c| c.attributes.iter())
                    .collect();

                for i in 0..attributes.len() {
                    for j in (i + 1)..attributes.len() {
                        products.push(CreativeProduct {
                            product_id: generate_id(),
                            product_type: ProductType::Concept,
                            description: format!(
                                "Combined concept: {} + {}",
                                attributes[i].name, attributes[j].name
                            ),
                            novelty: 0.6,
                            value: 0.5,
                            inspiration_sources: concepts.iter()
                                .map(|c| c.name.clone())
                                .collect(),
                            generation_process: GenerationProcess {
                                process_type: ProcessType::Combinational,
                                steps: vec![],
                                techniques_used: vec![CreativityTechnique::MorphologicalAnalysis],
                                incubation_period: None,
                            },
                            evaluation: None,
                        });
                    }
                }
            }
            CombinationStrategy::ConceptualBlend => {
                // Conceptual blending
                if concepts.len() >= 2 {
                    products.push(CreativeProduct {
                        product_id: generate_id(),
                        product_type: ProductType::Concept,
                        description: format!(
                            "Blend of {} and {}",
                            concepts[0].name, concepts[1].name
                        ),
                        novelty: 0.8,
                        value: 0.7,
                        inspiration_sources: concepts.iter()
                            .map(|c| c.name.clone())
                            .collect(),
                        generation_process: GenerationProcess {
                            process_type: ProcessType::Combinational,
                            steps: vec![],
                            techniques_used: vec![CreativityTechnique::AnalogicalReasoning],
                            incubation_period: None,
                        },
                        evaluation: None,
                    });
                }
            }
        }

        products
    }

    /// Generate metaphors
    pub fn generate_metaphor(&self, source: &str, target: &str) -> Metaphor {
        let mapping = self.find_surface_mappings(source, target);

        Metaphor {
            metaphor_id: generate_id(),
            source_domain: source.to_string(),
            target_domain: target.to_string(),
            mappings: mapping,
            literal_claim: format!("{} is {}", target, source),
            metaphorical_interpretation: format!("Understanding {} in terms of {}", target, source),
            novel_insight: self.generate_metaphorical_insight(source, target),
        }
    }

    fn find_surface_mappings(&self, source: &str, target: &str) -> Vec<SurfaceMapping> {
        // Simple surface-level mappings
        vec![SurfaceMapping {
            source_element: source.to_string(),
            target_element: target.to_string(),
            mapping_type: "surface".to_string(),
        }]
    }

    fn generate_metaphorical_insight(&self, source: &str, target: &str) -> String {
        format!(
            "Understanding {} as {} reveals that both share fundamental structures",
            target, source
        )
    }

    /// Divergent thinking - generate many ideas
    pub fn divergent_think(
        &mut self,
        prompt: &str,
        quantity_target: usize,
    ) -> Vec<Idea> {
        let mut ideas = Vec::new();
        let categories = vec!["improvement", "alternative", "extension", "combination"];

        for i in 0..quantity_target {
            let category = categories[i % categories.len()].to_string();

            ideas.push(Idea {
                idea_id: generate_id(),
                description: format!("Idea {} for: {}", i + 1, prompt),
                category: category.clone(),
                novelty: self.compute_novelty(&category),
                feasibility: self.compute_feasibility(&category),
                elaboration_level: 1,
            });
        }

        // Update divergent thinking metrics
        self.divergent_thinking.fluency = ideas.len();
        self.divergent_thinking.flexibility = categories.len();
        self.divergent_thinking.originality = ideas.iter()
            .map(|i| i.novelty)
            .sum::<f32>() / ideas.len() as f32;
        self.divergent_thinking.elaboration = ideas.iter()
            .map(|i| i.elaboration_level)
            .sum::<usize>();
        self.divergent_thinking.ideas_generated = ideas.clone();

        ideas
    }

    fn compute_novelty(&self, category: &str) -> f32 {
        match category {
            "improvement" => 0.5,
            "alternative" => 0.7,
            "extension" => 0.6,
            "combination" => 0.8,
            _ => 0.5,
        }
    }

    fn compute_feasibility(&self, category: &str) -> f32 {
        match category {
            "improvement" => 0.8,
            "alternative" => 0.6,
            "extension" => 0.7,
            "combination" => 0.5,
            _ => 0.6,
        }
    }

    /// Convergent thinking - evaluate and select best ideas
    pub fn convergent_think(
        &mut self,
        ideas: Vec<Idea>,
        criteria: Vec<EvaluationCriterion>,
    ) -> Vec<RankedIdea> {
        let mut ranked: Vec<RankedIdea> = ideas.iter()
            .map(|idea| {
                let mut scores = HashMap::new();
                let mut total_score = 0.0f32;

                for criterion in &criteria {
                    let score = match criterion.name.as_str() {
                        "novelty" => idea.novelty * criterion.weight,
                        "feasibility" => idea.feasibility * criterion.weight,
                        "value" => idea.novelty * idea.feasibility * criterion.weight,
                        _ => 0.5 * criterion.weight,
                    };
                    scores.insert(criterion.name.clone(), score);
                    total_score += score;
                }

                RankedIdea {
                    idea_id: idea.idea_id.clone(),
                    score: total_score,
                    rank: 0,
                    scores_by_criterion: scores,
                }
            })
            .collect();

        // Sort by score
        ranked.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));

        // Assign ranks
        for (i, ranked_idea) in ranked.iter_mut().enumerate() {
            ranked_idea.rank = i + 1;
        }

        // Update convergent thinking
        self.convergent_thinking.criteria = criteria;
        self.convergent_thinking.ranked_ideas = ranked.clone();
        self.convergent_thinking.best_ideas = ranked.iter()
            .take(3)
            .map(|r| r.idea_id.clone())
            .collect();

        ranked
    }

    /// Evaluate creativity
    pub fn evaluate_creativity(&self, product: &CreativeProduct) -> Evaluation {
        let novelty_score = product.novelty;
        let value_score = product.value;

        // Feasibility is often inversely related to novelty
        let feasibility_score = 1.0 - (novelty_score * 0.3);

        let overall = (novelty_score * 0.4 + value_score * 0.4 + feasibility_score * 0.2);

        Evaluation {
            novelty_score,
            value_score,
            feasibility_score,
            overall_score: overall,
            evaluator_notes: "Creativity evaluation complete".to_string(),
        }
    }

    /// Transform existing concept
    pub fn transformational_creativity(
        &self,
        concept: &Concept,
        transformation: Transformation,
    ) -> Concept {
        let mut new_concept = concept.clone();
        new_concept.concept_id = generate_id();
        new_concept.name = format!("Transformed: {}", concept.name);

        match transformation {
            Transformation::AttributeChange(attr_name, new_value) => {
                for attr in &mut new_concept.attributes {
                    if attr.name == attr_name {
                        attr.value = new_value;
                        break;
                    }
                }
            }
            Transformation::RelationInversion => {
                for rel in &mut new_concept.relations {
                    rel.bidirectional = !rel.bidirectional;
                }
            }
            Transformation::AbstractionLevel(level) => {
                new_concept.abstractions = new_concept.abstractions.iter()
                    .take(level)
                    .cloned()
                    .collect();
            }
        }

        new_concept
    }
}

impl Default for CreativityEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Analogy structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analogy {
    pub analogy_id: String,
    pub source_element: String,
    pub target_element: String,
    pub similarity: f32,
    pub mapping_type: MappingType,
    pub inference: String,
}

/// Combination strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombinationStrategy {
    Random,
    Systematic,
    ConceptualBlend,
}

/// Metaphor structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metaphor {
    pub metaphor_id: String,
    pub source_domain: String,
    pub target_domain: String,
    pub mappings: Vec<SurfaceMapping>,
    pub literal_claim: String,
    pub metaphorical_interpretation: String,
    pub novel_insight: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceMapping {
    pub source_element: String,
    pub target_element: String,
    pub mapping_type: String,
}

/// Transformation types
#[derive(Debug, Clone, PartialEq)]
pub enum Transformation {
    AttributeChange(String, AttributeValue),
    RelationInversion,
    AbstractionLevel(usize),
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("crt_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creativity_engine_creation() {
        let engine = CreativityEngine::new();
        assert!(!engine.engine_id.is_empty());
    }

    #[test]
    fn test_divergent_thinking() {
        let mut engine = CreativityEngine::new();
        let ideas = engine.divergent_think("improve cooking", 5);

        assert_eq!(engine.divergent_thinking.fluency, 5);
        assert_eq!(ideas.len(), 5);
    }

    #[test]
    fn test_convergent_thinking() {
        let mut engine = CreativityEngine::new();
        let ideas = engine.divergent_think("solve problem", 3);

        let criteria = vec![EvaluationCriterion {
            criterion_id: "crit1".to_string(),
            name: "novelty".to_string(),
            weight: 0.5,
            description: "How novel is the idea".to_string(),
        }];

        let ranked = engine.convergent_think(ideas, criteria);
        assert!(!ranked.is_empty());
        assert_eq!(ranked[0].rank, 1);
    }
}
