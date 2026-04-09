//! ATVE Engine: Orchestrates various test and validation methodologies.

use super::formal_verification::{self, FormalVerificationEngine, VerificationProof, VerificationStatus};
use super::generative_testing;
use super::performance_benchmarking;
use super::resilience_testing;
use super::security_auditing;
use super::compatibility_assurance;

#[derive(Debug, Clone)]
pub struct TestReport {
    pub entity_name: String,
    pub entity_type: String,
    pub formal_verification_results: Vec<VerificationProof>, // Changed to Vec<VerificationProof>
    pub generative_testing_result: String,
    pub performance_benchmarking_result: String,
    pub resilience_testing_result: String,
    pub security_auditing_result: String,
    pub compatibility_assurance_result: String,
    pub overall_status: TestStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestStatus {
    Pass,
    Fail,
    Warning,
    OptimizationRequired,
}

pub struct AtveEngine {
    formal_verification_engine: FormalVerificationEngine, // New: Formal Verification Engine
}

impl AtveEngine {
    pub fn new() -> Self {
        AtveEngine {
            formal_verification_engine: FormalVerificationEngine::new(), // New: Initialize
        }
    }

    /// Orchestrates a comprehensive self-test and validation process for TUMC and generated artifacts.
    pub fn run_comprehensive_test(&self, target_entity: &str, entity_type: &str) -> String {
        println!("ATVE: Initiating comprehensive test for {} (Type: {}).", target_entity, entity_type);
        
        // Define some properties to verify for demonstration
        let common_properties = &["safety_property_1", "liveness_property_A", "absence_of_deadlock"];

        let fv_results = self.formal_verification_engine.run_verification(target_entity, common_properties);

        let report = TestReport {
            entity_name: target_entity.to_string(),
            entity_type: entity_type.to_string(),
            formal_verification_results: fv_results.clone(), // Clone if needed, or adjust return type of run_verification
            generative_testing_result: generative_testing::run_fuzzing(target_entity),
            performance_benchmarking_result: performance_benchmarking::run_benchmarks(target_entity, entity_type),
            resilience_testing_result: resilience_testing::run_resilience_tests(target_entity),
            security_auditing_result: security_auditing::run_security_audit(target_entity),
            compatibility_assurance_result: compatibility_assurance::run_compatibility_checks(target_entity),
            overall_status: if fv_results.iter().any(|p| p.status == VerificationStatus::Refuted) {
                TestStatus::Fail
            } else {
                TestStatus::Pass // Simplified for now
            },
        };

        let summary = format!(
            "Comprehensive test complete for {}. Status: {:?}. Details: [FV: {:?}, GT: {}, PB: {}, RT: {}, SA: {}, CA: {}]",
            report.entity_name, report.overall_status,
            report.formal_verification_results, report.generative_testing_result,
            report.performance_benchmarking_result, report.resilience_testing_result,
            report.security_auditing_result, report.compatibility_assurance_result
        );

        println!("ATVE: Report summary: {}.", summary);
        summary
    }
}
