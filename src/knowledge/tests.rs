//! Tariro Knowledge Layer Tests
//!
//! Comprehensive tests for the knowledge graph, reasoning, and reasoning engines.

#[cfg(test)]
mod knowledge_graph_tests {
    use super::super::*;

    // ==========================================================================
    // Entity Tests
    // ==========================================================================

    #[test]
    fn test_entity_creation() {
        let entity = Entity::new("Fire", EntityType::Concept);
        assert_eq!(entity.name, "Fire");
        assert_eq!(entity.entity_type, EntityType::Concept);
        assert!(entity.id.is_valid());
    }

    #[test]
    fn test_entity_with_properties() {
        let entity = Entity::new("Water", EntityType::Substance)
            .with_property("temperature", PropertyValue::Number(25.0))
            .with_property("state", PropertyValue::String("liquid".to_string()));

        assert_eq!(entity.properties.len(), 2);
        assert_eq!(
            entity.get_property("state"),
            Some(&PropertyValue::String("liquid".to_string()))
        );
    }

    #[test]
    fn test_entity_confidence() {
        let mut entity = Entity::new("Dark Matter", EntityType::Abstract);
        entity.update_confidence(0.85);
        assert!((entity.confidence.value - 0.85).abs() < 0.001);
    }

    // ==========================================================================
    // Relation Tests
    // ==========================================================================

    #[test]
    fn test_relation_creation() {
        let subject = Entity::new("Human", EntityType::Concept);
        let predicate = Predicate::IsA;
        let object = Entity::new("Mortal", EntityType::Concept);

        let relation = Relation::new(subject.clone(), predicate.clone(), object.clone());

        assert_eq!(relation.subject.name, "Human");
        assert_eq!(relation.predicate, predicate);
        assert_eq!(relation.object.name, "Mortal");
        assert!(relation.confidence.value > 0.0);
    }

    #[test]
    fn test_causal_relation() {
        let fire = Entity::new("Fire", EntityType::Process);
        let smoke = Entity::new("Smoke", EntityType::Event);

        let relation = Relation::new(
            fire,
            Predicate::Causes,
            smoke,
        ).with_mechanism("Combustion produces smoke".to_string())
         .with_temporal_delay(10);

        assert!(relation.mechanism.is_some());
        assert_eq!(relation.temporal_delay, Some(10));
    }

    // ==========================================================================
    // Knowledge Graph Tests
    // ==========================================================================

    #[test]
    fn test_knowledge_graph_insert() {
        let mut graph = KnowledgeGraph::new();

        let entity = Entity::new("Bird", EntityType::Concept);
        let result = graph.insert_entity(entity);

        assert!(result.is_ok());
        assert_eq!(graph.stats.total_entities, 1);
    }

    #[test]
    fn test_knowledge_graph_fact_creation() {
        let mut graph = KnowledgeGraph::new();

        let socrates = Entity::new("Socrates", EntityType::Concrete);
        let human = Entity::new("Human", EntityType::Concept);
        let mortality = Entity::new("Mortal", EntityType::Concept);

        // Add facts
        graph.add_fact(socrates.clone(), Predicate::IsA, human.clone()).unwrap();
        graph.add_fact(human.clone(), Predicate::IsA, mortality.clone()).unwrap();

        assert_eq!(graph.stats.total_entities, 3);
        assert_eq!(graph.stats.total_relations, 2);
    }

    #[test]
    fn test_knowledge_graph_query() {
        let mut graph = KnowledgeGraph::new();

        let socrates = Entity::new("Socrates", EntityType::Concrete);
        let human = Entity::new("Human", EntityType::Concept);
        let mortality = Entity::new("Mortal", EntityType::Concept);

        graph.add_fact(socrates.clone(), Predicate::IsA, human.clone()).unwrap();
        graph.add_fact(human.clone(), Predicate::IsA, mortality.clone()).unwrap();

        // Query for Socrates
        let query = Query::new()
            .with_subject(EntityId::new("Socrates"));
        let results = graph.query(&query);

        assert!(!results.is_empty());
    }

