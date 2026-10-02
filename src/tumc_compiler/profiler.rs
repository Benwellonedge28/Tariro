//! TUMC Profiler and Performance Analysis
//!
//! Performance profiling, benchmarking, and optimization analysis tools
//! for the TUMC compiler.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Compiler phase timing
#[derive(Debug, Clone)]
pub struct PhaseTiming {
    /// Phase name
    pub phase: String,
    /// Start time
    pub start: Instant,
    /// End time
    pub end: Option<Instant>,
    /// Duration in nanoseconds
    pub duration_ns: u64,
}

impl PhaseTiming {
    /// Create new phase timing
    pub fn new(phase: impl Into<String>) -> Self {
        Self {
            phase: phase.into(),
            start: Instant::now(),
            end: None,
            duration_ns: 0,
        }
    }

    /// End the timing
    pub fn end(&mut self) {
        let now = Instant::now();
        self.end = Some(now);
        self.duration_ns = now.duration_since(self.start).as_nanos() as u64;
    }

    /// Get duration
    pub fn duration(&self) -> Duration {
        Duration::from_nanos(self.duration_ns)
    }
}

/// Scoped timer for measuring code blocks
pub struct ScopedTimer<'a> {
    profiler: &'a mut CompilerProfiler,
    phase: String,
    start: Instant,
}

impl<'a> ScopedTimer<'a> {
    /// Start timing a block
    pub fn new(profiler: &'a mut CompilerProfiler, phase: impl Into<String>) -> Self {
        let start = Instant::now();
        Self {
            profiler,
            phase: phase.into(),
            start,
        }
    }
}

impl<'a> Drop for ScopedTimer<'a> {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.profiler.record_phase(&self.phase, duration);
    }
}

/// Compiler profiler
pub struct CompilerProfiler {
    /// Phase timings
    timings: HashMap<String, Vec<Duration>>,
    /// Total compilation time
    total_time: Duration,
    /// Start time
    start: Instant,
    /// Memory samples
    memory_samples: Vec<MemorySample>,
    /// Function call counts
    call_counts: HashMap<String, u64>,
}

/// Memory sample
#[derive(Debug, Clone)]
pub struct MemorySample {
    /// Timestamp
    pub timestamp: Duration,
    /// Allocated bytes
    pub allocated: u64,
    /// Deallocated bytes
    pub deallocated: u64,
}

impl Default for CompilerProfiler {
    fn default() -> Self {
        Self::new()
    }
}

impl CompilerProfiler {
    /// Create new profiler
    pub fn new() -> Self {
        Self {
            timings: HashMap::new(),
            total_time: Duration::ZERO,
            start: Instant::now(),
            memory_samples: Vec::new(),
            call_counts: HashMap::new(),
        }
    }

    /// Start profiling
    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    /// Record phase timing
    pub fn record_phase(&mut self, phase: &str, duration: Duration) {
        self.timings
            .entry(phase.to_string())
            .or_default()
            .push(duration);
    }

    /// Record function call
    pub fn record_call(&mut self, function: &str) {
        *self.call_counts.entry(function.to_string()).or_insert(0) += 1;
    }

    /// Record memory usage
    pub fn record_memory(&mut self, allocated: u64, deallocated: u64) {
        let elapsed = self.start.elapsed();
        self.memory_samples.push(MemorySample {
            timestamp: elapsed,
            allocated,
            deallocated,
        });
    }

    /// End profiling and get report
    pub fn finish(&mut self) -> ProfileReport {
        self.total_time = self.start.elapsed();
        self.generate_report()
    }

    /// Generate profile report
    pub fn generate_report(&self) -> ProfileReport {
        let mut phase_stats = Vec::new();

        for (phase, timings) in &self.timings {
            if timings.is_empty() {
                continue;
            }

            let total: Duration = timings.iter().sum();
            let avg = total / timings.len() as u32;
            let min = *timings.iter().min().unwrap();
            let max = *timings.iter().max().unwrap();

            phase_stats.push(PhaseStats {
                phase: phase.clone(),
                total,
                average: avg,
                min,
                max,
                count: timings.len(),
                percentage: (total.as_nanos() as f64 / self.total_time.as_nanos() as f64) * 100.0,
            });
        }

        // Sort by total time descending
        phase_stats.sort_by(|a, b| b.total.cmp(&a.total));

        let mut function_calls = self
            .call_counts
            .iter()
            .map(|(name, count)| FunctionCallStats {
                function: name.clone(),
                count: *count,
            })
            .collect::<Vec<_>>();
        function_calls.sort_by(|a, b| b.count.cmp(&a.count));

        let memory_stats = self.compute_memory_stats();

        ProfileReport {
            total_time: self.total_time,
            phase_stats,
            function_calls,
            memory_stats,
        }
    }

