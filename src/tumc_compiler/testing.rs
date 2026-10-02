//! TUMC Testing Framework
//!
//! Comprehensive testing infrastructure for TUMC programs including
//! unit tests, integration tests, snapshot testing, and test harnesses.

use crate::tumc_compiler::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Test configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Test directory
    pub test_dir: PathBuf,
    /// Snapshot directory
    pub snapshot_dir: PathBuf,
    /// Timeout for each test (ms)
    pub timeout_ms: u64,
    /// Should capture stdout
    pub capture_stdout: bool,
    /// Should capture stderr
    pub capture_stderr: bool,
    /// Parallel test execution
    pub parallel: bool,
    /// Number of workers if parallel
    pub workers: usize,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            test_dir: PathBuf::from("tests"),
            snapshot_dir: PathBuf::from("tests/snapshots"),
            timeout_ms: 30000,
            capture_stdout: true,
            capture_stderr: true,
            parallel: true,
            workers: num_cpus::get(),
        }
    }
}

/// Test result
#[derive(Debug, Clone)]
pub struct TestResult {
    /// Test name
    pub name: String,
    /// Whether test passed
    pub passed: bool,
    /// Duration
    pub duration: std::time::Duration,
    /// stdout captured
    pub stdout: Option<String>,
    /// stderr captured
    pub stderr: Option<String>,
    /// Error message if failed
    pub error: Option<String>,
    /// Panic info if panicked
    pub panic_info: Option<String>,
}

/// Test suite
#[derive(Debug, Clone)]
pub struct TestSuite {
    /// Suite name
    pub name: String,
    /// Tests in suite
    pub tests: Vec<TestCase>,
    /// Setup function
    pub setup: Option<Box<dyn Fn() + Send + Sync>>,
    /// Teardown function
    pub teardown: Option<Box<dyn Fn() + Send + Sync>>,
}

/// Test case
#[derive(Debug, Clone)]
pub struct TestCase {
    /// Test name
    pub name: String,
    /// Test function
    pub func: TestFn,
    /// Tags for filtering
    pub tags: Vec<String>,
    /// Should be ignored
    pub ignore: bool,
    /// Expected failure
    pub should_fail: bool,
}

/// Test function signature
pub type TestFn = Box<dyn Fn() -> Result<(), TestError> + Send + Sync>;

/// Test error
#[derive(Debug)]
pub enum TestError {
    /// Assertion failed
    AssertionFailed {
        expected: String,
        actual: String,
        message: Option<String>,
    },
    /// Panic occurred
    Panic { message: String },
    /// Timeout
    Timeout { duration_ms: u64 },
    /// Compilation error
    CompilationError { error: String },
    /// Runtime error
    RuntimeError { error: String },
    /// Custom error
    Custom(String),
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestError::AssertionFailed { expected, actual, message } => {
                write!(f, "Assertion failed: expected {}, got {}", expected, actual)?;
                if let Some(msg) = message {
                    write!(f, " - {}", msg)?;
                }
                Ok(())
            }
            TestError::Panic { message } => write!(f, "Panic: {}", message),
            TestError::Timeout { duration_ms } => write!(f, "Timeout after {}ms", duration_ms),
            TestError::CompilationError { error } => write!(f, "Compilation error: {}", error),
            TestError::RuntimeError { error } => write!(f, "Runtime error: {}", error),
            TestError::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for TestError {}

/// Test runner
pub struct TestRunner {
    /// Configuration
    config: TestConfig,
    /// Registered suites
    suites: HashMap<String, TestSuite>,
    /// Results
    results: Vec<TestResult>,
}

impl Default for TestRunner {
    fn default() -> Self {
        Self::new(TestConfig::default())
    }
}

impl TestRunner {
    /// Create new test runner
    pub fn new(config: TestConfig) -> Self {
        Self {
            config,
            suites: HashMap::new(),
            results: Vec::new(),
        }
    }

    /// Register a test suite
    pub fn register_suite(&mut self, suite: TestSuite) {
        self.suites.insert(suite.name.clone(), suite);
    }

    /// Run all tests
    pub fn run_all(&mut self) -> TestReport {
        let start = std::time::Instant::now();
        let mut report = TestReport::new();

        for (name, suite) in &self.suites {
            let suite_report = self.run_suite(name);
            report.add_suite(name.clone(), suite_report);
        }

        report.duration = start.elapsed();
        report
    }

