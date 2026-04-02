---

## **3. High-Level Architectural Overview**

Tariro's architecture is modular and layered, designed for scalability, continuous integration, and dynamic evolution.

*   **Core AGI Layer:** Houses the fundamental reasoning modules (Causal, Analogical, Abductive, Counterfactual, Probabilistic, Deductive/Inductive, Common Sense, Fuzzy Logic) and meta-cognitive functions (Self-Modeling, Metacognitive Monitoring, Cognitive Bias Detection).
*   **Governance & Control Layer:** Incorporates H.O.I.P., E.V.A.S., `AutonomyPolicy`, and `AGIControl` settings.
*   **Memory & Data Layer:** Manages persistent entities (`KnowledgeItem`, `LearningInsight`, `ManagedFile`, `Conversation`, `PerformanceLog`, `ExplainabilityLog`, `FeatureProposal`, `WorkflowRule`, `WorkflowLog`, `SystemHealthLog`) and dynamic memory.
*   **Tooling & Interface Layer:** Provides access to external tools (`web_search`) and internal APIs for interacting with memory and managing system state. Includes communication modules for user interaction.
*   **Self-Improvement & Development Layer:** Encompasses modules for `FeatureProposal` generation, **Sandboxed Environment Module (NEW)**, recursive self-optimization, and automated testing.