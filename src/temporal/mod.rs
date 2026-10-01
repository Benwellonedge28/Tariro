//! Tariro Temporal Reasoning Module
//!
//! Implements temporal cognition:
//! - Temporal representation
//! - Time perception and reasoning
//! - Episode construction
//! - Future projection

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};

// ============================================================================
// Core Types
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TemporalId(pub Uuid);

impl TemporalId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TemporalId {
    fn default() -> Self {
        Self::new()
    }
}

/// Point in time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TimePoint {
    pub timestamp: DateTime<Utc>,
    pub certainty: f64,
}

impl TimePoint {
    pub fn now() -> Self {
        Self {
            timestamp: Utc::now(),
            certainty: 1.0,
        }
    }

    pub fn seconds_ago(seconds: i64) -> Self {
        Self {
            timestamp: Utc::now() - Duration::seconds(seconds),
            certainty: 1.0,
        }
    }
}

/// Time interval
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeInterval {
    pub start: TimePoint,
    pub end: TimePoint,
    pub duration_seconds: i64,
}

impl TimeInterval {
    pub fn new(start: TimePoint, end: TimePoint) -> Self {
        let duration = (end.timestamp - start.timestamp).num_seconds();
        Self {
            start,
            end,
            duration_seconds: duration,
        }
    }

    pub fn contains(&self, time: &TimePoint) -> bool {
        time.timestamp >= self.start.timestamp && time.timestamp <= self.end.timestamp
    }
}

/// Temporal relation between events
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemporalRelation {
    Before,      // A before B
    After,       // A after B
    During,      // A during B
    Overlaps,    // A overlaps B
    Simultaneous, // A at same time as B
    Within,      // A within B
}

/// Temporal event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalEvent {
    pub id: TemporalId,
    pub description: String,
    pub time: TimePoint,
    pub duration: Option<TimeInterval>,
    pub temporal_relation: Option<TemporalRelation>,
}

// ============================================================================
// Temporal Reasoning
// ============================================================================

/// Temporal reasoning engine
#[derive(Debug, Clone)]
pub struct TemporalReasoner {
    pub clock: DateTime<Utc>,
    pub temporal_resolution: i64, // in seconds
}

impl TemporalReasoner {
    pub fn new() -> Self {
        Self {
            clock: Utc::now(),
            temporal_resolution: 1,
        }
    }

    /// Get current time
    pub fn now(&self) -> TimePoint {
        TimePoint::now()
    }

    /// Calculate temporal distance
    pub fn distance(&self, time1: &TimePoint, time2: &TimePoint) -> i64 {
        (time1.timestamp - time2.timestamp).num_seconds().abs()
    }

    /// Determine temporal relation
    pub fn relate(&self, event1: &TemporalEvent, event2: &TemporalEvent) -> TemporalRelation {
        let diff = (event1.time.timestamp - event2.time.timestamp).num_seconds();

        if diff < -60 {
            TemporalRelation::Before
        } else if diff > 60 {
            TemporalRelation::After
        } else {
            TemporalRelation::Simultaneous
        }
    }

    /// Project to future
    pub fn project_future(&self, duration_seconds: i64) -> TimePoint {
        TimePoint {
            timestamp: self.clock + Duration::seconds(duration_seconds),
            certainty: 0.8,
        }
    }

    /// Parse temporal expressions
    pub fn parse_temporal(&self, expression: &str) -> Option<TimePoint> {
        let lower = expression.to_lowercase();

        if lower.contains("now") || lower.contains("today") {
            Some(TimePoint::now())
        } else if lower.contains("yesterday") {
            Some(TimePoint::seconds_ago(86400))
        } else if lower.contains("tomorrow") {
            Some(TimePoint {
                timestamp: Utc::now() + Duration::days(1),
                certainty: 0.9,
            })
        } else {
            None
        }
    }
}

