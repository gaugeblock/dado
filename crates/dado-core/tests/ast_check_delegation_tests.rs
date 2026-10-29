//! Regression tests for GB-148: dado must validate with the workspace grammar table, not its own.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use dado_core::splice::ast_check::{check_hole_boundary, validate_syntax};
use gaugeblock_contracts::Language;

#[test]
fn python_validates_through_the_shared_grammar_table() {
    // dado used to answer "Python grammar not yet enabled in dado" while miter read Python daily.
    let python = "def greet(name):\n    return f\"hello {name}\"\n";
    let (valid, diagnostics) = validate_syntax(python, Language::Python).expect("python parses");

    assert!(valid, "well-formed Python must validate: {diagnostics:?}");
    assert!(diagnostics.is_empty());
}

#[test]
fn a_broken_python_source_reports_its_position() {
    let python = "def greet(name)\n    return 1\n";
    let (valid, diagnostics) = validate_syntax(python, Language::Python).expect("python parses");

    assert!(!valid, "a missing colon is a syntax error");
    assert!(
        diagnostics.iter().any(|d| d.contains("Syntax")),
        "diagnostics name the problem: {diagnostics:?}"
    );
}

#[test]
fn rust_errors_still_render_the_established_shape() {
    let (valid, diagnostics) =
        validate_syntax("fn main( {\n", Language::Rust).expect("rust parses");

    assert!(!valid);
    assert!(
        diagnostics[0].starts_with("Syntax ERROR at L"),
        "the splice report format is unchanged: {}",
        diagnostics[0]
    );
}

#[test]
fn a_declarative_format_is_still_refused_by_name() {
    let error = validate_syntax("key = \"value\"\n", Language::Env).expect_err("no grammar");
    let message = error.to_string();
    assert!(
        message.contains("declarative format with no syntax tree"),
        "got: {message}"
    );
}

#[test]
fn hole_boundaries_are_answered_for_a_grammar_backed_language() {
    let code = "fn one() {}\nfn two() {}\n";
    assert!(
        check_hole_boundary(code, Language::Rust, 0, 12).expect("boundary"),
        "a span inside the first item is a node range"
    );
    assert!(
        check_hole_boundary(code, Language::Rust, 0, code.len()).expect("boundary"),
        "a whole-file span is still inside the root node"
    );
    assert!(
        check_hole_boundary("key = 1\n", Language::Env, 0, 3).expect("boundary"),
        "a declarative format has no boundaries to violate"
    );
}
