//! Inline hole parser and template validator.

use crate::error::{DadoError, Result};
use crate::model::{DadoPattern, TypedHole};
use std::collections::{BTreeMap, BTreeSet};

/// A parsed inline hole marker with its byte offsets in the template string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineHoleSpan {
    /// Identifier of the hole (e.g. "query_sql").
    pub hole_id: String,
    /// Declared expected type (e.g. "&str", "AppError").
    pub expected_type: String,
    /// Target AST node kind if specified inline with `@ kind`.
    pub ast_node_kind: Option<String>,
    /// Byte offsets `(start, end)` inclusive of `?[` and `]`.
    pub span: (usize, usize),
}

/// Parses all `?[hole_id: ExpectedType]` or `?[hole_id: ExpectedType @ kind]` markers.
pub fn parse_inline_holes(template: &str) -> Result<Vec<InlineHoleSpan>> {
    let mut holes = Vec::new();
    let mut seen_ids = BTreeSet::new();
    let mut idx = 0;
    let bytes = template.as_bytes();

    while idx < bytes.len() {
        if idx + 1 < bytes.len() && bytes[idx] == b'?' && bytes[idx + 1] == b'[' {
            let start = idx;
            idx += 2;
            let content_start = idx;
            let mut found_end = false;

            while idx < bytes.len() {
                if bytes[idx] == b']' {
                    found_end = true;
                    break;
                }
                idx += 1;
            }

            if !found_end {
                return Err(DadoError::HoleSyntax {
                    message: "Unclosed '?[' hole delimiter in template".to_string(),
                    offset: start,
                });
            }

            let end = idx + 1;
            let raw_content = template[content_start..idx].trim();
            let (hole_id, expected_type, ast_node_kind) = parse_hole_content(raw_content, start)?;

            if !seen_ids.insert(hole_id.clone()) {
                return Err(DadoError::DuplicateHole { hole_id });
            }

            holes.push(InlineHoleSpan {
                hole_id,
                expected_type,
                ast_node_kind,
                span: (start, end),
            });

            idx += 1;
        } else {
            idx += 1;
        }
    }

    Ok(holes)
}

fn parse_hole_content(content: &str, offset: usize) -> Result<(String, String, Option<String>)> {
    let colon_pos = content.find(':').ok_or_else(|| DadoError::HoleSyntax {
        message: "Missing ':' separator in hole marker. Expected '?[id: Type]'".to_string(),
        offset,
    })?;

    let hole_id = content[..colon_pos].trim().to_string();
    if hole_id.is_empty() || !hole_id.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(DadoError::HoleSyntax {
            message: format!(
                "Invalid hole identifier '{hole_id}'. Must be alphanumeric/underscores"
            ),
            offset,
        });
    }

    let remainder = content[colon_pos + 1..].trim();
    if let Some(at_pos) = remainder.find('@') {
        let expected_type = remainder[..at_pos].trim().to_string();
        let ast_node_kind = remainder[at_pos + 1..].trim().to_string();
        if expected_type.is_empty() || ast_node_kind.is_empty() {
            return Err(DadoError::HoleSyntax {
                message: "Empty type or ast_node_kind in '@' hole marker".to_string(),
                offset,
            });
        }
        Ok((hole_id, expected_type, Some(ast_node_kind)))
    } else {
        if remainder.is_empty() {
            return Err(DadoError::HoleSyntax {
                message: "Empty expected type in hole marker".to_string(),
                offset,
            });
        }
        Ok((hole_id, remainder.to_string(), None))
    }
}

/// Validates that inline holes match declared `TypedHole` metadata in the pattern.
pub fn validate_pattern_holes(pattern: &DadoPattern) -> Result<Vec<InlineHoleSpan>> {
    let inline_spans = parse_inline_holes(&pattern.template_source)?;
    let declared_map: BTreeMap<&str, &TypedHole> = pattern
        .holes
        .iter()
        .map(|h| (h.hole_id.as_str(), h))
        .collect();

    for inline in &inline_spans {
        let declared =
            declared_map
                .get(inline.hole_id.as_str())
                .ok_or_else(|| DadoError::HoleMismatch {
                    hole_id: inline.hole_id.clone(),
                    expected: "Declared in manifest holes list".to_string(),
                    found: "Only present inline in template".to_string(),
                })?;

        if declared.expected_type.trim() != inline.expected_type.trim() {
            return Err(DadoError::HoleMismatch {
                hole_id: inline.hole_id.clone(),
                expected: declared.expected_type.clone(),
                found: inline.expected_type.clone(),
            });
        }

        if let Some(ref inline_kind) = inline.ast_node_kind
            && declared.ast_node_kind.trim() != inline_kind.trim()
        {
            return Err(DadoError::HoleMismatch {
                hole_id: inline.hole_id.clone(),
                expected: declared.ast_node_kind.clone(),
                found: inline_kind.clone(),
            });
        }
    }

    for declared in &pattern.holes {
        if !inline_spans.iter().any(|s| s.hole_id == declared.hole_id) {
            return Err(DadoError::HoleMismatch {
                hole_id: declared.hole_id.clone(),
                expected: "Present in template source".to_string(),
                found: "Missing in template source".to_string(),
            });
        }
    }

    Ok(inline_spans)
}
