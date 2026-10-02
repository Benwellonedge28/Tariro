//! TUMC Compiler Comprehensive Tests
//!
//! This module provides comprehensive test coverage for the TUMC compiler
//! including lexer, parser, type checker, optimizer, and code generator tests.

#[cfg(test)]
mod tests {
    use tariro::tumc_compiler::*;

    // =======================================================================
    // Lexer Tests
    // =======================================================================

    mod lexer_tests {
        use super::*;

        #[test]
        fn test_lexer_basic_tokens() {
            let source = "fn main() -> i32 { 42 }";
            let lexer = lexer::Lexer::new(source);
            let tokens = lexer.tokenize().expect("Lexer should tokenize successfully");

            assert!(!tokens.is_empty());
            assert_eq!(tokens[0].kind, lexer::TokenKind::Fn);
            assert_eq!(tokens[1].kind, lexer::TokenKind::Identifier);
            assert_eq!(tokens[1].lexeme, "main");
        }

        #[test]
        fn test_lexer_numbers() {
            let source = "42 3.14 0xFF 0b1010";
            let lexer = lexer::Lexer::new(source);
            let tokens = lexer.tokenize().expect("Lexer should tokenize numbers");

            assert!(tokens.iter().any(|t| t.kind == lexer::TokenKind::IntLiteral));
            assert!(tokens.iter().any(|t| t.kind == lexer::TokenKind::FloatLiteral));
        }

        #[test]
        fn test_lexer_strings() {
            let source = "\"hello\" \"world\\n\"";
            let lexer = lexer::Lexer::new(source);
            let tokens = lexer.tokenize().expect("Lexer should tokenize strings");

            assert!(tokens.iter().any(|t| t.kind == lexer::TokenKind::StringLiteral));
        }

        #[test]
        fn test_lexer_keywords() {
            let keywords = ["fn", "let", "mut", "if", "else", "loop", "while", "return", "struct", "enum"];
            for kw in keywords {
                let source = kw;
                let lexer = lexer::Lexer::new(source);
                let tokens = lexer.tokenize().expect("Lexer should tokenize keyword");
                assert!(!tokens.is_empty(), "Keyword '{}' should produce tokens", kw);
            }
        }

        #[test]
        fn test_lexer_operators() {
            let source = "+ - * / % = == != < > <= >= && || ! & | ^ << >>";
            let lexer = lexer::Lexer::new(source);
            let tokens = lexer.tokenize().expect("Lexer should tokenize operators");

            assert!(tokens.iter().any(|t| t.kind == lexer::TokenKind::Plus));
            assert!(tokens.iter().any(|t| t.kind == lexer::TokenKind::Star));
            assert!(tokens.iter().any(|t| t.kind == lexer::TokenKind::EqEq));
        }

        #[test]
        fn test_lexer_comments() {
            let source = "// This is a comment\n42 // another comment";
            let lexer = lexer::Lexer::new(source);
            let tokens = lexer.tokenize().expect("Lexer should handle comments");

            // Comments should be filtered out, leaving only the number
            assert!(tokens.iter().any(|t| matches!(t.kind, lexer::TokenKind::IntLiteral)));
        }

        #[test]
        fn test_lexer_multiline_comment() {
            let source = "/* This is\na multiline\ncomment */ 42";
            let lexer = lexer::Lexer::new(source);
            let tokens = lexer.tokenize().expect("Lexer should handle multiline comments");

            assert!(tokens.iter().any(|t| matches!(t.kind, lexer::TokenKind::IntLiteral)));
        }
    }

    // =======================================================================
    // Parser Tests
    // =======================================================================

    mod parser_tests {
        use super::*;

        #[test]
        fn test_parse_expression() {
            let source = "let x = 42;";
            let tokens = lexer::Lexer::new(source)
                .tokenize()
                .expect("Should tokenize");
            let ast = parser::Parser::new(tokens)
                .parse()
                .expect("Should parse");

            assert!(!ast.items.is_empty());
        }

        #[test]
        fn test_parse_function() {
            let source = r#"
                fn add(a: i32, b: i32) -> i32 {
                    a + b
                }
            "#;
            let tokens = lexer::Lexer::new(source)
                .tokenize()
                .expect("Should tokenize");
            let ast = parser::Parser::new(tokens)
                .parse()
                .expect("Should parse function");

            assert!(!ast.items.is_empty());
        }

        #[test]
        fn test_parse_struct() {
            let source = r#"
                struct Point {
                    x: f64,
                    y: f64,
                }
            "#;
            let tokens = lexer::Lexer::new(source)
                .tokenize()
                .expect("Should tokenize");
            let ast = parser::Parser::new(tokens)
                .parse()
                .expect("Should parse struct");

            assert!(!ast.items.is_empty());
        }

