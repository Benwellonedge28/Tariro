#![no_main]

//! Fuzz target for the TUMC lexer + recursive descent parser.
//!
//! The parser must never panic, never hang and never read out of bounds on
//! arbitrary input. Recoverable syntax errors are expected and ignored.

use libfuzzer_sys::fuzz_target;
use tariro_agi_core::tumc_compiler::lexer::Lexer;
use tariro_agi_core::tumc_compiler::parser::Parser;

fuzz_target!(|data: &[u8]| {
    let Ok(source) = std::str::from_utf8(data) else {
        return;
    };

    // A lexer error is a normal outcome for arbitrary input.
    let Ok(tokens) = Lexer::new(source).tokenize() else {
        return;
    };

    let mut parser = Parser::new(tokens);
    let _ = parser.parse();
});
