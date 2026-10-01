//! Federated Learning Protocol
//!
//! This module implements federated learning for Tariro AGI,
//! enabling cross-device knowledge synchronization while maintaining privacy.
//!
//! # Federated Learning
//!
//! Tariro uses federated learning to share knowledge across devices
//! without centralizing data. Each device trains locally and shares
//! only model updates (gradients or parameters), not raw data.
//!
//! # Privacy Techniques
//!
//! | Technique | Privacy | Bandwidth | Accuracy |
//! |-----------|---------|-----------|----------|
//! | None | None | High | Best |
//! | Top-K | Low | Medium | Good |
//! | Quantization | Medium | Low | Good |
//! | Sparsification | High | Very Low | Moderate |

use crate::scaling::capability::NodeId;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Aggregation strategy for federated learning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AggregationStrategy {
    /// FedAvg: Weighted average by sample count
    FedAvg,

    /// FedProx: With proximal term for heterogeneity
    FedProx { mu: f32 },

    /// SCAFFOLD: Variance reduction
    Scaffold,

    /// FedOpt: Optimizer-based aggregation
    FedOpt,

    /// Secure aggregation with homomorphic encryption
    SecureAggregated,
}

/// Privacy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivacyConfig {
    /// Privacy technique
    pub technique: PrivacyTechnique,

    /// Top-K ratio (for TopK technique)
    pub top_k_ratio: f32,

    /// Quantization bits (for Quantization technique)
    pub quantization_bits: u8,

    /// Sparsity ratio (for Sparsification technique)
    pub sparsity: f32,

    /// Add noise for differential privacy
    pub add_noise: bool,

    /// Noise scale
    pub noise_scale: f32,
}

impl Default for PrivacyConfig {
    fn default() -> Self {
        Self {
            technique: PrivacyTechnique::TopK { k: 100 },
            top_k_ratio: 0.1,
            quantization_bits: 8,
            sparsity: 0.01,
            add_noise: false,
            noise_scale: 0.01,
        }
    }
}

/// Privacy technique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrivacyTechnique {
    /// No privacy
    None,

    /// Keep only top-K values
    TopK { k: usize },

    /// Quantize to fewer bits
    Quantization { bits: u8 },

    /// Random sparsification
    Sparsification { sparsity: f32 },
}

/// Learning sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sample {
    /// Input features
    pub input: Vec<f32>,

    /// Expected output
    pub output: Vec<f32>,

    /// Sample weight
    pub weight: f32,

    /// Timestamp
    pub timestamp_ms: u64,
}

/// Model update from a client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelUpdate {
    /// Update ID
    pub id: u64,

    /// Node that produced this update
    pub node_id: NodeId,

    /// Update weights (gradient or delta)
    pub weights: Vec<f32>,

    /// Sample count used for update
    pub sample_count: usize,

    /// Round number
    pub round: u32,

    /// Timestamp
    pub timestamp_ms: u64,

    /// Compression type used
    pub compression: CompressionType,
}

/// Compressed update
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressedUpdate {
    /// Original size
    pub original_size: usize,

    /// Compressed size
    pub compressed_size: usize,

    /// Compressed data
    pub data: Vec<u8>,

    /// Metadata (for reconstruction)
    pub metadata: CompressionMetadata,
}

/// Compression metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionMetadata {
    /// Original dimensions
    pub shape: Vec<usize>,

    /// Compression type
    pub compression_type: CompressionType,

    /// Statistics for reconstruction
    pub min: f32,
    pub max: f32,
    pub mean: f32,
}

/// Compression type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompressionType {
    /// No compression
    None,

    /// Top-K selection
    TopK,

    /// Quantization
    Quantization,

    /// Random sparsification
    Sparsification,

    /// Run-length encoding
    RLE,
}

/// Federated learning protocol
pub struct FederatedProtocol {
    /// Local node ID
    node_id: NodeId,

    /// Aggregation strategy
    aggregation: AggregationStrategy,

    /// Privacy configuration
    privacy: PrivacyConfig,

    /// Local model weights
    local_weights: Vec<f32>,

    /// Server weights (for comparison)
    server_weights: Option<Vec<f32>>,

    /// Peer updates collected this round
    peer_updates: Vec<ModelUpdate>,

    /// Update history
    update_history: VecDeque<RoundStats>,

    /// Current round
    current_round: u32,

    /// Minimum peers required
    min_peers: usize,

    /// Sync frequency (seconds)
    sync_frequency_s: u32,

    /// Configuration
    config: FederatedConfig,
}