    /// Run a specific suite
    pub fn run_suite(&mut self, name: &str) -> SuiteReport {
        let suite = match self.suites.get(name) {
            Some(s) => s,
            None => return SuiteReport::new(name),
        };

        let mut report = SuiteReport::new(name);

        // Run setup
        if let Some(setup) = &suite.setup {
            setup();
        }

        for test in &suite.tests {
            if test.ignore {
                report.add_skipped(test.name.clone());
                continue;
            }

            let result = self.run_test(&suite.name, &test);
            report.add_result(result);
        }

        // Run teardown
        if let Some(teardown) = &suite.teardown {
            teardown();
        }

        report
    }

    /// Run a single test
    fn run_test(&self, suite_name: &str, test: &TestCase) -> TestResult {
        let start = std::time::Instant::now();
        let name = format!("{}::{}", suite_name, test.name);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (test.func)()
        }));

        let duration = start.elapsed();
        let passed = match result {
            Ok(Ok(())) => !test.should_fail,
            Ok(Err(_)) => test.should_fail,
            Err(e) => {
                let panic_msg = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic".to_string()
                };

                return TestResult {
                    name,
                    passed: test.should_fail,
                    duration,
                    stdout: None,
                    stderr: None,
                    error: Some(panic_msg.clone()),
                    panic_info: Some(panic_msg),
                };
            }
        };

        TestResult {
            name,
            passed,
            duration,
            stdout: None,
            stderr: None,
            error: None,
            panic_info: None,
        }
    }

    /// Filter tests by tag
    pub fn filter_by_tag(&mut self, tag: &str) -> &mut Self {
        for suite in self.suites.values_mut() {
            suite.tests.retain(|t| t.tags.contains(&tag.to_string()));
        }
        self
    }

    /// Filter tests by name pattern
    pub fn filter_by_name(&mut self, pattern: &str) -> &mut Self {
        for suite in self.suites.values_mut() {
            suite.tests.retain(|t| t.name.contains(pattern));
        }
        self
    }
}

/// Test report
#[derive(Debug, Clone)]
pub struct TestReport {
    /// Total duration
    pub duration: std::time::Duration,
    /// Suite reports
    pub suites: Vec<SuiteReport>,
    /// Total passed
    pub total_passed: usize,
    /// Total failed
    pub total_failed: usize,
    /// Total skipped
    pub total_skipped: usize,
}

impl TestReport {
    /// Create new report
    pub fn new() -> Self {
        Self {
            duration: std::time::Duration::ZERO,
            suites: Vec::new(),
            total_passed: 0,
            total_failed: 0,
            total_skipped: 0,
        }
    }

    /// Add suite report
    pub fn add_suite(&mut self, name: String, report: SuiteReport) {
        self.total_passed += report.passed;
        self.total_failed += report.failed;
        self.total_skipped += report.skipped;
        self.suites.push(report);
    }

    /// Check if all passed
    pub fn all_passed(&self) -> bool {
        self.total_failed == 0
    }
}

/// Suite report
#[derive(Debug, Clone)]
pub struct SuiteReport {
    /// Suite name
    pub name: String,
    /// Test results
    pub results: Vec<TestResult>,
    /// Passed count
    pub passed: usize,
    /// Failed count
    pub failed: usize,
    /// Skipped count
    pub skipped: usize,
}

impl SuiteReport {
    /// Create new suite report
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            results: Vec::new(),
            passed: 0,
            failed: 0,
            skipped: 0,
        }
    }

    /// Add result
    pub fn add_result(&mut self, result: TestResult) {
        if result.passed {
            self.passed += 1;
        } else {
            self.failed += 1;
        }
        self.results.push(result);
    }

    /// Add skipped test
    pub fn add_skipped(&mut self, name: String) {
        self.skipped += 1;
        self.results.push(TestResult {
            name,
            passed: true, // Skipped doesn't count as failure
            duration: std::time::Duration::ZERO,
            stdout: None,
            stderr: None,
            error: Some("SKIPPED".to_string()),
            panic_info: None,
        });
    }
}

