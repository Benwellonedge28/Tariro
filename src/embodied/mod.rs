//! Tariro Embodied Cognition Module
//!
//! Implements simulated embodied cognition:
//! - Spatial reasoning
//! - Tool use knowledge
//! - Environmental interaction
//! - Body schema

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use chrono::Utc;

// ============================================================================
// Core Types
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmbodiedId(pub Uuid);

impl EmbodiedId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EmbodiedId {
    fn default() -> Self {
        Self::new()
    }
}

/// Spatial position
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn distance(&self, other: &Position) -> f64 {
        ((self.x - other.x).powi(2)
            + (self.y - other.y).powi(2)
            + (self.z - other.z).powi(2))
        .sqrt()
    }

    pub fn add(&self, other: &Position) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Spatial orientation
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Orientation {
    pub yaw: f64,   // Rotation around vertical axis (degrees)
    pub pitch: f64, // Rotation around horizontal axis
    pub roll: f64,  // Rotation around forward axis
}

impl Orientation {
    pub fn forward(&self) -> Position {
        let yaw_rad = self.yaw.to_radians();
        Position {
            x: yaw_rad.sin(),
            y: 0.0,
            z: yaw_rad.cos(),
        }
    }
}

/// Body part representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyPart {
    pub id: String,
    pub name: String,
    pub position: Position,
    pub orientation: Orientation,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub capabilities: Vec<Capability>,
}

impl BodyPart {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            position: Position::origin(),
            orientation: Orientation { yaw: 0.0, pitch: 0.0, roll: 0.0 },
            parent: None,
            children: Vec::new(),
            capabilities: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub description: String,
    pub active: bool,
}

// ============================================================================
// Body Schema
// ============================================================================

/// Body schema - internal representation of body
#[derive(Debug, Clone)]
pub struct BodySchema {
    pub parts: HashMap<String, BodyPart>,
    pub root: Option<String>,
    pub spatial_bounds: SpatialBounds,
}

impl BodySchema {
    pub fn new() -> Self {
        let mut schema = Self {
            parts: HashMap::new(),
            root: None,
            spatial_bounds: SpatialBounds::default(),
        };
        schema.initialize_default();
        schema
    }

    fn initialize_default(&mut self) {
        // Create default body parts
        let torso = BodyPart::new("torso");
        self.root = Some(torso.id.clone());
        self.parts.insert(torso.id.clone(), torso);

        // Add head
        let mut head = BodyPart::new("head");
        head.parent = self.root.clone();
        self.parts.insert(head.id.clone(), head);

        // Add arms
        let mut left_arm = BodyPart::new("left_arm");
        left_arm.position = Position::new(-1.0, 0.0, 0.0);
        if let Some(ref root) = self.root {
            left_arm.parent = Some(root.clone());
        }
        self.parts.insert(left_arm.id.clone(), left_arm);

        let mut right_arm = BodyPart::new("right_arm");
        right_arm.position = Position::new(1.0, 0.0, 0.0);
        if let Some(ref root) = self.root {
            right_arm.parent = Some(root.clone());
        }
        self.parts.insert(right_arm.id.clone(), right_arm);

        // Add legs
        let mut left_leg = BodyPart::new("left_leg");
        left_leg.position = Position::new(-0.3, -1.0, 0.0);
        if let Some(ref root) = self.root {
            left_leg.parent = Some(root.clone());
        }
        self.parts.insert(left_leg.id.clone(), left_leg);

        let mut right_leg = BodyPart::new("right_leg");
        right_leg.position = Position::new(0.3, -1.0, 0.0);
        if let Some(ref root) = self.root {
            right_leg.parent = Some(root.clone());
        }
        self.parts.insert(right_leg.id.clone(), right_leg);
    }

    /// Get part by name
    pub fn get_part(&self, name: &str) -> Option<&BodyPart> {
        self.parts.values().find(|p| p.name == name)
    }

