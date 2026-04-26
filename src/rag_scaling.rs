/// Module for Scalable RAG Indexing and Retrieval in Rust.
///
/// This module provides conceptual structures and functions for building
/// and querying a scalable RAG index, focusing on efficient storage and retrieval.

use std::collections::{HashMap, HashSet};
use parking_lot::RwLock;
use uuid::Uuid;

/// Represents a document stored in the RAG index.
#[derive(Debug, Clone)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
}

/// A simple inverted index for keyword search.
pub type InvertedIndex = HashMap<String, HashSet<String>>;

/// Manages the RAG index.
pub struct RAGIndexer {
    documents: RwLock<HashMap<String, Document>>,
    inverted_index: RwLock<InvertedIndex>,
}

impl RAGIndexer {
    pub fn new() -> Self {
        RAGIndexer {
            documents: RwLock::new(HashMap::new()),
            inverted_index: RwLock::new(HashMap::new()),
        }
    }

    /// Adds a document to the index.
    pub fn add_document(&self, content: String, metadata: HashMap<String, String>) -> Document {
        let id = Uuid::new_v4().to_string();
        let doc = Document { id: id.clone(), content: content.clone(), metadata };

        let mut documents = self.documents.write();
        documents.insert(id.clone(), doc.clone());

        // Update inverted index (simple tokenization)
        let mut inverted_index = self.inverted_index.write();
        for word in content.to_lowercase().split_whitespace() {
            if word.len() > 2 { // Ignore very short words
                inverted_index.entry(word.to_string()).or_default().insert(id.clone());
            }
        }
        doc
    }

    /// Builds or rebuilds the index (conceptual).
    /// In a real system, this could involve more complex vector embeddings and storage.
    pub fn build_index(&self) {
        println!("RAGIndexer: Building/updating index...");
        // In a real scenario, this might trigger an embedding process,
        // write to disk, or optimize data structures.
        println!("RAGIndexer: Index build complete.");
    }
}

/// Retrieves documents from the RAG index.
pub struct RAGRetriever<'a> {
    indexer: &'a RAGIndexer,
}

impl<'a> RAGRetriever<'a> {
    pub fn new(indexer: &'a RAGIndexer) -> Self {
        RAGRetriever { indexer }
    }

    /// Searches the index for relevant documents based on the query.
    /// Returns document IDs and a conceptual relevance score.
    pub fn search(&self, query: &str, top_k: usize) -> Vec<(String, f32)> {
        let query_words: HashSet<String> = query.to_lowercase().split_whitespace().map(String::from).collect();
        let inverted_index = self.indexer.inverted_index.read();
        let documents = self.indexer.documents.read();

        let mut candidate_doc_ids = HashSet::new();
        for word in &query_words {
            if let Some(doc_ids) = inverted_index.get(word) {
                candidate_doc_ids.extend(doc_ids.iter().cloned());
            }
        }

        let mut scored_results: Vec<(String, f32)> = Vec::new();
        for doc_id in candidate_doc_ids {
            if let Some(doc) = documents.get(&doc_id) {
                let mut score = 0.0;
                let doc_lower = doc.content.to_lowercase();
                for q_word in &query_words {
                    if doc_lower.contains(q_word) {
                        score += 1.0; // Simple match score
                    }
                }
                if score > 0.0 {
                    scored_results.push((doc_id.clone(), score));
                }
            }
        }

        scored_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        scored_results.into_iter().take(top_k).collect()
    }

    /// Retrieves the full Document objects based on their IDs.
    pub fn get_documents(&self, doc_ids: &[String]) -> Vec<Document> {
        let documents = self.indexer.documents.read();
        doc_ids.iter().filter_map(|id| documents.get(id).cloned()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_document_and_index() {
        let indexer = RAGIndexer::new();
        let doc = indexer.add_document("Hello world, this is a test document.".to_string(), HashMap::new());
        assert!(!indexer.documents.read().is_empty());
        assert!(!indexer.inverted_index.read().is_empty());
        assert!(indexer.inverted_index.read().get("world").unwrap().contains(&doc.id));
    }

    #[test]
    fn test_search_and_retrieve() {
        let indexer = RAGIndexer::new();
        indexer.add_document("Tariro AGI is an advanced AI system.".to_string(), HashMap::new());
        indexer.add_document("This document talks about Rust programming.".to_string(), HashMap::new());
        indexer.add_document("Tariro also uses Rust for its core.".to_string(), HashMap::new());

        let retriever = RAGRetriever::new(&indexer);
        let results = retriever.search("Tariro Rust", 2);

        assert_eq!(results.len(), 2);
        assert!(results.iter().any(|(id, _)| {
            let docs = indexer.documents.read();
            docs.get(id).unwrap().content.contains("Tariro AGI")
        }));
        assert!(results.iter().any(|(id, _)| {
            let docs = indexer.documents.read();
            docs.get(id).unwrap().content.contains("Tariro also uses Rust")
        }));
    }
}

fn main() {
    let indexer = RAGIndexer::new();

    // Add some conceptual documents
    println!("Adding documents...");
    indexer.add_document("Tariro AGI is an autonomous AI agent focused on continuous self-improvement.".to_string(), HashMap::from([("author".to_string(), "Samuel".to_string())]));
    indexer.add_document("RAG systems combine retrieval and generation for advanced NLP.".to_string(), HashMap::from([("topic".to_string(), "AI".to_string())]));
    indexer.add_document("SIMD instructions enable parallel data processing in modern CPUs.".to_string(), HashMap::from([("category".to_string(), "Hardware".to_string())]));
    indexer.add_document("The OMEGA OS aims for digital age sovereignty and child protection.".to_string(), HashMap::new());
    indexer.add_document("Rust is a programming language known for safety and performance.".to_string(), HashMap::new());
    indexer.add_document("Tariro's core is written in Rust.".to_string(), HashMap::new());
    indexer.add_document("Scalable architectures are essential for large-scale AI.".to_string(), HashMap::new());

    indexer.build_index();

    let retriever = RAGRetriever::new(&indexer);

    println!("\nSearching for 'Tariro AGI Rust':");
    let search_results = retriever.search("Tariro AGI Rust", 3);
    let document_ids: Vec<String> = search_results.iter().map(|(id, _)| id.clone()).collect();
    let documents = retriever.get_documents(&document_ids);

    for (i, (doc, score)) in documents.iter().zip(search_results.iter()).enumerate() {
        println!("  {}. [Score: {:.2}] (ID: {})", i + 1, score.1, doc.id);
        println!("     Content: {}", doc.content);
        if !doc.metadata.is_empty() {
            println!("     Metadata: {:?}", doc.metadata);
        }
        println!("--------------------------------------------------");
    }

    println!("\nSearching for 'Scalable AI systems':");
    let search_results_2 = retriever.search("Scalable AI systems", 2);
    let document_ids_2: Vec<String> = search_results_2.iter().map(|(id, _)| id.clone()).collect();
    let documents_2 = retriever.get_documents(&document_ids_2);

    for (i, (doc, score)) in documents_2.iter().zip(search_results_2.iter()).enumerate() {
        println!("  {}. [Score: {:.2}] (ID: {})", i + 1, score.1, doc.id);
        println!("     Content: {}", doc.content);
        println!("--------------------------------------------------");
    }
}
