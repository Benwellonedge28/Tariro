//! ATVE Engine: Orchestrates various test and validation methodologies.

use super::formal_verification::{self, FormalVerificationEngine, VerificationProof, VerificationStatus};
use super::generative_testing::{self, GenerativeTestingEngine, FuzzingResult, FuzzingStatus};
use super::performance_benchmarking::{self, PerformanceBenchmarkingEngine, PerformanceMetrics};
use super::resilience_testing::{self, ResilienceTestingEngine, ResilienceReport, ResilienceTestStatus};
use super::security_auditing;
use super::compatibility_assurance;

#[derive(Debug, Clone)]
pub struct TestReport {
    pub entity_name: String,
    pub entity_type: String,
    pub formal_verification_results: Vec<VerificationProof>,
    pub generative_testing_results: FuzzingResult,
    pub performance_benchmarking_results: PerformanceMetrics,
    pub resilience_testing_results: ResilienceReport,
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
    formal_verification_engine: FormalVerificationEngine,
    generative_testing_engine: GenerativeTestingEngine,
    performance_benchmarking_engine: PerformanceBenchmarkingEngine,
    resilience_testing_engine: ResilienceTestingEngine,
}

impl AtveEngine {
    pub fn new() -> Self {
        AtveEngine {
            formal_verification_engine: FormalVerificationEngine::new(),
            generative_testing_engine: GenerativeTestingEngine::new(),
            performance_benchmarking_engine: PerformanceBenchmarkingEngine::new(),
            resilience_testing_engine: ResilienceTestingEngine::new(),
        }
    }

    /// Orchestrates a comprehensive self-test and validation process for TUMC and generated artifacts.
    pub fn run_comprehensive_test(&self, target_entity: &str, entity_type: &str) -> String {
        println!("ATVE: Initiating comprehensive test for {} (Type: {}).", target_entity, entity_type);
        
        let common_properties = &["safety_property_1", "liveness_property_A", "absence_of_deadlock"];

        let fv_results = self.formal_verification_engine.run_verification(target_entity, common_properties);
        let gt_results = self.generative_testing_engine.run_fuzzing(target_entity);
        let pb_results = self.performance_benchmarking_engine.run_benchmarks(target_entity, entity_type);
        let rt_results = self.resilience_testing_engine.run_resilience_tests(target_entity);

        let report = TestReport {
            entity_name: target_entity.to_string(),
            entity_type: entity_type.to_string(),
            formal_verification_results: fv_results.clone(),
            generative_testing_results: gt_results.clone(),
            performance_benchmarking_results: pb_results.clone(),
            resilience_testing_results: rt_results.clone(),
            security_auditing_result: security_auditing::run_security_audit(target_entity),
            compatibility_assurance_result: compatibility_assurance::run_compatibility_checks(target_entity),
            overall_status: if fv_results.iter().any(|p| p.status == VerificationStatus::Refuted) || gt_results.status != FuzzingStatus::CompletedWithoutIssues || pb_results.efficiency_score < 99.0 || rt_results.status != ResilienceTestStatus::SelfHealed {
                TestStatus::Fail
            } else {
                TestStatus::Pass
            },
        };

        let summary = format!(
            "Comprehensive test complete for {}. Status: {:?}. Details: [FV: {:?}, GT: {:?}, PB: {:?}, RT: {:?}, SA: {}, CA: {}]",
            report.entity_name, report.overall_status,
            report.formal_verification_results, report.generative_testing_results.status,
            report.performance_benchmarking_results,
            report.resilience_testing_results.status,
            report.security_auditing_result, report.compatibility_assurance_result
        );

        println!("ATVE: Report summary: {}.", summary);
        summary
    }
}