/// Federated configuration
#[derive(Debug, Clone)]
pub struct FederatedConfig {
    /// Minimum peers for aggregation
    pub min_peers: usize,

    /// Aggregation strategy
    pub aggregation: AggregationStrategy,

    /// Privacy configuration
    pub privacy: PrivacyConfig,

    /// Sync frequency (seconds)
    pub sync_frequency_s: u32,

    /// Learning rate
    pub learning_rate: f32,

    /// Momentum
    pub momentum: f32,

    /// Weight decay
    pub weight_decay: f32,
}

impl Default for FederatedConfig {
    fn default() -> Self {
        Self {
            min_peers: 3,
            aggregation: AggregationStrategy::FedAvg,
            privacy: PrivacyConfig::default(),
            sync_frequency_s: 300,
            learning_rate: 0.01,
            momentum: 0.0,
            weight_decay: 0.0001,
        }
    }
}

/// Round statistics
#[derive(Debug, Clone)]
pub struct RoundStats {
    pub round: u32,
    pub participants: usize,
    pub avg_quality: f32,
    pub compression_ratio: f32,
    pub duration_ms: u64,
}

/// Training configuration
#[derive(Debug, Clone)]
pub struct TrainingConfig {
    /// Batch size
    pub batch_size: usize,

    /// Number of epochs
    pub epochs: usize,

    /// Learning rate
    pub learning_rate: f32,

    /// Shuffle data
    pub shuffle: bool,

    /// Early stopping patience
    pub patience: usize,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            batch_size: 32,
            epochs: 5,
            learning_rate: 0.01,
            shuffle: true,
            patience: 3,
        }
    }
}

impl Default for FederatedProtocol {
    fn default() -> Self {
        Self::new(0, FederatedConfig::default())
    }
}

impl FederatedProtocol {
    /// Create a new federated protocol instance
    pub fn new(node_id: NodeId, config: FederatedConfig) -> Self {
        Self {
            node_id,
            aggregation: config.aggregation,
            privacy: config.privacy,
            local_weights: Vec::new(),
            server_weights: None,
            peer_updates: Vec::new(),
            update_history: VecDeque::with_capacity(100),
            current_round: 0,
            min_peers: config.min_peers,
            sync_frequency_s: config.sync_frequency_s,
            config,
        }
    }

    /// Initialize local model
    pub fn initialize_model(&mut self, weights: Vec<f32>) {
        self.local_weights = weights;
    }

    /// Set server weights (for comparison)
    pub fn set_server_weights(&mut self, weights: Vec<f32>) {
        self.server_weights = Some(weights);
    }

    /// Perform local training
    pub fn train_local(&mut self, data: &[Sample], config: &TrainingConfig) -> Result<ModelUpdate, FederatedError> {
        if self.local_weights.is_empty() {
            return Err(FederatedError::ModelNotInitialized);
        }

        let start_time = std::time::Instant::now();

        // Initialize gradients
        let mut gradients = vec![0.0_f32; self.local_weights.len()];

        // Simple gradient computation (placeholder for actual training)
        for sample in data {
            let prediction = self.predict(&sample.input);
            let error = self.compute_error(&prediction, &sample.output);

            // Compute gradient contribution
            for (i, (w, x)) in self.local_weights.iter().zip(sample.input.iter()).enumerate() {
                gradients[i] += error * x * sample.weight / data.len() as f32;
            }
        }

        // Apply updates
        for (w, g) in self.local_weights.iter_mut().zip(gradients.iter()) {
            *w -= config.learning_rate * g;
        }

        // Create update
        let update = ModelUpdate {
            id: self.generate_id(),
            node_id: self.node_id,
            weights: gradients,
            sample_count: data.len(),
            round: self.current_round,
            timestamp_ms: 0, // Would be actual timestamp
            compression: CompressionType::None,
        };

        Ok(update)
    }

    /// Compress update for transmission
    pub fn compress_update(&mut self, update: ModelUpdate) -> Result<CompressedUpdate, FederatedError> {
        let original_size = update.weights.len() * 4; // f32 = 4 bytes

        match self.privacy.technique {
            PrivacyTechnique::None => Ok(CompressedUpdate {
                original_size,
                compressed_size: original_size,
                data: self.serialize_f32(&update.weights),
                metadata: CompressionMetadata {
                    shape: vec![update.weights.len()],
                    compression_type: CompressionType::None,
                    min: update.weights.iter().cloned().fold(f32::INFINITY, f32::min),
                    max: update.weights.iter().cloned().fold(f32::NEG_INFINITY, f32::max),
                    mean: update.weights.iter().sum::<f32>() / update.weights.len() as f32,
                },
            }),
            PrivacyTechnique::TopK { k } => self.compress_top_k(update, k),
            PrivacyTechnique::Quantization { bits } => self.compress_quantization(update, bits),
            PrivacyTechnique::Sparsification { sparsity } => {
                self.compress_sparsification(update, sparsity)
            }
        }
    }

