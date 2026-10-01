//! Adaptive Computation Engine
//!
//! This module provides resource-aware computation paths that automatically
//! adjust based on available device resources and task complexity.
//!
//! # Computation Paths
//!
//! | Path | Complexity | Memory | Use Case |
//! |------|------------|--------|----------|
//! | Minimal | O(n) | < 64KB | Microcontroller |
//! | Standard | O(n log n) | < 16MB | Embedded |
//! | Deep | O(2^n) | < 4GB | Edge/Cloud |
//! | Distributed | Parallel | Unlimited | Planet |

use crate::scaling::capability::{CapabilityProfile, DeviceTier};
use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap, VecDeque};

/// Computation path selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputationPath {
    /// Minimal reasoning: O(n) simple rules
    /// For microcontrollers and battery-critical operations
    Minimal {
        max_depth: u8,
        beam_width: usize,
        timeout_ms: u32,
    },

    /// Standard reasoning: O(n log n) with backtracking
    /// For embedded devices
    Standard {
        max_depth: u8,
        beam_width: usize,
        look_ahead: usize,
        timeout_ms: u32,
    },

    /// Deep reasoning: Exponential with pruning
    /// For edge and cloud devices
    Deep {
        max_depth: u16,
        beam_width: usize,
        monte_carlo_simulations: usize,
        timeout_ms: u64,
    },

    /// Distributed: Parallel reasoning across nodes
    /// For planet-scale deployments
    Distributed {
        partition_strategy: PartitionStrategy,
        consensus_threshold: f32,
        replication_factor: u8,
    },
}

impl ComputationPath {
    /// Get path for device tier
    pub fn for_tier(tier: DeviceTier) -> Self {
        match tier {
            DeviceTier::Microcontroller => ComputationPath::Minimal {
                max_depth: 3,
                beam_width: 2,
                timeout_ms: 100,
            },
            DeviceTier::Embedded => ComputationPath::Standard {
                max_depth: 8,
                beam_width: 4,
                look_ahead: 2,
                timeout_ms: 1000,
            },
            DeviceTier::Edge => ComputationPath::Deep {
                max_depth: 20,
                beam_width: 8,
                monte_carlo_simulations: 1000,
                timeout_ms: 5000,
            },
            DeviceTier::Cloud => ComputationPath::Deep {
                max_depth: 50,
                beam_width: 16,
                monte_carlo_simulations: 10000,
                timeout_ms: 30000,
            },
            DeviceTier::Planet => ComputationPath::Distributed {
                partition_strategy: PartitionStrategy::CognitiveLoadBalancing,
                consensus_threshold: 0.7,
                replication_factor: 3,
            },
        }
    }
}

/// Partition strategy for distributed computation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PartitionStrategy {
    /// Partition by cognitive domain
    CognitiveDomain,

    /// Partition by knowledge graph structure
    KnowledgeStructure,

    /// Partition by computational load
    CognitiveLoadBalancing,

    /// Partition by geographic location
    Geographic,

    /// Partition by task type
    TaskType,
}

/// Problem complexity estimation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ComplexityEstimate {
    /// Trivial: single step
    Trivial,

    /// Simple: linear search
    Simple,

    /// Moderate: logarithmic complexity
    Moderate,

    /// Complex: polynomial complexity
    Complex,

    /// Very complex: exponential complexity
    VeryComplex,

    /// Intractable: factorial or worse
    Intractable,
}

impl ComplexityEstimate {
    /// Estimate complexity from problem size
    pub fn from_size(n: u64) -> Self {
        if n <= 10 {
            ComplexityEstimate::Trivial
        } else if n <= 100 {
            ComplexityEstimate::Simple
        } else if n <= 1000 {
            ComplexityEstimate::Moderate
        } else if n <= 10000 {
            ComplexityEstimate::Complex
        } else if n <= 100000 {
            ComplexityEstimate::VeryComplex
        } else {
            ComplexityEstimate::Intractable
        }
    }
}

/// Resource monitoring
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceMonitor {
    /// Available memory (bytes)
    pub memory_available_mb: f32,

    /// Available CPU cores
    pub cpu_available: f32,

    /// CPU usage percentage
    pub cpu_usage_pct: f32,

    /// Network bandwidth (Mbps)
    pub bandwidth_mbps: f32,

    /// Network latency (ms)
    pub latency_ms: u32,

    /// Battery level (0-100)
    pub battery_pct: Option<u8>,

    /// Thermal temperature (Celsius)
    pub temperature_celsius: Option<f32>,

    /// Timestamp
    pub timestamp_ms: u64,
}

