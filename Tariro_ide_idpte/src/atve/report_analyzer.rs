// src/atve/report_analyzer.rs

//! Analyzes test results, generates reports, and identifies regressions or new issues.

use crate::atve::test_executor::{TestResult, TestStatus};
use crate::code_analysis::analyzer::{CodeAnalysisResult, CodeIssue, IssueSeverity};
use std::collections::HashMap;

/// A comprehensive report summarizing the ATVE run.
#[derive(Debug, Clone)]
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
    pub overall_status: AtveStatus,
    pub summary_message: String,
    pub detailed_results: Vec<TestResult>,
    pub code_analysis_summary: Vec<CodeAnalysisResult>,
}

/// Overall status of the ATVE run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtveStatus {
    Success,
    PartialSuccess,
    Failure,
}

/// The `ReportAnalyzer` processes raw test results and code analysis to create meaningful reports.
pub struct ReportAnalyzer {
    // Can hold historical test data for regression analysis.
}

impl ReportAnalyzer {
    pub fn new() -> Self {
        ReportAnalyzer {}
    }

    /// Analyzes a set of test results and code analysis to produce an `AtveReport`.
    pub fn analyze_results(
        &self,
        test_results: Vec<TestResult>,
        code_analysis_results: Vec<CodeAnalysisResult>,
        _previous_report: Option<&AtveReport>, // For regression analysis
    ) -> AtveReport {
        let mut passed_tests = 0;
        let mut failed_tests = 0;
        let mut error_tests = 0;
        let mut timeout_tests = 0;
        let total_tests = test_results.len();

        for tr in &test_results {
            match tr.status {
                TestStatus::Passed => passed_tests += 1,
                TestStatus::Failed => failed_tests += 1,
                TestStatus::Error => error_tests += 1,
                TestStatus::Timeout => timeout_tests += 1,
                _ => { /* Skipped tests */ }
            }
        }

        let mut code_issues_found = 0;
        let mut new_issues_identified = Vec::new();
        for car in &code_analysis_results {
            code_issues_found += car.issues.len();
            for issue in &car.issues {
                new_issues_identified.push(format!("Code Issue: {} (File: {}, Severity: {:?})", issue.description, car.file_path, issue.severity));
            }
        }

        let overall_status = if failed_tests == 0 && error_tests == 0 && timeout_tests == 0 {
            AtveStatus::Success
        } else if passed_tests > 0 {
            AtveStatus::PartialSuccess
        } else {
            AtveStatus::Failure
        };

        let summary_message = format!(
            "ATVE Run Complete: {}/{} tests passed, {} failed, {} errors, {} timeouts. {} code issues detected.",
            passed_tests, total_tests, failed_tests, error_tests, timeout_tests, code_issues_found
        );

        // TODO: Implement actual regression detection using _previous_report
        let regressions_detected = Vec::new();

        AtveReport {
            timestamp: chrono::Utc::now().to_rfc3339(),
            total_tests,
            passed_tests,
            failed_tests,
            error_tests,
            timeout_tests,
            code_issues_found,
            regressions_detected,
            new_issues_identified,
            overall_status,
            summary_message,
            detailed_results: test_results,
            code_analysis_summary: code_analysis_results,
        }
    }
}