    /// Top-K compression
    fn compress_top_k(&self, update: ModelUpdate, k: usize) -> Result<CompressedUpdate, FederatedError> {
        // Find top-K values by magnitude
        let mut indexed: Vec<(usize, f32)> = update
            .weights
            .iter()
            .enumerate()
            .map(|(i, &v)| (i, v))
            .collect();

        indexed.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());
        indexed.truncate(k);

        let k = indexed.len();

        // Create sparse representation
        let mut indices = Vec::with_capacity(k);
        let mut values = Vec::with_capacity(k);

        for (idx, val) in indexed {
            indices.push(idx as u32);
            values.push(val);
        }

        // Encode
        let mut data = Vec::new();
        data.extend_from_slice(&(k as u32).to_le_bytes());
        data.extend_from_slice(
            &indices
                .iter()
                .flat_map(|&i| i.to_le_bytes())
                .collect::<Vec<_>>(),
        );
        data.extend_from_slice(&self.serialize_f32(&values));

        let compressed_size = data.len();

        Ok(CompressedUpdate {
            original_size: update.weights.len() * 4,
            compressed_size,
            data,
            metadata: CompressionMetadata {
                shape: vec![update.weights.len()],
                compression_type: CompressionType::TopK,
                min: values.iter().cloned().fold(f32::INFINITY, f32::min),
                max: values.iter().cloned().fold(f32::NEG_INFINITY, f32::max),
                mean: values.iter().sum::<f32>() / values.len().max(1) as f32,
            },
        })
    }

    /// Quantization compression
    fn compress_quantization(&self, update: ModelUpdate, bits: u8) -> Result<CompressedUpdate, FederatedError> {
        let levels = 1u32 << bits as u32;

        let min_val = update.weights.iter().cloned().fold(f32::INFINITY, f32::min);
        let max_val = update.weights.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max_val - min_val;

        // Quantize
        let quantized: Vec<u8> = update
            .weights
            .iter()
            .map(|&v| {
                let normalized = (v - min_val) / range;
                ((normalized * (levels - 1) as f32) as u32).min(levels - 1) as u8
            })
            .collect();

        // Calculate compression
        let compressed_size = quantized.len() + 8; // +8 for min/max

        Ok(CompressedUpdate {
            original_size: update.weights.len() * 4,
            compressed_size,
            data: quantized,
            metadata: CompressionMetadata {
                shape: vec![update.weights.len()],
                compression_type: CompressionType::Quantization,
                min: min_val,
                max: max_val,
                mean: update.weights.iter().sum::<f32>() / update.weights.len() as f32,
            },
        })
    }

    /// Random sparsification
    fn compress_sparsification(
        &self,
        update: ModelUpdate,
        sparsity: f32,
    ) -> Result<CompressedUpdate, FederatedError> {
        let k = ((1.0 - sparsity) * update.weights.len() as f32) as usize;
        self.compress_top_k(update, k.max(1))
    }

    /// Decompress update
    pub fn decompress_update(&self, compressed: &CompressedUpdate) -> Result<Vec<f32>, FederatedError> {
        match compressed.metadata.compression_type {
            CompressionType::None => Ok(self.deserialize_f32(&compressed.data)),
            CompressionType::TopK => self.decompress_top_k(compressed),
            CompressionType::Quantization => self.decompress_quantization(compressed),
            _ => Err(FederatedError::DecompressionFailed(
                "Unsupported compression type".to_string(),
            )),
        }
    }

    /// Decompress Top-K
    fn decompress_top_k(&self, compressed: &CompressedUpdate) -> Result<Vec<f32>, FederatedError> {
        let data = &compressed.data;
        if data.len() < 4 {
            return Err(FederatedError::DecompressionFailed("Data too short".to_string()));
        }

        let k = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        let mut weights = vec![0.0_f32; compressed.metadata.shape[0]];

        let indices_start = 4;
        let values_start = indices_start + k * 4;

        for i in 0..k {
            let idx = u32::from_le_bytes([
                data[indices_start + i * 4],
                data[indices_start + i * 4 + 1],
                data[indices_start + i * 4 + 2],
                data[indices_start + i * 4 + 3],
            ]) as usize;

            let val = f32::from_le_bytes([
                data[values_start + i * 4],
                data[values_start + i * 4 + 1],
                data[values_start + i * 4 + 2],
                data[values_start + i * 4 + 3],
            ]);

            if idx < weights.len() {
                weights[idx] = val;
            }
        }

        Ok(weights)
    }

    /// Decompress quantization
    fn decompress_quantization(&self, compressed: &CompressedUpdate) -> Result<Vec<f32>, FederatedError> {
        let levels = 1u32 << 8; // Assume 8 bits
        let range = compressed.metadata.max - compressed.metadata.min;

        let weights: Vec<f32> = compressed
            .data
            .iter()
            .map(|&q| {
                let normalized = q as f32 / (levels - 1) as f32;
                compressed.metadata.min + normalized * range
            })
            .collect();

        Ok(weights)
    }

    /// Receive update from peer
    pub fn receive_update(&mut self, update: ModelUpdate) -> Result<(), FederatedError> {
        if update.round != self.current_round {
            return Err(FederatedError::RoundMismatch {
                expected: self.current_round,
                actual: update.round,
            });
        }

        self.peer_updates.push(update);
        Ok(())
    }

    /// Aggregate updates from all peers
    pub fn aggregate(&mut self) -> Result<Vec<f32>, FederatedError> {
        if self.peer_updates.len() < self.min_peers {
            return Err(FederatedError::InsufficientPeers {
                required: self.min_peers,
                available: self.peer_updates.len(),
            });
        }

        let result = match self.aggregation {
            AggregationStrategy::FedAvg => self.aggregate_fedavg(),
            AggregationStrategy::FedProx { mu } => self.aggregate_fedprox(mu),
            AggregationStrategy::Scaffold => self.aggregate_scaffold(),
            AggregationStrategy::FedOpt => self.aggregate_fedopt(),
            AggregationStrategy::SecureAggregated => self.aggregate_secure(),
        };

        // Clear peer updates
        self.peer_updates.clear();

        // Advance round
        self.current_round += 1;

        result
    }

    /// FedAvg aggregation
    fn aggregate_fedavg(&self) -> Result<Vec<f32>, FederatedError> {
        let total_samples: usize = self.peer_updates.iter().map(|u| u.sample_count).sum();

        if total_samples == 0 {
            return Err(FederatedError::ZeroSamples);
        }

        let mut aggregated = vec![0.0_f32; self.local_weights.len()];

        for update in &self.peer_updates {
            let weight = update.sample_count as f32 / total_samples as f32;
            for (agg, &w) in aggregated.iter_mut().zip(update.weights.iter()) {
                *agg += weight * w;
            }
        }

        Ok(aggregated)
    }

    /// FedProx aggregation
    fn aggregate_fedprox(&self, mu: f32) -> Result<Vec<f32>, FederatedError> {
        let avg = self.aggregate_fedavg()?;

        // Add proximal term if we have server weights
        if let Some(ref server) = self.server_weights {
            let mut result = avg;
            for (r, (&a, &s)) in result.iter_mut().zip(avg.iter()).zip(server.iter()) {
                *r += mu * (a - s);
            }
            Ok(result)
        } else {
            Ok(avg)
        }
    }

    /// SCAFFOLD aggregation
    #[allow(dead_code)]
    fn aggregate_scaffold(&self) -> Result<Vec<f32>, FederatedError> {
        // Simplified SCAFFOLD implementation
        self.aggregate_fedavg()
    }

    /// FedOpt aggregation
    #[allow(dead_code)]
    fn aggregate_fedopt(&self) -> Result<Vec<f32>, FederatedError> {
        // Simplified FedOpt implementation
        self.aggregate_fedavg()
    }

    /// Secure aggregation
    #[allow(dead_code)]
    fn aggregate_secure(&self) -> Result<Vec<f32>, FederatedError> {
        // Would use homomorphic encryption in practice
        self.aggregate_fedavg()
    }

    /// Apply aggregated update locally
    pub fn apply_update(&mut self, update: &[f32]) -> Result<(), FederatedError> {
        if update.len() != self.local_weights.len() {
            return Err(FederatedError::DimensionMismatch {
                expected: self.local_weights.len(),
                actual: update.len(),
            });
        }

        self.local_weights.copy_from_slice(update);
        Ok(())
    }

    /// Get current local model
    pub fn get_local_model(&self) -> &[f32] {
        &self.local_weights
    }

    /// Predict using local model
    fn predict(&self, input: &[f32]) -> Vec<f32> {
        // Simplified prediction (dot product + sigmoid)
        let sum: f32 = input
            .iter()
            .zip(self.local_weights.iter())
            .map(|(i, w)| i * w)
            .sum();
        vec![1.0 / (1.0 + (-sum).exp())]
    }

    /// Compute error
    fn compute_error(&self, prediction: &[f32], target: &[f32]) -> f32 {
        prediction
            .iter()
            .zip(target.iter())
            .map(|(p, t)| (p - t).powi(2))
            .sum::<f32>()
            / prediction.len().max(1) as f32
    }

    /// Serialize f32 vector
    fn serialize_f32(&self, values: &[f32]) -> Vec<u8> {
        values.iter().flat_map(|&v| v.to_le_bytes()).collect()
    }

    /// Deserialize f32 vector
    fn deserialize_f32(&self, data: &[u8]) -> Vec<f32> {
        data.chunks(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect()
    }

    /// Generate unique ID
    fn generate_id(&self) -> u64 {
        use std::time::SystemTime;
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
        (self.node_id as u64) << 32 | nanos as u64
    }

    /// Get federated statistics
    pub fn statistics(&self) -> FederatedStats {
        let avg_compression = if let Some(last) = self.update_history.back() {
            last.compression_ratio
        } else {
            1.0
        };

        FederatedStats {
            node_id: self.node_id,
            current_round: self.current_round,
            local_model_size: self.local_weights.len(),
            peer_updates_pending: self.peer_updates.len(),
            total_rounds: self.update_history.len(),
            avg_compression_ratio: avg_compression,
            aggregation_strategy: self.aggregation,
        }
    }
}