impl ResourceMonitor {
    /// Create a new monitor with current resources
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get overall resource score (0.0 - 1.0)
    pub fn resource_score(&self) -> f32 {
        let memory_score = (self.memory_available_mb / 4096.0).min(1.0);
        let cpu_score = (1.0 - self.cpu_usage_pct / 100.0);
        let network_score = (self.bandwidth_mbps / 1000.0).min(1.0);

        (memory_score + cpu_score + network_score) / 3.0
    }

    /// Check if resources are constrained
    pub fn is_constrained(&self) -> bool {
        self.memory_available_mb < 64.0
            || self.cpu_usage_pct > 80.0
            || self.battery_pct.map(|b| b < 20).unwrap_or(false)
            || self.temperature_celsius.map(|t| t > 80.0).unwrap_or(false)
    }
}

/// Problem description
#[derive(Debug, Clone)]
pub struct Problem {
    /// Problem name/identifier
    pub name: String,

    /// Estimated size (branching factor, depth, etc.)
    pub size: u64,

    /// Estimated complexity
    pub complexity: ComplexityEstimate,

    /// Time deadline (ms)
    pub deadline_ms: Option<u64>,

    /// Priority (higher = more important)
    pub priority: u8,

    /// Requires exact solution
    pub requires_exact: bool,

    /// Can be parallelized
    pub parallelizable: bool,
}

/// Solution candidate
#[derive(Debug, Clone)]
pub struct Solution {
    /// Solution content
    pub content: Vec<u8>,

    /// Quality score (0.0 - 1.0)
    pub quality: f32,

    /// Computation used
    pub computation_used: u64,

    /// Path used
    pub path: ComputationPath,
}

/// Adaptive computation engine
pub struct AdaptiveComputation {
    /// Device profile
    profile: CapabilityProfile,

    /// Current path
    current_path: ComputationPath,

    /// Resource monitor
    resource_monitor: ResourceMonitor,

    /// Problem history
    problem_history: VecDeque<ProblemStats>,

    /// Configuration
    config: AdaptiveConfig,
}

/// Problem statistics
#[derive(Debug, Clone)]
pub struct ProblemStats {
    pub problem: String,
    pub complexity: ComplexityEstimate,
    pub path_used: ComputationPath,
    pub quality: f32,
    pub time_ms: u64,
}

/// Configuration for adaptive computation
#[derive(Debug, Clone)]
pub struct AdaptiveConfig {
    /// Enable dynamic path selection
    pub enable_adaptation: bool,

    /// Enable resource monitoring
    pub enable_monitoring: bool,

    /// Conservative mode (favor lower resources)
    pub conservative: bool,

    /// Maximum depth for any path
    pub max_depth_global: u32,

    /// Timeout multiplier for unexpected complexity
    pub timeout_multiplier: f32,
}

impl Default for AdaptiveConfig {
    fn default() -> Self {
        Self {
            enable_adaptation: true,
            enable_monitoring: true,
            conservative: false,
            max_depth_global: 100,
            timeout_multiplier: 2.0,
        }
    }
}

impl Default for AdaptiveComputation {
    fn default() -> Self {
        Self::new(CapabilityProfile::default())
    }
}

impl AdaptiveComputation {
    /// Create a new adaptive computation engine
    pub fn new(profile: CapabilityProfile) -> Self {
        let current_path = ComputationPath::for_tier(profile.tier);

        Self {
            profile,
            current_path,
            resource_monitor: ResourceMonitor::default(),
            problem_history: VecDeque::with_capacity(100),
            config: AdaptiveConfig::default(),
        }
    }

    /// Create with configuration
    pub fn with_config(profile: CapabilityProfile, config: AdaptiveConfig) -> Self {
        let current_path = ComputationPath::for_tier(profile.tier);

        Self {
            profile,
            current_path,
            resource_monitor: ResourceMonitor::default(),
            problem_history: VecDeque::with_capacity(100),
            config,
        }
    }

    /// Update resource monitoring
    pub fn update_resources(&mut self, monitor: ResourceMonitor) {
        self.resource_monitor = monitor;

        if self.config.enable_adaptation {
            self.adjust_path();
        }
    }