/// Assertion helpers
pub mod assertions {
    /// Assert equal
    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(
        expected: T,
        actual: T,
    ) -> Result<(), TestError> {
        if expected == actual {
            Ok(())
        } else {
            Err(TestError::AssertionFailed {
                expected: format!("{:?}", expected),
                actual: format!("{:?}", actual),
                message: None,
            })
        }
    }

    /// Assert not equal
    pub fn assert_ne<T: PartialEq + std::fmt::Debug>(
        not_expected: T,
        actual: T,
    ) -> Result<(), TestError> {
        if not_expected != actual {
            Ok(())
        } else {
            Err(TestError::AssertionFailed {
                expected: format!("not {:?}", not_expected),
                actual: format!("{:?}", actual),
                message: None,
            })
        }
    }

    /// Assert true
    pub fn assert_true(condition: bool) -> Result<(), TestError> {
        if condition {
            Ok(())
        } else {
            Err(TestError::AssertionFailed {
                expected: "true".to_string(),
                actual: "false".to_string(),
                message: None,
            })
        }
    }

    /// Assert false
    pub fn assert_false(condition: bool) -> Result<(), TestError> {
        if !condition {
            Ok(())
        } else {
            Err(TestError::AssertionFailed {
                expected: "false".to_string(),
                actual: "true".to_string(),
                message: None,
            })
        }
    }

    /// Assert contains
    pub fn assert_contains(haystack: &str, needle: &str) -> Result<(), TestError> {
        if haystack.contains(needle) {
            Ok(())
        } else {
            Err(TestError::AssertionFailed {
                expected: format!("'{}' in '{}'", needle, haystack),
                actual: format!("'{}' not found", needle),
                message: None,
            })
        }
    }

    /// Assert panics
    pub fn assert_panics<F: FnOnce()>(f: F) -> Result<(), TestError> {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
        match result {
            Ok(_) => Err(TestError::AssertionFailed {
                expected: "panic".to_string(),
                actual: "no panic".to_string(),
                message: None,
            }),
            Err(_) => Ok(()),
        }
    }

    /// Assert approx equal (for floating point)
    pub fn assert_approx_eq(expected: f64, actual: f64, epsilon: f64) -> Result<(), TestError> {
        if (expected - actual).abs() < epsilon {
            Ok(())
        } else {
            Err(TestError::AssertionFailed {
                expected: format!("~{}", expected),
                actual: format!("{}", actual),
                message: Some(format!("epsilon = {}", epsilon)),
            })
        }
    }
}

/// Snapshot testing
pub struct SnapshotTester {
    /// Snapshot directory
    dir: PathBuf,
}

impl SnapshotTester {
    /// Create new snapshot tester
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    /// Assert snapshot matches
    pub fn assert_match(&self, name: &str, value: &str) -> Result<(), TestError> {
        let snapshot_path = self.dir.join(format!("{}.snap", name));

        if snapshot_path.exists() {
            let expected = std::fs::read_to_string(&snapshot_path)
                .map_err(|e| TestError::Custom(format!("Failed to read snapshot: {}", e)))?;

            let expected = expected.trim();
            let actual = value.trim();

            if expected == actual {
                Ok(())
            } else {
                Err(TestError::AssertionFailed {
                    expected: format!("Snapshot '{}'", name),
                    actual: format!("{} chars vs {} chars", actual.len(), expected.len()),
                    message: Some(format!(
                        "Snapshot mismatch for '{}'. Run with UPDATE_SNAPSHOTS=1 to update.",
                        name
                    )),
                })
            }
        } else {
            // Create snapshot
            std::fs::create_dir_all(&self.dir).ok();
            std::fs::write(&snapshot_path, value)
                .map_err(|e| TestError::Custom(format!("Failed to create snapshot: {}", e)))?;
            Ok(())
        }
    }

    /// Update all snapshots
    pub fn update_snapshots(&self, name: &str, value: &str) -> Result<(), TestError> {
        let snapshot_path = self.dir.join(format!("{}.snap", name));
        std::fs::create_dir_all(&self.dir).ok();
        std::fs::write(&snapshot_path, value)
            .map_err(|e| TestError::Custom(format!("Failed to update snapshot: {}", e)))
    }
}