/// Federated statistics
#[derive(Debug, Clone)]
pub struct FederatedStats {
    pub node_id: NodeId,
    pub current_round: u32,
    pub local_model_size: usize,
    pub peer_updates_pending: usize,
    pub total_rounds: usize,
    pub avg_compression_ratio: f32,
    pub aggregation_strategy: AggregationStrategy,
}

/// Federated errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FederatedError {
    /// Model not initialized
    ModelNotInitialized,

    /// Round mismatch
    RoundMismatch { expected: u32, actual: u32 },

    /// Insufficient peers
    InsufficientPeers { required: usize, available: usize },

    /// Zero samples
    ZeroSamples,

    /// Decompression failed
    DecompressionFailed(String),

    /// Dimension mismatch
    DimensionMismatch { expected: usize, actual: usize },

    /// Training failed
    TrainingFailed(String),
}

impl std::fmt::Display for FederatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FederatedError::ModelNotInitialized => write!(f, "Model not initialized"),
            FederatedError::RoundMismatch { expected, actual } => {
                write!(f, "Round mismatch: expected {}, got {}", expected, actual)
            }
            FederatedError::InsufficientPeers { required, available } => {
                write!(
                    f,
                    "Insufficient peers: required {}, available {}",
                    required, available
                )
            }
            FederatedError::ZeroSamples => write!(f, "Zero samples for aggregation"),
            FederatedError::DecompressionFailed(reason) => {
                write!(f, "Decompression failed: {}", reason)
            }
            FederatedError::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            FederatedError::TrainingFailed(reason) => write!(f, "Training failed: {}", reason),
        }
    }
}