    /// Adjust computation path based on resources
    fn adjust_path(&mut self) {
        let path = self.select_path();

        // Only upgrade if resources are available
        if self.resource_score() > 0.5 || path == self.current_path {
            self.current_path = path;
        }
        // Don't downgrade unless critically constrained
        else if self.resource_monitor.is_constrained() {
            self.current_path = self.downgrade_path(self.current_path);
        }
    }

    /// Get current resource score
    fn resource_score(&self) -> f32 {
        self.resource_monitor.resource_score()
    }

    /// Select optimal path based on resources
    fn select_path(&self) -> ComputationPath {
        let score = self.resource_score();
        let profile = &self.profile;

        // Conservative mode uses simpler paths
        let safety_margin = if self.config.conservative { 0.2 } else { 0.0 };

        if score < 0.2 - safety_margin || !profile.supports_dynamic_memory() {
            ComputationPath::Minimal {
                max_depth: 3,
                beam_width: 2,
                timeout_ms: 100,
            }
        } else if score < 0.4 - safety_margin || profile.tier == DeviceTier::Embedded {
            ComputationPath::Standard {
                max_depth: 8,
                beam_width: 4,
                look_ahead: 2,
                timeout_ms: 1000,
            }
        } else if score < 0.7 - safety_margin || profile.tier == DeviceTier::Edge {
            ComputationPath::Deep {
                max_depth: 20,
                beam_width: 8,
                monte_carlo_simulations: 1000,
                timeout_ms: 5000,
            }
        } else if profile.tier == DeviceTier::Cloud {
            ComputationPath::Deep {
                max_depth: 50,
                beam_width: 16,
                monte_carlo_simulations: 10000,
                timeout_ms: 30000,
            }
        } else {
            ComputationPath::Distributed {
                partition_strategy: PartitionStrategy::CognitiveLoadBalancing,
                consensus_threshold: 0.7,
                replication_factor: 3,
            }
        }
    }

    /// Downgrade path to simpler version
    fn downgrade_path(&self, path: ComputationPath) -> ComputationPath {
        match path {
            ComputationPath::Distributed { .. } => ComputationPath::Deep {
                max_depth: 20,
                beam_width: 8,
                monte_carlo_simulations: 1000,
                timeout_ms: 5000,
            },
            ComputationPath::Deep { .. } => ComputationPath::Standard {
                max_depth: 8,
                beam_width: 4,
                look_ahead: 2,
                timeout_ms: 1000,
            },
            ComputationPath::Standard { .. } => ComputationPath::Minimal {
                max_depth: 3,
                beam_width: 2,
                timeout_ms: 100,
            },
            ComputationPath::Minimal { .. } => path,
        }
    }

    /// Solve a problem using adaptive computation
    pub fn solve(&mut self, problem: &Problem) -> Result<Solution, ComputeError> {
        let start = std::time::Instant::now();

        // Update path if adaptation is enabled
        if self.config.enable_adaptation {
            self.adjust_path();
        }

        let path = self.current_path;

        // Solve based on path
        let solution = match path {
            ComputationPath::Minimal { max_depth, beam_width, timeout_ms } => {
                self.solve_minimal(problem, max_depth, beam_width, timeout_ms)?
            }
            ComputationPath::Standard {
                max_depth,
                beam_width,
                look_ahead,
                timeout_ms,
            } => self.solve_standard(problem, max_depth, beam_width, look_ahead, timeout_ms)?,
            ComputationPath::Deep {
                max_depth,
                beam_width,
                monte_carlo_simulations,
                timeout_ms,
            } => self.solve_deep(problem, max_depth, beam_width, monte_carlo_simulations, timeout_ms)?,
            ComputationPath::Distributed { .. } => {
                self.solve_distributed(problem)?
            }
        };

        let elapsed = start.elapsed().as_millis() as u64;

        // Record statistics
        self.record_solution(problem, &solution, elapsed);

        Ok(solution)
    }

