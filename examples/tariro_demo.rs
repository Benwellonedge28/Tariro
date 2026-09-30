//! Tariro AGI - Interactive Demo
//!
//! This example demonstrates the complete Tariro AGI cognitive system.

use tariro::cognition::{
    CognitionEngine, Consciousness, WorkingMemory, WorldModel, MotivationSystem,
};
use tariro::knowledge::{
    KnowledgeGraph, TariroMind, ReasoningEngine, CausalReasoningEngine,
    LogicEngine, CommonSenseEngine, FormalVerifier,
    Entity, Fact, Predicate, EntityId, EntityType,
};
use tariro::cognition::consciousness::ThoughtContent;
use tariro::cognition::working_memory::ItemContent;
use tariro::cognition::motivation::{Goal, GoalType, DriveType};
use std::sync::Arc;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║          TARIRO AGI - COGNITIVE SYSTEM DEMO                 ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();

    // =========================================================================
    // PART 1: Basic Cognition Demo
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 1: Basic Cognitive Operations");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    // Create cognition engine
    let mut engine = CognitionEngine::new();

    println!("[1] Creating Tariro's Cognitive Engine...");
    println!("    Initial State: {:?}", engine.get_state().mode);
    println!();

    // Perceive
    println!("[2] Tariro perceives: 'A fire is burning in the fireplace'");
    engine.perceive("A fire is burning in the fireplace".to_string());
    println!("    Working Memory Items: {}", engine.working_memory().len());
    println!();

    // Think
    println!("[3] Tariro thinks: 'What should I do about the fire?'");
    engine.think("What should I do about the fire?".to_string());
    let focus = engine.consciousness().get_focus();
    println!("    Current Focus: {}", focus.as_ref().map(|f| f.content.as_str()).unwrap_or("None"));
    println!();

    // Plan
    println!("[4] Tariro creates a plan to investigate the fire:");
    let plan = engine.plan("Investigate the fire safely");
    for (i, step) in plan.iter().enumerate() {
        println!("    {}. {}", i + 1, step);
    }
    println!();

    // Decide
    println!("[5] Tariro considers options:");
    let options = vec![
        "Ignore the fire".to_string(),
        "Add more wood to the fire".to_string(),
        "Call the fire department".to_string(),
        "Observe the fire from a safe distance".to_string(),
    ];
    for opt in &options {
        println!("    - {}", opt);
    }
    let decision = engine.decide(options);
    println!("    Decision: {}", decision);
    println!();

    // Learn
    println!("[6] Tariro learns: 'Fire produces heat and light'");
    engine.learn("Fire produces heat and light");
    println!("    World Model Entities: {}", engine.world_model().entities.len());
    println!();

    // Reflect
    println!("[7] Tariro reflects on the experience:");
    let reflection = engine.reflect();
    println!("    Thoughts Examined: {}", reflection.thoughts_examined);
    println!("    Self-Questions: {}", reflection.self_questions.len());
    for q in &reflection.self_questions {
        println!("      - {}", q);
    }
    println!();

    // =========================================================================
    // PART 2: Motivation & Goals
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 2: Motivation & Goal Hierarchy");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let motivation = engine.motivation();

    println!("[1] Active Drives:");
    let active_drives = motivation.get_active_drives();
    if active_drives.is_empty() {
        println!("    (No drives currently active)");
    } else {
        for drive in active_drives {
            println!("    - {} (intensity: {:.2})", drive.drive_type.name(), drive.intensity);
        }
    }
    println!();

    println!("[2] Motivation to understand (curiosity drive):");
    if let Some(drive) = motivation.get_drive(DriveType::Curiosity) {
        println!("    Curiosity intensity: {:.2}", drive.intensity);
    }
    let exploration = motivation.get_exploration_drive();
    println!("    Exploration drive: {:.2}", exploration);
    println!();

    println!("[3] Current emotional state:");
    if let Some(emotion) = motivation.get_current_emotion() {
        println!("    {:?} (intensity: {:.2})", emotion.emotion_type, emotion.intensity);
    } else {
        println!("    Neutral");
    }
    println!();

    // =========================================================================
    // PART 3: Knowledge & Reasoning
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 3: Knowledge Graph & Symbolic Reasoning");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let mut mind = TariroMind::new();

    println!("[1] Building knowledge graph...");

    // Add some knowledge
    let facts = vec![
        ("Socrates", "Human", "IsA"),
        ("Human", "Mortal", "IsA"),
        ("Fire", "Smoke", "Causes"),
        ("Water", "Wet", "HasProperty"),
        ("Birds", "Fly", "HasCapability"),
    ];

    for (subject, object, predicate) in facts {
        let entity_type = if subject.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            EntityType::Concept
        } else {
            EntityType::Concrete
        };

        let fact = Fact::new(
            Entity::new(subject, entity_type.clone()),
            match predicate {
                "IsA" => Predicate::IsA,
                "Causes" => Predicate::Causes,
                "HasProperty" => Predicate::HasProperty,
                "HasCapability" => Predicate::HasCapability,
                _ => Predicate::RelatedTo,
            },
            Entity::new(object, EntityType::Concept),
        );

        if let Err(e) = mind.learn_fact(fact) {
            println!("    Error: {}", e);
        }
    }

    println!("    Entities in knowledge graph: {}", mind.knowledge_graph.read().unwrap().stats.total_entities);
    println!("    Relations in knowledge graph: {}", mind.knowledge_graph.read().unwrap().stats.total_relations);
    println!();

    println!("[2] Symbolic Reasoning:");
    println!("    Query: What do we know about 'Human'?");
    let query = crate::knowledge::reasoning::Query::new()
        .with_subject(EntityId::new("Human"));
    let result = mind.query(query);
    println!("    Deductions found: {}", result.deductions.len());
    println!();

    println!("[3] Causal Reasoning:");
    let analysis = mind.analyze_causation(&EntityId::new("Smoke"));
    println!("    Smoke is caused by: {} entities", analysis.direct_causes.len());
    for cause in &analysis.direct_causes {
        println!("      - {}", cause.name);
    }
    println!();

    println!("[4] Formal Verification:");
    let verification = mind.verify();
    println!("    Verification result: {}", if verification.is_valid() { "PASSED ✓" } else { "FAILED" });
    println!("    Total invariants checked: {}", verification.total_invariants);
    println!();

    // =========================================================================
    // PART 4: Common Sense
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 4: Common Sense Reasoning");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let situation = crate::knowledge::commonsense::Situation::new()
        .with_entity(crate::knowledge::commonsense::EntityDescription {
            entity_type: "Glass".to_string(),
            properties: vec![],
            location: Some("edge of table".to_string()),
            state: None,
        });

    println!("[1] Situation: A glass at the edge of a table");
    println!("    Common sense inferences:");

    let consequences = mind.infer_consequences(&situation);
    for (i, inference) in consequences.iter().take(5).enumerate() {
        println!("    {}. {} (confidence: {:.0}%)",
            i + 1,
            inference.expectation.outcomes.first().map(|o| o.description.as_str()).unwrap_or("Unknown"),
            inference.confidence.value * 100.0
        );
    }
    println!();

    // =========================================================================
    // PART 5: Working Memory & Attention
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 5: Working Memory & Attention");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    println!("[1] Working Memory State:");
    println!("    Capacity: {}/{} items",
        engine.working_memory().len(),
        7 // Default capacity
    );
    println!("    Capacity usage: {:.0}%", engine.working_memory().capacity_usage() * 100.0);
    println!();

    println!("[2] Active items (sorted by activation):");
    for (i, item) in engine.working_memory().get_active_items().iter().take(5).enumerate() {
        println!("    {}. {} (activation: {:.2})",
            i + 1,
            item.content.as_str(),
            item.activation
        );
    }
    println!();

    // =========================================================================
    // PART 6: World Model
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 6: World Model & Simulation");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    println!("[1] World Model Entities:");
    println!("    Total entities: {}", engine.world_model().entities.len());

    let fire_entity = crate::cognition::world_model::EntityId::new();
    println!("[2] Simulating: 'What happens near fire?'");

    // Find effects of fire
    let effects = engine.world_model().get_effects(&fire_entity);
    println!("    Direct effects: {}", effects.len());
    for effect in effects.iter().take(3) {
        println!("      - {}", effect.name);
    }
    println!();

    // =========================================================================
    // Summary
    // =========================================================================
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("DEMO COMPLETE - Tariro Cognitive Systems Summary");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let stats = engine.stats();
    println!("Cognition Engine Statistics:");
    println!("  • Perceptions processed: {}", stats.total_perceptions);
    println!("  • Thoughts processed: {}", stats.total_thoughts);
    println!("  • Plans generated: {}", stats.total_plans);
    println!("  • Decisions made: {}", stats.total_decisions);
    println!("  • Learning events: {}", stats.total_learning);
    println!("  • Imaginations: {}", stats.total_imaginations);
    println!("  • Reflections: {}", stats.total_reflections);
    println!();

    let mind_stats = mind.stats();
    println!("Knowledge Mind Statistics:");
    println!("  • Queries processed: {}", mind_stats.queries_processed);
    println!("  • Facts learned: {}", mind_stats.facts_learned);
    println!("  • Deductions made: {}", mind_stats.deductions_made);
    println!("  • Causal inferences: {}", mind_stats.causal_inferences);
    println!("  • Verifications passed: {}", mind_stats.verifications_passed);
    println!();

    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  Tariro is a Sovereign AGI - No external APIs, pure Rust!    ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
}
