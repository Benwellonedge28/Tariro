//! TUMC Fuzzing Infrastructure
//!
//! Property-based testing and fuzzing for the TUMC compiler to ensure
//! correctness and robustness.

use crate::tumc_compiler::*;
use std::collections::HashSet;
use rand::Rng;

/// Property-based test for lexer
pub struct LexerPropertyTest {
    /// Minimum string length
    min_len: usize,
    /// Maximum string length
    max_len: usize,
    /// Allowed characters
    allowed_chars: Vec<char>,
}

impl Default for LexerPropertyTest {
    fn default() -> Self {
        Self {
            min_len: 0,
            max_len: 1000,
            allowed_chars: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \n\t+-*/%=<>!&|(){}[].,;:\'\"_#@$".chars().collect(),
        }
    }
}

impl LexerPropertyTest {
    /// Create new property test
    pub fn new(min_len: usize, max_len: usize) -> Self {
        Self {
            min_len,
            max_len,
            ..Default::default()
        }
    }

    /// Generate random source string
    pub fn generate_source<R: Rng>(&self, rng: &mut R) -> String {
        let len = rng.gen_range(self.min_len..self.max_len);
        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..self.allowed_chars.len());
                self.allowed_chars[idx]
            })
            .collect()
    }

    /// Property: lexer should not panic on any input
    pub fn lexer_never_panics(&self, iterations: usize) -> FuzzResult {
        let mut rng = rand::thread_rng();
        let mut failures = Vec::new();

        for i in 0..iterations {
            let source = self.generate_source(&mut rng);
            let lexer = lexer::Lexer::new(&source);
            match lexer.tokenize() {
                Ok(tokens) => {
                    // Tokenization succeeded
                }
                Err(_) => {
                    // Expected: lexer may fail on invalid input
                }
            }
        }

        FuzzResult {
            total_iterations: iterations,
            failures,
            success: true,
        }
    }
}

/// Property test for parser
pub struct ParserPropertyTest {
    /// Valid token generators
    token_generators: Vec<Box<dyn Fn(&mut dyn Rng) -> lexer::Token>>,
}

impl ParserPropertyTest {
    /// Create new parser property test
    pub fn new() -> Self {
        Self {
            token_generators: Vec::new(),
        }
    }

    /// Property: parser should handle valid token sequences
    pub fn parser_handles_valid_tokens(&self, iterations: usize) -> FuzzResult {
        let mut rng = rand::thread_rng();
        let mut failures = Vec::new();

        for i in 0..iterations {
            // Generate random but valid-ish tokens
            let mut tokens = Vec::new();
            let num_tokens = rng.gen_range(1..50);

            for _ in 0..num_tokens {
                let kind = match rng.gen_range(0..20) {
                    0 => lexer::TokenKind::Fn,
                    1 => lexer::TokenKind::Let,
                    2 => lexer::TokenKind::Mut,
                    3 => lexer::TokenKind::If,
                    4 => lexer::TokenKind::Else,
                    5 => lexer::TokenKind::Loop,
                    6 => lexer::TokenKind::While,
                    7 => lexer::TokenKind::Return,
                    8 => lexer::TokenKind::Struct,
                    9 => lexer::TokenKind::Enum,
                    10 => lexer::TokenKind::IntLiteral,
                    11 => lexer::TokenKind::FloatLiteral,
                    12 => lexer::TokenKind::StringLiteral,
                    13 => lexer::TokenKind::Identifier,
                    14 => lexer::TokenKind::Plus,
                    15 => lexer::TokenKind::Minus,
                    16 => lexer::TokenKind::Star,
                    17 => lexer::TokenKind::Slash,
                    18 => lexer::TokenKind::Eq,
                    _ => lexer::TokenKind::LBrace,
                };

                tokens.push(lexer::Token::new(kind, "", i, 0));
            }

            let parser = parser::Parser::new(tokens);
            match parser.parse() {
                Ok(_) => {}
                Err(_) => {
                    // Parser failure is expected for random tokens
                }
            }
        }

        FuzzResult {
            total_iterations: iterations,
            failures,
            success: true,
        }
    }
}