    /// Calculate reach position
    pub fn calculate_reach(&self, arm: &str, extension: f64) -> Position {
        if let Some(part) = self.get_part(arm) {
            let forward = part.orientation.forward();
            Position {
                x: part.position.x + forward.x * extension,
                y: part.position.y + forward.y * extension,
                z: part.position.z + forward.z * extension,
            }
        } else {
            Position::origin()
        }
    }

    /// Get body extent
    pub fn get_extent(&self) -> Position {
        let mut max_pos = Position::origin();

        for part in self.parts.values() {
            max_pos.x = max_pos.x.max(part.position.x.abs());
            max_pos.y = max_pos.y.max(part.position.y.abs());
            max_pos.z = max_pos.z.max(part.position.z.abs());
        }

        max_pos
    }
}

impl Default for BodySchema {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialBounds {
    pub min: Position,
    pub max: Position,
}

impl Default for SpatialBounds {
    fn default() -> Self {
        Self {
            min: Position { x: -2.0, y: -1.0, z: -2.0 },
            max: Position { x: 2.0, y: 2.0, z: 2.0 },
        }
    }
}

// ============================================================================
// Spatial Reasoning
// ============================================================================

/// Spatial reasoning engine
#[derive(Debug, Clone)]
pub struct SpatialReasoner {
    pub objects: HashMap<String, SpatialObject>,
    pub relations: Vec<SpatialRelation>,
}

impl SpatialReasoner {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            relations: Vec::new(),
        }
    }

    /// Add spatial object
    pub fn add_object(&mut self, name: &str, position: Position, size: (f64, f64, f64)) {
        self.objects.insert(
            name.to_string(),
            SpatialObject {
                name: name.to_string(),
                position,
                size,
                orientation: Orientation::default(),
            },
        );
    }

    /// Get object position
    pub fn get_position(&self, name: &str) -> Option<Position> {
        self.objects.get(name).map(|o| o.position)
    }

    /// Determine spatial relation
    pub fn get_relation(&self, obj1: &str, obj2: &str) -> Option<SpatialRelationType> {
        let pos1 = self.get_position(obj1)?;
        let pos2 = self.get_position(obj2)?;

        let dist = pos1.distance(&pos2);

        if dist < 1.0 {
            Some(SpatialRelationType::Near)
        } else if dist < 5.0 {
            Some(SpatialRelationType::Adjacent)
        } else {
            Some(SpatialRelationType::Far)
        }
    }

    /// Check if path is clear
    pub fn is_path_clear(&self, from: &Position, to: &Position) -> bool {
        // Simplified path checking
        let dist = from.distance(to);
        dist < 10.0
    }

    /// Calculate trajectory
    pub fn calculate_trajectory(&self, start: Position, end: Position, steps: usize) -> Vec<Position> {
        let mut trajectory = Vec::new();

        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            trajectory.push(Position {
                x: start.x + (end.x - start.x) * t,
                y: start.y + (end.y - start.y) * t,
                z: start.z + (end.z - start.z) * t,
            });
        }

        trajectory
    }
}

