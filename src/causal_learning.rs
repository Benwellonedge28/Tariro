// Tariro AGI - Causal Learning Module
// Learning causal relationships from observations

use std::collections::{HashMap, HashSet, VecDeque};
use chrono::{DateTime, Utc};
use std::time::Duration;
use rand::Rng;

// ============================================================================
// Causal Learning Types
// ============================================================================

/// Causal learning engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLearning {
    pub learning_id: String,
    pub causal_graph: CausalGraph,
    pub mechanisms: Vec<CausalMechanism>,
    pub interventions: Vec<Intervention>,
    pub observations: Vec<Observation>,
    pub learned_structure: CausalStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalGraph {
    pub nodes: Vec<CausalNode>,
    pub edges: Vec<CausalEdge>,
    pub adj_matrix: Vec<Vec<f32>>,
}

#[derive(Debug, Clone)]
pub struct CausalNode {
    pub node_id: String,
    pub name: String,
    pub node_type: CausalNodeType,
    pub properties: HashMap<String, String>,
    pub domain: Vec<String>,
    pub is_observed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalNodeType {
    Continuous,
    Discrete,
    Categorical,
    Counterfactual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEdge {
    pub source: String,
    pub target: String,
    pub edge_type: CausalEdgeType,
    pub strength: f32,
    pub mechanism: Option<String>,
    pub confounders: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CausalEdgeType {
    Direct,
    Indirect,
    Bidirected,
    Confounded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalMechanism {
    pub mechanism_id: String,
    pub cause_variables: Vec<String>,
    pub effect_variable: String,
    pub function_type: MechanismType,
    pub parameters: Vec<f32>,
    pub noise_distribution: NoiseDistribution,
    pub learned_from: Vec<String>,  // Observation IDs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MechanismType {
    Linear,
    Nonlinear,
    Step,
    Threshold,
    Probabilistic,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NoiseDistribution {
    Gaussian(f32),
    Uniform(f32, f32),
    Exponential(f32),
    Cauchy,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    pub intervention_id: String,
    pub variable: String,
    pub value: String,
    pub effect_on_graph: InterventionEffect,
    pub observed_outcomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InterventionEffect {
    DoAssign,
    DoRemove,
    DoConditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Observation {
    pub observation_id: String,
    pub timestamp: DateTime<Utc>,
    pub values: HashMap<String, String>,
    pub context: String,
    pub is_intervention: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalStructure {
    pub structure_confidence: f32,
    pub missing_edges: Vec<(String, String)>,
    pub additional_edges: Vec<(String, String)>,
    pub equivalent_classes: Vec<CausalGraph>,
}

// ============================================================================
// PC Algorithm for Causal Discovery
// ============================================================================

/// Causal structure learner using PC algorithm
pub struct CausalLearner {
    // Statistical tests
    statistical_tests: HashMap<String, StatisticalTest>,

    // Conditional independence tests
    ci_tests: Vec<ConditionalIndependenceTest>,

    // Learned structure
    skeleton: Skeleton,
    oriented_edges: Vec<OrientedEdge>,

    // Configuration
    config: CausalLearningConfig,

    // Statistics
    stats: CausalLearningStats,
}

#[derive(Debug, Clone)]
pub struct StatisticalTest {
    pub test_id: String,
    pub test_type: TestType,
    pub statistic: f32,
    pub p_value: f32,
    pub sample_size: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestType {
    ChiSquare,
    GTest,
    FisherExact,
    MutualInformation,
    PartialCorrelation,
    GrangerCausality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalIndependenceTest {
    pub variable_x: String,
    pub variable_y: String,
    pub conditioning_set: Vec<String>,
    pub is_independent: bool,
    pub p_value: f32,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct Skeleton {
    pub undirected_edges: Vec<UndirectedEdge>,
    pub separation_sets: HashMap<(String, String), Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct UndirectedEdge {
    pub node1: String,
    pub node2: String,
    pub strength: f32,
    pub removed: bool,
}

#[derive(Debug, Clone)]
pub struct OrientedEdge {
    pub source: String,
    pub target: String,
    pub orientation_confidence: f32,
    pub rule_applied: OrientationRule,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrientationRule {
    VStructure,
    MeekRule1,
    MeekRule2,
    MeekRule3,
    MeekRule4,
    Triangle,
    MarkovCondition,
}

#[derive(Debug, Clone)]
pub struct CausalLearningConfig {
    pub alpha: f32,  // Significance level
    pub max_conditioning_set: usize,
    pub orient_using_background_knowledge: bool,
    pub handle_latent_confounders: bool,
    pub use_intervention_data: bool,
    pub iterative_screening: bool,
    pub conservative_pc: bool,
}

impl Default for CausalLearningConfig {
    fn default() -> Self {
        Self {
            alpha: 0.05,
            max_conditioning_set: 3,
            orient_using_background_knowledge: true,
            handle_latent_confounders: true,
            use_intervention_data: true,
            iterative_screening: true,
            conservative_pc: false,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct CausalLearningStats {
    pub ci_tests_performed: usize,
    pub edges_removed: usize,
    pub orientations_applied: usize,
    pub structures_learned: usize,
    pub learning_time_ms: u64,
}

// ============================================================================
// Implementation
// ============================================================================

impl CausalLearner {
    pub fn new() -> Self {
        Self {
            statistical_tests: HashMap::new(),
            ci_tests: Vec::new(),
            skeleton: Skeleton {
                undirected_edges: Vec::new(),
                separation_sets: HashMap::new(),
            },
            oriented_edges: Vec::new(),
            config: CausalLearningConfig::default(),
            stats: CausalLearningStats::default(),
        }
    }

    /// Learn causal structure from observations
    pub fn learn_structure(&mut self, variables: &[String], observations: &[Observation]) -> CausalGraph {
        let start_time = Utc::now();

        // Step 1: Initialize complete undirected graph
        self.initialize_skeleton(variables);

        // Step 2: Edge removal through conditional independence tests
        self.conditional_independence_screening(variables, observations);

        // Step 3: Orient edges
        self.orient_edges(variables);

        // Step 4: Apply Meek rules for additional orientations
        self.apply_meek_rules();

        // Build final causal graph
        let graph = self.build_causal_graph(variables);

        self.stats.learning_time_ms = Utc::now().signed_duration_since(start_time).to_std().unwrap_or_default().as_millis() as u64;

        graph
    }

    /// Initialize complete undirected graph
    fn initialize_skeleton(&mut self, variables: &[String]) {
        self.skeleton.undirected_edges.clear();
        self.skeleton.separation_sets.clear();

        for i in 0..variables.len() {
            for j in (i + 1)..variables.len() {
                self.skeleton.undirected_edges.push(UndirectedEdge {
                    node1: variables[i].clone(),
                    node2: variables[j].clone(),
                    strength: 1.0,
                    removed: false,
                });
            }
        }
    }

    /// Conditional independence screening
    fn conditional_independence_screening(&mut self, variables: &[String], observations: &[Observation]) {
        let mut depth = 0;

        while depth <= self.config.max_conditioning_set {
            for edge in &mut self.skeleton.undirected_edges {
                if edge.removed {
                    continue;
                }

                // Get conditioning set candidates
                let others: Vec<String> = variables.iter()
                    .filter(|v| *v != &edge.node1 && *v != &edge.node2)
                    .cloned()
                    .collect();

                // Skip if not enough variables for conditioning
                if others.len() < depth {
                    continue;
                }

                // Test all conditioning sets of current depth
                for cond_set in self.combinations(&others, depth) {
                    let test = self.test_conditional_independence(
                        &edge.node1,
                        &edge.node2,
                        &cond_set,
                        observations
                    );

                    self.stats.ci_tests_performed += 1;

                    if test.is_independent {
                        edge.removed = true;
                        self.stats.edges_removed += 1;

                        // Record separation set
                        let key = if edge.node1 < edge.node2 {
                            (edge.node1.clone(), edge.node2.clone())
                        } else {
                            (edge.node2.clone(), edge.node1.clone())
                        };
                        self.skeleton.separation_sets.insert(key, cond_set);
                        break;
                    }
                }
            }

            depth += 1;
        }
    }

    /// Test conditional independence
    fn test_conditional_independence(
        &self,
        x: &str,
        y: &str,
        z: &[String],
        observations: &[Observation],
    ) -> ConditionalIndependenceTest {
        // Compute partial correlation or conditional mutual information
        let x_values: Vec<f32> = observations.iter()
            .filter_map(|o| o.values.get(x).and_then(|v| v.parse().ok()))
            .collect();

        let y_values: Vec<f32> = observations.iter()
            .filter_map(|o| o.values.get(y).and_then(|v| v.parse().ok()))
            .collect();

        let z_values: Vec<Vec<f32>> = z.iter()
            .map(|zv| {
                observations.iter()
                    .filter_map(|o| o.values.get(zv).and_then(|v| v.parse().ok()))
                    .collect()
            })
            .collect();

        // Compute conditional mutual information or partial correlation
        let cmi = self.compute_cmi(&x_values, &y_values, &z_values);

        // Chi-square test or Fisher's z-test
        let statistic = cmi * 2.0 * observations.len() as f32;
        let df = 1;  // Degrees of freedom

        // Approximate p-value using chi-square distribution
        let p_value = self.chi_square_pvalue(statistic, df);

        let is_independent = p_value > self.config.alpha;

        ConditionalIndependenceTest {
            variable_x: x.to_string(),
            variable_y: y.to_string(),
            conditioning_set: z.to_vec(),
            is_independent,
            p_value,
            confidence: 1.0 - p_value,
        }
    }

    /// Compute conditional mutual information
    fn compute_cmi(&self, x: &[f32], y: &[f32], z: &[Vec<f32>]) -> f32 {
        if x.is_empty() || y.is_empty() || z.is_empty() {
            return 0.0;
        }

        // Simple mutual information for now
        let n = x.len().max(y.len()) as f32;
        let mean_x: f32 = x.iter().sum::<f32>() / n;
        let mean_y: f32 = y.iter().sum::<f32>() / n;

        // Correlation-based approximation
        let covariance: f32 = x.iter()
            .zip(y.iter())
            .map(|(xi, yi)| (xi - mean_x) * (yi - mean_y))
            .sum::<f32>() / n;

        let std_x = (x.iter().map(|xi| (xi - mean_x).powi(2)).sum::<f32>() / n).sqrt();
        let std_y = (y.iter().map(|yi| (yi - mean_y).powi(2)).sum::<f32>() / n).sqrt();

        if std_x == 0.0 || std_y == 0.0 {
            return 0.0;
        }

        let correlation = covariance / (std_x * std_y);

        // Transform to mutual information
        -0.5 * (1.0 - correlation.powi(2)).max(0.001).ln()
    }

    /// Chi-square p-value approximation
    fn chi_square_pvalue(&self, statistic: f32, df: usize) -> f32 {
        // Simplified approximation
        let gamma = |x: f32, a: f32| (x.powi(2) * x.exp() / (a + x / 2.0)).ln() - x;

        let x = statistic;
        let k = df as f32;

        if x <= 0.0 {
            return 1.0;
        }

        // Wilson-Hilferty approximation
        let z = ((x / k).powf(1.0/3.0) - (1.0 - 2.0/(9.0*k))).sqrt() * 3.0 / (2.0 / (9.0 * k)).sqrt();

        // Standard normal CDF approximation
        let cdf = 0.5 * (1.0 + (z / (1.0 + 0.2316419 * z.abs())).exp().powf(-2.0 * z.powi(2)));

        1.0 - cdf
    }

    /// Generate combinations of size k
    fn combinations(&self, items: &[String], k: usize) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let n = items.len();

        if k == 0 {
            return vec![vec![]];
        }

        if k > n {
            return result;
        }

        // Simple recursive combination generator
        self.combinations_recursive(items, 0, k, &mut Vec::new(), &mut result);

        result
    }

    fn combinations_recursive(
        &self,
        items: &[String],
        start: usize,
        k: usize,
        current: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if k == 0 {
            result.push(current.clone());
            return;
        }

        for i in start..items.len() {
            if items.len() - i < k {
                break;
            }
            current.push(items[i].clone());
            self.combinations_recursive(items, i + 1, k - 1, current, result);
            current.pop();
        }
    }

    /// Orient edges using v-structures
    fn orient_edges(&mut self, variables: &[String]) {
        self.oriented_edges.clear();

        for edge in &self.skeleton.undirected_edges {
            if edge.removed {
                continue;
            }

            // Find v-structures: X -> Z <- Y where X and Y are not adjacent
            for z in variables {
                if z == &edge.node1 || z == &edge.node2 {
                    continue;
                }

                // Check if Z is adjacent to both nodes
                let z_adjacent_to_x = self.is_adjacent(&edge.node1, z);
                let z_adjacent_to_y = self.is_adjacent(&edge.node2, z);
                let x_adjacent_to_y = self.is_adjacent(&edge.node1, &edge.node2);

                // V-structure: Z is common effect, X and Y are not adjacent
                if z_adjacent_to_x && z_adjacent_to_y && !x_adjacent_to_y {
                    // Check separation set condition
                    let key = if edge.node1 < edge.node2 {
                        (edge.node1.clone(), edge.node2.clone())
                    } else {
                        (edge.node2.clone(), edge.node1.clone())
                    };

                    let sep_set = self.skeleton.separation_sets.get(&key);

                    // Z should not be in the separation set
                    if sep_set.map(|s| !s.contains(z)).unwrap_or(true) {
                        self.oriented_edges.push(OrientedEdge {
                            source: edge.node1.clone(),
                            target: z.clone(),
                            orientation_confidence: 0.9,
                            rule_applied: OrientationRule::VStructure,
                        });

                        self.oriented_edges.push(OrientedEdge {
                            source: edge.node2.clone(),
                            target: z.clone(),
                            orientation_confidence: 0.9,
                            rule_applied: OrientationRule::VStructure,
                        });

                        self.stats.orientations_applied += 2;
                    }
                }
            }
        }
    }

    /// Check if two nodes are adjacent
    fn is_adjacent(&self, node1: &str, node2: &str) -> bool {
        self.skeleton.undirected_edges.iter()
            .any(|e| {
                !e.removed &&
                ((e.node1 == node1 && e.node2 == node2) ||
                 (e.node1 == node2 && e.node2 == node1))
            })
    }

    /// Apply Meek rules for additional orientations
    fn apply_meek_rules(&mut self) {
        let mut changed = true;
        let mut iterations = 0;
        let max_iterations = 100;

        while changed && iterations < max_iterations {
            changed = false;
            iterations += 1;

            // Meek Rule 1: If X -> Y and Y -- Z (undirected) and X and Z are not adjacent,
            // then orient Y -> Z
            for edge in &self.skeleton.undirected_edges {
                if edge.removed {
                    continue;
                }

                // Check for patterns
                let oriented_as_source = |target: &str| -> Vec<String> {
                    self.oriented_edges.iter()
                        .filter(|e| e.target == target)
                        .map(|e| e.source.clone())
                        .collect()
                };

                let y_parents = oriented_as_source(&edge.node1);
                let y_children = self.oriented_edges.iter()
                    .filter(|e| e.source == edge.node1)
                    .map(|e| e.target.clone())
                    .collect::<Vec<_>>();

                // Check if there are parents of node1 that are not adjacent to node2
                for parent in &y_parents {
                    if !self.is_adjacent(parent, &edge.node2) && !y_children.contains(&parent) {
                        // Orient node1 -> node2
                        if !self.is_oriented(&edge.node1, &edge.node2) {
                            self.oriented_edges.push(OrientedEdge {
                                source: edge.node1.clone(),
                                target: edge.node2.clone(),
                                orientation_confidence: 0.8,
                                rule_applied: OrientationRule::MeekRule1,
                            });
                            changed = true;
                            self.stats.orientations_applied += 1;
                        }
                    }
                }
            }
        }
    }

    /// Check if edge is already oriented
    fn is_oriented(&self, source: &str, target: &str) -> bool {
        self.oriented_edges.iter()
            .any(|e| e.source == source && e.target == target)
    }

    /// Build final causal graph
    fn build_causal_graph(&self, variables: &[String]) -> CausalGraph {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Create nodes
        for var in variables {
            nodes.push(CausalNode {
                node_id: var.clone(),
                name: var.clone(),
                node_type: CausalNodeType::Continuous,
                properties: HashMap::new(),
                domain: vec![],
                is_observed: true,
            });
        }

        // Create edges
        for edge in &self.skeleton.undirected_edges {
            if !edge.removed {
                // Check orientation
                if self.is_oriented(&edge.node1, &edge.node2) {
                    edges.push(CausalEdge {
                        source: edge.node1.clone(),
                        target: edge.node2.clone(),
                        edge_type: CausalEdgeType::Direct,
                        strength: edge.strength,
                        mechanism: None,
                        confounders: vec![],
                    });
                } else if self.is_oriented(&edge.node2, &edge.node1) {
                    edges.push(CausalEdge {
                        source: edge.node2.clone(),
                        target: edge.node1.clone(),
                        edge_type: CausalEdgeType::Direct,
                        strength: edge.strength,
                        mechanism: None,
                        confounders: vec![],
                    });
                } else {
                    // Undirected edge - add as bidirected for now
                    edges.push(CausalEdge {
                        source: edge.node1.clone(),
                        target: edge.node2.clone(),
                        edge_type: CausalEdgeType::Bidirected,
                        strength: edge.strength,
                        mechanism: None,
                        confounders: vec![],
                    });
                }
            }
        }

        // Build adjacency matrix
        let n = variables.len();
        let mut adj_matrix = vec![vec![0.0; n]; n];

        for edge in &edges {
            if let (Some(i), Some(j)) = (
                variables.iter().position(|v| v == &edge.source),
                variables.iter().position(|v| v == &edge.target),
            ) {
                adj_matrix[i][j] = edge.strength;
                if edge.edge_type == CausalEdgeType::Bidirected {
                    adj_matrix[j][i] = edge.strength;
                }
            }
        }

        CausalGraph {
            nodes,
            edges,
            adj_matrix,
        }
    }

    /// Estimate causal effect using do-calculus
    pub fn estimate_causal_effect(
        &self,
        graph: &CausalGraph,
        treatment: &str,
        outcome: &str,
        covariates: &[String],
    ) -> CausalEffect {
        // Check for backdoor path
        let has_backdoor = self.has_backdoor_path(graph, treatment, outcome, covariates);

        // Check for frontdoor path
        let has_frontdoor = self.has_frontdoor_path(graph, treatment, outcome);

        let identification_method = if !has_backdoor {
            IdentificationMethod::BackdoorAdjustment
        } else if has_frontdoor {
            IdentificationMethod::FrontdoorAdjustment
        } else {
            IdentificationMethod::NotIdentifiable
        };

        CausalEffect {
            treatment: treatment.to_string(),
            outcome: outcome.to_string(),
            effect_type: EffectType::AverageTreatmentEffect,
            estimate: 0.0,  // Would be computed from data
            standard_error: 0.0,
            confidence_interval: (0.0, 0.0),
            identification_method,
            assumptions: vec![],
        }
    }

    fn has_backdoor_path(&self, graph: &CausalGraph, treatment: &str, outcome: &str, covariates: &[String]) -> bool {
        // Check if covariates block all backdoor paths
        !self.find_backdoor_paths(graph, treatment, outcome).is_empty()
    }

    fn has_frontdoor_path(&self, graph: &CausalGraph, treatment: &str, outcome: &str) -> bool {
        // Check for frontdoor criterion
        let paths = self.find_paths(graph, treatment, outcome);

        for path in paths {
            if path.iter().any(|e| e.starts_with("->")) && path.len() > 2 {
                return true;
            }
        }
        false
    }

    fn find_backdoor_paths(&self, graph: &CausalGraph, source: &str, target: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        let mut visited = HashSet::new();
        visited.insert(source.to_string());

        self.dfs_backdoor(graph, source, target, &mut visited, &mut paths, vec![source.to_string()]);

        paths
    }

    fn dfs_backdoor(&self, graph: &CausalGraph, current: &str, target: &str, visited: &mut HashSet<String>, paths: &mut Vec<Vec<String>>, current_path: &mut Vec<String>) {
        if current == target && current_path.len() > 1 {
            paths.push(current_path.clone());
            return;
        }

        for edge in &graph.edges {
            if edge.source == current && !visited.contains(&edge.target) {
                if edge.edge_type == CausalEdgeType::Bidirected ||
                   edge.edge_type == CausalEdgeType::Indirect {
                    visited.insert(edge.target.clone());
                    current_path.push(edge.target.clone());
                    self.dfs_backdoor(graph, &edge.target, target, visited, paths, current_path);
                    current_path.pop();
                    visited.remove(&edge.target);
                }
            }
        }
    }

    fn find_paths(&self, graph: &CausalGraph, source: &str, target: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        self.dfs_paths(graph, source, target, &mut HashSet::new(), &mut paths, vec![source.to_string()]);
        paths
    }

    fn dfs_paths(&self, graph: &CausalGraph, current: &str, target: &str, visited: &mut HashSet<String>, paths: &mut Vec<Vec<String>>, current_path: &mut Vec<String>) {
        if current == target && current_path.len() > 1 {
            paths.push(current_path.clone());
            return;
        }

        visited.insert(current.to_string());

        for edge in &graph.edges {
            if edge.source == current && !visited.contains(&edge.target) {
                current_path.push(edge.target.clone());
                self.dfs_paths(graph, &edge.target, target, visited, paths, current_path);
                current_path.pop();
            }
        }

        visited.remove(current);
    }

    /// Get statistics
    pub fn get_stats(&self) -> &CausalLearningStats {
        &self.stats
    }
}

impl Default for CausalLearner {
    fn default() -> Self {
        Self::new()
    }
}

/// Causal effect estimation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalEffect {
    pub treatment: String,
    pub outcome: String,
    pub effect_type: EffectType,
    pub estimate: f32,
    pub standard_error: f32,
    pub confidence_interval: (f32, f32),
    pub identification_method: IdentificationMethod,
    pub assumptions: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EffectType {
    AverageTreatmentEffect,
    LocalAverageTreatmentEffect,
    MarginalTreatmentEffect,
    ConditionalAverageTreatmentEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentificationMethod {
    BackdoorAdjustment,
    FrontdoorAdjustment,
    InstrumentalVariable,
    DifferenceInDifferences,
    SyntheticControl,
    NotIdentifiable,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("cl_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_causal_learner_creation() {
        let learner = CausalLearner::new();
        assert_eq!(learner.get_stats().ci_tests_performed, 0);
    }

    #[test]
    fn test_combinations() {
        let learner = CausalLearner::new();
        let items = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let combos = learner.combinations(&items, 2);
        assert_eq!(combos.len(), 3);
    }
}
