# Tariro target tree and restructure phases

**Status:** agreed plan, not yet executed. No file has been moved.

**Decisions locked for this plan**

| # | Decision |
|---|---|
| 1 | **Module style: `mod.rs` everywhere.** A directory with children is `foo/mod.rs` + `foo/*.rs`; a leaf capability is `foo.rs`. No mixing of the two styles. |
| 2 | **Move 2 is a fine split: 12–14 crates** (plus `tariro_sdk`), one crate per capability cluster. |
| 3 | **`tariro_sdk` stays a separate published crate.** Its entities are the contract boundary the split grows from. |

## 1. Two moves, kept apart

| | **Move 1 — regroup inside the crate** | **Move 2 — split into workspace crates** |
|---|---|---|
| Public API | **unchanged** | breaks (`crate::x::y` → `tariro_x::y`) |
| Shape of the diff | mechanical `git mv` | moves + API migration + workspace wiring |
| Payoff | navigability, one owner per concept | build time, build memory, parallel CI shards |
| When | now, phase by phase | after Move 1 has settled the seams |

Move 1 is cheap because Rust 2018+ resolves `src/foo.rs` together with `src/foo/bar.rs`
as `crate::foo::bar` — exactly today's path. So `src/knowledge/mind.rs` →
`src/knowledge/mind/mod.rs` is path-preserving, and `src/world_model.rs` →
`src/world_model/mod.rs` is path-preserving. No shims, no semver break.

Move 2 is where the build-memory ceiling is actually solved: a single 3,000-file
crate no longer has to type-check in one rustc process, and CI shards by crate
instead of one unit.

## 2. The tree as it stands

Measured on `main` at `90e2c878`:

| | count |
|---|---|
| Loose files in `src/` | **479** (148,636 lines) |
| Directories in `src/` | **110** (198,204 lines) |
| Module declarations in `src/lib.rs` | **114** |

The loose files are not a random pile — they are dominated by one generated
family:

| family | files |
|---|---|
| `*_engine.rs` | **274** |
| `*_learning.rs` | 8 |
| `*_manager.rs` | 6 |
| everything else | 181 |

The largest existing directories: `tumc_compiler/` (67,913 lines / 103 files),
`scaling/` (6,873 / 8), `knowledge/` (6,433 / 9), `cognition/` (4,826 / 7),
`autonomous_business_management/` (3,945 / 8), `admin_omniversal_control/`
(3,907 / 110), `ip_management/` (3,767 / 30), `modules/` (3,446 / 17),
`threat_intelligence_ascension/` (3,407 / 6),
`omniversal_command_cognition_hub/` (3,012 / **300**), `unified/` (2,932 / 8).

Two things fall out of this measurement:

- **274 `*_engine.rs` files at the crate root** is the single biggest navigation
  problem. They belong in one directory, not at the root.
- `news_engine/` (18 files) is **not declared in `src/lib.rs`** — it is dead
  weight today. Either wire it in (it then earns a home) or it stays out of the
  plan until it is finished.

## 3. Target tree

```
src/
  core/            primitives, errors, config, prelude, utils, logging
  compiler/
    ast/           source AST, literals, spans
    ir/            SSA IR, dialects, CFG
    types/         type system, inference, checker
    frontend/      lexer, parser, frontends (rust, python, ts, go)
    backend/       codegen, wasm / native / llvm / dotnet / vm targets
    optimize/      optimizer, passes, vectorizer
    tooling/       lsp, profiler, testing, docs
  runtime/         interpreter, vm, gc/, scheduler, memory
  cognition/
    memory/        working, episodic, semantic
    world_model/
    attention/     attention, workspace
    motivation/    motivation, intrinsic
    meta/          metacognition, meta_cognition, meta_learning
  knowledge/       knowledge_graph, reasoning, logic, causal, commonsense, formal
  planning/        htn, mcts, executive, search
  agency/          agency, autonomy/, governance, policy
  learning/        cognitive_learning, self_supervised, continual, self_correction
  scaling/         memory/, federated, resource_loader, consensus, capability
  io/              database, serialization, telemetry, persistence
  interfaces/      lsp, cli, admin/, api
  engines/         the 274 `*_engine.rs` files, one directory
```

Two levels by default; three only where a cluster is genuinely large
(`compiler/*`, `cognition/*`). A flat "god directory" is the failure mode to
avoid — if `engines/` grows past ~300 files again, it splits by family.

## 4. Placement rules

1. **One directory = one capability, one file = one concern.** No `misc`, no
   `core2`, no `utils_v2`.
2. **`mod.rs` everywhere** (decision 1). `foo/mod.rs` for a directory with
   children, `foo.rs` for a leaf capability — never both styles for the same
   directory.
3. **One owner per concept.** Where two modules implement the same concept, one
   becomes the implementation and the other a `pub use` re-export at the old path
   (path preserved, no duplication).
4. **Feature-gated modules live beside their feature**, not in a pile.
5. **`tests/` mirrors `src/`**, so a failing test points at its owner.
6. **No path renames in Move 1.** If a move needs a path change, it belongs to
   Move 2.

## 5. Duplicates that die in Phase 1

