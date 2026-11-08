#![no_main]

//! Fuzz targets for the source-language frontends.
//!
//! The Go, Python and TypeScript frontends are recursive descent parsers fed
//! by hand written lexers. Arbitrary input must never make them panic or hang;
//! syntax errors are expected and ignored.

use libfuzzer_sys::fuzz_target;
use tariro_agi_core::tumc_compiler::lang_go::GoParser;
use tariro_agi_core::tumc_compiler::lang_python::PythonParser;
use tariro_agi_core::tumc_compiler::lang_typescript::TypeScriptParser;

fuzz_target!(|data: &[u8]| {
    let Ok(source) = std::str::from_utf8(data) else {
        return;
    };

    let _ = GoParser::new(source).parse();
    let _ = PythonParser::new(source).parse();
    let _ = TypeScriptParser::new(source).parse();
});
