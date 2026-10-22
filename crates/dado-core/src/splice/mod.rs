//! AST template hole substitution and missing import injection engine.

pub mod ast_check;

use crate::error::{DadoError, Result};
use crate::hole::parse_inline_holes;
use crate::model::{DadoPattern, SpliceResult};
use ast_check::validate_syntax;
use std::collections::BTreeMap;

/// Splicing options for controlling import injection and AST validation.
#[derive(Debug, Clone, Default)]
pub struct SpliceOptions<'a> {
    /// Already available imports in the target file/scope to avoid duplicate imports.
    pub available_imports: Option<&'a [String]>,
    /// Whether to prepend injected imports to the rendered code output.
    pub include_imports_header: bool,
}

/// Substitutes filled hole expressions into the pattern and validates the AST.
pub fn splice_pattern(
    pattern: &DadoPattern,
    fills: &BTreeMap<String, String>,
    options: &SpliceOptions<'_>,
) -> Result<SpliceResult> {
    for hole in &pattern.holes {
        if !fills.contains_key(&hole.hole_id) {
            return Err(DadoError::MissingHoleFill {
                hole_id: hole.hole_id.clone(),
                pattern_id: pattern.pattern_id.clone(),
            });
        }
    }

    let mut inline_spans = parse_inline_holes(&pattern.template_source)?;
    inline_spans.sort_by_key(|h| h.span.0);

    let mut spliced_body = String::new();
    let mut last_idx = 0;
    let mut hole_spans = BTreeMap::new();

    for hole in inline_spans {
        spliced_body.push_str(&pattern.template_source[last_idx..hole.span.0]);
        let start_byte = spliced_body.len();
        let fill_val = fills
            .get(&hole.hole_id)
            .ok_or_else(|| DadoError::MissingHoleFill {
                hole_id: hole.hole_id.clone(),
                pattern_id: pattern.pattern_id.clone(),
            })?;
        spliced_body.push_str(fill_val);
        let end_byte = spliced_body.len();
        hole_spans.insert(hole.hole_id, (start_byte, end_byte));
        last_idx = hole.span.1;
    }
    spliced_body.push_str(&pattern.template_source[last_idx..]);

    let available = options.available_imports.unwrap_or(&[]);
    let mut injected_imports = Vec::new();
    for imp in &pattern.required_imports {
        if !available.contains(imp) && !spliced_body.contains(imp) {
            injected_imports.push(imp.clone());
        }
    }

    let rendered_code = if options.include_imports_header && !injected_imports.is_empty() {
        let imports_header = injected_imports.join("\n") + "\n\n";
        let offset = imports_header.len();
        for span in hole_spans.values_mut() {
            span.0 += offset;
            span.1 += offset;
        }
        format!("{imports_header}{spliced_body}")
    } else {
        spliced_body
    };

    let (ast_valid, diagnostics) = validate_syntax(&rendered_code, pattern.language)?;

    Ok(SpliceResult {
        rendered_code,
        injected_imports,
        hole_spans,
        ast_valid,
        diagnostics,
    })
}