/// Property-based test runner
pub struct PropertyRunner {
    /// Number of iterations
    iterations: usize,
    /// Maximum test size
    max_size: usize,
}

impl Default for PropertyRunner {
    fn default() -> Self {
        Self {
            iterations: 100,
            max_size: 1000,
        }
    }
}

impl PropertyRunner {
    /// Create new property runner
    pub fn new() -> Self {
        Self::default()
    }

    /// Set iterations
    pub fn iterations(mut self, n: usize) -> Self {
        self.iterations = n;
        self
    }

    /// Set max size
    pub fn max_size(mut self, size: usize) -> Self {
        self.max_size = size;
        self
    }

    /// Run property test
    pub fn run<F>(&self, property: &str, mut f: F) -> PropertyResult
    where
        F: FnMut(usize) -> Result<(), TestError>,
    {
        let mut failures = Vec::new();

        for i in 0..self.iterations {
            if let Err(e) = f(i) {
                failures.push(PropertyFailure {
                    iteration: i,
                    error: e,
                });
            }
        }

        PropertyResult {
            property: property.to_string(),
            iterations: self.iterations,
            failures,
        }
    }
}

/// Property test result
#[derive(Debug)]
pub struct PropertyResult {
    pub property: String,
    pub iterations: usize,
    pub failures: Vec<PropertyFailure>,
}

impl PropertyResult {
    /// Check if property holds
    pub fn holds(&self) -> bool {
        self.failures.is_empty()
    }
}

/// Property failure
#[derive(Debug)]
pub struct PropertyFailure {
    pub iteration: usize,
    pub error: TestError,
}

/// Benchmark test
pub struct BenchmarkTest {
    /// Name
    name: String,
    /// Setup
    setup: Option<Box<dyn Fn()>>,
    /// Benchmark function
    func: Box<dyn FnMut()>,
    /// Teardown
    teardown: Option<Box<dyn Fn()>>,
    /// Warmup iterations
    warmup: usize,
    /// Benchmark iterations
    iterations: usize,
}

impl BenchmarkTest {
    /// Create new benchmark
    pub fn new(name: &str, f: impl FnMut() + 'static) -> Self {
        Self {
            name: name.to_string(),
            setup: None,
            func: Box::new(f),
            teardown: None,
            warmup: 3,
            iterations: 10,
        }
    }

    /// Set setup
    pub fn setup(mut self, f: impl Fn() + 'static) -> Self {
        self.setup = Some(Box::new(f));
        self
    }

    /// Set teardown
    pub fn teardown(mut self, f: impl Fn() + 'static) -> Self {
        self.teardown = Some(Box::new(f));
        self
    }

    /// Set warmup
    pub fn warmup(mut self, n: usize) -> Self {
        self.warmup = n;
        self
    }

    /// Set iterations
    pub fn iterations(mut self, n: usize) -> Self {
        self.iterations = n;
        self
    }

    /// Run benchmark
    pub fn run(&mut self) -> BenchmarkResult {
        // Warmup
        for _ in 0..self.warmup {
            if let Some(ref setup) = self.setup {
                setup();
            }
            (self.func)();
            if let Some(ref teardown) = self.teardown {
                teardown();
            }
        }

        // Benchmark
        let mut samples = Vec::with_capacity(self.iterations);
        for _ in 0..self.iterations {
            if let Some(ref setup) = self.setup {
                setup();
            }

            let start = std::time::Instant::now();
            (self.func)();
            let duration = start.elapsed();

            if let Some(ref teardown) = self.teardown {
                teardown();
            }

            samples.push(duration);
        }

        let total: std::time::Duration = samples.iter().sum();
        let average = total / self.iterations as u32;
        let min = *samples.iter().min().unwrap();
        let max = *samples.iter().max().unwrap();

        let mean = average.as_nanos() as f64;
        let variance = samples
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / samples.len() as f64;
        let std_dev = std::time::Duration::from_nanos(variance.sqrt() as u64);

        BenchmarkResult {
            name: self.name.clone(),
            iterations: self.iterations,
            total,
            average,
            min,
            max,
            std_dev,
        }
    }
}

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total: std::time::Duration,
    pub average: std::time::Duration,
    pub min: std::time::Duration,
    pub max: std::time::Duration,
    pub std_dev: std::time::Duration,
}