        #[test]
        fn test_parse_enum() {
            let source = r#"
                enum Option {
                    Some(T),
                    None,
                }
            "#;
            let tokens = lexer::Lexer::new(source)
                .tokenize()
                .expect("Should tokenize");
            let ast = parser::Parser::new(tokens)
                .parse()
                .expect("Should parse enum");

            assert!(!ast.items.is_empty());
        }

        #[test]
        fn test_parse_if_expression() {
            let source = "if x > 0 { 1 } else { -1 }";
            let tokens = lexer::Lexer::new(source)
                .tokenize()
                .expect("Should tokenize");
            let ast = parser::Parser::new(tokens)
                .parse()
                .expect("Should parse if expression");

            assert!(!ast.items.is_empty());
        }

        #[test]
        fn test_parse_loop() {
            let source = r#"
                fn count() {
                    let mut i = 0;
                    loop {
                        i = i + 1;
                        if i >= 10 {
                            break;
                        }
                    }
                }
            "#;
            let tokens = lexer::Lexer::new(source)
                .tokenize()
                .expect("Should tokenize");
            let ast = parser::Parser::new(tokens)
                .parse()
                .expect("Should parse loop");

            assert!(!ast.items.is_empty());
        }
    }

    // =======================================================================
    // Type Checker Tests
    // =======================================================================

    mod type_checker_tests {
        use super::*;

        #[test]
        fn test_type_inference_int() {
            let source = "let x = 42;";
            let tokens = lexer::Lexer::new(source).tokenize().unwrap();
            let ast = parser::Parser::new(tokens).parse().unwrap();

            let mut type_context = type_checker::TypeContext::default();
            let result = type_checker::TypeChecker::new(&mut type_context).check(&ast);

            assert!(result.is_ok(), "Type inference should succeed");
        }

        #[test]
        fn test_type_inference_function() {
            let source = r#"
                fn identity(x: i32) -> i32 {
                    x
                }
            "#;
            let tokens = lexer::Lexer::new(source).tokenize().unwrap();
            let ast = parser::Parser::new(tokens).parse().unwrap();

            let mut type_context = type_checker::TypeContext::default();
            let result = type_checker::TypeChecker::new(&mut type_context).check(&ast);

            assert!(result.is_ok(), "Function type inference should succeed");
        }

        #[test]
        fn test_type_unification() {
            let t1 = Type::I32;
            let t2 = Type::I64;

            // Numeric types can unify
            let result = t1.unify(&t2);
            assert!(result.is_some());
        }

        #[test]
        fn test_type_function_unify() {
            let fn1 = Type::Function(Box::new(FunctionType {
                parameters: vec![Type::I32],
                return_type: Box::new(Type::I32),
                variadic: false,
            }));
            let fn2 = Type::Function(Box::new(FunctionType {
                parameters: vec![Type::I64],
                return_type: Box::new(Type::I32),
                variadic: false,
            }));

            let result = fn1.unify(&fn2);
            assert!(result.is_some());
        }

        #[test]
        fn test_type_size_calculation() {
            assert_eq!(Type::I32.size(), Some(4));
            assert_eq!(Type::F64.size(), Some(8));
            assert_eq!(Type::Bool.size(), Some(1));
            assert_eq!(Type::Unit.size(), Some(0));
        }

        #[test]
        fn test_type_name() {
            assert_eq!(Type::I32.name(), "i32");
            assert_eq!(Type::F64.name(), "f64");
            assert_eq!(Type::Bool.name(), "bool");
            assert_eq!(Type::String.name(), "string");
        }
    }

    // =======================================================================
    // Optimizer Tests
    // =======================================================================

    mod optimizer_tests {
        use super::*;

        #[test]
        fn test_constant_folding() {
            let source = r#"
                const A: i32 = 2 + 3;
                const B: i32 = A * 4;
            "#;
            let tokens = lexer::Lexer::new(source).tokenize().unwrap();
            let ast = parser::Parser::new(tokens).parse().unwrap();

            let mut type_context = type_checker::TypeContext::default();
            let typed_ast = type_checker::TypeChecker::new(&mut type_context)
                .check(&ast)
                .unwrap();

            let optimized = optimizer::Optimizer::new(OptimizationLevel::Standard)
                .optimize(typed_ast);

            assert!(!optimized.items.is_empty());
        }

