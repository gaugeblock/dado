//! Structural mixin composition and envelope chaining.

use crate::error::{DadoError, Result};
use crate::hole::parse_inline_holes;
use crate::model::{DadoMixin, DadoPattern};
use std::collections::BTreeSet;

/// Applies a single `DadoMixin` onto a base `DadoPattern`.
///
/// Multi-Mixin Chaining:
/// Finds the target hole in `base.template_source` (default "body"), replaces `?[__inner__]`
/// inside `mixin.envelope_template` with the complete original hole tag (preserving its id, type,
/// and AST kind), then substitutes the wrapped envelope into the template.
pub fn apply_mixin(base: &DadoPattern, mixin: &DadoMixin) -> Result<DadoPattern> {
    let inline_holes = parse_inline_holes(&base.template_source)?;
    let target_span = inline_holes
        .iter()
        .find(|h| h.hole_id == mixin.target_hole_id)
        .ok_or_else(|| DadoError::TargetHoleNotFound {
            target_hole_id: mixin.target_hole_id.clone(),
            pattern_id: base.pattern_id.clone(),
        })?;

    let original_tag = &base.template_source[target_span.span.0..target_span.span.1];

    // Find ?[__inner__] in the mixin envelope template
    let envelope = if mixin.envelope_template.contains("?[__inner__]") {
        mixin
            .envelope_template
            .replace("?[__inner__]", original_tag)
    } else {
        // Also support parameterized ?[__inner__: ...] if present
        let inner_holes = parse_inline_holes(&mixin.envelope_template)?;
        if let Some(inner) = inner_holes.iter().find(|h| h.hole_id == "__inner__") {
            let mut s = mixin.envelope_template.clone();
            s.replace_range(inner.span.0..inner.span.1, original_tag);
            s
        } else {
            return Err(DadoError::HoleSyntax {
                message: format!(
                    "Mixin '{}' envelope missing '?[__inner__]' placeholder",
                    mixin.mixin_id
                ),
                offset: 0,
            });
        }
    };

    // Replace target hole in base with the constructed envelope
    let mut new_template = base.template_source.clone();
    new_template.replace_range(target_span.span.0..target_span.span.1, &envelope);

    // Merge imports preserving order and deduplicating
    let mut merged_imports = base.required_imports.clone();
    for imp in &mixin.required_imports {
        if !merged_imports.contains(imp) {
            merged_imports.push(imp.clone());
        }
    }

    // Merge holes: original target hole remains alive plus mixin additional holes
    let mut merged_holes = base.holes.clone();
    let mut existing_ids: BTreeSet<String> =
        merged_holes.iter().map(|h| h.hole_id.clone()).collect();
    for add_hole in &mixin.additional_holes {
        if !existing_ids.contains(&add_hole.hole_id) {
            existing_ids.insert(add_hole.hole_id.clone());
            merged_holes.push(add_hole.clone());
        }
    }

    // Merge canonical fills
    let mut merged_fills = base.canonical_example_fills.clone();
    for (k, v) in &mixin.canonical_example_fills {
        merged_fills.insert(k.clone(), v.clone());
    }

    Ok(DadoPattern {
        pattern_id: format!("{}+{}", base.pattern_id, mixin.mixin_id),
        language: base.language,
        description: format!("{} with mixin {}", base.description, mixin.mixin_id),
        template_source: new_template,
        required_imports: merged_imports,
        holes: merged_holes,
        canonical_example_fills: merged_fills,
    })
}

/// Applies multiple mixins sequentially in order onto a base pattern.
pub fn apply_mixins(base: &DadoPattern, mixins: &[&DadoMixin]) -> Result<DadoPattern> {
    let mut current = base.clone();
    for mixin in mixins {
        current = apply_mixin(&current, mixin)?;
    }
    Ok(current)
}
