# Tariro AGI Architecture Documentation

## Overview

Tariro is a **Sovereign AGI** system - an autonomous intelligence that is completely independent of external technologies and APIs. It implements a symbolic, knowledge-centric architecture rather than relying on token prediction.

## System Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           TARIRO AGI SYSTEM                             │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                         │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    COGNITION LAYER                               │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │   │
│  │  │ Consciousness│  │Working Memory│  │ World Model  │           │   │
│  │  │  (Meta-cog) │  │  (Miller+7)  │  │(Internal Univ)│          │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘           │   │
│  │  ┌──────────────┐  ┌──────────────────────────────────────┐     │   │
│  │  │ Motivation   │  │     Cognition Engine                  │     │   │
│  │  │ (Drives/Goals│  │ (Perceive/Think/Plan/Decide/Learn)   │     │   │
│  │  └──────────────┘  └──────────────────────────────────────┘     │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                    │                                    │
│                                    ▼                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                      MEMORY LAYER                               │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │   │
│  │  │  Episodic    │  │   Semantic   │  │  Procedural  │          │   │
│  │  │  (Events)    │  │  (Concepts)  │  │  (Skills)    │          │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                    │                                    │
│                                    ▼                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                    KNOWLEDGE LAYER                              │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │   │
│  │  │ Knowledge    │  │  Reasoning   │  │    Logic     │          │   │
│  │  │   Graph      │  │   Engine     │  │   (Datalog)  │          │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘          │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │   │
│  │  │   Causal     │  │ Common Sense │  │    Formal    │          │   │
│  │  │ (Pearl's Do) │  │  (Physical) │  │  Verification│          │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                    │                                    │
│                                    ▼                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                   PERCEPTION LAYER                              │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │   │
│  │  │   Tokenizer  │  │   Entity     │  │    Intent    │          │   │
│  │  │              │  │   Extractor  │  │  Classifier  │          │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘          │   │
│  │  ┌──────────────┐  ┌──────────────┐                             │   │
│  │  │   Sentiment  │  │   Relation   │                             │   │
│  │  │   Analyzer   │  │   Extractor  │                             │   │
│  │  └──────────────┘  └──────────────┘                             │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                    │                                    │
│                                    ▼                                    │
│  ┌─────────────────────────────────────────────────────────────────┐   │
│  │                   PLANNING LAYER                               │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐          │   │
│  │  │     HTN      │  │     MCTS     │  │ Counterfactual│          │   │
│  │  │  (Hierarch.) │  │ (Stochastic) │  │  (What-if)   │          │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘          │   │
│  └─────────────────────────────────────────────────────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

## Module Descriptions

### 1. Cognition Layer (`src/cognition/`)

The cognition layer implements Tariro's cognitive architecture:

#### Consciousness (`consciousness.rs`)
- **Meta-cognitive awareness**: Monitors its own thought processes
- **Attention focus**: Maintains a stream of consciousness
- **Self-model**: Maintains beliefs about its own capabilities
- **Insight generation**: Detects patterns across thoughts

#### Working Memory (`working_memory.rs`)
- **Limited capacity**: Implements Miller's law (7±2 items)
- **Activation-based**: Items decay based on recency and attention
- **Chunking**: Groups related items into chunks
- **Binding**: Creates temporary associations

#### World Model (`world_model.rs`)
- **Internal universe**: Represents entities and their relationships
- **Causal graphs**: Models cause-effect relationships
- **Theory of Mind**: Understands other agents' mental states
- **Imagination**: Simulates hypothetical scenarios

#### Motivation (`motivation.rs`)
- **Drive system**: Primary (survival) and secondary (curiosity) drives
- **Goal hierarchy**: Organizes goals by priority and dependency
- **Emotion system**: Tracks emotional states
- **Value system**: Maintains ethical principles

#### Cognition Engine (`cognition_engine.rs`)
Unifies all cognitive components:
- `perceive()`: Process sensory input
- `think()`: Generate thoughts and reasoning
- `plan()`: Create action sequences
- `decide()`: Make choices
- `learn()`: Acquire new knowledge
- `reflect()`: Examine own processes

### 2. Memory Layer (`src/memory/`)

The memory layer implements the dual-store memory model:

#### Episodic Memory (`episodic.rs`)
- Stores autobiographical experiences
- Indexed by time, location, entities, and event type
- Emotional tagging for importance
- Consolidation to semantic memory

#### Semantic Memory (`semantic.rs`)
- Stores abstract knowledge and concepts
- Hierarchical categorization
- Association networks
- Common sense knowledge base

#### Procedural Memory (`procedural.rs`)
- Stores learned skills and habits
- Tracks success rates
- Automation level (conscious → automatic)

### 3. Knowledge Layer (`src/knowledge/`)

The knowledge layer provides symbolic reasoning:

#### Knowledge Graph (`knowledge_graph.rs`)
- **First-class entities**: Explicit knowledge representation
- **Predicates**: IsA, Causes, Before, After, HasProperty, etc.
- **Confidence scoring**: Probabilistic knowledge
- **Path finding**: Transitive closure, causal chains

#### Reasoning Engine (`reasoning.rs`)
- **Pattern matching**: Unification-based inference
- **Rule-based deduction**: Forward/backward chaining
- **Inference rules**: Modus ponens, resolution, etc.

#### Logic Engine (`logic.rs`)
- **Datalog/Prolog**: Logic programming
- **SLD resolution**: Backtracking inference
- **Occurs check**: Prevents infinite loops

#### Causal Reasoning (`causal.rs`)
- **Pearl's do-calculus**: Interventional reasoning
- **Structural causal models**: DAG representations
- **Counterfactuals**: "What if" reasoning

#### Common Sense (`commonsense.rs`)
- Physical rules (fire→heat, glass→break)
- Social rules (greetings, reciprocity)
- Mental rules (hunger→seek food)

#### Formal Verification (`formal.rs`)
- Type system definitions
- Invariants and constraints
- Theorem proving

### 4. Perception Layer (`src/perception/`)

Transforms raw input into structured representations:

#### Tokenizer
- Splits text into tokens
- Identifies word types (word, number, punctuation)

#### Entity Extractor
- Recognizes named entities (person, location, organization)
- Identifies concepts and actions

#### Relation Extractor
- Extracts subject-predicate-object triples
- Identifies causal and temporal relations

#### Intent Classifier
- Classifies speech acts (question, command, greeting)
- Confidence scoring

#### Sentiment Analyzer
- Valence, arousal, dominance
- Emotion detection

### 5. Planning Layer (`src/planning/`)

Provides executive function for goal-directed behavior:

#### HTN Planner (Hierarchical Task Network)
- Decomposes complex tasks into subtasks
- Uses methods for task decomposition
- Handles both primitive and compound tasks

#### MCTS Planner (Monte Carlo Tree Search)
- Stochastic planning under uncertainty
- UCB1 exploration/exploitation
- Rollout-based evaluation

#### Counterfactual Reasoner
- "What if" reasoning
- "Why" explanation generation
- Causal attribution

## Key Design Principles

### 1. Symbolic AI, Not Token Prediction
Tariro uses explicit symbolic representations:
- Entities have names and types
- Relations are typed predicates
- Reasoning follows logical rules
- Knowledge is first-class, not emergent

### 2. Sovereign Architecture
- No external API dependencies
- All processing is self-contained
- Can run on any platform
- No network requirements

### 3. Cognitive realism
- Miller's law for working memory
- Dual-process theory for cognition
- Drive-based motivation
- Emotional tagging for memory

### 4. Verification
- Formal specifications
- Type-safe Rust
- Invariant checking
- Theorem proving integration

## Usage Example

```rust
use tariro::{
    cognition::CognitionEngine,
    memory::MemorySystem,
    knowledge::TariroMind,
    perception::PerceptionEngine,
    planning::HTNPlanner,
};

// Create components
let mut cognition = CognitionEngine::new();
let mut memory = MemorySystem::new();
let mut knowledge = TariroMind::new();
let mut perception = PerceptionEngine::new();

// Initialize
memory.initialize();
knowledge.initialize();

// Process input
let input = "Fire produces heat";
let perception = perception.process_text(input.to_string());

// Perceive and think
cognition.perceive(perception.tokens.join(" "));
cognition.think("What are the implications of fire?");

// Learn
knowledge.learn_fact(/* fire → heat fact */);

// Plan
let plan = planner.plan(&state, tasks);
```

## File Structure

```
src/
├── cognition/
│   ├── mod.rs
│   ├── consciousness.rs
│   ├── working_memory.rs
│   ├── world_model.rs
│   ├── motivation.rs
│   ├── cognition_engine.rs
│   └── tests.rs
├── knowledge/
│   ├── mod.rs
│   ├── knowledge_graph.rs
│   ├── reasoning.rs
│   ├── logic.rs
│   ├── causal.rs
│   ├── commonsense.rs
│   ├── formal.rs
│   ├── mind.rs
│   └── tests.rs
├── memory/
│   ├── mod.rs
│   ├── episodic.rs
│   ├── semantic.rs
│   ├── procedural.rs
│   └── tests.rs
├── perception/
│   ├── mod.rs
│   ├── tokenizer.rs
│   ├── entity_extractor.rs
│   ├── intent_classifier.rs
│   ├── sentiment_analyzer.rs
│   ├── relation_extractor.rs
│   └── tests.rs
└── planning/
    ├── mod.rs
    ├── htn_planner.rs
    ├── mcts_planner.rs
    ├── counterfactual.rs
    ├── world_state.rs
    └── tests.rs
```

## Version History

- **1.0.12**: Current version with full cognitive architecture
- Added memory layer (episodic, semantic, procedural)
- Added perception layer (tokenization, entity extraction, intent, sentiment)
- Added planning layer (HTN, MCTS, counterfactual)
- Full test coverage for all modules