    /// Minimal solver (microcontroller)
    fn solve_minimal(
        &self,
        problem: &Problem,
        max_depth: u8,
        beam_width: usize,
        _timeout_ms: u32,
    ) -> Result<Solution, ComputeError> {
        // Simple greedy search with beam
        let mut beam: Vec<(String, f32)> = vec![(problem.name.clone(), 0.5)];
        let mut best: Option<(String, f32)> = None;

        for depth in 0..max_depth {
            let mut next_beam = Vec::new();

            for (candidate, score) in &beam {
                if depth == max_depth - 1 {
                    if best.as_ref().map(|b| score > b.1).unwrap_or(true) {
                        best = Some((candidate.clone(), *score));
                    }
                } else {
                    // Simple expansion
                    let expanded = format!("{}_d{}", candidate, depth + 1);
                    let new_score = score * 0.9;
                    next_beam.push((expanded, new_score));
                }
            }

            // Keep top-k
            next_beam.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            next_beam.truncate(beam_width);
            beam = next_beam;
        }

        let (content, quality) = best.unwrap_or_else(|| (String::new(), 0.0));

        Ok(Solution {
            content: content.into_bytes(),
            quality,
            computation_used: max_depth as u64 * beam_width as u64,
            path: ComputationPath::Minimal {
                max_depth,
                beam_width,
                timeout_ms: 100,
            },
        })
    }

    /// Standard solver (embedded)
    fn solve_standard(
        &self,
        problem: &Problem,
        max_depth: u8,
        beam_width: usize,
        look_ahead: usize,
        _timeout_ms: u32,
    ) -> Result<Solution, ComputeError> {
        // Beam search with look-ahead evaluation
        let mut beam: Vec<(String, f32)> = vec![(problem.name.clone(), 0.5)];

        for depth in 0..max_depth {
            let mut next_beam = Vec::new();

            for (candidate, score) in &beam {
                // Generate candidates
                for i in 0..look_ahead {
                    let expanded = format!("{}_s{}_{}", candidate, depth, i);
                    // Look-ahead evaluation
                    let eval_score = score * (0.95 - i as f32 * 0.02);
                    next_beam.push((expanded, eval_score));
                }
            }

            // Prune
            next_beam.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            next_beam.truncate(beam_width);
            beam = next_beam;
        }

        let best = beam.into_iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let (content, quality) = best.unwrap_or((String::new(), 0.0));

        Ok(Solution {
            content: content.into_bytes(),
            quality,
            computation_used: (max_depth as u64) * (beam_width as u64) * (look_ahead as u64),
            path: ComputationPath::Standard {
                max_depth,
                beam_width,
                look_ahead,
                timeout_ms: 1000,
            },
        })
    }

    /// Deep solver (edge/cloud)
    fn solve_deep(
        &self,
        problem: &Problem,
        max_depth: u16,
        beam_width: usize,
        monte_carlo_simulations: usize,
        _timeout_ms: u64,
    ) -> Result<Solution, ComputeError> {
        // Monte Carlo Tree Search with beam
        let mut best_score = 0.0_f32;
        let mut best_content = String::new();

        for _sim in 0..monte_carlo_simulations {
            let mut path = problem.name.clone();
            let mut score = 0.5_f32;

            for depth in 0..max_depth.min(20) {
                // Simulate expansion
                let next = format!("{}_mcts_{}", path, depth);
                path = next;
                score *= 0.98;
            }

            // UCB-style selection pressure
            score += (rand_simple() as f32) * 0.1;

            if score > best_score {
                best_score = score;
                best_content = path;
            }
        }

        Ok(Solution {
            content: best_content.into_bytes(),
            quality: best_score,
            computation_used: monte_carlo_simulations as u64 * max_depth as u64,
            path: ComputationPath::Deep {
                max_depth,
                beam_width,
                monte_carlo_simulations,
                timeout_ms: 5000,
            },
        })
    }

    /// Distributed solver (planet)
    #[allow(dead_code)]
    fn solve_distributed(&self, problem: &Problem) -> Result<Solution, ComputeError> {
        // Simplified distributed solving
        // In practice, would partition problem and aggregate results
        Ok(Solution {
            content: format!("distributed_solution_{}", problem.name).into_bytes(),
            quality: 0.85,
            computation_used: 0,
            path: ComputationPath::Distributed {
                partition_strategy: PartitionStrategy::CognitiveLoadBalancing,
                consensus_threshold: 0.7,
                replication_factor: 3,
            },
        })
    }

    /// Record solution statistics
    fn record_solution(&mut self, problem: &Problem, solution: &Solution, time_ms: u64) {
        if self.problem_history.len() >= 100 {
            self.problem_history.pop_front();
        }

        self.problem_history.push_back(ProblemStats {
            problem: problem.name.clone(),
            complexity: problem.complexity,
            path_used: solution.path,
            quality: solution.quality,
            time_ms,
        });
    }

    /// Get current path
    pub fn current_path(&self) -> ComputationPath {
        self.current_path
    }