| Concept | Current locations | Resolution |
|---|---|---|
| World model | `src/world_model.rs`, `src/cognition/world_model.rs` | one owner under `cognition/world_model/`; the other re-exports |
| Unified memory | `src/unified_memory.rs`, `src/unified/unified_memory.rs` | one owner under `unified/`; the other re-exports |
| Engine registry | `src/unified/engine_registry.rs`, `src/unified/tariro_integration/engine_registry.rs` | **done** — the duplicate now re-exports the canonical one |
| Memory | `src/memory/`, `src/working_memory.rs`, `src/episodic_memory.rs`, `src/semantic_memory.rs` | `cognition/memory/` becomes the owner; the three loose files move under it |
| Self-modification | `src/self_modification_module.rs` | deleted upstream (`088eb0db`) |

## 6. Phases

| Phase | Work | Gate |
|---|---|---|
| **0** | Fix the red test build (missing imports in the `#[cfg(test)]` modules); land a test that asserts the current public module paths | `cargo test --lib` green, path test green |
| **1** | Kill the duplicate modules of §5 | check + path test |
| **2** | Move the 274 `*_engine.rs` files into `engines/`, then give every remaining loose `src/*.rs` its own directory (path-preserving) | check + path test per batch |
| **3** | Cluster by capability into the target tree of §3 | check + path test per batch |
| **4** | Nest the oversized clusters (`compiler/*`, `cognition/*`, `interfaces/admin/`) | check + path test per batch |
| **5** | *Separate effort:* split into the workspace crates of §7 | workspace build + publish dry-run |

Phase 0 is the gate for everything else. CI runs `cargo test --all-features`, so
that build has to be green before a restructure starts, otherwise a regression
cannot be told from the existing noise.

## 7. Move 2: the crate split (12–14 crates)

```
tariro_sdk            exists, unchanged — the contract boundary
tariro-core           primitives, errors, config, prelude, utils, logging
tariro-compiler-ast    source AST, literals, spans
tariro-compiler-ir     SSA IR, dialects, CFG
tariro-compiler-types  type system, inference, checker
tariro-compiler-front  lexer, parser, rust/python/ts/go frontends
tariro-compiler-back   codegen, wasm / native / llvm / dotnet / vm targets
tariro-compiler-opt    optimizer, passes, vectorizer
tariro-runtime        interpreter, VM, GC, scheduler, memory
tariro-cognition      consciousness, memory, attention, motivation, world model
tariro-knowledge      knowledge graph, reasoning, logic, causal, commonsense
tariro-planning       htn, mcts, executive, search
tariro-agency         agency, autonomy, governance, policy
tariro-learning       cognitive learning, self-supervised, continual
tariro-scaling        tiered memory, federated, resource loader, consensus
tariro-io             database, serialization, telemetry, persistence
tariro-interfaces     lsp, cli, admin, api
```

That is 15 with the SDK. The one honest fork in the road: the six
`tariro-compiler-*` crates may be better as a single `tariro-compiler`, because
`ast`, `ir`, `types` and `backend` reference each other constantly. The target
is *one crate per capability*, and the compiler is the place where a coarse
split could win. Decide that with compile timings in hand, not on paper.

Each crate gets its own `Cargo.toml`, its own feature flags, and its own CI
shard. The workspace keeps `tariro` as a facade crate that re-exports the
public API of the others, so downstream `use tariro::…` keeps working.

## 8. The path-test guardrail

The thing that makes Move 1 safe is a small test that pins the public module
paths. It fails loudly if a move silently changes one:

```rust
// tests/public_paths.rs
#[test]
fn public_module_paths_are_stable() {
    // The compiler front ends.
    let _ = tariro::tumc_compiler::frontend::Program::default();
    // The knowledge layer.
    let _ = tariro::knowledge::knowledge_graph::KnowledgeGraph::new();
    // The cognition layer.
    let _ = tariro::cognition::world_model::WorldModel::new();
}
```

It is deliberately small: one anchor per cluster is enough to catch a broken
move, and it is cheap to extend as each phase lands.

## 9. Regenerating the inventory

The file-level map is generated, never hand-maintained:

```bash
# every module in the tree
find src -name '*.rs' | sed 's|^src/||' | sort

# the loose files that Phase 2 has to place
find src -maxdepth 1 -name '*.rs' | sed 's|^src/||' | sort

# the duplicate-module check
rg -l 'pub (struct|enum|trait) (\w+)' src --no-heading | sed 's|.*pub ||' | sort | uniq -d
```

The last one is the check that keeps §5 honest: it lists every type name that
is declared in more than one place, which is exactly the duplicate set.

## 10. Risks worth naming

- **The diff is huge even though it is mechanical.** 479 loose files moving is
  reviewable only as `git mv` batches with a path test between them. Never one
  commit with 479 renames and logic changes mixed in.
- **`mod.rs` everywhere** makes every `mod` declaration change shape
  (`mod x;` → `mod x;` stays, but `x.rs` becomes `x/mod.rs` ✓) — mechanical ✓.
- **The `*_engine.rs` family is generated-looking.** If a generator produces
  them, fix the generator to emit into `engines/` rather than moving 274 files
  by hand and having the next generation scatter them again.
- **`news_engine/` is not compiled.** Whatever home it gets, it must be wired
  into `lib.rs` in the same phase, or it stays invisible.
- **Move 2 breaks the public API.** `crate::x::y` paths move. A facade crate
  re-exporting the old paths is the mitigation; without it this is a semver
  major.