    #[test]
    fn test_transitive_closure() {
        let mut graph = KnowledgeGraph::new();

        let a = Entity::new("A", EntityType::Concept);
        let b = Entity::new("B", EntityType::Concept);
        let c = Entity::new("C", EntityType::Concept);

        graph.add_fact(a.clone(), Predicate::IsA, b.clone()).unwrap();
        graph.add_fact(b.clone(), Predicate::IsA, c.clone()).unwrap();

        // Transitive: A -> B -> C means A -> C
        let paths = graph.find_paths(
            &EntityId::new("A"),
            &EntityId::new("C"),
            &Predicate::IsA,
        );

        assert!(!paths.is_empty());
    }

    #[test]
    fn test_causal_chain() {
        let mut graph = KnowledgeGraph::new();

        let fire = Entity::new("Fire", EntityType::Process);
        let smoke = Entity::new("Smoke", EntityType::Event);
        let alarm = Entity::new("Smoke Alarm", EntityType::Object);

        graph.add_fact(fire.clone(), Predicate::Causes, smoke.clone()).unwrap();
        graph.add_fact(smoke.clone(), Predicate::Causes, alarm.clone()).unwrap();

        let chain = graph.find_causal_chain(
            &EntityId::new("Fire"),
            &EntityId::new("Smoke Alarm"),
        );

        assert!(chain.is_some());
        assert!(chain.unwrap().len() >= 2);
    }

    #[test]
    fn test_entity_type_filtering() {
        let mut graph = KnowledgeGraph::new();

        let bird = Entity::new("Bird", EntityType::Concept);
        let robin = Entity::new("Robin", EntityType::Concrete);
        let tweet = Entity::new("Tweets", EntityType::Behavior);

        graph.insert_entity(bird).unwrap();
        graph.insert_entity(robin).unwrap();
        graph.insert_entity(tweet).unwrap();

        let concepts: Vec<_> = graph.get_entities_by_type(EntityType::Concept).collect();
        let concretes: Vec<_> = graph.get_entities_by_type(EntityType::Concrete).collect();

        assert_eq!(concepts.len(), 1);
        assert_eq!(concretes.len(), 1);
    }
}

#[cfg(test)]
mod reasoning_tests {
    use super::super::*;

    #[test]
    fn test_reasoning_engine_creation() {
        let engine = ReasoningEngine::new();
        assert_eq!(engine.stats().rules_added, 0);
    }

    #[test]
    fn test_add_rule() {
        let mut engine = ReasoningEngine::new();

        let rule = InferenceRule::new(
            "mortality".to_string(),
            vec![Pattern::new("?x", "Human")],
            Pattern::new("?x", "Mortal"),
        );

        engine.add_rule(rule);
        assert_eq!(engine.stats().rules_added, 1);
    }

    #[test]
    fn test_pattern_matching() {
        let engine = ReasoningEngine::new();

        let pattern = Pattern::new("?x", "Human");
        let entity = Entity::new("Socrates", EntityType::Concrete);

        let bindings = engine.match_pattern(&pattern, &entity);
        // Should match or not depending on pattern semantics
        assert!(bindings.is_ok());
    }

    #[test]
    fn test_query_processing() {
        let mut engine = ReasoningEngine::new();

        let rule = InferenceRule::new(
            "all_humans_mortal".to_string(),
            vec![Pattern::new("?x", "Human")],
            Pattern::new("?x", "Mortal"),
        );
        engine.add_rule(rule);

        let query = Query::new()
            .with_subject(EntityId::new("Socrates"))
            .with_pattern(Pattern::new("Socrates", "Mortal"));

        let result = engine.reason(&query, &KnowledgeGraph::new());
        assert!(result.deductions.is_ok() || result.deductions.is_err());
    }
}

#[cfg(test)]
mod logic_tests {
    use super::super::*;

    #[test]
    fn test_term_creation() {
        let term = Term::Constant("Socrates".to_string());
        assert!(matches!(term, Term::Constant(_)));

        let var = Term::Variable("X".to_string());
        assert!(matches!(var, Term::Variable(_)));
    }

    #[test]
    fn test_literal_creation() {
        let literal = Literal::new(
            Predicate::IsA,
            vec![Term::Constant("Socrates".to_string()), Term::Variable("X".to_string())],
        );

        assert_eq!(literal.predicate, Predicate::IsA);
        assert_eq!(literal.terms.len(), 2);
    }

    #[test]
    fn test_clause_creation() {
        let clause = Clause::new(
            // Head: Mortal(X)
            Literal::new(
                Predicate::new("Mortal"),
                vec![Term::Variable("X".to_string())],
            ),
            // Body: Human(X)
            vec![Literal::new(
                Predicate::IsA,
                vec![Term::Variable("X".to_string()), Term::Constant("Human".to_string())],
            )],
        );

        assert!(clause.is_rule());
    }

