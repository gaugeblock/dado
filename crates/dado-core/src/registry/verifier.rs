//! Pack verification runner against `canonical_example_fills`.

use super::PatternRegistry;
use crate::hole::validate_pattern_holes;
use crate::model::PatternVerificationReceipt;
use crate::splice::{SpliceOptions, splice_pattern};

/// Verifies all registered patterns against their declared `canonical_example_fills`.
///
/// Ensures:
/// 1. Declared manifest holes match inline `?[...]` spans exactly.
/// 2. Splicing with canonical fills succeeds with 0 Tree-sitter ERROR/MISSING nodes.
pub fn verify_pack(registry: &PatternRegistry) -> Vec<PatternVerificationReceipt> {
    let mut receipts = Vec::new();

    for pattern in registry.patterns.values() {
        let mut diagnostics = Vec::new();

        if let Err(e) = validate_pattern_holes(pattern) {
            diagnostics.push(format!("Hole validation failed: {e}"));
        }

        let options = SpliceOptions {
            available_imports: None,
            include_imports_header: true,
        };

        match splice_pattern(pattern, &pattern.canonical_example_fills, &options) {
            Ok(res) => {
                let mut diag = res.diagnostics;
                diagnostics.append(&mut diag);
                let passed = res.ast_valid && diagnostics.is_empty();
                receipts.push(PatternVerificationReceipt {
                    pattern_id: pattern.pattern_id.clone(),
                    passed,
                    injected_imports_count: res.injected_imports.len(),
                    diagnostics,
                });
            }
            Err(e) => {
                diagnostics.push(format!("Splicing failed: {e}"));
                receipts.push(PatternVerificationReceipt {
                    pattern_id: pattern.pattern_id.clone(),
                    passed: false,
                    injected_imports_count: 0,
                    diagnostics,
                });
            }
        }
    }

    receipts
}