    /// Compute memory statistics
    fn compute_memory_stats(&self) -> Option<MemoryStats> {
        if self.memory_samples.is_empty() {
            return None;
        }

        let mut peak_allocated = 0u64;
        let mut current_allocated = 0i64;

        for sample in &self.memory_samples {
            current_allocated += sample.allocated as i64;
            current_allocated -= sample.deallocated as i64;

            if current_allocated as u64 > peak_allocated {
                peak_allocated = current_allocated as u64;
            }
        }

        Some(MemoryStats {
            peak_allocated,
            final_allocated: current_allocated.max(0) as u64,
            samples: self.memory_samples.len(),
        })
    }

    /// Get scoped timer
    pub fn timer(&mut self, phase: impl Into<String>) -> ScopedTimer {
        ScopedTimer::new(self, phase)
    }
}

/// Phase statistics
#[derive(Debug, Clone)]
pub struct PhaseStats {
    pub phase: String,
    pub total: Duration,
    pub average: Duration,
    pub min: Duration,
    pub max: Duration,
    pub count: usize,
    pub percentage: f64,
}

/// Function call statistics
#[derive(Debug, Clone)]
pub struct FunctionCallStats {
    pub function: String,
    pub count: u64,
}

/// Memory statistics
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub peak_allocated: u64,
    pub final_allocated: u64,
    pub samples: usize,
}

/// Profile report
#[derive(Debug, Clone)]
pub struct ProfileReport {
    pub total_time: Duration,
    pub phase_stats: Vec<PhaseStats>,
    pub function_calls: Vec<FunctionCallStats>,
    pub memory_stats: Option<MemoryStats>,
}

impl ProfileReport {
    /// Print report to string
    pub fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str("=== TUMC Compiler Profile Report ===\n\n");
        output.push_str(&format!(
            "Total compilation time: {:?}\n\n",
            self.total_time
        ));

        output.push_str("Phase Breakdown:\n");
        output.push_str("----------------\n");
        for stats in &self.phase_stats {
            output.push_str(&format!(
                "{:20} {:>10?} ({:5.1}%)\n",
                stats.phase, stats.total, stats.percentage
            ));
            output.push_str(&format!(
                "  avg: {:?}, min: {:?}, max: {:?} ({} calls)\n",
                stats.average, stats.min, stats.max, stats.count
            ));
        }

        if !self.function_calls.is_empty() {
            output.push_str("\nFunction Calls:\n");
            output.push_str("---------------\n");
            for stats in self.function_calls.iter().take(10) {
                output.push_str(&format!("{:40} {:>10}\n", stats.function, stats.count));
            }
        }

        if let Some(ref mem) = self.memory_stats {
            output.push_str("\nMemory Usage:\n");
            output.push_str("-------------\n");
            output.push_str(&format!("Peak allocated: {} bytes\n", mem.peak_allocated));
            output.push_str(&format!("Final allocated: {} bytes\n", mem.final_allocated));
            output.push_str(&format!("Samples taken: {}\n", mem.samples));
        }

        output
    }

    /// Export to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Benchmark result
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub iterations: usize,
    pub total_time: Duration,
    pub average_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub std_dev: Duration,
}

