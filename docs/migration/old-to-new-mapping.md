# Migration mapping

This maps the modules that exist in the tree today onto the crate
boundaries of the target workspace. It is generated from the actual
source tree, so the file counts are real.

## Crate boundaries

| Crate | Owns | Files | Lines |
|---|---|---:|---:|
| `tariro-compiler` | Universal meta-compiler (TUMC): frontends, IR, optimisation, codegen, tooling | 104 | 66,741 |
| `tariro-ir` | Compiler intermediate representation and its dialects | 0 | 0 |
| `tariro-runtime` | Execution runtime, VM and interpreter | 0 | 0 |
| `tariro-knowledge` | Knowledge representation and retrieval | 11 | 7,356 |
| `tariro-memory` | Episodic, semantic, procedural and working memory | 53 | 7,964 |
| `tariro-cognition` | Attention, metacognition, agency, social and embodied cognition | 8 | 6,856 |
| `tariro-reasoning` | Symbolic, causal, probabilistic and counterfactual reasoning | 7 | 4,563 |
| `tariro-planning` | Goal decomposition, HTN and MCTS planning, scheduling | 3 | 2,530 |
| `tariro-learning` | Continual, meta, reinforcement, causal and self-supervised learning | 6 | 3,598 |
| `tariro-agents` | Agent identity, goals, tools, permissions and swarm coordination | 2 | 738 |
| `tariro-tools` | Tool registry, invocation, permissions and sandboxing | 24 | 4,815 |
| `tariro-security` | Cryptography, zero trust, trusted execution and adversarial defence | 6 | 5,368 |
| `tariro-safety` | Value alignment, interruptibility and shutdown | 5 | 3,083 |
| `tariro-governance` | Autonomy policies, approvals and accountability | 4 | 834 |
| `tariro-verification` | Formal verification, property testing and model checking | 2 | 816 |
| `tariro-distributed` | Cluster, scheduling, federation and edge sync | 16 | 7,942 |
| `tariro-storage` | Database service, repositories and migrations | 2 | 547 |
| `tariro-inference` | Model loading, local inference and quantisation | 2 | 1,506 |
| `tariro-creativity` | Creative generation and authorship | 11 | 3,083 |
| `tariro-perception` | Multimodal perception pipeline | 2 | 3,861 |
| `tariro-config` | Configuration loading and validation | 1 | 267 |
| `tariro-telemetry` | Logging, metrics and tracing | 1 | 209 |
| `tariro-events` | Event bus and domain events | 1 | 225 |
| `tariro-core` | Shared identity, types and cross-cutting contracts | 1 | 202 |
| `tariro-application` | The unified application layer | 10 | 3,825 |

**Assigned:** 282 files, 136,929 lines  
**Not yet placed:** 2,786 files, 201,797 lines

## Not yet placed

These modules have no home in the table above yet. They are either
leaf features that belong with a crate above, or genuinely new
capabilities. They need an ownership decision before the split.

| Area | Files | Lines |
|---|---:|---:|
| `cognition` | 7 | 4,749 |
| `autonomous_business_management` | 8 | 3,945 |
| `admin_omniversal_control` | 110 | 3,843 |
| `ip_management` | 30 | 3,773 |
| `threat_intelligence_ascension` | 6 | 3,407 |
| `omniversal_command_cognition_hub` | 300 | 3,012 |
| `news_engine` | 18 | 2,348 |
| `threat_intel` | 10 | 2,232 |
| `security` | 10 | 2,056 |
| `self_repair` | 9 | 2,021 |
| `core` | 10 | 1,869 |
| `meta_projects` | 7 | 1,815 |
| `deep_language_understanding.rs` | 1 | 1,779 |
| `evas` | 8 | 1,628 |
| `analytics` | 9 | 1,570 |
| `omni_domain_operation` | 19 | 1,565 |
| `content_generation` | 5 | 1,529 |
| `author_module` | 102 | 1,503 |
| `explainability` | 8 | 1,483 |
| `universal_problem_solving` | 20 | 1,466 |
| `sacc_asi` | 20 | 1,435 |
| `planetary_cosmic_sovereignty` | 20 | 1,397 |
| `nano_technological_omnipresence` | 20 | 1,384 |
| `biological_neural_symbiosis` | 20 | 1,367 |
| `emotional_architecture.rs` | 1 | 1,349 |
| `sdk` | 5 | 1,336 |
| `omniversal_control_ultimate` | 115 | 1,265 |
| `meta_cognition.rs` | 1 | 1,255 |
| `orchestration` | 9 | 1,237 |
| `finance` | 100 | 1,229 |

## How to use this

1. **Finish making the crate compile first.** Every crate boundary below is a
   public API change. Splitting a crate that does not build means there is no
   compiler to tell you whether the split preserved behaviour.
2. **Move one crate at a time, bottom-up.** `tariro-core`, `tariro-types`,
   `tariro-errors` and `tariro-config` first, because everything else depends
   on them. Each move keeps the tree green and leaves the public crate
   re-exporting from the old path so callers do not break.
3. **Resolve the duplicate clusters explicitly.** The inventory shows several
   capabilities implemented more than once — episodic and semantic memory each
   exist as a flat module *and* under `src/memory/`; the same pattern appears
   for learning, quantum security and the scaling engines. Pick one owner per
   capability and delete the other only after its replacement is verified.
4. **Do not move a directory that is not self-contained.** `src/tumc_compiler/`
   is the largest and the most coherent candidate: it has its own `ast`, `ir`,
   `runtime` and `target` subtrees and is the natural first `tariro-compiler`
   crate.

## Known duplicate capability clusters

| Capability | Competing implementations |
|---|---|
| Memory | `src/episodic_memory.rs`, `src/semantic_memory.rs`, `src/working_memory.rs`, `src/memory/`, `src/infinity_memory/`, `src/infinite_learning_memory/` |
| Learning | `src/cognitive_learning/`, `src/continuous_learning.rs`, `src/self_supervised_learning.rs`, `src/causal_learning.rs` |
| Quantum security | `src/quantum_security.rs` and `src/quantum_security/` |
| Scaling | `src/scaling/`, `src/federated*`, `src/edge_sync*` |
| Knowledge | `src/knowledge/`, `src/ontology*`, `src/world_model.rs` |
