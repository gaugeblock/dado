//! Tree-sitter syntax and boundary validation, delegated to `miter-polyglot`'s grammar table.
//!
//! This module used to own a second parser: its own language-to-grammar match and its own copies of
//! `tree-sitter-rust` and `tree-sitter-typescript`. That copy had drifted - Python was rejected with
//! "not yet enabled" while the workspace grammar table had read it for months (`GB-148`). The
//! grammar table, the parse, and the diagnostics now live in `miter-polyglot`; this module
//! converts dado's wire language and formats the messages the splice pipeline reports.
//!
//! The import names `miter-polyglot` directly rather than reaching through `miter-engine`'s
//! re-export (`GB-204`): the engine is a facade, and a facade dependency for one module's grammar
//! calls is a lateral edge between two products for no benefit.

use crate::error::{DadoError, Result};
use gaugeblock_contracts::Language;
use miter_polyglot::Language as PolyglotLanguage;
use miter_polyglot::syntax::{
    SyntaxDiagnostic, SyntaxDiagnosticKind, span_within_node, syntax_diagnostics,
};

/// Validates whether `code` parses cleanly with zero ERROR or MISSING syntax nodes.
///
/// Returns the diagnostics as human-readable lines; an empty vector means the grammar accepted the
/// code.
pub fn validate_syntax(code: &str, lang: Language) -> Result<(bool, Vec<String>)> {
    let polyglot = polyglot_language(lang)?;
    let diagnostics = syntax_diagnostics(code, polyglot).map_err(tree_sitter_error)?;

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
    let Ok(polyglot) = polyglot_language(lang) else {
        return Ok(true);
    };
    match span_within_node(code, polyglot, start_byte, end_byte) {
        Ok(within) => Ok(within),
        // No grammar compiled for this format: nothing to check against.
        Err(_) => Ok(true),
    }
}

/// Maps dado's wire language onto the workspace's polyglot taxonomy.
fn polyglot_language(lang: Language) -> Result<PolyglotLanguage> {
    let polyglot = PolyglotLanguage::from(lang);
    if polyglot.index_language().is_none() {
        return Err(DadoError::TreeSitter(format!(
            "'{name}' is a declarative format with no syntax tree to validate",
            name = lang.name()
        )));
    }
    Ok(polyglot)
}

/// Turns a `miter-engine` failure into the splice pipeline's own error.
fn tree_sitter_error(error: miter_polyglot::PolyglotError) -> DadoError {
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