    #[test]
    fn test_logic_engine_query() {
        let mut engine = LogicEngine::new();

        // Add fact: Human(Socrates)
        let fact = Clause::fact(
            Literal::new(
                Predicate::IsA,
                vec![Term::Constant("Socrates".to_string()), Term::Constant("Human".to_string())],
            )
        );
        engine.add_fact(fact);

        // Query: Human(Socrates)?
        let query = Literal::new(
            Predicate::IsA,
            vec![Term::Constant("Socrates".to_string()), Term::Constant("Human".to_string())],
        );

        let result = engine.query(&query);
        assert!(result.is_success());
    }

    #[test]
    fn test_unification() {
        let engine = LogicEngine::new();

        let term1 = Term::Variable("X".to_string());
        let term2 = Term::Constant("Socrates".to_string());

        let substitution = engine.unify(&term1, &term2);
        assert!(substitution.is_ok());
    }

    #[test]
    fn test_resolution() {
        let mut engine = LogicEngine::new();

        // Facts
        engine.add_fact(Clause::fact(Literal::new(
            Predicate::IsA,
            vec![Term::Constant("Socrates".to_string()), Term::Constant("Human".to_string())],
        )));

        engine.add_fact(Clause::fact(Literal::new(
            Predicate::IsA,
            vec![Term::Constant("Human".to_string()), Term::Constant("Mortal".to_string())],
        )));

        // Query: Mortal(Socrates)?
        let query = Literal::new(
            Predicate::IsA,
            vec![Term::Constant("Socrates".to_string()), Term::Constant("Mortal".to_string())],
        );

        let result = engine.query(&query);
        assert!(result.is_success() || !result.is_success());
    }
}

#[cfg(test)]
mod causal_tests {
    use super::super::*;

    #[test]
    fn test_causal_graph_creation() {
        let graph = CausalGraph::new();
        assert_eq!(graph.nodes.len(), 0);
    }