impl Default for TemporalReasoner {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Episode Construction
// ============================================================================

/// A temporal episode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Episode {
    pub id: TemporalId,
    pub events: Vec<TemporalEvent>,
    pub time_span: TimeInterval,
    pub coherence: f64,
    pub narrative: String,
}

impl Episode {
    pub fn new(start: TimePoint, end: TimePoint) -> Self {
        Self {
            id: TemporalId::new(),
            events: Vec::new(),
            time_span: TimeInterval::new(start, end),
            coherence: 0.5,
            narrative: String::new(),
        }
    }

    pub fn add_event(&mut self, event: TemporalEvent) {
        self.events.push(event);
    }

    pub fn duration(&self) -> i64 {
        self.time_span.duration_seconds
    }
}

/// Episode constructor
#[derive(Debug, Clone)]
pub struct EpisodeConstructor {
    pub events: VecDeque<TemporalEvent>,
    pub current_episode: Option<Episode>,
    pub coherence_threshold: f64,
}

impl EpisodeConstructor {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
            current_episode: None,
            coherence_threshold: 0.5,
        }
    }

    /// Add event to stream
    pub fn add_event(&mut self, event: TemporalEvent) {
        if self.current_episode.is_none() {
            self.current_episode = Some(Episode::new(event.time, event.time));
        }

        if let Some(ref mut episode) = self.current_episode {
            episode.add_event(event.clone());

            // Check if event is too far from episode start
            let gap = (event.time.timestamp - episode.time_span.start.timestamp).num_seconds();
            if gap > 3600 {
                // > 1 hour gap, start new episode
                self.finalize_episode();
                self.current_episode = Some(Episode::new(event.time, event.time));
            }
        }
    }

    /// Finalize current episode
    pub fn finalize_episode(&mut self) -> Option<Episode> {
        let episode = self.current_episode.take();
        if episode.is_some() {
            self.events.push_back(TemporalEvent {
                id: TemporalId::new(),
                description: "Episode boundary".to_string(),
                time: TimePoint::now(),
                duration: None,
                temporal_relation: None,
            });
        }
        episode
    }

    /// Construct episode from events
    pub fn construct(&mut self, events: Vec<TemporalEvent>) -> Episode {
        let start = events.first().map(|e| e.time).unwrap_or_else(TimePoint::now);
        let end = events.last().map(|e| e.time).unwrap_or_else(TimePoint::now);

        let mut episode = Episode::new(start, end);

        for event in events {
            episode.add_event(event);
        }

        episode.coherence = self.calculate_coherence(&episode);
        episode
    }

    fn calculate_coherence(&self, episode: &Episode) -> f64 {
        if episode.events.len() < 2 {
            return 0.5;
        }

        let mut total_gap = 0i64;
        for i in 1..episode.events.len() {
            total_gap += (episode.events[i].time.timestamp - episode.events[i-1].time.timestamp)
                .num_seconds()
                .abs();
        }

        let avg_gap = total_gap as f64 / (episode.events.len() - 1) as f64;

        // Closer gaps = higher coherence
        (1.0 / (1.0 + avg_gap / 60.0)).min(1.0)
    }
}

impl Default for EpisodeConstructor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Timeline Construction
// ============================================================================

/// Timeline representation
#[derive(Debug, Clone)]
pub struct Timeline {
    pub episodes: Vec<Episode>,
    pub markers: Vec<TimelineMarker>,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            episodes: Vec::new(),
            markers: Vec::new(),
        }
    }

    pub fn add_episode(&mut self, episode: Episode) {
        self.episodes.push(episode);
        self.episodes.sort_by(|a, b| a.time_span.start.timestamp.cmp(&b.time_span.start.timestamp));
    }

    /// Get episode at time
    pub fn get_episode_at(&self, time: &TimePoint) -> Option<&Episode> {
        self.episodes.iter().find(|e| e.time_span.contains(time))
    }

    /// Get recent episodes
    pub fn get_recent(&self, count: usize) -> Vec<&Episode> {
        let mut episodes: Vec<_> = self.episodes.iter().collect();
        episodes.sort_by(|a, b| b.time_span.end.timestamp.cmp(&a.time_span.end.timestamp));
        episodes.into_iter().take(count).collect()
    }
}

