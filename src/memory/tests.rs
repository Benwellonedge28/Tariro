//! Tariro Memory System Tests

#[cfg(test)]
mod episodic_memory_tests {
    use super::super::*;

    #[test]
    fn test_episode_creation() {
        let event = Event::new("Saw fire".to_string(), EventType::Perception);
        let episode = Episode::new(event);

        assert_eq!(episode.event.description, "Saw fire");
        assert_eq!(episode.event.event_type, EventType::Perception);
        assert!(episode.id.is_valid());
    }

    #[test]
    fn test_episode_with_emotion() {
        let event = Event::new("Won lottery".to_string(), EventType::Learning);
        let episode = Episode::new(event)
            .with_emotion(0.9, 0.8) // Positive valence, high arousal
            .with_importance(0.9);

        assert!((episode.emotional_valence - 0.9).abs() < 0.001);
        assert!((episode.emotional_arousal - 0.8).abs() < 0.001);
        assert!((episode.importance - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_episode_with_entity() {
        let entity = MemoryEntity::new("Fire", "Process", EntityRole::Object);
        let event = Event::new("Observed fire".to_string(), EventType::Perception);
        let episode = Episode::new(event).with_entity(entity);

        assert_eq!(episode.entities.len(), 1);
        assert_eq!(episode.entities[0].name, "Fire");
    }

    #[test]
    fn test_episode_retrieval_recording() {
        let event = Event::new("Test event".to_string(), EventType::Thought);
        let mut episode = Episode::new(event);

        assert_eq!(episode.retrieval_count, 0);

        episode.record_retrieval();
        assert_eq!(episode.retrieval_count, 1);
        assert!(episode.last_retrieval.is_some());

        episode.record_retrieval();
        assert_eq!(episode.retrieval_count, 2);
    }

    #[test]
    fn test_episodic_memory_store() {
        let mut memory = EpisodicMemory::new();

        let event = Event::new("First memory".to_string(), EventType::Perception);
        let episode = Episode::new(event);
        let id = memory.store(episode).unwrap();

        assert_eq!(memory.len(), 1);
        assert!(memory.retrieve(&id).is_some());
    }

    #[test]
    fn test_episodic_memory_retrieve_temporal() {
        let mut memory = EpisodicMemory::new();

        let event1 = Event::new("Yesterday".to_string(), EventType::Thought);
        let event2 = Event::new("Today".to_string(), EventType::Thought);

        let ep1 = Episode::new(event1);
        let ep2 = Episode::new(event2);

        memory.store(ep1).unwrap();
        memory.store(ep2).unwrap();

        let now = Utc::now();
        let yesterday = now - chrono::Duration::days(1);

        let recent = memory.retrieve_temporal(yesterday, now);
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_episodic_memory_retrieve_by_entity() {
        let mut memory = EpisodicMemory::new();

        let entity = MemoryEntity::new("Fire", "Process", EntityRole::Object);
        let event = Event::new("Fire observed".to_string(), EventType::Perception);
        let episode = Episode::new(event).with_entity(entity);

        memory.store(episode).unwrap();

        let results = memory.retrieve_by_entity("Fire");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_episodic_memory_search() {
        let mut memory = EpisodicMemory::new();

        let event = Event::new("Fire produces smoke".to_string(), EventType::Perception);
        let episode = Episode::new(event);

        memory.store(episode).unwrap();

        let results = memory.search("fire");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_episodic_memory_retrieve_recent() {
        let mut memory = EpisodicMemory::new();

        for i in 0..5 {
            let event = Event::new(format!("Memory {}", i), EventType::Thought);
            memory.store(Episode::new(event)).unwrap();
        }

        let recent = memory.retrieve_recent(3);
        assert_eq!(recent.len(), 3);
    }

    #[test]
    fn test_episodic_memory_capacity() {
        let mut memory = EpisodicMemory::with_capacity(3);

        for i in 0..5 {
            let event = Event::new(format!("Memory {}", i), EventType::Thought);
            memory.store(Episode::new(event)).unwrap();
        }

        assert_eq!(memory.len(), 3); // Should evict oldest
    }
}

#[cfg(test)]
mod semantic_memory_tests {
    use super::super::*;

    #[test]
    fn test_semantic_knowledge_creation() {
        let knowledge = SemanticKnowledge::new(
            "Fire",
            KnowledgeCategory::Physical,
            "Fire is hot and produces light",
        );

        assert_eq!(knowledge.concept, "Fire");
        assert_eq!(knowledge.category, KnowledgeCategory::Physical);
        assert_eq!(knowledge.confidence, 0.5);
    }

    #[test]
    fn test_semantic_memory_store() {
        let mut memory = SemanticMemory::new();

        let knowledge = SemanticKnowledge::new(
            "Water",
            KnowledgeCategory::Physical,
            "Water is essential for life",
        );

        let id = memory.store(knowledge).unwrap();

        assert_eq!(memory.len(), 1);
        assert!(memory.retrieve(&id).is_some());
    }

    #[test]
    fn test_semantic_memory_retrieve_by_name() {
        let mut memory = SemanticMemory::new();

        let knowledge = SemanticKnowledge::new(
            "Fire",
            KnowledgeCategory::Physical,
            "Fire is hot",
        );
        memory.store(knowledge).unwrap();

        let retrieved = memory.retrieve_by_name("Fire");
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().concept, "Fire");
    }

    #[test]
    fn test_semantic_memory_retrieve_by_category() {
        let mut memory = SemanticMemory::new();

        memory
            .store(SemanticKnowledge::new(
                "Fire",
                KnowledgeCategory::Physical,
                "Physical knowledge",
            ))
            .unwrap();

        memory
            .store(SemanticKnowledge::new(
                "Society",
                KnowledgeCategory::Social,
                "Social knowledge",
            ))
            .unwrap();

        let physical = memory.retrieve_by_category(KnowledgeCategory::Physical);
        assert_eq!(physical.len(), 1);
    }

    #[test]
    fn test_semantic_memory_search() {
        let mut memory = SemanticMemory::new();

        memory
            .store(SemanticKnowledge::new(
                "Fire",
                KnowledgeCategory::Physical,
                "Fire produces heat",
            ))
            .unwrap();

        let results = memory.search("heat");
        assert!(!results.is_empty());
    }

    #[test]
    fn test_semantic_memory_add_association() {
        let mut memory = SemanticMemory::new();

        let fire = memory
            .store(SemanticKnowledge::new(
                "Fire",
                KnowledgeCategory::Physical,
                "Fire is hot",
            ))
            .unwrap();

        let smoke = memory
            .store(SemanticKnowledge::new(
                "Smoke",
                KnowledgeCategory::Physical,
                "Smoke rises",
            ))
            .unwrap();

        memory.add_association(&fire, &smoke, AssociationType::Causes);

        let associations = memory.get_associations(&fire);
        assert!(!associations.is_empty());
    }

    #[test]
    fn test_semantic_memory_get_related() {
        let mut memory = SemanticMemory::new();

        let a = memory
            .store(SemanticKnowledge::new("A", KnowledgeCategory::Abstract, "A"))
            .unwrap();

        let b = memory
            .store(SemanticKnowledge::new("B", KnowledgeCategory::Abstract, "B"))
            .unwrap();

        let c = memory
            .store(SemanticKnowledge::new("C", KnowledgeCategory::Abstract, "C"))
            .unwrap();

        memory.add_association(&a, &b, AssociationType::Causes);
        memory.add_association(&b, &c, AssociationType::Causes);

        let related = memory.get_related(&a, 2);
        assert!(related.len() >= 2); // Should include B and C
    }

    #[test]
    fn test_initialize_commonsense() {
        let mut memory = SemanticMemory::new();
        memory.initialize_commonsense();

        // Should have common sense knowledge
        assert!(memory.retrieve_by_name("Fire").is_some());
        assert!(memory.retrieve_by_name("Water").is_some());
    }
}

#[cfg(test)]
mod procedural_memory_tests {
    use super::super::*;

    #[test]
    fn test_procedure_creation() {
        let procedure = Procedure::new("Make Coffee", "Kitchen");

        assert_eq!(procedure.name, "Make Coffee");
        assert_eq!(procedure.domain, "Kitchen");
        assert_eq!(procedure.success_rate, 0.0);
    }

    #[test]
    fn test_procedure_with_step() {
        let procedure = Procedure::new("Make Coffee", "Kitchen")
            .with_step(ProcedureStep {
                order: 1,
                description: "Boil water".to_string(),
                action: "boil".to_string(),
                conditions: vec![],
                duration_ms: Some(30000),
            });

        assert_eq!(procedure.steps.len(), 1);
    }

    #[test]
    fn test_procedure_practice() {
        let mut procedure = Procedure::new("Test", "Test");

        procedure.record_practice(true);
        assert!((procedure.success_rate - 0.2).abs() < 0.001);
        assert_eq!(procedure.practice_count, 1);

        procedure.record_practice(true);
        assert!((procedure.success_rate - 0.36).abs() < 0.001);

        procedure.record_practice(false);
        assert!(procedure.success_rate < 0.36);
    }

    #[test]
    fn test_procedural_memory_store() {
        let mut memory = ProceduralMemory::new();

        let procedure = Procedure::new("Make Coffee", "Kitchen");
        let id = memory.store(procedure).unwrap();

        assert_eq!(memory.len(), 1);
        assert!(memory.retrieve(&id).is_some());
    }

    #[test]
    fn test_procedural_memory_retrieve_by_domain() {
        let mut memory = ProceduralMemory::new();

        memory
            .store(Procedure::new("Make Coffee", "Kitchen"))
            .unwrap();

        memory
            .store(Procedure::new("Fix Car", "Automotive"))
            .unwrap();

        let kitchen_procs = memory.retrieve_by_domain("Kitchen");
        assert_eq!(kitchen_procs.len(), 1);
    }

    #[test]
    fn test_procedural_memory_get_expert() {
        let mut memory = ProceduralMemory::new();

        let mut proc1 = Procedure::new("Skill 1", "Domain");
        proc1.automation_level = 0.3;

        let mut proc2 = Procedure::new("Skill 2", "Domain");
        proc2.automation_level = 0.8;

        memory.store(proc1).unwrap();
        memory.store(proc2).unwrap();

        let experts = memory.get_expert_procedures();
        assert!(!experts.is_empty());
        // Should be sorted by automation level
    }
}

#[cfg(test)]
mod memory_system_tests {
    use super::super::*;

    #[test]
    fn test_memory_system_creation() {
        let system = MemorySystem::new();

        assert!(system.episodic.is_empty());
        assert!(system.semantic.is_empty());
        assert!(system.procedural.is_empty());
    }

    #[test]
    fn test_memory_system_initialize() {
        let mut system = MemorySystem::new();
        system.initialize();

        // Should have common sense knowledge
        assert!(!system.semantic.is_empty());
    }

    #[test]
    fn test_memory_system_store_episode() {
        let mut system = MemorySystem::new();

        let event = Event::new("First experience".to_string(), EventType::Perception);
        let episode = Episode::new(event);
        let id = system.store_episode(episode).unwrap();

        assert!(system.recall_episode(&id).is_some());
    }

    #[test]
    fn test_memory_system_store_knowledge() {
        let mut system = MemorySystem::new();

        let knowledge = SemanticKnowledge::new(
            "Test Concept",
            KnowledgeCategory::Abstract,
            "Test description",
        );
        let id = system.store_knowledge(knowledge).unwrap();

        assert!(system.recall_knowledge(&id).is_some());
    }

    #[test]
    fn test_memory_system_store_procedure() {
        let mut system = MemorySystem::new();

        let procedure = Procedure::new("Test Procedure", "Test");
        let id = system.store_procedure(procedure).unwrap();

        assert!(system.recall_procedure(&id).is_some());
    }

    #[test]
    fn test_memory_system_search() {
        let mut system = MemorySystem::new();

        let event = Event::new("Fire burns".to_string(), EventType::Perception);
        system.store_episode(Episode::new(event)).unwrap();

        let knowledge = SemanticKnowledge::new(
            "Fire",
            KnowledgeCategory::Physical,
            "Fire is hot",
        );
        system.store_knowledge(knowledge).unwrap();

        let results = system.search("fire");
        assert!(!results.is_empty());
        assert!(results.total() >= 2);
    }

    #[test]
    fn test_memory_system_get_recent() {
        let mut system = MemorySystem::new();

        for i in 0..3 {
            let event = Event::new(format!("Memory {}", i), EventType::Thought);
            system.store_episode(Episode::new(event)).unwrap();
        }

        let recent = system.get_recent_experiences(2);
        assert_eq!(recent.len(), 2);
    }

    #[test]
    fn test_memory_system_decay() {
        let mut system = MemorySystem::new();

        let event = Event::new("Test memory".to_string(), EventType::Thought);
        system.store_episode(Episode::new(event)).unwrap();

        system.decay();

        // Should have performed decay operation
        let stats = system.stats();
        assert_eq!(stats.system.decay_operations, 1);
    }
}

#[cfg(test)]
mod trace_strength_tests {
    use super::super::*;

    #[test]
    fn test_trace_strength_creation() {
        let trace = TraceStrength::new(0.7);
        assert!((trace.0 - 0.7).abs() < 0.001);
    }

    #[test]
    fn test_trace_strength_clamping() {
        let trace_high = TraceStrength::new(1.5);
        assert!((trace_high.0 - 1.0).abs() < 0.001);

        let trace_low = TraceStrength::new(-0.5);
        assert!((trace_low.0 - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_trace_strength_decay() {
        let trace = TraceStrength::new(1.0);
        let decayed = trace.decay(0.1);
        assert!((decayed.0 - 0.9).abs() < 0.001);

        let decayed2 = trace.decay(1.0);
        assert!((decayed2.0 - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_trace_strength_boost() {
        let trace = TraceStrength::new(0.5);
        let boosted = trace.boost(0.3);
        assert!((boosted.0 - 0.8).abs() < 0.001);

        // Capped at 1.0
        let boosted_high = trace.boost(0.8);
        assert!((boosted_high.0 - 1.0).abs() < 0.001);
    }
}

#[cfg(test)]
mod event_type_tests {
    use super::super::*;

    #[test]
    fn test_event_types() {
        assert_eq!(EventType::Perception.as_str(), "perception");
        assert_eq!(EventType::Thought.as_str(), "thought");
        assert_eq!(EventType::Action.as_str(), "action");
        assert_eq!(EventType::Learning.as_str(), "learning");
    }
}

#[cfg(test)]
mod knowledge_category_tests {
    use super::super::*;

    #[test]
    fn test_knowledge_categories() {
        assert_eq!(KnowledgeCategory::Physical.as_str(), "physical");
        assert_eq!(KnowledgeCategory::Social.as_str(), "social");
        assert_eq!(KnowledgeCategory::Mental.as_str(), "mental");
        assert_eq!(KnowledgeCategory::Causal.as_str(), "causal");
    }
}