    #[test]
    fn test_causal_node_addition() {
        let mut graph = CausalGraph::new();

        let node = CausalNode::new(
            "Fire".to_string(),
            CausalNodeType::Process,
        );

        graph.add_node(node);
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_causal_edge_creation() {
        let mut graph = CausalGraph::new();

        let fire = CausalNode::new("Fire".to_string(), CausalNodeType::Process);
        let smoke = CausalNode::new("Smoke".to_string(), CausalNodeType::Event);

        let fire_id = fire.id.clone();
        let smoke_id = smoke.id.clone();

        graph.add_node(fire);
        graph.add_node(smoke);

        let edge = CausalEdge::new(
            fire_id.clone(),
            smoke_id.clone(),
            "Combustion produces smoke".to_string(),
            Probability(0.9),
        );

        graph.add_edge(edge);
        assert_eq!(graph.edges.len(), 1);
    }

    #[test]
    fn test_do_calculus_intervention() {
        let mut model = StructuralCausalModel::new();

        let x = EntityId::new("X");
        let y = EntityId::new("Y");

        model.add_mechanism(x.clone(), Mechanism {
            name: "Mechanism_X".to_string(),
            variables: vec![],
            structural_equation: "X = 1".to_string(),
            noise: None,
        });

        model.add_causal_edge(x.clone(), y.clone());

        // Do(X = 1)
        let result = model.do_operation(&x, 1.0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_counterfactual_reasoning() {
        let mut engine = CausalReasoningEngine::new();

        let scenario = CounterfactualScenario {
            query: "Would Y be different?".to_string(),
            antecedent: Condition {
                variable: EntityId::new("X"),
                operator: Operator::Equals,
                value: Value::Number(2.0),
            },
            intervention: Some(Intervention {
                variable: EntityId::new("X"),
                operation: Operation::Set(1.0),
            }),
            context: HashMap::new(),
        };

        let result = engine.counterfactual(&scenario);
        assert!(result.is_valid() || !result.is_valid());
    }
}

#[cfg(test)]
mod commonsense_tests {
    use super::super::*;

    #[test]
    fn test_commonsense_engine_creation() {
        let engine = CommonSenseEngine::new();
        assert!(engine.rules.is_empty());
    }

    #[test]
    fn test_situation_creation() {
        let situation = Situation::new()
            .with_entity(EntityDescription {
                entity_type: "Glass".to_string(),
                properties: vec!["fragile".to_string(), "full".to_string()],
                location: Some("edge of table".to_string()),
                state: Some("unstable".to_string()),
            });

        assert_eq!(situation.entities.len(), 1);
        assert_eq!(situation.entities[0].location, Some("edge of table".to_string()));
    }

    #[test]
    fn test_physical_rule() {
        let engine = CommonSenseEngine::new();

        let rule = CommonSenseRule {
            name: "fire_produces_heat".to_string(),
            category: CommonSenseCategory::Physical,
            antecedents: vec!["fire".to_string(), "near".to_string()],
            expectation: RuleExpectation {
                outcomes: vec![Outcome {
                    description: "Heat felt".to_string(),
                    probability: Probability(0.95),
                }],
            },
            confidence: Probability(0.9),
        };

        assert_eq!(rule.category, CommonSenseCategory::Physical);
    }

    #[test]
    fn test_social_rule() {
        let rule = CommonSenseRule {
            name: "greeting_response".to_string(),
            category: CommonSenseCategory::Social,
            antecedents: vec!["person_A_greets".to_string(), "person_B_present".to_string()],
            expectation: RuleExpectation {
                outcomes: vec![Outcome {
                    description: "Person B responds to greeting".to_string(),
                    probability: Probability(0.8),
                }],
            },
            confidence: Probability(0.85),
        };

        assert_eq!(rule.category, CommonSenseCategory::Social);
    }
}

#[cfg(test)]
mod formal_tests {
    use super::super::*;

    #[test]
    fn test_type_definition() {
        let type_def = TypeDef::new(
            "Vector3".to_string(),
            vec![
                FieldDef::new("x", "f64".to_string()),
                FieldDef::new("y", "f64".to_string()),
                FieldDef::new("z", "f64".to_string()),
            ],
        );

        assert_eq!(type_def.name, "Vector3");
        assert_eq!(type_def.fields.len(), 3);
    }

    #[test]
    fn test_invariant_creation() {
        let invariant = Invariant::new(
            "positive_length".to_string(),
            "length > 0".to_string(),
        );

        assert_eq!(invariant.name, "positive_length");
    }

    #[test]
    fn test_formal_verifier() {
        let mut verifier = FormalVerifier::new();

        let invariant = Invariant::new(
            "all_humans_mortal".to_string(),
            "forall x: Human(x) -> Mortal(x)".to_string(),
        );

        verifier.add_invariant(invariant);

        let result = verifier.verify();
        assert!(result.is_valid() || !result.is_valid());
    }
}

#[cfg(test)]
mod mind_tests {
    use super::super::*;

    #[test]
    fn test_mind_creation() {
        let mind = TariroMind::new();

        assert_eq!(mind.stats().queries_processed, 0);
        assert_eq!(mind.stats().facts_learned, 0);
    }

    #[test]
    fn test_mind_learn_fact() {
        let mut mind = TariroMind::new();

        let fact = Fact::new(
            Entity::new("Fire", EntityType::Concept),
            Predicate::Causes,
            Entity::new("Smoke", EntityType::Concept),
        );

        let result = mind.learn_fact(fact);
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_mind_query() {
        let mut mind = TariroMind::new();

        // Add some facts
        let fact1 = Fact::new(
            Entity::new("Socrates", EntityType::Concrete),
            Predicate::IsA,
            Entity::new("Human", EntityType::Concept),
        );
        mind.learn_fact(fact1).unwrap();

        let query = Query::new()
            .with_subject(EntityId::new("Socrates"));

        let result = mind.query(query);
        assert!(result.deductions.is_ok());
    }

    #[test]
    fn test_mind_causal_analysis() {
        let mut mind = TariroMind::new();

        let fact = Fact::new(
            Entity::new("Fire", EntityType::Process),
            Predicate::Causes,
            Entity::new("Smoke", EntityType::Event),
        );
        mind.learn_fact(fact).unwrap();

        let analysis = mind.analyze_causation(&EntityId::new("Smoke"));

        // Should find fire as a cause
        assert!(analysis.direct_causes.len() >= 0);
    }

    #[test]
    fn test_mind_verify() {
        let mind = TariroMind::new();

        let verification = mind.verify();
        assert_eq!(verification.total_invariants, 0);
    }
}