impl Default for Timeline {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineMarker {
    pub time: TimePoint,
    pub label: String,
    pub marker_type: MarkerType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MarkerType {
    Start,
    End,
    Milestone,
    Significant,
}

// ============================================================================
// Future Projection
// ============================================================================

/// Future projection engine
#[derive(Debug, Clone)]
pub struct FutureProjector {
    pub projection_horizon: i64, // seconds
    pub uncertainty_growth: f64,
}

impl FutureProjector {
    pub fn new() -> Self {
        Self {
            projection_horizon: 3600, // 1 hour default
            uncertainty_growth: 0.1,
        }
    }

    /// Project future state
    pub fn project(&self, current_time: &TimePoint, duration: i64) -> FutureProjection {
        let projected_time = TimePoint {
            timestamp: current_time.timestamp + Duration::seconds(duration),
            certainty: (1.0 - duration as f64 * self.uncertainty_growth / 60.0).max(0.1),
        };

        FutureProjection {
            current_time: *current_time,
            projected_time,
            duration,
            scenarios: Vec::new(),
        }
    }

    /// Generate possible futures
    pub fn generate_scenarios(&self, base: &str, count: usize) -> Vec<Scenario> {
        let mut scenarios = Vec::new();

        for i in 0..count {
            scenarios.push(Scenario {
                description: format!("{} - scenario {}", base, i + 1),
                probability: 1.0 / count as f64,
                certainty: 0.8 - i as f64 * 0.1,
            });
        }

        scenarios
    }
}

impl Default for FutureProjector {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FutureProjection {
    pub current_time: TimePoint,
    pub projected_time: TimePoint,
    pub duration: i64,
    pub scenarios: Vec<Scenario>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    pub description: String,
    pub probability: f64,
    pub certainty: f64,
}

// ============================================================================
// Unified Temporal System
// ============================================================================

/// Complete temporal reasoning system
#[derive(Debug, Clone)]
pub struct TemporalSystem {
    pub reasoner: TemporalReasoner,
    pub constructor: EpisodeConstructor,
    pub timeline: Timeline,
    pub projector: FutureProjector,
}

impl TemporalSystem {
    pub fn new() -> Self {
        Self {
            reasoner: TemporalReasoner::new(),
            constructor: EpisodeConstructor::new(),
            timeline: Timeline::new(),
            projector: FutureProjector::new(),
        }
    }

    /// Process temporal experience
    pub fn process(&mut self, event: TemporalEvent) {
        self.constructor.add_event(event.clone());
        self.timeline.add_episode(Episode {
            id: TemporalId::new(),
            events: vec![event],
            time_span: TimeInterval::new(event.time, event.time),
            coherence: 1.0,
            narrative: String::new(),
        });
    }

    /// Query timeline
    pub fn query(&self, time: &TimePoint) -> Option<&Episode> {
        self.timeline.get_episode_at(time)
    }

    /// Project into future
    pub fn project(&self, duration_seconds: i64) -> FutureProjection {
        self.projector.project(&TimePoint::now(), duration_seconds)
    }
}

impl Default for TemporalSystem {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod temporal_reasoner_tests {
    use super::*;

    #[test]
    fn test_now() {
        let reasoner = TemporalReasoner::new();
        let now = reasoner.now();

        assert!(now.certainty > 0.0);
    }

    #[test]
    fn test_distance() {
        let reasoner = TemporalReasoner::new();
        let time1 = TimePoint::now();
        let time2 = TimePoint::seconds_ago(60);

        let distance = reasoner.distance(&time1, &time2);
        assert!(distance >= 60);
    }

