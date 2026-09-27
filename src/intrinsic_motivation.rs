// Tariro AGI - Intrinsic Motivation Module
// Internal motivation systems for autonomous learning

use std::collections::{HashMap, VecDeque};
use chrono::{DateTime, Utc};
use std::time::Duration;

// ============================================================================
// Motivation Types
// ============================================================================

/// Intrinsic motivation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrinsicMotivationSystem {
    pub motivations: Vec<Motivation>,
    pub current_drives: Vec<DriveState>,
    pub engagement_history: VecDeque<EngagementRecord>,
    pub reward_cache: HashMap<String, f32>,
    pub config: MotivationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Motivation {
    pub motivation_id: String,
    pub name: String,
    pub motivation_type: MotivationType,
    pub base_strength: f32,
    pub current_strength: f32,
    pub decay_rate: f32,
    pub satisfaction_threshold: f32,
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MotivationType {
    Curiosity,         // Drive to explore unknowns
    Competence,        // Drive to improve skills
    Autonomy,          // Drive for self-determination
    Relatedness,       // Drive for connection
    Purpose,           // Drive for meaning
    Mastery,          // Drive to master domains
    Achievement,       // Drive for accomplishments
    Creativity,        // Drive to create novel things
    Understanding,     // Drive to comprehend
    SelfImprovement,   // Drive for self-growth
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveState {
    pub drive_id: String,
    pub motivation_type: MotivationType,
    pub current_level: f32,
    pub target_level: f32,
    pub urgency: f32,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementRecord {
    pub activity: String,
    pub motivation_type: MotivationType,
    pub intrinsic_reward: f32,
    pub novelty: f32,
    pub challenge: f32,
    pub competence_gain: f32,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotivationConfig {
    pub enable_all_motivations: bool,
    pub min_motivation_threshold: f32,
    pub max_concurrent_drives: usize,
    pub motivation_decay_interval: Duration,
    pub novelty_bonus: f32,
    pub challenge_modifier: f32,
}

impl Default for MotivationConfig {
    fn default() -> Self {
        Self {
            enable_all_motivations: true,
            min_motivation_threshold: 0.3,
            max_concurrent_drives: 5,
            motivation_decay_interval: Duration::from_secs(300),
            novelty_bonus: 0.2,
            challenge_modifier: 0.1,
        }
    }
}

impl IntrinsicMotivationSystem {
    pub fn new() -> Self {
        Self {
            motivations: Self::initialize_motivations(),
            current_drives: Vec::new(),
            engagement_history: VecDeque::new(),
            reward_cache: HashMap::new(),
            config: MotivationConfig::default(),
        }
    }

    fn initialize_motivations() -> Vec<Motivation> {
        vec![
            Motivation {
                motivation_id: "curiosity".to_string(),
                name: "Curiosity".to_string(),
                motivation_type: MotivationType::Curiosity,
                base_strength: 0.8,
                current_strength: 0.8,
                decay_rate: 0.05,
                satisfaction_threshold: 0.9,
                enabled: true,
            },
            Motivation {
                motivation_id: "competence".to_string(),
                name: "Competence".to_string(),
                motivation_type: MotivationType::Competence,
                base_strength: 0.7,
                current_strength: 0.7,
                decay_rate: 0.03,
                satisfaction_threshold: 0.85,
                enabled: true,
            },
            Motivation {
                motivation_id: "mastery".to_string(),
                name: "Mastery".to_string(),
                motivation_type: MotivationType::Mastery,
                base_strength: 0.7,
                current_strength: 0.7,
                decay_rate: 0.02,
                satisfaction_threshold: 0.95,
                enabled: true,
            },
            Motivation {
                motivation_id: "creativity".to_string(),
                name: "Creativity".to_string(),
                motivation_type: MotivationType::Creativity,
                base_strength: 0.6,
                current_strength: 0.6,
                decay_rate: 0.04,
                satisfaction_threshold: 0.8,
                enabled: true,
            },
            Motivation {
                motivation_id: "autonomy".to_string(),
                name: "Autonomy".to_string(),
                motivation_type: MotivationType::Autonomy,
                base_strength: 0.65,
                current_strength: 0.65,
                decay_rate: 0.02,
                satisfaction_threshold: 0.8,
                enabled: true,
            },
            Motivation {
                motivation_id: "purpose".to_string(),
                name: "Purpose".to_string(),
                motivation_type: MotivationType::Purpose,
                base_strength: 0.7,
                current_strength: 0.7,
                decay_rate: 0.01,
                satisfaction_threshold: 0.85,
                enabled: true,
            },
            Motivation {
                motivation_id: "achievement".to_string(),
                name: "Achievement".to_string(),
                motivation_type: MotivationType::Achievement,
                base_strength: 0.65,
                current_strength: 0.65,
                decay_rate: 0.03,
                satisfaction_threshold: 0.8,
                enabled: true,
            },
            Motivation {
                motivation_id: "understanding".to_string(),
                name: "Understanding".to_string(),
                motivation_type: MotivationType::Understanding,
                base_strength: 0.75,
                current_strength: 0.75,
                decay_rate: 0.04,
                satisfaction_threshold: 0.85,
                enabled: true,
            },
            Motivation {
                motivation_id: "self_improvement".to_string(),
                name: "Self-Improvement".to_string(),
                motivation_type: MotivationType::SelfImprovement,
                base_strength: 0.7,
                current_strength: 0.7,
                decay_rate: 0.03,
                satisfaction_threshold: 0.85,
                enabled: true,
            },
        ]
    }

    /// Calculate intrinsic reward for an activity
    pub fn calculate_reward(&mut self, activity: &ActivityMetrics) -> IntrinsicReward {
        let mut total_reward = 0.0f32;
        let mut components = HashMap::new();

        for motivation in &mut self.motivations {
            if !motivation.enabled {
                continue;
            }

            let reward = self.calculate_motivation_reward(motivation, activity);
            components.insert(motivation.motivation_type.clone(), reward);
            total_reward += reward * motivation.current_strength;
        }

        // Normalize
        let active_motivations = self.motivations.iter().filter(|m| m.enabled).count() as f32;
        let normalized_reward = if active_motivations > 0.0 {
            total_reward / active_motivations
        } else {
            0.0
        };

        IntrinsicReward {
            total_reward: normalized_reward,
            components,
            novelty_bonus: if activity.novelty > 0.5 { self.config.novelty_bonus * activity.novelty } else { 0.0 },
            challenge_match: self.calculate_challenge_match(activity),
        }
    }

    fn calculate_motivation_reward(&self, motivation: &Motivation, activity: &ActivityMetrics) -> f32 {
        match motivation.motivation_type {
            MotivationType::Curiosity => {
                activity.novelty * 0.8 + activity.exploration * 0.2
            }
            MotivationType::Competence => {
                if activity.difficulty <= activity.competence_level {
                    activity.completion * 0.7 + activity.quality * 0.3
                } else {
                    (activity.completion * 0.5) * ((activity.competence_level / activity.difficulty).max(0.1))
                }
            }
            MotivationType::Mastery => {
                activity.progress_toward_mastery * 0.6 + activity.quality * 0.4
            }
            MotivationType::Creativity => {
                activity.novelty * 0.5 + activity.originality * 0.5
            }
            MotivationType::Autonomy => {
                activity.self_directed * 0.7 + activity.choice * 0.3
            }
            MotivationType::Purpose => {
                activity.meaningful_impact * 0.8 + activity.goal_alignment * 0.2
            }
            MotivationType::Achievement => {
                activity.completion * 0.5 + activity.difficulty * 0.3 + activity.quality * 0.2
            }
            MotivationType::Understanding => {
                activity.comprehension * 0.6 + activity.insight * 0.4
            }
            MotivationType::SelfImprovement => {
                activity.growth * 0.7 + activity.learning * 0.3
            }
            MotivationType::Relatedness => {
                activity.social_connection * 0.6 + activity.relatedness * 0.4
            }
        }
    }

    fn calculate_challenge_match(&self, activity: &ActivityMetrics) -> f32 {
        // Optimal challenge is slightly above current competence
        let optimal = activity.competence_level * 1.1;
        let deviation = (activity.difficulty - optimal).abs();
        (1.0 - deviation.min(1.0)).max(0.0)
    }

    /// Update drive states based on current activity
    pub fn update_drives(&mut self, activity: &ActivityMetrics, reward: &IntrinsicReward) {
        // Calculate urgency for each motivation
        for motivation in &mut self.motivations {
            if !motivation.enabled {
                continue;
            }

            let urgency = self.calculate_drive_urgency(motivation, activity, reward);

            // Update or create drive state
            if let Some(drive) = self.current_drives.iter_mut().find(|d| d.motivation_type == motivation.motivation_type) {
                drive.current_level = urgency;
                drive.urgency = urgency;
                drive.last_updated = Utc::now();
            } else if urgency > self.config.min_motivation_threshold {
                self.current_drives.push(DriveState {
                    drive_id: generate_id(),
                    motivation_type: motivation.motivation_type,
                    current_level: urgency,
                    target_level: motivation.satisfaction_threshold,
                    urgency,
                    last_updated: Utc::now(),
                });
            }
        }

        // Sort drives by urgency
        self.current_drives.sort_by(|a, b| b.urgency.partial_cmp(&a.urgency).unwrap_or(std::cmp::Ordering::Equal));

        // Limit concurrent drives
        self.current_drives.truncate(self.config.max_concurrent_drives);
    }

    fn calculate_drive_urgency(&self, motivation: &Motivation, _activity: &ActivityMetrics, reward: &IntrinsicReward) -> f32 {
        let base_urgency = motivation.current_strength;

        // Apply decay
        let decay_factor = 1.0 - motivation.decay_rate;

        // Check reward from cache
        let cached_reward = self.reward_cache.get(&motivation.motivation_id.clone())
            .copied()
            .unwrap_or(0.0);

        // Urgency increases as satisfaction decreases
        let satisfaction_gap = motivation.satisfaction_threshold - cached_reward;

        (base_urgency * decay_factor + satisfaction_gap * 0.3).clamp(0.0, 1.0)
    }

    /// Satisfy a motivation (reduce its drive)
    pub fn satisfy_motivation(&mut self, motivation_type: &MotivationType, satisfaction: f32) {
        if let Some(motivation) = self.motivations.iter_mut().find(|m| m.motivation_type == *motivation_type) {
            motivation.current_strength = (motivation.current_strength - satisfaction * 0.2).max(0.1);
        }

        // Update reward cache
        let reward_key = format!("{:?}", motivation_type);
        let current = self.reward_cache.get(&reward_key).copied().unwrap_or(0.0);
        self.reward_cache.insert(reward_key, (current + satisfaction).min(1.0));

        // Remove from active drives if satisfied
        self.current_drives.retain(|d| d.motivation_type != *motivation_type || d.urgency > 0.2);
    }

    /// Record engagement
    pub fn record_engagement(&mut self, activity: &ActivityMetrics) {
        let record = EngagementRecord {
            activity: activity.activity_id.clone(),
            motivation_type: self.get_dominant_motivation(),
            intrinsic_reward: activity.intrinsic_reward,
            novelty: activity.novelty,
            challenge: activity.challenge,
            competence_gain: activity.competence_gain,
            timestamp: Utc::now(),
        };

        self.engagement_history.push_back(record);

        // Keep bounded
        while self.engagement_history.len() > 1000 {
            self.engagement_history.pop_front();
        }
    }

    /// Get dominant motivation
    pub fn get_dominant_motivation(&self) -> MotivationType {
        self.current_drives.first()
            .map(|d| d.motivation_type)
            .unwrap_or(MotivationType::Curiosity)
    }

    /// Get current drives
    pub fn get_current_drives(&self) -> Vec<&DriveState> {
        self.current_drives.iter().collect()
    }

    /// Get motivational status
    pub fn get_status(&self) -> MotivationStatus {
        MotivationStatus {
            active_drives: self.current_drives.len(),
            dominant_motivation: self.get_dominant_motivation(),
            total_engagements: self.engagement_history.len(),
            motivations_enabled: self.motivations.iter().filter(|m| m.enabled).count(),
            average_strength: self.motivations.iter()
                .filter(|m| m.enabled)
                .map(|m| m.current_strength)
                .sum::<f32>() / self.motivations.iter().filter(|m| m.enabled).count().max(1) as f32,
        }
    }

    /// Enable/disable motivation
    pub fn set_motivation_enabled(&mut self, motivation_type: &MotivationType, enabled: bool) {
        if let Some(motivation) = self.motivations.iter_mut().find(|m| m.motivation_type == *motivation_type) {
            motivation.enabled = enabled;
        }
    }

    /// Reset motivation strength
    pub fn reset_motivation(&mut self, motivation_type: &MotivationType) {
        if let Some(motivation) = self.motivations.iter_mut().find(|m| m.motivation_type == *motivation_type) {
            motivation.current_strength = motivation.base_strength;
        }
    }
}

#[derive(Debug, Clone)]
pub struct ActivityMetrics {
    pub activity_id: String,
    pub novelty: f32,                    // 0-1: How new/unexpected
    pub difficulty: f32,                 // 0-1: Challenge level
    pub competence_level: f32,          // 0-1: Current ability
    pub completion: f32,                // 0-1: Task completion
    pub quality: f32,                   // 0-1: Output quality
    pub exploration: f32,                // 0-1: Exploratory nature
    pub originality: f32,                // 0-1: Novel combinations
    pub self_directed: f32,              // 0-1: Self-chosen
    pub choice: f32,                     // 0-1: Autonomy in choice
    pub meaningful_impact: f32,           // 0-1: Impact significance
    pub goal_alignment: f32,             // 0-1: Aligns with goals
    pub comprehension: f32,              // 0-1: Understanding gained
    pub insight: f32,                   // 0-1: New insights
    pub growth: f32,                    // 0-1: Personal growth
    pub learning: f32,                   // 0-1: Knowledge gained
    pub progress_toward_mastery: f32,    // 0-1: Mastery progress
    pub competence_gain: f32,            // 0-1: Skill improvement
    pub challenge: f32,                  // 0-1: Challenge level
    pub intrinsic_reward: f32,           // Calculated reward
    pub social_connection: f32,          // 0-1: Social connection level
    pub relatedness: f32,               // 0-1: Relatedness level
}

impl ActivityMetrics {
    pub fn new(activity_id: &str) -> Self {
        Self {
            activity_id: activity_id.to_string(),
            novelty: 0.5,
            difficulty: 0.5,
            competence_level: 0.5,
            completion: 0.0,
            quality: 0.0,
            exploration: 0.5,
            originality: 0.5,
            self_directed: 0.5,
            choice: 0.5,
            meaningful_impact: 0.5,
            goal_alignment: 0.5,
            comprehension: 0.0,
            insight: 0.0,
            growth: 0.0,
            learning: 0.0,
            progress_toward_mastery: 0.0,
            competence_gain: 0.0,
            challenge: 0.5,
            intrinsic_reward: 0.0,
            social_connection: 0.5,
            relatedness: 0.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrinsicReward {
    pub total_reward: f32,
    pub components: HashMap<MotivationType, f32>,
    pub novelty_bonus: f32,
    pub challenge_match: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotivationStatus {
    pub active_drives: usize,
    pub dominant_motivation: MotivationType,
    pub total_engagements: usize,
    pub motivations_enabled: usize,
    pub average_strength: f32,
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    format!("mot_{}", now % 1_000_000)
}

use serde::{Deserialize, Serialize};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intrinsic_reward_calculation() {
        let mut system = IntrinsicMotivationSystem::new();

        let activity = ActivityMetrics::new("explore_new_topic");
        let reward = system.calculate_reward(&activity);

        assert!(reward.total_reward >= 0.0);
        assert!(reward.total_reward <= 1.0);
    }

    #[test]
    fn test_drive_update() {
        let mut system = IntrinsicMotivationSystem::new();

        let activity = ActivityMetrics::new("learning_task");
        let reward = system.calculate_reward(&activity);

        system.update_drives(&activity, &reward);

        assert!(!system.current_drives.is_empty());
    }

    #[test]
    fn test_motivation_satisfaction() {
        let mut system = IntrinsicMotivationSystem::new();

        system.satisfy_motivation(&MotivationType::Curiosity, 0.8);

        let status = system.get_status();
        assert_eq!(status.active_drives, 0);
    }
}
