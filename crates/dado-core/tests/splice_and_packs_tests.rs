#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Tests for embedded packs, pattern registry, and tree-sitter validated splicing.

use dado_core::model::DadoPattern;
use dado_core::registry::PatternRegistry;
use dado_core::registry::verifier::verify_pack;
use dado_core::splice::{SpliceOptions, splice_pattern};
use miter_engine::Language;
use std::collections::BTreeMap;

#[test]
fn test_embedded_packs_and_verify_pack() {
    let registry = PatternRegistry::new().expect("should load embedded packs");
    let patterns = registry.list_patterns(None, None);
    let mixins = registry.list_mixins(None);

    // At least 15 starter patterns (10 Rust + 5 TS) and 5 mixins
    assert!(
        patterns.len() >= 15,
        "Expected >=15 patterns, found {}",
        patterns.len()
    );
    assert!(
        mixins.len() >= 5,
        "Expected >=5 mixins, found {}",
        mixins.len()
    );

    let receipts = verify_pack(&registry);
    assert_eq!(receipts.len(), patterns.len());

    for receipt in &receipts {
        assert!(
            receipt.passed,
            "Pattern '{}' failed pack verification: {:?}",
            receipt.pattern_id, receipt.diagnostics
        );
    }
}

#[test]
fn test_splice_pattern_with_syntax_error_detected() {
    let bad_pattern = DadoPattern {
        pattern_id: "test.broken.rust".into(),
        language: Language::Rust,
        description: "Broken syntax test".into(),
        template_source: "fn test() { ?[bad_expr: Expr @ expression] }".into(),
        required_imports: vec![],
        holes: vec![dado_core::model::TypedHole::new(
            "bad_expr",
            "Expr",
            "expression",
        )],
        canonical_example_fills: BTreeMap::new(),
    };

    let fills = BTreeMap::from([("bad_expr".into(), "let = = = ;".into())]);
    let opts = SpliceOptions::default();
    let res = splice_pattern(&bad_pattern, &fills, &opts).expect("splicing executes");

    assert!(
        !res.ast_valid,
        "Tree-sitter should flag invalid Rust syntax"
    );
    assert!(!res.diagnostics.is_empty());
}