impl BenchmarkResult {
    /// Create new benchmark result
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            iterations: 0,
            total_time: Duration::ZERO,
            average_time: Duration::ZERO,
            min_time: Duration::MAX,
            max_time: Duration::ZERO,
            std_dev: Duration::ZERO,
        }
    }

    /// Add sample
    pub fn add_sample(&mut self, duration: Duration) {
        self.iterations += 1;
        self.total_time += duration;

        if duration < self.min_time {
            self.min_time = duration;
        }
        if duration > self.max_time {
            self.max_time = duration;
        }

        self.average_time = self.total_time / self.iterations as u32;
    }

    /// Calculate standard deviation
    pub fn calculate_std_dev(&mut self, samples: &[Duration]) {
        if samples.is_empty() {
            return;
        }

        let mean = self.average_time.as_nanos() as f64;
        let variance = samples
            .iter()
            .map(|d| {
                let diff = d.as_nanos() as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / samples.len() as f64;

        self.std_dev = Duration::from_nanos(variance.sqrt() as u64);
    }

    /// Format for display
    pub fn format(&self) -> String {
        format!(
            "{}: {:?} (avg: {:?}, min: {:?}, max: {:?}, std: {:?})",
            self.name, self.total_time, self.average_time, self.min_time, self.max_time, self.std_dev
        )
    }
}

/// Benchmark runner
pub struct BenchmarkRunner {
    /// Warmup iterations
    warmup_iterations: usize,
    /// Benchmark iterations
    benchmark_iterations: usize,
    /// Results
    results: Vec<BenchmarkResult>,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl BenchmarkRunner {
    /// Create new benchmark runner
    pub fn new() -> Self {
        Self {
            warmup_iterations: 3,
            benchmark_iterations: 10,
            results: Vec::new(),
        }
    }

    /// Set warmup iterations
    pub fn warmup(mut self, iterations: usize) -> Self {
        self.warmup_iterations = iterations;
        self
    }

    /// Set benchmark iterations
    pub fn iterations(mut self, iterations: usize) -> Self {
        self.benchmark_iterations = iterations;
        self
    }

    /// Run benchmark
    pub fn benchmark<F, R>(&mut self, name: impl Into<String>, mut f: F) -> &BenchmarkResult
    where
        F: FnMut() -> R,
    {
        let name = name.into();

        // Warmup
        for _ in 0..self.warmup_iterations {
            std::hint::black_box(f());
        }

        // Benchmark
        let mut result = BenchmarkResult::new(name.clone());
        let mut samples = Vec::with_capacity(self.benchmark_iterations);

        for _ in 0..self.benchmark_iterations {
            let start = Instant::now();
            std::hint::black_box(f());
            let duration = start.elapsed();
            result.add_sample(duration);
            samples.push(duration);
        }

        result.calculate_std_dev(&samples);
        self.results.push(result);

        self.results.last().unwrap()
    }

    /// Run comparison benchmark
    pub fn compare<A, B, R>(
        &mut self,
        name: impl Into<String>,
        a: A,
        b: B,
        f: impl Fn(&str, &dyn Fn()),
    ) where
        A: AsRef<str>,
        B: AsRef<str>,
    {
        let name = name.into();

        for (label, _sel) in [(a.as_ref(), "A"), (b.as_ref(), "B")] {
            self.benchmark(format!("{} [{}]", name, label), || {
                f(label, || {});
            });
        }
    }

    /// Get results
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    /// Print results
    pub fn print_results(&self) {
        for result in &self.results {
            println!("{}", result.format());
        }
    }
}

/// Performance regression detector
pub struct RegressionDetector {
    /// Baseline results
    baseline: HashMap<String, Duration>,
    /// Threshold for regression (percentage)
    threshold_percent: f64,
}

impl RegressionDetector {
    /// Create new detector
    pub fn new(threshold_percent: f64) -> Self {
        Self {
            baseline: HashMap::new(),
            threshold_percent,
        }
    }

    /// Set baseline
    pub fn set_baseline(&mut self, results: &[BenchmarkResult]) {
        for result in results {
            self.baseline.insert(result.name.clone(), result.average_time);
        }
    }

    /// Check for regression
    pub fn check(&self, result: &BenchmarkResult) -> Option<Regression> {
        if let Some(baseline) = self.baseline.get(&result.name) {
            let diff = result.average_time.as_nanos() as f64 - baseline.as_nanos() as f64;
            let percent = (diff / baseline.as_nanos() as f64) * 100.0;

            if percent > self.threshold_percent {
                return Some(Regression {
                    benchmark: result.name.clone(),
                    baseline: *baseline,
                    current: result.average_time,
                    percent_increase: percent,
                });
            }
        }
        None
    }
}

/// Performance regression
#[derive(Debug)]
pub struct Regression {
    pub benchmark: String,
    pub baseline: Duration,
    pub current: Duration,
    pub percent_increase: f64,
}

/// Compiler performance benchmarks
pub mod benchmarks {
    use super::*;

    /// Benchmark lexer
    pub fn bench_lexer(source: &str, iterations: usize) -> BenchmarkResult {
        let mut runner = BenchmarkRunner::new();
        runner.warmup(5).iterations(iterations);
        runner.benchmark("lexer", || {
            let _ = lexer::Lexer::new(source).tokenize();
        });
        runner.results()[0].clone()
    }

    /// Benchmark parser
    pub fn bench_parser(source: &str, iterations: usize) -> BenchmarkResult {
        let tokens = lexer::Lexer::new(source).tokenize().unwrap();
        let mut runner = BenchmarkRunner::new();
        runner.warmup(5).iterations(iterations);
        runner.benchmark("parser", move || {
            let _ = parser::Parser::new(tokens.clone()).parse();
        });
        runner.results()[0].clone()
    }

    /// Benchmark full compilation
    pub fn bench_compilation(source: &str, iterations: usize) -> BenchmarkResult {
        let mut runner = BenchmarkRunner::new();
        runner.warmup(3).iterations(iterations);
        runner.benchmark("full_compilation", || {
            let mut compiler = crate::tumc_compiler::Compiler::new(
                crate::tumc_compiler::SourceLanguage::TumcIR,
                crate::tumc_compiler::TargetPlatform::Wasm,
            );
            let _ = compiler.compile(source);
        });
        runner.results()[0].clone()
    }

    /// Run all benchmarks
    pub fn run_all_benchmarks() -> Vec<BenchmarkResult> {
        let sources = vec![
            ("small", "fn test() -> i32 { 42 }"),
            ("medium", r#"
                fn fib(n: i32) -> i32 {
                    if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }
                }
                fn main() -> i32 { fib(10) }
            "#),
            ("large", &format!(
                "fn add(a: i32, b: i32) -> i32 {{ a + b }}\n{}",
                (0..100).map(|i| format!("fn f{}() -> i32 {{ {} }}\n", i, i)).collect::<String>()
            )),
        ];

        let mut results = Vec::new();

        for (name, source) in sources {
            println!("Benchmarking {} source...", name);
            results.push(bench_lexer(source, 1000));
            results.push(bench_parser(source, 100));
            results.push(bench_compilation(source, 50));
        }

        results
    }
}

/// Flamegraph data collector
pub struct FlamegraphCollector {
    /// Stack frames
    frames: Vec<FlameFrame>,
}

#[derive(Debug, Clone)]
pub struct FlameFrame {
    pub name: String,
    pub start_ns: u64,
    pub end_ns: u64,
    pub children: Vec<FlameFrame>,
}

impl Default for FlamegraphCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl FlamegraphCollector {
    /// Create new collector
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    /// Record frame
    pub fn record(&mut self, name: &str, start_ns: u64, end_ns: u64) {
        self.frames.push(FlameFrame {
            name: name.to_string(),
            start_ns,
            end_ns,
            children: Vec::new(),
        });
    }

    /// Export to collapsed stack format
    pub fn to_collapsed_stacks(&self) -> String {
        let mut output = String::new();

        for frame in &self.frames {
            output.push_str(&format!(
                "{};{} {}\n",
                frame.name,
                frame.name,
                frame.end_ns - frame.start_ns
            ));
        }

        output
    }

    /// Export to JSON
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_timing() {
        let mut timing = PhaseTiming::new("test");
        std::thread::sleep(Duration::from_millis(1));
        timing.end();
        assert!(timing.duration_ns > 0);
    }

    #[test]
    fn test_profiler_record() {
        let mut profiler = CompilerProfiler::new();
        profiler.record_phase("lexer", Duration::from_millis(10));
        profiler.record_phase("parser", Duration::from_millis(20));
        profiler.record_phase("lexer", Duration::from_millis(15));

        let report = profiler.generate_report();
        assert!(!report.phase_stats.is_empty());
    }

    #[test]
    fn test_benchmark_result() {
        let mut result = BenchmarkResult::new("test");
        result.add_sample(Duration::from_millis(10));
        result.add_sample(Duration::from_millis(20));
        result.add_sample(Duration::from_millis(15));

        assert_eq!(result.iterations, 3);
        assert!(result.average_time > Duration::ZERO);
    }

    #[test]
    fn test_regression_detection() {
        let mut detector = RegressionDetector::new(10.0);
        detector.set_baseline(&[BenchmarkResult {
            name: "test".to_string(),
            iterations: 10,
            total_time: Duration::from_millis(100),
            average_time: Duration::from_millis(10),
            min_time: Duration::from_millis(8),
            max_time: Duration::from_millis(12),
            std_dev: Duration::from_millis(1),
        }]);

        let mut current = BenchmarkResult::new("test");
        current.add_sample(Duration::from_millis(15));

        let regression = detector.check(&current);
        assert!(regression.is_some());
    }
}