        #[test]
        fn test_dead_code_elimination() {
            let source = r#"
                fn test() {
                    return 42;
                    let x = 100; // Dead code
                }
            "#;
            let tokens = lexer::Lexer::new(source).tokenize().unwrap();
            let ast = parser::Parser::new(tokens).parse().unwrap();

            let mut type_context = type_checker::TypeContext::default();
            let typed_ast = type_checker::TypeChecker::new(&mut type_context)
                .check(&ast)
                .unwrap();

            let optimized = optimizer::Optimizer::new(OptimizationLevel::Standard)
                .optimize(typed_ast);

            assert!(!optimized.items.is_empty());
        }
    }

    // =======================================================================
    // Code Generator Tests
    // =======================================================================

    mod codegen_tests {
        use super::*;

        #[test]
        fn test_wasm_codegen() {
            let source = "fn main() -> i32 { 42 }";
            let tokens = lexer::Lexer::new(source).tokenize().unwrap();
            let ast = parser::Parser::new(tokens).parse().unwrap();

            let mut type_context = type_checker::TypeContext::default();
            let typed_ast = type_checker::TypeChecker::new(&mut type_context)
                .check(&ast)
                .unwrap();

            let optimized = optimizer::Optimizer::new(OptimizationLevel::Standard)
                .optimize(typed_ast);

            let codegen = codegen::CodeGenerator::new(TargetPlatform::Wasm);
            let result = codegen.generate(&optimized);

            assert!(result.is_ok(), "WASM codegen should succeed");
            if let Ok(output) = result {
                assert!(!output.is_empty(), "WASM output should not be empty");
            }
        }

        #[test]
        fn test_wat_output() {
            let source = "fn main() -> i32 { 42 }";
            let tokens = lexer::Lexer::new(source).tokenize().unwrap();
            let ast = parser::Parser::new(tokens).parse().unwrap();

            let mut type_context = type_checker::TypeContext::default();
            let typed_ast = type_checker::TypeChecker::new(&mut type_context)
                .check(&ast)
                .unwrap();

            let codegen = codegen::CodeGenerator::new(TargetPlatform::Wasm);
            let result = codegen.generate(&optimized);

            assert!(result.is_ok());
        }
    }

    // =======================================================================
    // Integration Tests
    // =======================================================================

    mod integration_tests {
        use super::*;

        #[test]
        fn test_full_compilation_pipeline() {
            let source = r#"
                fn add(a: i32, b: i32) -> i32 {
                    a + b
                }

                fn main() -> i32 {
                    add(1, 2)
                }
            "#;

            let mut compiler = Compiler::new(SourceLanguage::TumcIR, TargetPlatform::Wasm);
            compiler.set_optimization(OptimizationLevel::Standard);

            let result = compiler.compile(source);

            assert!(result.success, "Full compilation should succeed");
            assert!(!result.output.is_empty(), "Output should not be empty");
            assert_eq!(result.diagnostics.len(), 0, "Should have no diagnostics");
        }

        #[test]
        fn test_compilation_with_errors() {
            let source = "fn main() { 42 + }"; // Syntax error
            let mut compiler = Compiler::new(SourceLanguage::TumcIR, TargetPlatform::Wasm);

            let result = compiler.compile(source);

            assert!(!result.success, "Compilation should fail");
            assert!(!result.errors.is_empty(), "Should have errors");
        }

        #[test]
        fn test_optimization_levels() {
            let source = "fn main() -> i32 { 1 + 2 + 3 + 4 }";

            for level in [OptimizationLevel::None, OptimizationLevel::Basic,
                          OptimizationLevel::Standard, OptimizationLevel::Aggressive] {
                let mut compiler = Compiler::new(SourceLanguage::TumcIR, TargetPlatform::Wasm);
                compiler.set_optimization(level);

                let result = compiler.compile(source);
                assert!(result.success, "Compilation at {:?} should succeed", level);
            }
        }

        #[test]
        fn test_multiple_target_platforms() {
            let source = "fn main() -> i32 { 42 }";

            for platform in [TargetPlatform::Wasm, TargetPlatform::TariroBytecode] {
                let mut compiler = Compiler::new(SourceLanguage::TumcIR, platform);
                let result = compiler.compile(source);
                assert!(result.success, "Compilation to {:?} should succeed", platform);
            }
        }
    }

    // =======================================================================
    // SIMD Tests
    // =======================================================================

    mod simd_tests {
        use super::*;

        #[test]
        fn test_simd_instruction_creation() {
            let simd = simd_vectorizer::SimdVectorizer::new(simd_vectorizer::SimdIsa::Avx2);
            assert!(simd.is_available());
        }

        #[test]
        fn test_simd_vector_width() {
            for isa in [
                simd_vectorizer::SimdIsa::Sse,
                simd_vectorizer::SimdIsa::Avx,
                simd_vectorizer::SimdIsa::Avx2,
                simd_vectorizer::SimdIsa::Neon,
            ] {
                let vectorizer = simd_vectorizer::SimdVectorizer::new(isa);
                let width = vectorizer.vector_width();
                assert!(width > 0, "Vector width should be positive for {:?}", isa);
            }
        }
    }

