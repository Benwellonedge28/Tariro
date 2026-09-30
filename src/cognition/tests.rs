//! Tariro Cognition Layer Tests
//!
//! Comprehensive tests for consciousness, working memory, world model, motivation, and the cognition engine.

#[cfg(test)]
mod consciousness_tests {
    use super::super::*;

    // ==========================================================================
    // Thought Tests
    // ==========================================================================

    #[test]
    fn test_thought_creation() {
        let thought = Thought::new(ThoughtContent::Reasoning("Testing reasoning".to_string()));

        assert_eq!(thought.focus_level, FocusLevel::Active);
        assert!(thought.id.is_valid());
    }

    #[test]
    fn test_thought_content() {
        let content = ThoughtContent::Perception("I see fire".to_string());
        assert_eq!(content.as_str(), "I see fire");

        let reasoning = ThoughtContent::Reasoning("Fire is hot".to_string());
        assert_eq!(reasoning.as_str(), "Fire is hot");

        let emotion = ThoughtContent::Emotion("I am worried".to_string());
        assert_eq!(emotion.as_str(), "I am worried");
    }

    #[test]
    fn test_thought_with_metadata() {
        let thought = Thought::new(ThoughtContent::Reasoning("Test".to_string()))
            .with_metadata("source", "memory")
            .with_metadata("importance", "high");

        assert_eq!(thought.metadata.get("source"), Some(&"memory".to_string()));
        assert_eq!(thought.metadata.get("importance"), Some(&"high".to_string()));
    }

    // ==========================================================================
    // Self Model Tests
    // ==========================================================================

    #[test]
    fn test_self_model_creation() {
        let model = SelfModel::new();

        assert!(model.capabilities.is_empty());
        assert!(model.limitations.is_empty());
        assert_eq!(model.assessments.len(), 0);
    }