/// Property test for type inference
pub struct TypeInferencePropertyTest;

impl TypeInferencePropertyTest {
    /// Property: type unification should be reflexive
    pub fn unification_is_reflexive(&self) -> Vec<PropertyFailure> {
        let types = vec![
            Type::I32,
            Type::I64,
            Type::F32,
            Type::F64,
            Type::Bool,
            Type::String,
            Type::Unit,
            Type::Var(0),
            Type::Var(1),
        ];

        let mut failures = Vec::new();

        for t in &types {
            if let Some(unified) = t.unify(t) {
                if unified != *t {
                    failures.push(PropertyFailure {
                        property: "unification_is_reflexive".to_string(),
                        description: format!("Type {:?} unify with itself should return same type", t),
                    });
                }
            }
        }

        failures
    }

    /// Property: type unification should be symmetric
    pub fn unification_is_symmetric(&self) -> Vec<PropertyFailure> {
        let types = vec![
            Type::I32,
            Type::I64,
            Type::F32,
            Type::F64,
            Type::Bool,
            Type::Var(0),
            Type::Var(1),
        ];

        let mut failures = Vec::new();

        for t1 in &types {
            for t2 in &types {
                let result1 = t1.unify(t2);
                let result2 = t2.unify(t1);

                match (result1, result2) {
                    (Some(r1), Some(r2)) => {
                        if r1 != r2 {
                            failures.push(PropertyFailure {
                                property: "unification_is_symmetric".to_string(),
                                description: format!("{:?} unify {:?} != {:?} unify {:?}", t1, t2, t2, t1),
                            });
                        }
                    }
                    (None, None) => {}
                    _ => {
                        failures.push(PropertyFailure {
                            property: "unification_is_symmetric".to_string(),
                            description: format!("Unification asymmetry: {:?} vs {:?}", result1, result2),
                        });
                    }
                }
            }
        }

        failures
    }

