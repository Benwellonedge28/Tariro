// src/atve/report_analyzer.rs
use std::collections::HashMap;

pub struct AtveReport {
    pub timestamp: String,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub error_tests: usize,
    pub timeout_tests: usize,
    pub code_issues_found: usize,
    pub regressions_detected: Vec<String>,
    pub new_issues_identified: Vec<String>,
    pub overall_status: String,
    pub summary_message: String,
    pub detailed_results: Vec<String>,
    pub code_analysis_summary: Vec<String>,
}

pub struct ReportAnalyzer;

impl ReportAnalyzer {
    pub fn new() -> Self { ReportAnalyzer }

    pub fn analyze_results(
        &self,
        test_results: Vec<String>,
        code_analysis_results: Vec<String>,
    ) -> AtveReport {
        let mut passed_tests = 0;
        let mut failed_tests = 0;
        let mut error_tests = 0;
        let mut timeout_tests = 0;
        let total_tests = test_results.len();

        for tr in &test_results {
            if tr.contains("Passed") { passed_tests += 1; }
            else if tr.contains("Failed") { failed_tests += 1; }
            else if tr.contains("Error") { error_tests += 1; }
            else if tr.contains("Timeout") { timeout_tests += 1; }
        }

        let mut code_issues_found = 0;
        let mut new_issues_identified = Vec::new();
        for car in &code_analysis_results {
            if car.contains("issue") { code_issues_found += 1; new_issues_identified.push(car.clone()); }
        }

        let overall_status = if failed_tests == 0 && error_tests == 0 && timeout_tests == 0 {
            "Success".to_string()
        } else if passed_tests > 0 {
            "PartialSuccess".to_string()
        } else {
            "Failure".to_string()
        };

        let summary_message = format!(
            "ATVE Run Complete: {}/{} tests passed, {} failed, {} errors, {} timeouts. {} code issues detected.",
            passed_tests, total_tests, failed_tests, error_tests, timeout_tests, code_issues_found
        );

        AtveReport {
            timestamp: chrono::Utc::now().to_rfc3339(),
            total_tests,
            passed_tests,
            failed_tests,
            error_tests,
            timeout_tests,
            code_issues_found,
            regressions_detected: Vec::new(),
            new_issues_identified,
            overall_status,
            summary_message,
            detailed_results: test_results,
            code_analysis_summary: code_analysis_results,
        }
    }
}