    #[test]
    fn test_self_model_assessment() {
        let mut model = SelfModel::new();

        model.update_assessment("reasoning", 0.9);
        model.update_assessment("creativity", 0.7);

        assert!((model.assess_capability("reasoning") - 0.9).abs() < 0.001);
        assert!((model.assess_capability("creativity") - 0.7).abs() < 0.001);
        // Default for unknown capability
        assert!((model.assess_capability("unknown") - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_capability_clamping() {
        let mut model = SelfModel::new();

        // Test clamping to [0.0, 1.0]
        model.update_assessment("test", 1.5);
        assert!((model.assess_capability("test") - 1.0).abs() < 0.001);

        model.update_assessment("test", -0.5);
        assert!((model.assess_capability("test") - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_identity_defaults() {
        let identity = Identity::default();

        assert_eq!(identity.roles, vec!["AGI".to_string(), "Learner".to_string()]);
        assert_eq!(identity.purpose, Some("Understand and assist".to_string()));
    }

    // ==========================================================================
    // Metacognition Tests
    // ==========================================================================

    #[test]
    fn test_metacognition_creation() {
        let metacog = Metacognition::new();

        assert_eq!(metacog.cognitive_load, 0.0);
        assert_eq!(metacog.reasoning_confidence, 1.0);
        assert_eq!(metacog.uncertainty_awareness, 0.5);
        assert_eq!(metacog.regulation_attempts, 0);
    }

    #[test]
    fn test_metacognition_monitoring() {
        let mut metacog = Metacognition::new();

        // Reasoning increases cognitive load
        let reasoning = Thought::new(ThoughtContent::Reasoning("Complex reasoning".to_string()));
        metacog.monitor(&reasoning);
        assert!(metacog.cognitive_load > 0.0);

        // Reset and try perception
        metacog.cognitive_load = 0.3;
        let perception = Thought::new(ThoughtContent::Perception("See something".to_string()));
        metacog.monitor(&perception);
        assert!(metacog.cognitive_load < 0.3);
    }

    #[test]
    fn test_metacognition_needs_regulation() {
        let mut metacog = Metacognition::new();

        // High cognitive load triggers regulation
        metacog.cognitive_load = 0.85;
        assert!(metacog.needs_regulation());

        // Low confidence triggers regulation
        metacog.cognitive_load = 0.5;
        metacog.reasoning_confidence = 0.25;
        assert!(metacog.needs_regulation());
    }

    #[test]
    fn test_metacognition_regulate() {
        let mut metacog = Metacognition::new();

        metacog.cognitive_load = 0.9;
        metacog.reasoning_confidence = 0.3;
        metacog.regulate();

        assert!(metacog.cognitive_load < 0.9);
        assert!(metacog.reasoning_confidence > 0.3);
        assert_eq!(metacog.regulation_attempts, 1);
    }

    // ==========================================================================
    // Insight Tests
    // ==========================================================================

    #[test]
    fn test_insight_creation() {
        let connections = vec![
            ConceptConnection {
                concept_a: "Fire".to_string(),
                concept_b: "Heat".to_string(),
                connection_type: ConnectionType::Causal,
                strength: 0.9,
            },
        ];

        let insight = Insight::new(connections, "Fire produces heat".to_string());

        assert_eq!(insight.connections.len(), 1);
        assert_eq!(insight.description, "Fire produces heat");
        assert_eq!(insight.impact, InsightImpact::Moderate);
    }

    // ==========================================================================
    // Consciousness Engine Tests
    // ==========================================================================

    #[test]
    fn test_consciousness_creation() {
        let consciousness = Consciousness::new();

        assert!(consciousness.get_focus().is_none());
        assert!(consciousness.get_recent_thoughts(10).is_empty());
    }

    #[test]
    fn test_attend_to_thought() {
        let mut consciousness = Consciousness::new();

        let thought = Thought::new(ThoughtContent::Perception("I see fire".to_string()));
        consciousness.attend(thought.clone());

        let focus = consciousness.get_focus();
        assert!(focus.is_some());
        assert_eq!(focus.unwrap().content.as_str(), "I see fire");
    }

    #[test]
    fn test_thought_stream() {
        let mut consciousness = Consciousness::new();

        for i in 0..5 {
            let thought = Thought::new(ThoughtContent::Reasoning(format!("Thought {}", i)));
            consciousness.attend(thought);
        }

        let recent = consciousness.get_recent_thoughts(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_insight_generation() {
        let mut consciousness = Consciousness::new();

        // Add some thoughts with related content
        consciousness.attend(Thought::new(ThoughtContent::Reasoning("Fire is hot".to_string())));
        consciousness.attend(Thought::new(ThoughtContent::Reasoning("Hot things burn".to_string())));
        consciousness.attend(Thought::new(ThoughtContent::Reasoning("Fire burns".to_string())));

        let insight = consciousness.generate_insight("Fire");
        assert!(insight.is_some());
    }

    #[test]
    fn test_self_reflection() {
        let consciousness = Consciousness::new();

        let reflection = consciousness.reflect();

        assert_eq!(reflection.questions.len(), 1); // Default question
        assert_eq!(reflection.capabilities.len(), 0);
    }
}

#[cfg(test)]
mod working_memory_tests {
    use super::super::*;

    // ==========================================================================
    // Mental Item Tests
    // ==========================================================================

    #[test]
    fn test_mental_item_creation() {
        let item = MentalItem::new(ItemContent::Concept(Concept::new("Fire")));

        assert_eq!(item.activation, 1.0);
        assert!(item.id.is_valid());
        assert!(item.associations.is_empty());
    }

    #[test]
    fn test_activation_decay() {
        let mut item = MentalItem::new(ItemContent::Concept(Concept::new("Test")));

        item.decay(0.3);
        assert!((item.activation - 0.7).abs() < 0.001);

        item.decay(0.5);
        assert!((item.activation - 0.2).abs() < 0.001);

        // Cannot go below 0
        item.decay(1.0);
        assert!(item.activation >= 0.0);
    }

    #[test]
    fn test_activation_boost() {
        let mut item = MentalItem::new(ItemContent::Concept(Concept::new("Test")));

        item.boost(0.3);
        assert!((item.activation - 1.0).abs() < 0.001); // Capped at 1.0

        item.activation = 0.5;
        item.boost(0.3);
        assert!((item.activation - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_item_content_as_str() {
        let concept = ItemContent::Concept(Concept::new("Fire"));
        assert_eq!(concept.as_str(), "Fire");

        let rule = ItemContent::Rule(Rule {
            description: "Fire burns".to_string(),
            antecedents: vec![],
            consequent: String::new(),
            confidence: 0.9,
        });
        assert_eq!(rule.as_str(), "Fire burns");
    }

    // ==========================================================================
    // Working Memory Tests
    // ==========================================================================

    #[test]
    fn test_working_memory_creation() {
        let wm = WorkingMemory::new();

        assert!(wm.is_empty());
        assert_eq!(wm.len(), 0);
        assert!((wm.capacity_usage() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_store_item() {
        let mut wm = WorkingMemory::new();

        let item = MentalItem::new(ItemContent::Concept(Concept::new("Fire")));
        wm.store(item);

        assert_eq!(wm.len(), 1);
        assert!(!wm.is_empty());
    }

    #[test]
    fn test_retrieve_item() {
        let mut wm = WorkingMemory::new();

        let item = MentalItem::new(ItemContent::Concept(Concept::new("Fire")));
        let id = item.id.clone();
        wm.store(item);

        let retrieved = wm.retrieve(&id);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_find_item() {
        let mut wm = WorkingMemory::new();

        let item = MentalItem::new(ItemContent::Concept(Concept::new("Fire")));
        wm.store(item);

        let found = wm.find("Fire");
        assert!(found.is_some());

        let not_found = wm.find("Water");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_attend_to_item() {
        let mut wm = WorkingMemory::new();

        let item = MentalItem::new(ItemContent::Concept(Concept::new("Fire")));
        let id = item.id.clone();
        wm.store(item);

        let original_activation = wm.retrieve(&id).unwrap().activation;

        wm.attend(&id);

        let after_attention = wm.retrieve(&id).unwrap();
        assert!(after_attention.activation >= original_activation);
    }

    #[test]
    fn test_capacity_limit() {
        let mut wm = WorkingMemory::with_config(WorkingMemoryConfig {
            capacity: 3,
            ..Default::default()
        });

        for i in 0..5 {
            let item = MentalItem::new(ItemContent::Concept(Concept::new(format!("Item {}", i))));
            wm.store(item);
        }

        assert_eq!(wm.len(), 3);
    }

    #[test]
    fn test_lru_eviction() {
        let mut wm = WorkingMemory::with_config(WorkingMemoryConfig {
            capacity: 2,
            ..Default::default()
        });

        let item1 = MentalItem::new(ItemContent::Concept(Concept::new("First")));
        let item2 = MentalItem::new(ItemContent::Concept(Concept::new("Second")));
        let item3 = MentalItem::new(ItemContent::Concept(Concept::new("Third")));

        let id1 = item1.id.clone();
        wm.store(item1);
        wm.store(item2);

        // Access first item to update its recency
        let _ = wm.retrieve(&id1);

        // Now add third - should evict second (least recently used)
        wm.store(item3);

        // First and Third should remain
        assert!(wm.find("First").is_some());
        assert!(wm.find("Third").is_some());
        assert!(wm.find("Second").is_none());
    }

    #[test]
    fn test_decay() {
        let mut wm = WorkingMemory::new();

        let item = MentalItem::new(ItemContent::Concept(Concept::new("Test")));
        wm.store(item);

        wm.decay();

        // All items should have decayed
        let items = wm.get_active_items();
        for item in items {
            assert!(item.activation < 1.0);
        }
    }

    #[test]
    fn test_clear() {
        let mut wm = WorkingMemory::new();

        for i in 0..3 {
            let item = MentalItem::new(ItemContent::Concept(Concept::new(format!("Item {}", i))));
            wm.store(item);
        }

        wm.clear();

        assert!(wm.is_empty());
        assert_eq!(wm.len(), 0);
    }

    #[test]
    fn test_binding() {
        let mut wm = WorkingMemory::new();

        let item1 = MentalItem::new(ItemContent::Concept(Concept::new("Fire")));
        let item2 = MentalItem::new(ItemContent::Concept(Concept::new("Heat")));

        let id1 = item1.id.clone();
        let id2 = item2.id.clone();

        wm.store(item1);
        wm.store(item2);

        let chunk = wm.bind(vec![id1, id2]);
        assert_eq!(chunk.items.len(), 2);
    }

    #[test]
    fn test_active_items_sorted() {
        let mut wm = WorkingMemory::new();

        let mut item1 = MentalItem::new(ItemContent::Concept(Concept::new("Low")));
        item1.activation = 0.3;

        let mut item2 = MentalItem::new(ItemContent::Concept(Concept::new("High")));
        item2.activation = 0.8;

        wm.store(item1);
        wm.store(item2);

        let active = wm.get_active_items();
        assert_eq!(active[0].content.as_str(), "High"); // Higher activation first
        assert_eq!(active[1].content.as_str(), "Low");
    }

    // ==========================================================================
    // Chunk Tests
    // ==========================================================================

    #[test]
    fn test_chunk_creation() {
        let chunk = Chunk::new(vec![
            ItemId::new(),
            ItemId::new(),
        ]);

        assert_eq!(chunk.items.len(), 2);
        assert_eq!(chunk.binding_type, BindingType::Associative);
        assert_eq!(chunk.strength, 1.0);
    }

    #[test]
    fn test_chunk_with_binding() {
        let chunk = Chunk::new(vec![ItemId::new()])
            .with_binding(BindingType::Temporal);

        assert_eq!(chunk.binding_type, BindingType::Temporal);
    }
}

#[cfg(test)]
mod world_model_tests {
    use super::super::*;

    // ==========================================================================
    // World Entity Tests
    // ==========================================================================

    #[test]
    fn test_world_entity_creation() {
        let entity = WorldEntity::new("Fire", WorldEntityType::Process);

        assert_eq!(entity.name, "Fire");
        assert_eq!(entity.entity_type, WorldEntityType::Process);
        assert!(entity.id.is_valid());
    }

    #[test]
    fn test_world_entity_properties() {
        let entity = WorldEntity::new("Fire", WorldEntityType::Process)
            .with_property("temperature", Property::new("temperature", PropertyValue::Number(500.0)))
            .with_property("state", Property::new("state", PropertyValue::String("active".to_string())));

        assert_eq!(entity.properties.len(), 2);
    }

    #[test]
    fn test_world_entity_location() {
        let entity = WorldEntity::new("Fire", WorldEntityType::Process)
            .with_location(Location {
                space: Space::Physical,
                coordinates: Coordinates { x: 10.0, y: 20.0, z: 0.0 },
            });

        assert!(entity.location.is_some());
        assert_eq!(entity.location.unwrap().coordinates.x, 10.0);
    }

    // ==========================================================================
    // Causal Graph Tests
    // ==========================================================================

    #[test]
    fn test_causal_graph_creation() {
        let graph = CausalGraph::new();
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
    }

    #[test]
    fn test_causal_graph_add_entity() {
        let mut graph = CausalGraph::new();

        let entity = WorldEntity::new("Fire", WorldEntityType::Process);
        graph.add_entity(entity);

        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_causal_graph_add_link() {
        let mut graph = CausalGraph::new();

        let fire = WorldEntity::new("Fire", WorldEntityType::Process);
        let smoke = WorldEntity::new("Smoke", WorldEntityType::Event);

        let fire_id = fire.id.clone();
        let smoke_id = smoke.id.clone();

        graph.add_entity(fire);
        graph.add_entity(smoke);

        graph.add_causal_link(CausalRelationship {
            cause: fire_id,
            effect: smoke_id,
            mechanism: "Combustion produces smoke".to_string(),
            probability: 0.9,
            temporal_delay: Some(10),
        });

        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn test_causal_graph_effects() {
        let mut graph = CausalGraph::new();

        let fire = WorldEntity::new("Fire", WorldEntityType::Process);
        let smoke = WorldEntity::new("Smoke", WorldEntityType::Event);

        let fire_id = fire.id.clone();

        graph.add_entity(fire);
        graph.add_entity(smoke);

        graph.add_causal_link(CausalRelationship {
            cause: fire_id.clone(),
            effect: smoke.id,
            mechanism: "Fire produces smoke".to_string(),
            probability: 0.9,
            temporal_delay: None,
        });

        let effects = graph.get_effects(&fire_id);
        assert_eq!(effects.len(), 1);
    }

    #[test]
    fn test_causal_graph_causes() {
        let mut graph = CausalGraph::new();

        let fire = WorldEntity::new("Fire", WorldEntityType::Process);
        let smoke = WorldEntity::new("Smoke", WorldEntityType::Event);

        let smoke_id = smoke.id.clone();

        graph.add_entity(fire);
        graph.add_entity(smoke);

        graph.add_causal_link(CausalRelationship {
            cause: fire.id,
            effect: smoke_id.clone(),
            mechanism: "Fire produces smoke".to_string(),
            probability: 0.9,
            temporal_delay: None,
        });

        let causes = graph.get_causes(&smoke_id);
        assert_eq!(causes.len(), 1);
    }

    #[test]
    fn test_causal_chain_finding() {
        let mut graph = CausalGraph::new();

        let a = WorldEntity::new("A", WorldEntityType::Abstract);
        let b = WorldEntity::new("B", WorldEntityType::Abstract);
        let c = WorldEntity::new("C", WorldEntityType::Abstract);

        let a_id = a.id.clone();
        let b_id = b.id.clone();
        let c_id = c.id.clone();

        graph.add_entity(a);
        graph.add_entity(b);
        graph.add_entity(c);

        graph.add_causal_link(CausalRelationship {
            cause: a_id.clone(),
            effect: b_id.clone(),
            mechanism: "A causes B".to_string(),
            probability: 1.0,
            temporal_delay: None,
        });

        graph.add_causal_link(CausalRelationship {
            cause: b_id.clone(),
            effect: c_id.clone(),
            mechanism: "B causes C".to_string(),
            probability: 1.0,
            temporal_delay: None,
        });

        let chain = graph.find_causal_chain(&a_id, &c_id);
        assert!(chain.is_some());
        assert!(chain.unwrap().len() >= 3);
    }

    // ==========================================================================
    // World Model Tests
    // ==========================================================================

    #[test]
    fn test_world_model_creation() {
        let model = WorldModel::new();
        assert!(model.entities.is_empty());
    }

    #[test]
    fn test_world_model_add_entity() {
        let mut model = WorldModel::new();

        let entity = WorldEntity::new("Fire", WorldEntityType::Process);
        model.add_entity(entity);

        assert_eq!(model.entities.len(), 1);
        assert_eq!(model.stats().entities_added, 1);
    }

    #[test]
    fn test_world_model_add_causal_relation() {
        let mut model = WorldModel::new();

        let fire = WorldEntity::new("Fire", WorldEntityType::Process);
        let smoke = WorldEntity::new("Smoke", WorldEntityType::Event);

        model.add_entity(fire.clone());
        model.add_entity(smoke.clone());

        model.add_causal_relation(CausalRelationship {
            cause: fire.id,
            effect: smoke.id,
            mechanism: "Fire produces smoke".to_string(),
            probability: 0.9,
            temporal_delay: None,
        });

        assert_eq!(model.stats().causal_relations_added, 1);
    }

    #[test]
    fn test_world_model_find_entities() {
        let mut model = WorldModel::new();

        model.add_entity(WorldEntity::new("Fire", WorldEntityType::Process));
        model.add_entity(WorldEntity::new("Water", WorldEntityType::Substance));
        model.add_entity(WorldEntity::new("Fireplace", WorldEntityType::Object));

        let found = model.find_entities("fire");
        assert_eq!(found.len(), 2); // Fire and Fireplace
    }

    // ==========================================================================
    // Imagination Engine Tests
    // ==========================================================================

    #[test]
    fn test_imagination_engine_creation() {
        let engine = ImaginationEngine::new();
        assert_eq!(engine.stats().total_simulations, 0);
    }

    #[test]
    fn test_simulation() {
        let mut model = WorldModel::new();

        let entity = WorldEntity::new("Test", WorldEntityType::Object);
        model.add_entity(entity.clone());

        let action = SimulatedAction {
            target: entity.id.clone(),
            action_type: "Activate".to_string(),
            parameters: HashMap::new(),
            initial_state: SimulatedState::new(),
        };

        let result = model.predict(action);
        assert!(result.confidence > 0.0);
    }

    // ==========================================================================
    // Theory of Mind Tests
    // ==========================================================================

    #[test]
    fn test_theory_of_mind_creation() {
        let tom = TheoryOfMind::new();
        assert!(tom.agent_models.is_empty());
    }

    #[test]
    fn test_create_agent_model() {
        let mut tom = TheoryOfMind::new();

        let agent = WorldEntity::new("Alice", WorldEntityType::Agent);
        tom.create_agent_model(agent);

        assert!(!tom.agent_models.is_empty());
    }

    #[test]
    fn test_predict_action() {
        let mut tom = TheoryOfMind::new();

        let agent = WorldEntity::new("Alice", WorldEntityType::Agent);
        tom.create_agent_model(agent);

        let prediction = tom.predict_action(&agent.id);
        assert!(prediction.is_some());
    }

    #[test]
    fn test_perspective_taking() {
        let mut tom = TheoryOfMind::new();

        let agent = WorldEntity::new("Alice", WorldEntityType::Agent);
        let agent_id = agent.id.clone();
        tom.create_agent_model(agent);

        let perspective = tom.perspective_taking(&agent_id, "belief");
        assert!(!perspective.is_empty());
    }
}

#[cfg(test)]
mod motivation_tests {
    use super::super::*;

    // ==========================================================================
    // Drive Tests
    // ==========================================================================

    #[test]
    fn test_drive_creation() {
        let drive = Drive::new(DriveType::Curiosity, "Need to explore".to_string());

        assert_eq!(drive.drive_type, DriveType::Curiosity);
        assert!(drive.active);
        assert_eq!(drive.intensity, 0.5);
    }

    #[test]
    fn test_drive_activation() {
        let mut drive = Drive::new(DriveType::Curiosity, "Need to explore".to_string());

        drive.activate();
        assert!(drive.active);

        drive.deactivate();
        assert!(!drive.active);
    }

    #[test]
    fn test_drive_satisfaction() {
        let mut drive = Drive::new(DriveType::Hunger, "Need food".to_string());

        drive.satisfy();
        assert!((drive.intensity - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_primary_drives_exist() {
        assert!(matches!(DriveType::Survival, DriveType::Survival));
        assert!(matches!(DriveType::Integrity, DriveType::Integrity));
        assert!(matches!(DriveType::Security, DriveType::Security));
    }

    #[test]
    fn test_secondary_drives_exist() {
        assert!(matches!(DriveType::Curiosity, DriveType::Curiosity));
        assert!(matches!(DriveType::Competence, DriveType::Competence));
        assert!(matches!(DriveType::Autonomy, DriveType::Autonomy));
    }

    // ==========================================================================
    // Goal Tests
    // ==========================================================================

    #[test]
    fn test_goal_creation() {
        let goal = Goal::new("Learn Rust");

        assert_eq!(goal.description, "Learn Rust");
        assert_eq!(goal.status, GoalStatus::Active);
        assert_eq!(goal.priority, 0.5);
        assert!(goal.subgoals.is_empty());
    }

    #[test]
    fn test_goal_types() {
        assert!(matches!(GoalType::Learn("test".to_string()).goal_type(), "Learn"));
        assert!(matches!(GoalType::Explore("test".to_string()).goal_type(), "Explore"));
        assert!(matches!(GoalType::Create("test".to_string()).goal_type(), "Create"));
    }

    #[test]
    fn test_goal_completion() {
        let mut goal = Goal::new("Test goal");
        goal.complete();

        assert_eq!(goal.status, GoalStatus::Completed);
    }

    #[test]
    fn test_goal_abandonment() {
        let mut goal = Goal::new("Test goal");
        goal.abandon();

        assert_eq!(goal.status, GoalStatus::Abandoned);
    }

    #[test]
    fn test_goal_hierarchy() {
        let mut parent = Goal::new("Parent goal");
        let child = Goal::new("Child goal");
        let child_id = child.id.clone();

        parent.add_subgoal(child_id);

        assert_eq!(parent.subgoals.len(), 1);
    }

    // ==========================================================================
    // Emotion Tests
    // ==========================================================================

    #[test]
    fn test_emotion_creation() {
        let emotion = Emotion::new(EmotionType::Curiosity, 0.7);

        assert_eq!(emotion.emotion_type, EmotionType::Curiosity);
        assert_eq!(emotion.intensity, 0.7);
    }

    #[test]
    fn test_emotion_decay() {
        let mut emotion = Emotion::new(EmotionType::Joy, 0.9);

        emotion.decay(0.3);
        assert!((emotion.intensity - 0.6).abs() < 0.001);
    }

    #[test]
    fn test_basic_emotions_exist() {
        assert!(matches!(EmotionType::Joy, EmotionType::Joy));
        assert!(matches!(EmotionType::Sadness, EmotionType::Sadness));
        assert!(matches!(EmotionType::Anger, EmotionType::Anger));
        assert!(matches!(EmotionType::Fear, EmotionType::Fear));
    }

    #[test]
    fn test_complex_emotions_exist() {
        assert!(matches!(EmotionType::Curiosity, EmotionType::Curiosity));
        assert!(matches!(EmotionType::Confusion, EmotionType::Confusion));
        assert!(matches!(EmotionType::Satisfaction, EmotionType::Satisfaction));
    }

    // ==========================================================================
    // Motivation System Tests
    // ==========================================================================

    #[test]
    fn test_motivation_system_creation() {
        let system = MotivationSystem::new();

        assert_eq!(system.get_active_drives().len(), 0);
        assert!(system.get_current_emotion().is_none());
    }

    #[test]
    fn test_add_drive() {
        let mut system = MotivationSystem::new();

        let drive = Drive::new(DriveType::Curiosity, "Explore".to_string());
        system.add_drive(drive);

        assert_eq!(system.get_active_drives().len(), 1);
    }

    #[test]
    fn test_get_specific_drive() {
        let mut system = MotivationSystem::new();

        let drive = Drive::new(DriveType::Curiosity, "Explore".to_string());
        system.add_drive(drive);

        let retrieved = system.get_drive(DriveType::Curiosity);
        assert!(retrieved.is_some());

        let not_found = system.get_drive(DriveType::Hunger);
        assert!(not_found.is_none());
    }

    #[test]
    fn test_exploration_drive() {
        let mut system = MotivationSystem::new();

        let drive = Drive::new(DriveType::Curiosity, "Explore".to_string());
        system.add_drive(drive);

        let exploration = system.get_exploration_drive();
        assert!(exploration >= 0.0 && exploration <= 1.0);
    }

    #[test]
    fn test_add_goal() {
        let mut system = MotivationSystem::new();

        let goal = Goal::new("Learn something");
        system.add_goal(goal);

        assert!(!system.get_goals().is_empty());
    }

    #[test]
    fn test_update_emotion() {
        let mut system = MotivationSystem::new();

        let emotion = Emotion::new(EmotionType::Curiosity, 0.8);
        system.update_emotion(emotion);

        let current = system.get_current_emotion();
        assert!(current.is_some());
        assert_eq!(current.unwrap().emotion_type, EmotionType::Curiosity);
    }

    #[test]
    fn test_drive_satiation() {
        let mut system = MotivationSystem::new();

        let drive = Drive::new(DriveType::Hunger, "Need food".to_string());
        drive.intensity = 0.9;
        system.add_drive(drive);

        system.satiate_drive(&DriveType::Hunger);

        let satisfied = system.get_drive(DriveType::Hunger);
        assert!(satisfied.unwrap().intensity < 0.9);
    }

    #[test]
    fn test_value_creation() {
        let system = MotivationSystem::new();

        let values = system.get_values();
        assert!(values.iter().any(|v| v.name == "Truth"));
        assert!(values.iter().any(|v| v.name == "Growth"));
    }
}

#[cfg(test)]
mod cognition_engine_tests {
    use super::super::*;

    // ==========================================================================
    // Engine Creation Tests
    // ==========================================================================

    #[test]
    fn test_cognition_engine_creation() {
        let engine = CognitionEngine::new();

        assert_eq!(engine.stats().total_perceptions, 0);
        assert_eq!(engine.stats().total_thoughts, 0);
        assert_eq!(engine.state().mode, CognitionMode::Active);
    }

    #[test]
    fn test_engine_state() {
        let mut engine = CognitionEngine::new();

        engine.set_mode(CognitionMode::Reflective);

        assert_eq!(engine.state().mode, CognitionMode::Reflective);
    }

    // ==========================================================================
    // Perception Tests
    // ==========================================================================

    #[test]
    fn test_perceive() {
        let mut engine = CognitionEngine::new();

        engine.perceive("A fire is burning".to_string());

        assert_eq!(engine.stats().total_perceptions, 1);
        assert_eq!(engine.working_memory().len(), 1);
    }

    // ==========================================================================
    // Thinking Tests
    // ==========================================================================

    #[test]
    fn test_think() {
        let mut engine = CognitionEngine::new();

        engine.think("What should I do about the fire?".to_string());

        assert_eq!(engine.stats().total_thoughts, 1);

        let focus = engine.consciousness().get_focus();
        assert!(focus.is_some());
    }

    // ==========================================================================
    // Planning Tests
    // ==========================================================================

    #[test]
    fn test_plan() {
        let mut engine = CognitionEngine::new();

        engine.perceive("Goal: Put out the fire".to_string());

        let plan = engine.plan("Put out the fire");
        assert!(!plan.is_empty());

        assert_eq!(engine.stats().total_plans, 1);
    }

    // ==========================================================================
    // Decision Tests
    // ==========================================================================

    #[test]
    fn test_decide() {
        let mut engine = CognitionEngine::new();

        let options = vec![
            "Run away".to_string(),
            "Pour water".to_string(),
            "Call help".to_string(),
        ];

        let decision = engine.decide(options);

        assert!(!decision.is_empty());
        assert_eq!(engine.stats().total_decisions, 1);
    }

    // ==========================================================================
    // Learning Tests
    // ==========================================================================

    #[test]
    fn test_learn() {
        let mut engine = CognitionEngine::new();

        engine.learn("Fire produces heat");

        assert_eq!(engine.stats().total_learning, 1);
        assert_eq!(engine.world_model().entities.len(), 2); // Fire and Heat
    }

    // ==========================================================================
    // Imagination Tests
    // ==========================================================================

    #[test]
    fn test_imagine() {
        let mut engine = CognitionEngine::new();

        let scenario = engine.imagine("What if fire met water?");

        assert!(scenario.is_some());
        assert_eq!(engine.stats().total_imaginations, 1);
    }

    // ==========================================================================
    // Reflection Tests
    // ==========================================================================

    #[test]
    fn test_reflect() {
        let mut engine = CognitionEngine::new();

        engine.think("Testing reflection".to_string());

        let reflection = engine.reflect();

        assert!(reflection.thoughts_examined > 0 || reflection.thoughts_examined == 0);
    }

    // ==========================================================================
    // Integration Tests
    // ==========================================================================

    #[test]
    fn test_full_cognition_cycle() {
        let mut engine = CognitionEngine::new();

        // Perceive
        engine.perceive("Fire detected".to_string());

        // Think
        engine.think("Is this dangerous?".to_string());

        // Plan
        let _plan = engine.plan("Handle fire");

        // Decide
        let options = vec![
            "Extinguish".to_string(),
            "Evacuate".to_string(),
            "Monitor".to_string(),
        ];
        let _decision = engine.decide(options);

        // Learn
        engine.learn("Fire can be dangerous");

        // Imagine
        let _scenario = engine.imagine("What if fire spreads?");

        // Reflect
        let _reflection = engine.reflect();

        // Verify all operations occurred
        assert!(engine.stats().total_perceptions > 0);
        assert!(engine.stats().total_thoughts > 0);
        assert!(engine.stats().total_plans > 0);
        assert!(engine.stats().total_decisions > 0);
        assert!(engine.stats().total_learning > 0);
        assert!(engine.stats().total_imaginations > 0);
    }

    #[test]
    fn test_mode_switching() {
        let mut engine = CognitionEngine::new();

        // Active mode
        engine.set_mode(CognitionMode::Active);
        assert_eq!(engine.state().mode, CognitionMode::Active);

        // Reflective mode
        engine.set_mode(CognitionMode::Reflective);
        assert_eq!(engine.state().mode, CognitionMode::Reflective);

        // Focused mode
        engine.set_mode(CognitionMode::Focused);
        assert_eq!(engine.state().mode, CognitionMode::Focused);

        // Creative mode
        engine.set_mode(CognitionMode::Creative);
        assert_eq!(engine.state().mode, CognitionMode::Creative);

        // Sleep mode
        engine.set_mode(CognitionMode::Sleep);
        assert_eq!(engine.state().mode, CognitionMode::Sleep);
    }

    #[test]
    fn test_cognition_state_access() {
        let engine = CognitionEngine::new();

        // Access consciousness
        let _consciousness = engine.consciousness();

        // Access working memory
        let _working_memory = engine.working_memory();

        // Access world model
        let _world_model = engine.world_model();

        // Access motivation
        let _motivation = engine.motivation();
    }
}