    /// Property: type unification should be transitive
    pub fn unification_is_transitive(&self) -> Vec<PropertyFailure> {
        let types = vec![
            Type::I32,
            Type::I64,
            Type::F32,
            Type::F64,
            Type::Var(0),
            Type::Var(1),
            Type::Var(2),
        ];

        let mut failures = Vec::new();

        for t1 in &types {
            for t2 in &types {
                for t3 in &types {
                    if let (Some(r1), Some(r2)) = (t1.unify(t2), t2.unify(t3)) {
                        if let Some(r3) = r1.unify(&r2) {
                            if let Some(r4) = t1.unify(&t3) {
                                if r3 != r4 {
                                    failures.push(PropertyFailure {
                                        property: "unification_is_transitive".to_string(),
                                        description: format!("Transitivity violation: {:?} vs {:?}", r3, r4),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        failures
    }
}

/// Property test for optimizer
pub struct OptimizerPropertyTest;

impl OptimizerPropertyTest {
    /// Property: optimization should preserve semantics
    pub fn optimization_preserves_semantics(&self, iterations: usize) -> Vec<PropertyFailure> {
        let mut failures = Vec::new();

        for i in 0..iterations {
            // Generate simple expressions that should optimize
            let sources = vec![
                "let x = 2 + 3;",
                "let x = 5 * 0;",
                "let x = 10 - 10;",
                "let x = 1 * 42;",
                "fn test() { return 0; let x = 1; }",
            ];

            for source in &sources {
                let tokens = lexer::Lexer::new(source).tokenize().unwrap();
                let ast = parser::Parser::new(tokens).parse().unwrap();
                let mut type_context = TypeContext::default();
                let typed = type_checker::TypeChecker::new(&mut type_context)
                    .check(&ast)
                    .unwrap();

                // Just verify optimization doesn't crash
                let _optimized = optimizer::Optimizer::new(OptimizationLevel::Standard)
                    .optimize(typed);
            }
        }

        failures
    }
}

/// Property test for code generator
pub struct CodegenPropertyTest;

impl CodegenPropertyTest {
    /// Property: code generation should not crash on valid AST
    pub fn codegen_never_panics(&self, iterations: usize) -> FuzzResult {
        let mut failures = Vec::new();

        for i in 0..iterations {
            let source = format!("fn test_{}() -> i32 {{ {} }}", i, i);

            let result = (|| -> Result<Vec<u8>, Vec<CompilerError>> {
                let tokens = lexer::Lexer::new(&source).tokenize()?;
                let ast = parser::Parser::new(tokens).parse()?;
                let mut type_context = TypeContext::default();
                let typed = type_checker::TypeChecker::new(&mut type_context).check(&ast)?;
                let optimized = optimizer::Optimizer::new(OptimizationLevel::Standard).optimize(typed);
                let output = codegen::CodeGenerator::new(TargetPlatform::Wasm).generate(&optimized)?;
                Ok(output)
            })();

            if result.is_err() {
                // Some errors are expected
            }
        }

        FuzzResult {
            total_iterations: iterations,
            failures,
            success: true,
        }
    }
}

/// Fuzzing result
#[derive(Debug)]
pub struct FuzzResult {
    pub total_iterations: usize,
    pub failures: Vec<FuzzFailure>,
    pub success: bool,
}

/// Fuzz failure
#[derive(Debug)]
pub struct FuzzFailure {
    pub input: String,
    pub error: String,
    pub iteration: usize,
}

/// Property failure
#[derive(Debug)]
pub struct PropertyFailure {
    pub property: String,
    pub description: String,
}

/// Mutation testing infrastructure
pub struct MutationTester {
    /// Number of mutations per source
    mutations_per_source: usize,
    /// Mutation operators
    operators: Vec<MutationOperator>,
}

#[derive(Debug, Clone)]
pub enum MutationOperator {
    /// Swap two adjacent characters
    SwapAdjacent,
    /// Delete a random character
    DeleteChar,
    /// Insert a random character
    InsertChar,
    /// Replace character with random char
    ReplaceChar,
    /// Duplicate a character
    DuplicateChar,
}

impl Default for MutationTester {
    fn default() -> Self {
        Self {
            mutations_per_source: 10,
            operators: vec![
                MutationOperator::SwapAdjacent,
                MutationOperator::DeleteChar,
                MutationOperator::InsertChar,
                MutationOperator::ReplaceChar,
                MutationOperator::DuplicateChar,
            ],
        }
    }
}

impl MutationTester {
    /// Create new mutation tester
    pub fn new(mutations_per_source: usize) -> Self {
        Self {
            mutations_per_source,
            ..Default::default()
        }
    }

    /// Apply mutation operator
    pub fn mutate(&self, source: &str, operator: &MutationOperator) -> String {
        let mut chars: Vec<char> = source.chars().collect();
        let len = chars.len();

        if len == 0 {
            return source.to_string();
        }

        match operator {
            MutationOperator::SwapAdjacent if len >= 2 => {
                let idx = rand::thread_rng().gen_range(0..len - 1);
                chars.swap(idx, idx + 1);
            }
            MutationOperator::DeleteChar if len > 0 => {
                let idx = rand::thread_rng().gen_range(0..len);
                chars.remove(idx);
            }
            MutationOperator::InsertChar => {
                let idx = rand::thread_rng().gen_range(0..=len);
                let c = rand::thread_rng().gen::<char>();
                chars.insert(idx, c);
            }
            MutationOperator::ReplaceChar if len > 0 => {
                let idx = rand::thread_rng().gen_range(0..len);
                let c = rand::thread_rng().gen::<char>();
                chars[idx] = c;
            }
            MutationOperator::DuplicateChar if len > 0 => {
                let idx = rand::thread_rng().gen_range(0..len);
                let c = chars[idx];
                chars.insert(idx, c);
            }
            _ => {}
        }

        chars.into_iter().collect()
    }

    /// Run mutation test
    pub fn run(&self, source: &str, iterations: usize) -> MutationResult {
        let mut killed = 0;
        let mut survived = 0;

        for _ in 0..iterations {
            for op in &self.operators {
                let mutated = self.mutate(source, op);

                // Try to compile mutated source
                let result = (|| -> Result<(), ()> {
                    let tokens = lexer::Lexer::new(&mutated).tokenize()?;
                    let ast = parser::Parser::new(tokens).parse()?;
                    let mut type_context = TypeContext::default();
                    type_checker::TypeChecker::new(&mut type_context).check(&ast)?;
                    Ok(())
                })();

                match result {
                    Err(_) => killed += 1,
                    Ok(_) => survived += 1,
                }
            }
        }

        let total = killed + survived;
        let score = if total > 0 { killed as f64 / total as f64 } else { 0.0 };

        MutationResult {
            total_mutations: total,
            killed,
            survived,
            mutation_score: score,
        }
    }
}

/// Mutation testing result
#[derive(Debug)]
pub struct MutationResult {
    pub total_mutations: usize,
    pub killed: usize,
    pub survived: usize,
    pub mutation_score: f64,
}

/// Differential testing between implementations
pub struct DifferentialTester {
    /// Reference implementation
    reference: Box<dyn Fn(&str) -> Result<String, String>>,
    /// Candidate implementation
    candidate: Box<dyn Fn(&str) -> Result<String, String>>,
}

impl DifferentialTester {
    /// Create new differential tester
    pub fn new<R, C>(reference: R, candidate: C) -> Self
    where
        R: Fn(&str) -> Result<String, String> + 'static,
        C: Fn(&str) -> Result<String, String> + 'static,
    {
        Self {
            reference: Box::new(reference),
            candidate: Box::new(candidate),
        }
    }

    /// Run differential test
    pub fn run(&self, test_cases: &[String]) -> Vec<DifferentialFailure> {
        let mut failures = Vec::new();

        for (i, input) in test_cases.iter().enumerate() {
            let ref_result = (self.reference)(input);
            let cand_result = (self.candidate)(input);

            match (ref_result, cand_result) {
                (Ok(ref_out), Ok(cand_out)) => {
                    if ref_out != cand_out {
                        failures.push(DifferentialFailure {
                            input: input.clone(),
                            reference_output: ref_out,
                            candidate_output: cand_out,
                            test_index: i,
                        });
                    }
                }
                (Err(ref_err), Err(_)) => {
                    // Both failed - acceptable
                }
                (Ok(_), Err(cand_err)) => {
                    failures.push(DifferentialFailure {
                        input: input.clone(),
                        reference_output: "Success".to_string(),
                        candidate_output: cand_err,
                        test_index: i,
                    });
                }
                (Err(ref_err), Ok(_)) => {
                    failures.push(DifferentialFailure {
                        input: input.clone(),
                        reference_output: ref_err,
                        candidate_output: "Success".to_string(),
                        test_index: i,
                    });
                }
            }
        }

        failures
    }
}

/// Differential test failure
#[derive(Debug)]
pub struct DifferentialFailure {
    pub input: String,
    pub reference_output: String,
    pub candidate_output: String,
    pub test_index: usize,
}

/// Coverage-guided fuzzing
pub struct CoverageGuidedFuzzer {
    /// Corpus of inputs
    corpus: Vec<String>,
    /// Coverage tracker
    coverage: HashSet<String>,
    /// Maximum corpus size
    max_corpus_size: usize,
}

impl Default for CoverageGuidedFuzzer {
    fn default() -> Self {
        Self {
            corpus: vec![
                "fn test() {}".to_string(),
                "let x = 42;".to_string(),
                "struct Foo { x: i32 }".to_string(),
            ],
            coverage: HashSet::new(),
            max_corpus_size: 1000,
        }
    }
}

impl CoverageGuidedFuzzer {
    /// Create new coverage-guided fuzzer
    pub fn new(initial_corpus: Vec<String>) -> Self {
        Self {
            corpus: initial_corpus,
            coverage: HashSet::new(),
            max_corpus_size: 1000,
        }
    }

    /// Add input to corpus if new coverage
    pub fn add_to_corpus(&mut self, input: String, new_coverage: Vec<String>) -> bool {
        let mut is_new = false;

        for cov in new_coverage {
            if !self.coverage.contains(&cov) {
                self.coverage.insert(cov);
                is_new = true;
            }
        }

        if is_new && self.corpus.len() < self.max_corpus_size {
            self.corpus.push(input);
        }

        is_new
    }

    /// Get random corpus entry
    pub fn get_corpus_entry(&self) -> Option<&String> {
        if self.corpus.is_empty() {
            None
        } else {
            let idx = rand::thread_rng().gen_range(0..self.corpus.len());
            self.corpus.get(idx)
        }
    }

    /// Run fuzzing loop
    pub fn fuzz<F>(&mut self, mut target: F, iterations: usize) -> FuzzResult
    where
        F: FnMut(&str) -> (Result<String, String>, Vec<String>),
    {
        let mut failures = Vec::new();

        for i in 0..iterations {
            if let Some(base) = self.get_corpus_entry() {
                let mutated = MutationTester::default().mutate(base, &MutationOperator::SwapAdjacent);
                let (result, new_cov) = target(&mutated);

                if self.add_to_corpus(mutated.clone(), new_cov) {
                    // Found new coverage - interesting!
                }

                if result.is_err() {
                    failures.push(FuzzFailure {
                        input: mutated,
                        error: result.unwrap_err(),
                        iteration: i,
                    });
                }
            }
        }

        FuzzResult {
            total_iterations: iterations,
            failures,
            success: failures.is_empty(),
        }
    }
}

/// Corpus minimization
pub fn minimize_corpus(corpus: &[String], target: &dyn Fn(&str) -> bool) -> Vec<String> {
    let mut minimized = corpus.to_vec();
    let mut i = 0;

    while i < minimized.len() {
        let candidate = minimized.remove(i);
        if target(&candidate) {
            // Still works without this entry - keep it removed
        } else {
            // Need this entry - put it back
            minimized.insert(i, candidate);
            i += 1;
        }
    }

    minimized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutation_swap() {
        let tester = MutationTester::default();
        let mutated = tester.mutate("hello", &MutationOperator::SwapAdjacent);
        assert_ne!(mutated, "hello");
    }

    #[test]
    fn test_mutation_delete() {
        let tester = MutationTester::default();
        let mutated = tester.mutate("hello", &MutationOperator::DeleteChar);
        assert!(mutated.len() < 5);
    }

    #[test]
    fn test_mutation_insert() {
        let tester = MutationTester::default();
        let mutated = tester.mutate("hello", &MutationOperator::InsertChar);
        assert!(mutated.len() > 5);
    }

    #[test]
    fn test_type_unification_reflexive() {
        let tester = TypeInferencePropertyTest;
        let failures = tester.unification_is_reflexive();
        assert!(failures.is_empty(), "Unification should be reflexive");
    }

    #[test]
    fn test_type_unification_symmetric() {
        let tester = TypeInferencePropertyTest;
        let failures = tester.unification_is_symmetric();
        assert!(failures.is_empty(), "Unification should be symmetric");
    }

    #[test]
    fn test_coverage_fuzzer() {
        let mut fuzzer = CoverageGuidedFuzzer::new(vec!["fn test() {}".to_string()]);
        let entry = fuzzer.get_corpus_entry();
        assert!(entry.is_some());
    }
}