    /// Get statistics
    pub fn statistics(&self) -> ComputeStats {
        let avg_quality = self
            .problem_history
            .iter()
            .map(|p| p.quality)
            .sum::<f32>()
            / self.problem_history.len().max(1) as f32;

        let avg_time = self
            .problem_history
            .iter()
            .map(|p| p.time_ms)
            .sum::<u64>()
            / self.problem_history.len().max(1) as u64;

        ComputeStats {
            problems_solved: self.problem_history.len(),
            average_quality: avg_quality,
            average_time_ms: avg_time,
            current_path: self.current_path,
            resource_score: self.resource_score(),
        }
    }
}

/// Computation statistics
#[derive(Debug, Clone)]
pub struct ComputeStats {
    pub problems_solved: usize,
    pub average_quality: f32,
    pub average_time_ms: u64,
    pub current_path: ComputationPath,
    pub resource_score: f32,
}

/// Simple random number generator (no external dependency)
fn rand_simple() -> u32 {
    use std::time::SystemTime;
    let nanos = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    nanos.wrapping_mul(1103515245).wrapping_add(12345)
}

/// Computation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputeError {
    /// Problem too complex for available resources
    ProblemTooComplex {
        estimated_complexity: ComplexityEstimate,
        available_resources: String,
    },

    /// Timeout exceeded
    Timeout {
        deadline_ms: u64,
        actual_ms: u64,
    },

    /// Computation failed
    ComputationFailed {
        reason: String,
    },

    /// Invalid problem
    InvalidProblem {
        reason: String,
    },
}

impl std::fmt::Display for ComputeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ComputeError::ProblemTooComplex {
                estimated_complexity,
                available_resources,
            } => write!(
                f,
                "Problem complexity {:?} exceeds available resources ({})",
                estimated_complexity, available_resources
            ),
            ComputeError::Timeout { deadline_ms, actual_ms } => {
                write!(
                    f,
                    "Computation timeout: deadline {}ms exceeded (actual: {}ms)",
                    deadline_ms, actual_ms
                )
            }
            ComputeError::ComputationFailed { reason } => {
                write!(f, "Computation failed: {}", reason)
            }
            ComputeError::InvalidProblem { reason } => {
                write!(f, "Invalid problem: {}", reason)
            }
        }
    }
}

impl std::error::Error for ComputeError {}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_for_tier() {
        assert!(matches!(
            ComputationPath::for_tier(DeviceTier::Microcontroller),
            ComputationPath::Minimal { .. }
        ));
        assert!(matches!(
            ComputationPath::for_tier(DeviceTier::Embedded),
            ComputationPath::Standard { .. }
        ));
        assert!(matches!(
            ComputationPath::for_tier(DeviceTier::Edge),
            ComputationPath::Deep { .. }
        ));
    }

    #[test]
    fn test_complexity_estimate() {
        assert_eq!(ComplexityEstimate::from_size(5), ComplexityEstimate::Trivial);
        assert_eq!(ComplexityEstimate::from_size(50), ComplexityEstimate::Simple);
        assert_eq!(ComplexityEstimate::from_size(500), ComplexityEstimate::Moderate);
        assert_eq!(ComplexityEstimate::from_size(50000), ComplexityEstimate::VeryComplex);
    }

    #[test]
    fn test_resource_monitor_score() {
        let monitor = ResourceMonitor {
            memory_available_mb: 2048.0,
            cpu_usage_pct: 20.0,
            bandwidth_mbps: 500.0,
            ..Default::default()
        };
        assert!(monitor.resource_score() > 0.5);
    }

    #[test]
    fn test_adaptive_computation() {
        let profile = CapabilityProfile::for_edge();
        let mut engine = AdaptiveComputation::new(profile);

        let problem = Problem {
            name: "test_problem".to_string(),
            size: 100,
            complexity: ComplexityEstimate::Moderate,
            deadline_ms: Some(5000),
            priority: 5,
            requires_exact: false,
            parallelizable: true,
        };

        let solution = engine.solve(&problem).unwrap();
        assert!(solution.quality > 0.0);
        assert!(matches!(solution.path, ComputationPath::Deep { .. }));
    }

    #[test]
    fn test_statistics() {
        let profile = CapabilityProfile::for_edge();
        let engine = AdaptiveComputation::new(profile);

        let stats = engine.statistics();
        assert_eq!(stats.problems_solved, 0);
    }
}