impl BenchmarkResult {
    /// Format for display
    pub fn format(&self) -> String {
        format!(
            "{}: avg={:?} min={:?} max={:?} std={:?}",
            self.name, self.average, self.min, self.max, self.std_dev
        )
    }
}

/// Test builder
pub struct TestBuilder {
    name: String,
    tags: Vec<String>,
    ignore: bool,
    should_fail: bool,
}

impl TestBuilder {
    /// Create new test builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tags: Vec::new(),
            ignore: false,
            should_fail: false,
        }
    }

    /// Add tag
    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    /// Set ignore
    pub fn ignore(mut self) -> Self {
        self.ignore = true;
        self
    }

    /// Set should fail
    pub fn should_fail(mut self) -> Self {
        self.should_fail = true;
        self
    }

    /// Build test case
    pub fn build<F>(self, func: F) -> TestCase
    where
        F: Fn() -> Result<(), TestError> + Send + Sync + 'static,
    {
        TestCase {
            name: self.name,
            func: Box::new(func),
            tags: self.tags,
            ignore: self.ignore,
            should_fail: self.should_fail,
        }
    }
}

/// Test suite builder
pub struct SuiteBuilder {
    name: String,
    tests: Vec<TestCase>,
    setup: Option<Box<dyn Fn() + Send + Sync>>,
    teardown: Option<Box<dyn Fn() + Send + Sync>>,
}

impl SuiteBuilder {
    /// Create new suite builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tests: Vec::new(),
            setup: None,
            teardown: None,
        }
    }

    /// Add test
    pub fn test(mut self, name: &str) -> TestCaseBuilder {
        TestCaseBuilder {
            suite: self,
            test_name: name.to_string(),
        }
    }

    /// Add setup
    pub fn setup(mut self, f: impl Fn() + Send + Sync + 'static) -> Self {
        self.setup = Some(Box::new(f));
        self
    }

    /// Add teardown
    pub fn teardown(mut self, f: impl Fn() + Send + Sync + 'static) -> Self {
        self.teardown = Some(Box::new(f));
        self
    }

    /// Build suite
    pub fn build(self) -> TestSuite {
        TestSuite {
            name: self.name,
            tests: self.tests,
            setup: self.setup,
            teardown: self.teardown,
        }
    }
}

/// Test case builder
pub struct TestCaseBuilder {
    suite: SuiteBuilder,
    test_name: String,
}

impl TestCaseBuilder {
    /// Add tag
    pub fn tag(mut self, tag: &str) -> Self {
        self.suite.tests.push(TestCase {
            name: self.test_name.clone(),
            func: Box::new(|| Ok(())),
            tags: vec![tag.to_string()],
            ignore: false,
            should_fail: false,
        });
        self
    }

    /// Set ignore
    pub fn ignore(self) -> Self {
        self.suite.tests.push(TestCase {
            name: self.test_name.clone(),
            func: Box::new(|| Ok(())),
            tags: Vec::new(),
            ignore: true,
            should_fail: false,
        });
        self
    }

    /// Build test
    pub fn build<F>(self, func: F) -> SuiteBuilder
    where
        F: Fn() -> Result<(), TestError> + Send + Sync + 'static,
    {
        self.suite.tests.push(TestCase {
            name: self.test_name,
            func: Box::new(func),
            tags: Vec::new(),
            ignore: false,
            should_fail: false,
        });
        self.suite
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assertions() {
        assertions::assert_true(true).unwrap();
        assertions::assert_false(false).unwrap();
        assertions::assert_eq(42, 42).unwrap();
        assertions::assert_ne(42, 43).unwrap();
        assertions::assert_contains("hello world", "world").unwrap();
    }

    #[test]
    fn test_property_runner() {
        let runner = PropertyRunner::new().iterations(10);
        let result = runner.run("always_passes", |_| Ok(()));
        assert!(result.holds());
    }

    #[test]
    fn test_test_builder() {
        let test = TestBuilder::new("example")
            .tag("fast")
            .tag("unit")
            .build(|| Ok(()));

        assert_eq!(test.name, "example");
        assert!(test.tags.contains(&"fast".to_string()));
        assert!(test.tags.contains(&"unit".to_string()));
    }
}
