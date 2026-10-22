//! Tree-sitter AST syntax and boundary validator.

use crate::error::{DadoError, Result};
use miter_engine::Language;
use tree_sitter::{Node, Parser};

/// Validates whether `code` parses cleanly with zero ERROR or MISSING syntax nodes.
pub fn validate_syntax(code: &str, lang: Language) -> Result<(bool, Vec<String>)> {
    let mut parser = Parser::new();
    let grammar: tree_sitter::Language = match lang {
        Language::Rust => tree_sitter_rust::LANGUAGE.into(),
        Language::TypeScript | Language::Tsx | Language::JavaScript => {
            tree_sitter_typescript::LANGUAGE_TSX.into()
        }
        Language::Python => {
            return Err(DadoError::TreeSitter(
                "Python grammar not yet enabled in dado".into(),
            ));
        }
    };

    parser
        .set_language(&grammar)
        .map_err(|e| DadoError::TreeSitter(format!("Failed to load grammar for {lang:?}: {e}")))?;

    let tree = parser.parse(code, None).ok_or_else(|| {
        DadoError::TreeSitter("Tree-sitter parser returned no syntax tree".into())
    })?;

    let mut diagnostics = Vec::new();
    collect_syntax_errors(tree.root_node(), code, &mut diagnostics);

    let is_valid = diagnostics.is_empty();
    Ok((is_valid, diagnostics))
}

fn collect_syntax_errors(node: Node<'_>, source: &str, diagnostics: &mut Vec<String>) {
    if node.is_error() {
        let start = node.start_position();
        let snippet = node_snippet(node, source);
        diagnostics.push(format!(
            "Syntax ERROR at L{}:C{} (byte {}..{}): '{}'",
            start.row + 1,
            start.column + 1,
            node.start_byte(),
            node.end_byte(),
            snippet
        ));
    } else if node.is_missing() {
        let start = node.start_position();
        diagnostics.push(format!(
            "Syntax MISSING node '{}' at L{}:C{}",
            node.kind(),
            start.row + 1,
            start.column + 1
        ));
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_syntax_errors(child, source, diagnostics);
    }
}

fn node_snippet(node: Node<'_>, source: &str) -> String {
    let start = node.start_byte();
    let end = node.end_byte().min(source.len());
    if start >= end {
        return String::new();
    }
    let slice = &source[start..end];
    if slice.len() > 40 {
        format!("{}...", slice[..40].replace('\n', " "))
    } else {
        slice.replace('\n', " ")
    }
}

/// Confirms that a hole span `(start_byte, end_byte)` falls cleanly within an AST node boundary.
pub fn check_hole_boundary(
    code: &str,
    lang: Language,
    start_byte: usize,
    end_byte: usize,
) -> Result<bool> {
    let mut parser = Parser::new();
    let grammar: tree_sitter::Language = match lang {
        Language::Rust => tree_sitter_rust::LANGUAGE.into(),
        Language::TypeScript | Language::Tsx | Language::JavaScript => {
            tree_sitter_typescript::LANGUAGE_TSX.into()
        }
        _ => return Ok(true),
    };

    parser
        .set_language(&grammar)
        .map_err(|e| DadoError::TreeSitter(e.to_string()))?;

    let tree = match parser.parse(code, None) {
        Some(t) => t,
        None => return Ok(false),
    };

    let root = tree.root_node();
    let desc = root.descendant_for_byte_range(start_byte, end_byte);
    Ok(desc.is_some())
}
