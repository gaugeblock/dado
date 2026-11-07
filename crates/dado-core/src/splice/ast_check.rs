//! Tree-sitter syntax and boundary validation, delegated to the shared grammar table.
//!
//! Grammar and diagnostics come from `gaugeblock-syntax`, the shared table (`GB-246`): `dado` asked
//! `miter-polyglot` the question "does this parse?", which made a code generator depend on a code indexer
//! for something that belongs to neither product.
//!
//! This module used to own a second parser: its own language-to-grammar match and its own copies of
//! `tree-sitter-rust` and `tree-sitter-typescript`. That copy had drifted - Python was rejected with
//! "not yet enabled" while the workspace grammar table had read it for months (`GB-148`). The
//! grammar table, the parse and the diagnostics live in `gaugeblock-syntax`; this module formats the
//! messages the splice pipeline reports.
//!
//! The dependency is downward to a foundation crate (`GB-246`, `GB-204`): asking "does this parse?" is
//! not either product's property, so it is not something one product should reach sideways for.

use crate::error::{DadoError, Result};
use gaugeblock_contracts::Language;
use gaugeblock_syntax::{
    SyntaxDiagnostic, SyntaxDiagnosticKind, SyntaxError, span_within_node, syntax_diagnostics,
};

/// Validates whether `code` parses cleanly with zero ERROR or MISSING syntax nodes.
///
/// Returns the diagnostics as human-readable lines; an empty vector means the grammar accepted the
/// code.
pub fn validate_syntax(code: &str, lang: Language) -> Result<(bool, Vec<String>)> {
    let diagnostics = syntax_diagnostics(code, lang).map_err(tree_sitter_error)?;

    let rendered: Vec<String> = diagnostics.iter().map(describe).collect();
    let is_valid = rendered.is_empty();
    Ok((is_valid, rendered))
}

/// Confirms that a hole span `(start_byte, end_byte)` falls cleanly within an AST node boundary.
///
/// A format without a grammar has no node boundaries to violate, so it is accepted.
pub fn check_hole_boundary(
    code: &str,
    lang: Language,
    start_byte: usize,
    end_byte: usize,
) -> Result<bool> {
    match span_within_node(code, lang, start_byte, end_byte) {
        Ok(within) => Ok(within),
        // No grammar compiled for this format: nothing to check against.
        Err(_) => Ok(true),
    }
}

/// Turns a shared syntax failure into the splice pipeline's own error.
fn tree_sitter_error(error: SyntaxError) -> DadoError {
    DadoError::TreeSitter(error.to_string())
}

/// Renders one diagnostic the way the splice reports have always shown it.
fn describe(diagnostic: &SyntaxDiagnostic) -> String {
    match &diagnostic.kind {
        SyntaxDiagnosticKind::Error => format!(
            "Syntax ERROR at L{}:C{} (byte {}..{}): '{}'",
            diagnostic.line,
            diagnostic.column,
            diagnostic.start_byte,
            diagnostic.end_byte,
            diagnostic.snippet
        ),
        SyntaxDiagnosticKind::Missing { expected } => format!(
            "Syntax MISSING node '{}' at L{}:C{}",
            expected, diagnostic.line, diagnostic.column
        ),
    }
}