    // =======================================================================
    // Native CodeGen Tests
    // =======================================================================

    mod native_codegen_tests {
        use super::*;

        #[test]
        fn test_native_codegen_creation() {
            let codegen = native_codegen::NativeCodegen::new(native_codegen::Arch::X86_64);
            assert!(codegen.is_available());
        }

        #[test]
        fn test_instruction_encoding() {
            let mut codegen = native_codegen::NativeCodegen::new(native_codegen::Arch::X86_64);
            let instr = native_codegen::Instruction::Mov {
                dest: native_codegen::Operand::Register(native_codegen::Register::Rax),
                src: native_codegen::Operand::Immediate(42),
            };

            let encoded = codegen.encode_instruction(&instr);
            assert!(encoded.is_ok());
        }
    }

    // =======================================================================
    // VM Tests
    // =======================================================================

    mod vm_tests {
        use super::*;

        #[test]
        fn test_vm_creation() {
            let vm = tariro_vm::VM::new(1024);
            assert_eq!(vm.registers(), 256);
        }

        #[test]
        fn test_vm_execution() {
            let bytecode = vec![
                tariro_vm::Opcode::MovImm(tariro_vm::Value::Int(42), 0),
                tariro_vm::Opcode::Halt,
            ];

            let mut vm = tariro_vm::VM::new(1024);
            let result = vm.execute(&bytecode);
            assert!(result.is_ok());
        }

        #[test]
        fn test_vm_stack_operations() {
            let mut vm = tariro_vm::VM::new(1024);

            // Push value
            vm.push_stack(tariro_vm::Value::Int(100)).unwrap();
            assert_eq!(vm.stack_depth(), 1);

            // Pop value
            let val = vm.pop_stack().unwrap();
            assert!(matches!(val, tariro_vm::Value::Int(100)));
        }
    }

    // =======================================================================
    // Macro System Tests
    // =======================================================================

    mod macro_tests {
        use super::*;

        #[test]
        fn test_macro_pattern_matching() {
            let pattern = macro_system::Pattern::Any(macro_system::TypePattern::Any);
            let token = lexer::Token::new(
                lexer::TokenKind::Identifier,
                "x",
                0,
                0,
            );

            assert!(pattern.matches(&token, &mut macro_system::Bindings::new()));
        }

        #[test]
        fn test_macro_expansion() {
            let mut env = macro_system::MacroEnv::new();
            env.define_macro(
                "identity".to_string(),
                macro_system::MacroDefinition::Declarative(vec![
                    macro_system::MacroRule {
                        pattern: macro_system::Pattern::Sequence(vec![
                            macro_system::Pattern::Literal("$".into()),
                            macro_system::Pattern::Capture("expr".into()),
                        ]),
                        template: macro_system::Template::Expr(
                            macro_system::CapturedExpr {
                                name: "expr".into(),
                                fragment: macro_system::FragmentKind::Expr,
                            },
                        ),
                    },
                ]),
            );

            assert!(env.get_macro("identity").is_some());
        }
    }

    // =======================================================================
    // Derive System Tests
    // =======================================================================

    mod derive_tests {
        use super::*;

        #[test]
        fn test_derive_clone() {
            let derive = derive_system::DeriveMacro::Clone;
            assert_eq!(derive.name(), "Clone");
        }

        #[test]
        fn test_derive_debug() {
            let derive = derive_system::DeriveMacro::Debug;
            assert_eq!(derive.name(), "Debug");
        }

        #[test]
        fn test_derive_default() {
            let derive = derive_system::DeriveMacro::Default;
            assert_eq!(derive.name(), "Default");
        }

        #[test]
        fn test_derive_partial_eq() {
            let derive = derive_system::DeriveMacro::PartialEq;
            assert_eq!(derive.name(), "PartialEq");
        }
    }

    // =======================================================================
    // Performance Benchmark Tests
    // =======================================================================

    mod benchmark_tests {
        use super::*;

        #[test]
        fn test_large_source_compilation() {
            let mut source = String::new();
            for i in 0..100 {
                source.push_str(&format!(
                    "fn function_{}() -> i32 {{ {} }}\n",
                    i, i * 2
                ));
            }

            let mut compiler = Compiler::new(SourceLanguage::TumcIR, TargetPlatform::Wasm);
            let result = compiler.compile(&source);

            assert!(result.success);
            assert!(result.stats.compilation_time_ms < 1000,
                    "Compilation should complete within 1 second");
        }
    }
}
