/// Module for an Integrated RAG-SIMD Pipeline.
///
/// This module demonstrates how Retrieval-Augmented Generation (RAG)
/// and SIMD (Single Instruction, Multiple Data) processing can be combined
/// to enhance information retrieval and data processing tasks.

use crate::prelude::*;
mod rag_contextualization; // Assuming this file exists in the same directory
mod simd_processor;        // Assuming this file exists in the same directory

use rag_contextualization::{RagContextualizer, RagContextualizerConfig, RetrievedContext};
use simd_processor::SimdProcessor;

/// The Integrated RAG-SIMD Pipeline struct.
pub struct RagSimdPipeline {
    rag_contextualizer: RagContextualizer,
    simd_processor: SimdProcessor,
}

impl RagSimdPipeline {
    pub fn new() -> Self {
        let rag_config = RagContextualizerConfig::default();
        let rag_contextualizer = RagContextualizer::new(rag_config);
        let simd_processor = SimdProcessor::new();
        RagSimdPipeline {
            rag_contextualizer,
            simd_processor,
        }
    }

    /// Runs the integrated pipeline: first contextualizes using RAG, then
    /// conceptually processes some numerical aspect of the retrieved data using SIMD.
    pub fn run_pipeline(&self, query: &str) -> Vec<RetrievedContext> {
        println!("\n--- Running RAG-SIMD Pipeline for Query: '{}' ---", query);

        // Step 1: RAG Contextualization
        let mut contexts = self.rag_contextualizer.get_contextualized_information(query);
        println!("Pipeline Stage 1: RAG Contextualization completed. Retrieved {} contexts.", contexts.len());

        // Step 2: Conceptual SIMD Processing on context scores or other numerical features.
        // For this example, let's imagine we want to normalize or amplify scores using SIMD.
        // This is a placeholder for actual SIMD processing on features derived from contexts.

        if !contexts.is_empty() {
            let mut scores: Vec<f32> = contexts.iter().map(|c| c.score).collect();
            let multiplier = vec![1.2; scores.len()]; // Example: Amplify scores by 1.2

            // Ensure lengths are multiples of 4 for conceptual SIMD operation
            let padded_len = (scores.len() + 3) / 4 * 4;
            scores.resize(padded_len, 0.0);
            let mut padded_multiplier = vec![0.0; padded_len];
            padded_multiplier[..multiplier.len()].copy_from_slice(&multiplier);

            println!("Pipeline Stage 2: Applying conceptual SIMD processing to scores.");
            // Here, we're conceptually using SIMD for an operation on the scores
            // In a real scenario, this could be for feature vectors extracted from contexts
            let processed_scores_simd = self.simd_processor.process_data_f32(&scores, &padded_multiplier);

            // Update context scores with the SIMD-processed values (trimming padding)
            for (i, context) in contexts.iter_mut().enumerate() {
                if i < processed_scores_simd.len() {
                    context.score = processed_scores_simd[i];
                }
            }
            println!("Pipeline Stage 2: SIMD processing completed. Scores conceptually adjusted.");
        } else {
            println!("No contexts to apply SIMD processing on.");
        }

        // Re-sort after conceptual score adjustment
        contexts.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

        println!("--- Pipeline finished ---");
        contexts
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_rag_simd_pipeline_new() {
        let pipeline = RagSimdPipeline::new();
        assert!(!pipeline.rag_contextualizer.document_store.is_empty());
    }

    #[test]
    fn test_run_pipeline_with_query() {
        let pipeline = RagSimdPipeline::new();
        let query = "What is Tariro AGI?";
        let results = pipeline.run_pipeline(query);

        assert!(!results.is_empty());
        // Check if scores were conceptually adjusted (e.g., increased)
        for context in results.iter() {
            assert!(approx_eq!(f32, context.score, 0.8 * 1.2, epsilon=0.0001) || context.score > 0.8);
        }
        assert!(results.iter().any(|c| c.content.contains("Tariro AGI")));
    }

    #[test]
    fn test_run_pipeline_no_simd_if_no_contexts() {
        let pipeline = RagSimdPipeline::new();
        let query = "NonExistentTerm";
        let results = pipeline.run_pipeline(query);
        assert!(results.is_empty());
    }
}

fn main() {
    let pipeline = RagSimdPipeline::new();

    let query1 = "Tell me about Tariro AGI and its capabilities with RAG.";
    let results1 = pipeline.run_pipeline(query1);
    println!("\nFinal Pipeline Results for Query: '{}'", query1);
    for (i, context) in results1.iter().enumerate() {
        println!("  {}. [Adjusted Score: {:.2}] (ID: {}) {}\n     '{}'",
            i + 1,
            context.score,
            context.id,
            context.source,
            context.content.chars().take(100).collect::<String>()
        );
    }

    let query2 = "Explain how SIMD can be used to speed up data processing.";
    let results2 = pipeline.run_pipeline(query2);
    println!("\nFinal Pipeline Results for Query: '{}'", query2);
    for (i, context) in results2.iter().enumerate() {
        println!("  {}. [Adjusted Score: {:.2}] (ID: {}) {}\n     '{}'",
            i + 1,
            context.score,
            context.id,
            context.source,
            context.content.chars().take(100).collect::<String>()
        );
    }
}
