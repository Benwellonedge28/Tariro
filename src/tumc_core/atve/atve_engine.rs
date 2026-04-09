//! ATVE Engine: Orchestrates various test and validation methodologies.

use super::formal_verification;
use super::generative_testing;
use super::performance_benchmarking;
use super::resilience_testing;
use super::security_auditing;
use super::compatibility_assurance;

pub struct AtveEngine;

impl AtveEngine {
    pub fn new() -> Self {
        AtveEngine {}
    }

    /// Orchestrates a comprehensive self-test and validation process for TUMC and generated artifacts.
    pub fn run_comprehensive_test(&self, target_entity: &str, entity_type: &str) -> String {
        println!("ATVE: Initiating comprehensive test for {} (Type: {}).", target_entity, entity_type);
        let mut report_elements = Vec::new();

        report_elements.push(format!("Formal Verification: {}", formal_verification::run_verification(target_entity)));
        report_elements.push(format!("Generative Testing: {}", generative_testing::run_fuzzing(target_entity)));
        report_elements.push(format!("Performance Benchmarking: {}", performance_benchmarking::run_benchmarks(target_entity, entity_type)));
        report_elements.push(format!("Resilience Testing: {}", resilience_testing::run_resilience_tests(target_entity)));
        report_elements.push(format!("Security Auditing: {}", security_auditing::run_security_audit(target_entity)));
        report_elements.push(format!("Compatibility Assurance: {}", compatibility_assurance::run_compatibility_checks(target_entity)));

        let final_report = report_elements.join(" | ");
        println!("ATVE: Comprehensive test complete. Report: {}.", final_report);
        final_report
    }
}