    #[test]
    fn test_relate() {
        let reasoner = TemporalReasoner::new();
        let event1 = TemporalEvent {
            id: TemporalId::new(),
            description: "First".to_string(),
            time: TimePoint::seconds_ago(120),
            duration: None,
            temporal_relation: None,
        };
        let event2 = TemporalEvent {
            id: TemporalId::new(),
            description: "Second".to_string(),
            time: TimePoint::now(),
            duration: None,
            temporal_relation: None,
        };

        let relation = reasoner.relate(&event1, &event2);
        assert_eq!(relation, TemporalRelation::Before);
    }

    #[test]
    fn test_parse_temporal() {
        let reasoner = TemporalReasoner::new();

        assert!(reasoner.parse_temporal("now").is_some());
        assert!(reasoner.parse_temporal("today").is_some());
        assert!(reasoner.parse_temporal("tomorrow").is_some());
    }
}

#[cfg(test)]
mod episode_tests {
    use super::*;

    #[test]
    fn test_episode_creation() {
        let start = TimePoint::now();
        let end = TimePoint::seconds_ago(-60);
        let episode = Episode::new(start, end);

        assert!(episode.events.is_empty());
    }

    #[test]
    fn test_add_event() {
        let mut episode = Episode::new(TimePoint::now(), TimePoint::now());
        let event = TemporalEvent {
            id: TemporalId::new(),
            description: "Test event".to_string(),
            time: TimePoint::now(),
            duration: None,
            temporal_relation: None,
        };

        episode.add_event(event);
        assert_eq!(episode.events.len(), 1);
    }
}

#[cfg(test)]
mod episode_constructor_tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let mut constructor = EpisodeConstructor::new();

        let event = TemporalEvent {
            id: TemporalId::new(),
            description: "Test".to_string(),
            time: TimePoint::now(),
            duration: None,
            temporal_relation: None,
        };

        constructor.add_event(event);
        assert!(constructor.current_episode.is_some());
    }

    #[test]
    fn test_finalize() {
        let mut constructor = EpisodeConstructor::new();

        let event = TemporalEvent {
            id: TemporalId::new(),
            description: "Test".to_string(),
            time: TimePoint::now(),
            duration: None,
            temporal_relation: None,
        };

        constructor.add_event(event);
        let episode = constructor.finalize_episode();

        assert!(episode.is_some());
        assert!(constructor.current_episode.is_none());
    }
}

#[cfg(test)]
mod timeline_tests {
    use super::*;

    #[test]
    fn test_timeline() {
        let mut timeline = Timeline::new();
        let episode = Episode::new(TimePoint::now(), TimePoint::now());

        timeline.add_episode(episode);
        assert_eq!(timeline.episodes.len(), 1);
    }

    #[test]
    fn test_get_recent() {
        let mut timeline = Timeline::new();

        for i in 0..5 {
            let time = TimePoint::seconds_ago(-i * 100);
            let episode = Episode::new(time, time);
            timeline.add_episode(episode);
        }

        let recent = timeline.get_recent(3);
        assert_eq!(recent.len(), 3);
    }
}

#[cfg(test)]
mod future_projector_tests {
    use super::*;

    #[test]
    fn test_project() {
        let projector = FutureProjector::new();
        let now = TimePoint::now();

        let projection = projector.project(&now, 60);
        assert!(projection.projected_time.timestamp > now.timestamp);
    }

    #[test]
    fn test_generate_scenarios() {
        let projector = FutureProjector::new();
        let scenarios = projector.generate_scenarios("Future event", 3);

        assert_eq!(scenarios.len(), 3);
    }
}

#[cfg(test)]
mod temporal_system_tests {
    use super::*;

    #[test]
    fn test_process() {
        let mut system = TemporalSystem::new();

        let event = TemporalEvent {
            id: TemporalId::new(),
            description: "Process test".to_string(),
            time: TimePoint::now(),
            duration: None,
            temporal_relation: None,
        };

        system.process(event);
        assert!(!system.timeline.episodes.is_empty());
    }

    #[test]
    fn test_project() {
        let system = TemporalSystem::new();
        let projection = system.project(60);

        assert!(projection.duration == 60);
    }
}
