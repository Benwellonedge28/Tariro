// Tariro AGI - Few-Shot Learning Module
// Learning from few examples

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::cmp::Ordering;

// ============================================================================
// Few-Shot Learning Types
// ============================================================================

/// Few-shot learning framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FewShotLearner {
    pub learner_id: String,
    pub episodes: Vec<Episode>,
    pub prototypes: Vec<Prototype>,
    pub embeddings: Vec<Embedding>,
    pub meta_learning_state: MetaLearningState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub episode_id: String,
    pub support_set: SupportSet,
    pub query_set: QuerySet,
    pub n_way: usize,
    pub k_shot: usize,
    pub num_queries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportSet {
    pub samples: Vec<FewShotSample>,
    pub labels: Vec<String>,
    pub embeddings: Vec<Vec<f32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuerySet {
    pub samples: Vec<FewShotSample>,
    pub labels: Vec<String>,
    pub true_labels: Vec<String>,
    pub predictions: Vec<Prediction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FewShotSample {
    pub sample_id: String,
    pub features: Vec<f32>,
    pub label: String,
    pub distance_to_class: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub predicted_label: String,
    pub confidence: f32,
    pub probabilities: HashMap<String, f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prototype {
    pub prototype_id: String,
    pub class_label: String,
    pub embedding: Vec<f32>,
    pub support_count: usize,
    pub variance: f32,
    pub created_at: Episode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    pub embedding_id: String,
    pub representation: Vec<f32>,
    pub original_sample_id: String,
    pub transformation_history: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaLearningState {
    pub meta_parameters: Vec<f32>,
    pub inner_lr: f32,
    pub outer_lr: f32,
    pub adaptation_steps: usize,
    pub meta_loss_history: Vec<f32>,
}

// ============================================================================
// Prototypical Networks
// ============================================================================

/// Prototypical networks for few-shot learning
pub struct PrototypicalNetworks {
    // Embedding function
    embedding_dim: usize,
    hidden_dim: usize,

    // Class prototypes
    class_prototypes: HashMap<String, Vec<f32>>,

    // Configuration
    config: ProtoConfig,

    // Distance metric
    distance_metric: DistanceMetric,
}

#[derive(Debug, Clone)]
pub struct ProtoConfig {
    pub use_cosine_distance: bool,
    pub temperature: f32,
    pub prototype_aggregation: AggregationMethod,
    pub variance_weighting: bool,
    pub hidden_layers: usize,
    pub dropout: f32,
    pub batch_norm: bool,
}

impl Default for ProtoConfig {
    fn default() -> Self {
        Self {
            use_cosine_distance: false,
            temperature: 1.0,
            prototype_aggregation: AggregationMethod::Mean,
            variance_weighting: false,
            hidden_layers: 4,
            dropout: 0.1,
            batch_norm: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationMethod {
    Mean,
    Median,
    Mode,
    WeightedMean,
    Attention,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DistanceMetric {
    Euclidean,
    Cosine,
    Manhattan,
    Mahalanobis,
}

impl PrototypicalNetworks {
    pub fn new(embedding_dim: usize, hidden_dim: usize) -> Self {
        Self {
            embedding_dim,
            hidden_dim,
            class_prototypes: HashMap::new(),
            config: ProtoConfig::default(),
            distance_metric: DistanceMetric::Euclidean,
        }
    }

    /// Compute prototype for each class
    pub fn compute_prototypes(
        &mut self,
        support_embeddings: &[Vec<f32>],
        support_labels: &[String],
    ) -> HashMap<String, Vec<f32>> {
        let mut class_embeddings: HashMap<String, Vec<Vec<f32>>> = HashMap::new();

        // Group embeddings by class
        for (embedding, label) in support_embeddings.iter().zip(support_labels.iter()) {
            class_embeddings.entry(label.clone())
                .or_insert_with(Vec::new)
                .push(embedding.clone());
        }

        // Compute prototype for each class
        self.class_prototypes.clear();

        for (class, embeddings) in &class_embeddings {
            let prototype = match self.config.prototype_aggregation {
                AggregationMethod::Mean => self.compute_mean(embeddings),
                AggregationMethod::Median => self.compute_median(embeddings),
                AggregationMethod::WeightedMean => self.compute_weighted_mean(embeddings),
                AggregationMethod::Attention => self.compute_attention_prototype(embeddings),
                AggregationMethod::Mode => self.compute_mode(embeddings),
            };

            self.class_prototypes.insert(class.clone(), prototype);
        }

        self.class_prototypes.clone()
    }

    /// Compute mean embedding
    fn compute_mean(&self, embeddings: &[Vec<f32>]) -> Vec<f32> {
        if embeddings.is_empty() {
            return vec![0.0; self.embedding_dim];
        }

        let n = embeddings.len();
        let dim = embeddings[0].len();

        let mut mean = vec![0.0; dim];
        for embedding in embeddings {
            for (i, val) in embedding.iter().enumerate() {
                mean[i] += val;
            }
        }

        for val in &mut mean {
            *val /= n as f32;
        }

        mean
    }

    /// Compute median embedding
    fn compute_median(&self, embeddings: &[Vec<f32>]) -> Vec<f32> {
        if embeddings.is_empty() {
            return vec![0.0; self.embedding_dim];
        }

        let dim = embeddings[0].len();
        let mut median = vec![0.0; dim];

        for d in 0..dim {
            let mut values: Vec<f32> = embeddings.iter().map(|e| e[d]).collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            median[d] = values[values.len() / 2];
        }

        median
    }

    /// Compute weighted mean (weight by distance to class center)
    fn compute_weighted_mean(&self, embeddings: &[Vec<f32>]) -> Vec<f32> {
        if embeddings.is_empty() {
            return vec![0.0; self.embedding_dim];
        }

        // First compute unweighted mean as center
        let center = self.compute_mean(embeddings);

        // Compute distances and weights
        let distances: Vec<f32> = embeddings.iter()
            .map(|e| self.compute_distance(e, &center))
            .collect();

        let weights: Vec<f32> = distances.iter()
            .map(|d| 1.0 / (d + 1e-8))
            .collect();

        let total_weight: f32 = weights.iter().sum();

        // Compute weighted mean
        let dim = embeddings[0].len();
        let mut weighted_mean = vec![0.0; dim];

        for (embedding, weight) in embeddings.iter().zip(weights.iter()) {
            for (i, val) in embedding.iter().enumerate() {
                weighted_mean[i] += val * weight / total_weight;
            }
        }

        weighted_mean
    }

    /// Compute prototype with attention
    fn compute_attention_prototype(&self, embeddings: &[Vec<f32>]) -> Vec<f32> {
        if embeddings.is_empty() {
            return vec![0.0; self.embedding_dim];
        }

        // Simple attention: weight by inverse distance from mean
        let mean = self.compute_mean(embeddings);
        let mut attention_weights = Vec::new();

        for embedding in embeddings {
            let dist = self.compute_distance(embedding, &mean);
            let weight = (-dist / self.config.temperature).exp();
            attention_weights.push(weight);
        }

        let total: f32 = attention_weights.iter().sum();
        let normalized_weights: Vec<f32> = attention_weights.iter()
            .map(|w| w / total)
            .collect();

        let dim = embeddings[0].len();
        let mut proto = vec![0.0; dim];

        for (embedding, weight) in embeddings.iter().zip(normalized_weights.iter()) {
            for (i, val) in embedding.iter().enumerate() {
                proto[i] += val * weight;
            }
        }

        proto
    }

    /// Compute mode (for discrete/encoded embeddings)
    fn compute_mode(&self, embeddings: &[Vec<f32>]) -> Vec<f32> {
        // For continuous embeddings, mode is similar to median
        self.compute_median(embeddings)
    }

    /// Compute distance between two embeddings
    pub fn compute_distance(&self, a: &[f32], b: &[f32]) -> f32 {
        match self.distance_metric {
            DistanceMetric::Euclidean => {
                a.iter().zip(b.iter())
                    .map(|(x, y)| (x - y).powi(2))
                    .sum::<f32>()
                    .sqrt()
            }
            DistanceMetric::Cosine => {
                let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>();
                let norm_a = a.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
                let norm_b = b.iter().map(|x| x.powi(2)).sum::<f32>().sqrt();
                1.0 - (dot / (norm_a * norm_b + 1e-8))
            }
            DistanceMetric::Manhattan => {
                a.iter().zip(b.iter())
                    .map(|(x, y)| (x - y).abs())
                    .sum()
            }
            DistanceMetric::Mahalanobis => {
                // Simplified Mahalanobis (identity covariance)
                self.compute_distance(a, b)
            }
        }
    }

    /// Classify query sample
    pub fn classify(&self, query_embedding: &[f32]) -> HashMap<String, f32> {
        let mut distances = HashMap::new();

        for (class, prototype) in &self.class_prototypes {
            let dist = self.compute_distance(query_embedding, prototype);
            distances.insert(class.clone(), dist);
        }

        // Convert distances to probabilities (negative distance = higher prob)
        let mut probs = HashMap::new();
        let total: f32 = distances.values()
            .map(|d| (-*d / self.config.temperature).exp())
            .sum();

        for (class, dist) in &distances {
            let prob = (-dist / self.config.temperature).exp() / total;
            probs.insert(class.clone(), prob);
        }

        probs
    }

    /// Predict class for query embedding
    pub fn predict(&self, query_embedding: &[f32]) -> Prediction {
        let probs = self.classify(query_embedding);

        let (predicted_label, confidence) = probs.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal))
            .map(|(k, v)| (k.clone(), *v))
            .unwrap_or(("unknown".to_string(), 0.0));

        Prediction {
            predicted_label,
            confidence,
            probabilities: probs,
        }
    }

    /// Update prototypes online
    pub fn update_prototype(&mut self, class: &str, embedding: Vec<f32>, learning_rate: f32) {
        if let Some(existing) = self.class_prototypes.get_mut(class) {
            // Online update
            for (i, val) in embedding.iter().enumerate() {
                existing[i] = (1.0 - learning_rate) * existing[i] + learning_rate * val;
            }
        } else {
            self.class_prototypes.insert(class.to_string(), embedding);
        }
    }
}

// ============================================================================
// MAML Implementation
// ============================================================================

/// Model-Agnostic Meta-Learning (MAML)
pub struct MAML {
    // Meta-learning parameters
    meta_parameters: Vec<f32>,

    // Task-specific parameters
    task_adaptation: HashMap<String, Vec<f32>>,

    // Configuration
    config: MAMLConfig,
}

#[derive(Debug, Clone)]
pub struct MAMLConfig {
    pub inner_lr: f32,
    pub outer_lr: f32,
    pub inner_steps: usize,
    pub outer_steps: usize,
    pub support_set_size: usize,
    pub query_set_size: usize,
    pub first_order: bool,
    pub gradient_clip: f32,
}

impl Default for MAMLConfig {
    fn default() -> Self {
        Self {
            inner_lr: 0.01,
            outer_lr: 0.001,
            inner_steps: 5,
            outer_steps: 10000,
            support_set_size: 5,
            query_set_size: 5,
            first_order: false,
            gradient_clip: 1.0,
        }
    }
}

impl MAML {
    pub fn new(param_dim: usize) -> Self {
        Self {
            meta_parameters: vec![0.0; param_dim],
            task_adaptation: HashMap::new(),
            config: MAMLConfig::default(),
        }
    }

    /// Inner loop: adapt to a single task
    pub fn inner_update(&self, support_gradients: &[f32]) -> Vec<f32> {
        let mut adapted_params = self.meta_parameters.clone();

        for (i, grad) in support_gradients.iter().enumerate() {
            if i < adapted_params.len() {
                adapted_params[i] -= self.config.inner_lr * grad;
            }
        }

        adapted_params
    }

    /// Compute task-specific loss
    pub fn compute_task_loss(
        &self,
        params: &[f32],
        query_embeddings: &[Vec<f32>],
        query_labels: &[String],
    ) -> f32 {
        // Simplified loss computation
        let mut total_loss = 0.0;
        let n = query_embeddings.len();

        for (embedding, label) in query_embeddings.iter().zip(query_labels.iter()) {
            // Forward pass (simplified)
            let logits = self.forward(params, embedding);

            // Cross-entropy (simplified)
            let loss = -logits.iter()
                .zip(label.as_bytes())
                .map(|(l, y)| l * (*y as f32))
                .sum::<f32>();

            total_loss += loss;
        }

        total_loss / n as f32
    }

    /// Simplified forward pass
    fn forward(&self, params: &[f32], input: &[f32]) -> Vec<f32> {
        // Linear layer
        let output_dim = params.len() / input.len();
        let mut output = vec![0.0; output_dim.min(params.len())];

        for i in 0..output.len().min(params.len()) {
            for j in 0..input.len().min(params.len() / output_dim) {
                output[i] += params[i * input.len() + j] * input[j];
            }
        }

        output
    }

    /// Outer loop: meta-update
    pub fn meta_update(
        &mut self,
        task_losses: &[f32],
        adaptation_params: &[Vec<f32>],
    ) {
        // Compute meta-gradient
        let mut meta_grad = vec![0.0; self.meta_parameters.len()];

        for (loss, adapted) in task_losses.iter().zip(adaptation_params.iter()) {
            for (i, (orig, adapted_val)) in self.meta_parameters.iter()
                .zip(adapted.iter())
                .enumerate()
            {
                let grad = (adapted_val - orig) / (loss + 1e-8);
                meta_grad[i] += grad;
            }
        }

        // Apply meta-gradient
        let lr = self.config.outer_lr;
        for (i, grad) in meta_grad.iter().enumerate() {
            self.meta_parameters[i] -= lr * grad / task_losses.len() as f32;
        }
    }

    /// Episode-based training
    pub fn train_episode(
        &mut self,
        support_embeddings: &[Vec<f32>],
        support_labels: &[String],
        query_embeddings: &[Vec<f32>],
        query_labels: &[String],
    ) -> f32 {
        // Inner loop: adapt
        let mut adapted_params = self.meta_parameters.clone();

        for _ in 0..self.config.inner_steps {
            let gradients = self.compute_gradients(&adapted_params, support_embeddings, support_labels);

            for (i, grad) in gradients.iter().enumerate() {
                if i < adapted_params.len() {
                    adapted_params[i] -= self.config.inner_lr * grad;
                }
            }
        }

        // Outer loop: compute query loss
        let query_loss = self.compute_task_loss(&adapted_params, query_embeddings, query_labels);

        // Meta-update (simplified)
        let loss_for_update = query_loss;
        self.meta_update(&[loss_for_update], &[adapted_params]);

        query_loss
    }

    /// Compute gradients
    fn compute_gradients(
        &self,
        params: &[f32],
        embeddings: &[Vec<f32>],
        labels: &[String],
    ) -> Vec<f32> {
        let mut gradients = vec![0.0; params.len()];

        for (embedding, label) in embeddings.iter().zip(labels.iter()) {
            let output = self.forward(params, embedding);

            // Gradient of cross-entropy (simplified)
            for (i, (out, y)) in output.iter()
                .zip(label.as_bytes())
                .enumerate()
            {
                if i < gradients.len() {
                    gradients[i] += out - *y as f32;
                }
            }
        }

        // Normalize
        let n = embeddings.len() as f32;
        for grad in &mut gradients {
            *grad /= n;
        }

        gradients
    }
}

// ============================================================================
// Matching Networks
// ============================================================================

/// Matching Networks for few-shot learning
pub struct MatchingNetworks {
    embedding_fn: Box<dyn Fn(&[f32]) -> Vec<f32>>,
    attention_fn: AttentionFn,
    support_set: Vec<(Vec<f32>, String)>,
}

type AttentionFn = fn(&[f32], &[Vec<f32>], &DistanceMetric) -> Vec<f32>;

impl MatchingNetworks {
    pub fn new() -> Self {
        Self {
            embedding_fn: Box::new(|x| x.to_vec()),
            attention_fn: |query, supports, dist| {
                let mut attention = Vec::new();
                for support in supports {
                    let d = Self::euclidean_distance(query, support);
                    attention.push((-d / 0.5).exp());
                }
                let sum: f32 = attention.iter().sum();
                attention.iter().map(|a| a / sum).collect()
            },
            support_set: Vec::new(),
        }
    }

    /// Set support set
    pub fn set_support(&mut self, support: Vec<(Vec<f32>, String)>) {
        self.support_set = support;
    }

    /// Classify query using attention over support
    pub fn classify(&self, query: &[f32]) -> HashMap<String, f32> {
        let embeddings: Vec<Vec<f32>> = self.support_set.iter()
            .map(|(e, _)| e.clone())
            .collect();

        let labels: Vec<String> = self.support_set.iter()
            .map(|(_, l)| l.clone())
            .collect();

        // Compute attention weights
        let attention = (self.attention_fn)(query, &embeddings, &DistanceMetric::Euclidean);

        // Aggregate by class
        let mut class_weights: HashMap<String, (f32, f32)> = HashMap::new();

        for (weight, label) in attention.iter().zip(labels.iter()) {
            let entry = class_weights.entry(label.clone())
                .or_insert((0.0, 0.0));
            entry.0 += weight;
            entry.1 += 1.0;
        }

        // Normalize to probabilities
        let total: f32 = class_weights.values().map(|(w, _)| w).sum();

        class_weights.iter()
            .map(|(k, (w, c))| (k.clone(), w / total))
            .collect()
    }

    fn euclidean_distance(a: &[f32], b: &[f32]) -> f32 {
        a.iter().zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f32>()
            .sqrt()
    }

    /// Predict label for query
    pub fn predict(&self, query: &[f32]) -> Prediction {
        let probs = self.classify(query);

        let (label, confidence) = probs.iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal))
            .map(|(k, v)| (k.clone(), *v))
            .unwrap_or(("unknown".to_string(), 0.0));

        Prediction {
            predicted_label: label,
            confidence,
            probabilities: probs,
        }
    }
}

impl Default for MatchingNetworks {
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
    fn test_prototypical_networks() {
        let proto = PrototypicalNetworks::new(10, 20);

        let embeddings = vec![
            vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            vec![0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
        ];
        let labels = vec!["class_a".to_string(), "class_a".to_string()];

        proto.compute_prototypes(&embeddings, &labels);
        assert!(proto.class_prototypes.contains_key("class_a"));
    }

    #[test]
    fn test_matching_networks() {
        let mut matcher = MatchingNetworks::new();

        matcher.set_support(vec![
            (vec![1.0, 0.0], "A".to_string()),
            (vec![0.0, 1.0], "B".to_string()),
        ]);

        let query = vec![0.8, 0.2];
        let probs = matcher.classify(&query);
        assert!(probs.contains_key("A"));
    }
}