impl Default for SpatialReasoner {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialObject {
    pub name: String,
    pub position: Position,
    pub size: (f64, f64, f64),
    pub orientation: Orientation,
}

impl Default for Orientation {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpatialRelation {
    pub object1: String,
    pub object2: String,
    pub relation_type: SpatialRelationType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpatialRelationType {
    Near,
    Adjacent,
    Far,
    Above,
    Below,
    LeftOf,
    RightOf,
    Inside,
    Outside,
}

// ============================================================================
// Tool Use
// ============================================================================

/// Tool representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub id: EmbodiedId,
    pub name: String,
    pub tool_type: ToolType,
    pub capabilities: Vec<ToolCapability>,
    pub requirements: Vec<String>,
    pub state: ToolState,
}

impl Tool {
    pub fn new(name: &str, tool_type: ToolType) -> Self {
        Self {
            id: EmbodiedId::new(),
            name: name.to_string(),
            tool_type,
            capabilities: Vec::new(),
            requirements: Vec::new(),
            state: ToolState::Available,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolType {
    Reach,
    Grasp,
    Cut,
    Strike,
    Contain,
    Transport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCapability {
    pub name: String,
    pub effect: String,
    pub energy_cost: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolState {
    Available,
    InUse,
    Damaged,
    Unavailable,
}

/// Tool use knowledge base
#[derive(Debug, Clone)]
pub struct ToolKnowledge {
    pub tools: HashMap<String, Tool>,
    pub tool_affordances: HashMap<String, Vec<String>>,
}

impl ToolKnowledge {
    pub fn new() -> Self {
        let mut kb = Self {
            tools: HashMap::new(),
            tool_affordances: HashMap::new(),
        };
        kb.initialize_defaults();
        kb
    }

    fn initialize_defaults(&mut self) {
        // Hammer
        let mut hammer = Tool::new("hammer", ToolType::Strike);
        hammer.capabilities.push(ToolCapability {
            name: "drive_nails".to_string(),
            effect: "Apply force to drive nails".to_string(),
            energy_cost: 0.3,
        });
        hammer.capabilities.push(ToolCapability {
            name: "dismantle".to_string(),
            effect: "Apply force to separate objects".to_string(),
            energy_cost: 0.4,
        });
        hammer.requirements.push("grip_strength".to_string());
        self.tools.insert("hammer".to_string(), hammer);

        // Screwdriver
        let mut screwdriver = Tool::new("screwdriver", ToolType::Turn);
        screwdriver.capabilities.push(ToolCapability {
            name: "tighten_screw".to_string(),
            effect: "Rotate clockwise to tighten".to_string(),
            energy_cost: 0.2,
        });
        self.tools.insert("screwdriver".to_string(), screwdriver);

        // Container (cup)
        let mut cup = Tool::new("cup", ToolType::Contain);
        cup.capabilities.push(ToolCapability {
            name: "hold_liquid".to_string(),
            effect: "Contain liquid without leaking".to_string(),
            energy_cost: 0.0,
        });
        cup.capabilities.push(ToolCapability {
            name: "transport_liquid".to_string(),
            effect: "Move liquid between locations".to_string(),
            energy_cost: 0.1,
        });
        self.tools.insert("cup".to_string(), cup);
    }

    /// Find tool for task
    pub fn find_tool(&self, task: &str) -> Option<String> {
        // Simple keyword matching
        for (name, tool) in &self.tools {
            if name.contains(task) || tool.capabilities.iter().any(|c| c.name.contains(task)) {
                return Some(name.clone());
            }
        }
        None
    }

    /// Get tool affordances
    pub fn get_affordances(&self, tool_name: &str) -> Vec<&ToolCapability> {
        self.tools
            .get(tool_name)
            .map(|t| t.capabilities.iter().collect())
            .unwrap_or_default()
    }
}

impl Default for ToolKnowledge {
    fn default() -> Self {
        Self::new()
    }
}

impl Tool {
    pub fn new_tool_type(name: &str, tool_type: ToolType) -> Self {
        Self::new(name, tool_type)
    }
}

// ============================================================================
// Environmental Interaction
// ============================================================================

/// Environment representation
#[derive(Debug, Clone)]
pub struct Environment {
    pub objects: HashMap<String, EnvironmentObject>,
    pub agent_position: Position,
    pub agent_orientation: Orientation,
    pub boundaries: SpatialBounds,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
            agent_position: Position::origin(),
            agent_orientation: Orientation::default(),
            boundaries: SpatialBounds::default(),
        }
    }

    /// Add object to environment
    pub fn add_object(&mut self, name: &str, position: Position, object_type: EnvironmentObjectType) {
        self.objects.insert(
            name.to_string(),
            EnvironmentObject {
                name: name.to_string(),
                position,
                object_type,
                properties: HashMap::new(),
            },
        );
    }

    /// Move agent
    pub fn move_agent(&mut self, delta: Position) {
        self.agent_position = self.agent_position.add(&delta);
    }

    /// Rotate agent
    pub fn rotate_agent(&mut self, yaw_delta: f64) {
        self.agent_orientation.yaw += yaw_delta;
    }

    /// Check if position is valid
    pub fn is_valid_position(&self, pos: &Position) -> bool {
        pos.x >= self.boundaries.min.x
            && pos.x <= self.boundaries.max.x
            && pos.y >= self.boundaries.min.y
            && pos.y <= self.boundaries.max.y
            && pos.z >= self.boundaries.min.z
            && pos.z <= self.boundaries.max.z
    }

    /// Get nearby objects
    pub fn get_nearby(&self, position: &Position, radius: f64) -> Vec<&EnvironmentObject> {
        self.objects
            .values()
            .filter(|obj| position.distance(&obj.position) <= radius)
            .collect()
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentObject {
    pub name: String,
    pub position: Position,
    pub object_type: EnvironmentObjectType,
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EnvironmentObjectType {
    Obstacle,
    Target,
    Tool,
    Resource,
    Landmark,
}

// ============================================================================
// Unified Embodied Cognition
// ============================================================================

/// Complete embodied cognition system
#[derive(Debug, Clone)]
pub struct EmbodiedCognition {
    pub body_schema: BodySchema,
    pub spatial: SpatialReasoner,
    pub tools: ToolKnowledge,
    pub environment: Environment,
}

impl EmbodiedCognition {
    pub fn new() -> Self {
        Self {
            body_schema: BodySchema::new(),
            spatial: SpatialReasoner::new(),
            tools: ToolKnowledge::new(),
            environment: Environment::new(),
        }
    }

    /// Plan movement to target
    pub fn plan_movement(&mut self, target: Position) -> MovementPlan {
        let trajectory = self.spatial.calculate_trajectory(
            self.environment.agent_position,
            target,
            10,
        );

        MovementPlan {
            start: self.environment.agent_position,
            end: target,
            trajectory,
            energy_cost: self.estimate_energy(&trajectory),
        }
    }

    /// Plan tool use
    pub fn plan_tool_use(&self, task: &str) -> Option<ToolUsePlan> {
        let tool_name = self.tools.find_tool(task)?;

        Some(ToolUsePlan {
            tool_name: tool_name.clone(),
            tool: self.tools.tools.get(&tool_name).cloned(),
            steps: vec![format!("Reach for {}", tool_name)],
            energy_cost: 0.2,
        })
    }

    fn estimate_energy(&self, trajectory: &[Position]) -> f64 {
        let mut energy = 0.0;
        for i in 1..trajectory.len() {
            energy += trajectory[i].distance(&trajectory[i - 1]);
        }
        energy * 0.1 // Energy per unit distance
    }

    /// Get embodiment report
    pub fn get_embodiment_report(&self) -> String {
        format!(
            "Embodiment Report:\n\
            - Body parts: {}\n\
            - Spatial objects: {}\n\
            - Tools known: {}\n\
            - Agent position: ({:.1}, {:.1}, {:.1})\n\
            - Agent orientation: yaw={:.1}°",
            self.body_schema.parts.len(),
            self.spatial.objects.len(),
            self.tools.tools.len(),
            self.environment.agent_position.x,
            self.environment.agent_position.y,
            self.environment.agent_position.z,
            self.environment.agent_orientation.yaw
        )
    }
}

impl Default for EmbodiedCognition {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementPlan {
    pub start: Position,
    pub end: Position,
    pub trajectory: Vec<Position>,
    pub energy_cost: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUsePlan {
    pub tool_name: String,
    pub tool: Option<Tool>,
    pub steps: Vec<String>,
    pub energy_cost: f64,
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_distance() {
        let p1 = Position::new(0.0, 0.0, 0.0);
        let p2 = Position::new(3.0, 4.0, 0.0);

        assert!((p1.distance(&p2) - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_add() {
        let p1 = Position::new(1.0, 2.0, 3.0);
        let p2 = Position::new(4.0, 5.0, 6.0);

        let result = p1.add(&p2);
        assert_eq!(result.x, 5.0);
        assert_eq!(result.y, 7.0);
        assert_eq!(result.z, 9.0);
    }
}

#[cfg(test)]
mod body_schema_tests {
    use super::*;

    #[test]
    fn test_get_part() {
        let schema = BodySchema::new();

        let head = schema.get_part("head");
        assert!(head.is_some());
        assert_eq!(head.unwrap().name, "head");
    }

    #[test]
    fn test_calculate_reach() {
        let schema = BodySchema::new();

        let reach = schema.calculate_reach("left_arm", 1.0);
        assert!(reach.x < 0.0); // Should extend left
    }
}

#[cfg(test)]
mod spatial_reasoner_tests {
    use super::*;

    #[test]
    fn test_add_object() {
        let mut reasoner = SpatialReasoner::new();
        reasoner.add_object("table", Position::new(2.0, 0.0, 3.0), (1.0, 1.0, 1.0));

        assert!(reasoner.get_position("table").is_some());
    }

    #[test]
    fn test_get_relation() {
        let mut reasoner = SpatialReasoner::new();
        reasoner.add_object("A", Position::new(0.0, 0.0, 0.0), (1.0, 1.0, 1.0));
        reasoner.add_object("B", Position::new(0.5, 0.0, 0.0), (1.0, 1.0, 1.0));

        let relation = reasoner.get_relation("A", "B");
        assert_eq!(relation, Some(SpatialRelationType::Near));
    }

    #[test]
    fn test_trajectory() {
        let reasoner = SpatialReasoner::new();
        let trajectory = reasoner.calculate_trajectory(
            Position::new(0.0, 0.0, 0.0),
            Position::new(10.0, 0.0, 0.0),
            5,
        );

        assert_eq!(trajectory.len(), 6);
    }
}

#[cfg(test)]
mod tool_knowledge_tests {
    use super::*;

    #[test]
    fn test_find_tool() {
        let kb = ToolKnowledge::new();

        let hammer = kb.find_tool("drive");
        assert!(hammer.is_some());
        assert_eq!(hammer.unwrap(), "hammer");
    }

    #[test]
    fn test_get_affordances() {
        let kb = ToolKnowledge::new();

        let capabilities = kb.get_affordances("hammer");
        assert!(!capabilities.is_empty());
    }
}

#[cfg(test)]
mod environment_tests {
    use super::*;

    #[test]
    fn test_add_object() {
        let mut env = Environment::new();
        env.add_object(
            "box",
            Position::new(1.0, 0.0, 2.0),
            EnvironmentObjectType::Obstacle,
        );

        assert!(env.objects.contains_key("box"));
    }

    #[test]
    fn test_move_agent() {
        let mut env = Environment::new();
        let initial = env.agent_position;

        env.move_agent(Position::new(1.0, 0.0, 0.0));

        assert!(env.agent_position.x > initial.x);
    }

    #[test]
    fn test_get_nearby() {
        let mut env = Environment::new();
        env.add_object(
            "near",
            Position::new(1.0, 0.0, 0.0),
            EnvironmentObjectType::Target,
        );
        env.add_object(
            "far",
            Position::new(10.0, 0.0, 0.0),
            EnvironmentObjectType::Target,
        );

        let nearby = env.get_nearby(&Position::origin(), 5.0);
        assert_eq!(nearby.len(), 1);
    }
}

#[cfg(test)]
mod embodied_cognition_tests {
    use super::*;

    #[test]
    fn test_plan_movement() {
        let mut embodied = EmbodiedCognition::new();
        let target = Position::new(5.0, 0.0, 0.0);

        let plan = embodied.plan_movement(target);

        assert!(!plan.trajectory.is_empty());
        assert!(plan.energy_cost > 0.0);
    }

    #[test]
    fn test_plan_tool_use() {
        let embodied = EmbodiedCognition::new();

        let plan = embodied.plan_tool_use("drive");
        assert!(plan.is_some());
    }

    #[test]
    fn test_embodiment_report() {
        let embodied = EmbodiedCognition::new();
        let report = embodied.get_embodiment_report();

        assert!(report.contains("Embodiment Report"));
    }
}