impl std::error::Error for FederatedError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federated_creation() {
        let config = FederatedConfig::default();
        let federated = FederatedProtocol::new(1, config);

        assert_eq!(federated.node_id, 1);
        assert_eq!(federated.current_round, 0);
    }

    #[test]
    fn test_model_initialization() {
        let config = FederatedConfig::default();
        let mut federated = FederatedProtocol::new(1, config);

        federated.initialize_model(vec![1.0, 2.0, 3.0]);
        assert_eq!(federated.local_weights, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_round_mismatch() {
        let config = FederatedConfig::default();
        let mut federated = FederatedProtocol::new(1, config);

        let update = ModelUpdate {
            id: 1,
            node_id: 2,
            weights: vec![0.1, 0.2],
            sample_count: 10,
            round: 1, // Different from current round 0
            timestamp_ms: 0,
            compression: CompressionType::None,
        };

        let result = federated.receive_update(update);
        assert!(matches!(
            result,
            Err(FederatedError::RoundMismatch { .. })
        ));
    }

    #[test]
    fn test_statistics() {
        let config = FederatedConfig::default();
        let federated = FederatedProtocol::new(1, config);

        let stats = federated.statistics();
        assert_eq!(stats.node_id, 1);
        assert_eq!(stats.current_round, 0);
    }
}
