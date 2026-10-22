#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Unit tests for hole parser and mixin composition.

use dado_core::hole::{parse_inline_holes, validate_pattern_holes};
use dado_core::mixin::apply_mixins;
use dado_core::model::{DadoMixin, DadoPattern, TypedHole};
use miter_engine::Language;
use std::collections::BTreeMap;

#[test]
fn test_parse_inline_holes_valid() {
    let template = "fn handle(req: Request) -> ?[resp_type: Response @ expression] { ?[body: Result<Response, Error>] }";
    let holes = parse_inline_holes(template).expect("should parse valid holes");

    assert_eq!(holes.len(), 2);
    assert_eq!(holes[0].hole_id, "resp_type");
    assert_eq!(holes[0].expected_type, "Response");
    assert_eq!(holes[0].ast_node_kind.as_deref(), Some("expression"));

    assert_eq!(holes[1].hole_id, "body");
    assert_eq!(holes[1].expected_type, "Result<Response, Error>");
    assert_eq!(holes[1].ast_node_kind, None);
}

#[test]
fn test_parse_inline_holes_unclosed() {
    let template = "fn handle() { ?[unclosed: Type ";
    let res = parse_inline_holes(template);
    assert!(res.is_err());
}

#[test]
fn test_parse_inline_holes_duplicate() {
    let template = "let a = ?[x: i32]; let b = ?[x: i32];";
    let res = parse_inline_holes(template);
    assert!(res.is_err());
}

#[test]
fn test_validate_pattern_holes_success() {
    let pattern = DadoPattern {
        pattern_id: "test.pat".into(),
        language: Language::Rust,
        description: "Test".into(),
        template_source: "fn run() -> ?[res: i32 @ expression] { ?[body: i32] }".into(),
        required_imports: vec![],
        holes: vec![
            TypedHole::new("res", "i32", "expression"),
            TypedHole::new("body", "i32", "statement"),
        ],
        canonical_example_fills: BTreeMap::new(),
    };

    let spans = validate_pattern_holes(&pattern).expect("should validate");
    assert_eq!(spans.len(), 2);
}

#[test]
fn test_validate_pattern_holes_mismatch() {
    let pattern = DadoPattern {
        pattern_id: "test.pat".into(),
        language: Language::Rust,
        description: "Test".into(),
        template_source: "fn run() { ?[body: String] }".into(),
        required_imports: vec![],
        holes: vec![TypedHole::new("body", "i32", "statement")],
        canonical_example_fills: BTreeMap::new(),
    };

    let res = validate_pattern_holes(&pattern);
    assert!(res.is_err());
}

#[test]
fn test_multi_mixin_chaining() {
    let base = DadoPattern {
        pattern_id: "base.handler".into(),
        language: Language::Rust,
        description: "Base handler".into(),
        template_source: "async fn handle() -> Result<(), Error> {\n    ?[body: Result<(), Error> @ statement]\n}".into(),
        required_imports: vec!["use axum::response::Response;".into()],
        holes: vec![TypedHole::new("body", "Result<(), Error>", "statement")],
        canonical_example_fills: BTreeMap::from([("body".into(), "Ok(())".into())]),
    };

    let tracing_mixin = DadoMixin {
        mixin_id: "rust.mixin.tracing".into(),
        language: Language::Rust,
        target_hole_id: "body".into(),
        envelope_template: "let _span = tracing::info_span!(\"handler\").entered();\n?[__inner__]"
            .into(),
        required_imports: vec!["use tracing::info_span;".into()],
        additional_holes: vec![],
        canonical_example_fills: BTreeMap::new(),
    };

    let timeout_mixin = DadoMixin {
        mixin_id: "rust.mixin.timeout".into(),
        language: Language::Rust,
        target_hole_id: "body".into(),
        envelope_template: "tokio::time::timeout(std::time::Duration::from_secs(?[timeout_sec: u64 @ expression]), async {\n?[__inner__]\n}).await??".into(),
        required_imports: vec!["use tokio::time::timeout;".into()],
        additional_holes: vec![TypedHole::new("timeout_sec", "u64", "expression")],
        canonical_example_fills: BTreeMap::from([("timeout_sec".into(), "30".into())]),
    };

    let chained =
        apply_mixins(&base, &[&tracing_mixin, &timeout_mixin]).expect("chaining should succeed");

    // The body hole must still exist as the innermost anchor!
    let inline_holes =
        parse_inline_holes(&chained.template_source).expect("should parse chained holes");
    assert!(inline_holes.iter().any(|h| h.hole_id == "body"));
    assert!(inline_holes.iter().any(|h| h.hole_id == "timeout_sec"));

    // Imports must be deduplicated and merged
    assert_eq!(chained.required_imports.len(), 3);
    assert!(
        chained
            .required_imports
            .contains(&"use axum::response::Response;".into())
    );
    assert!(
        chained
            .required_imports
            .contains(&"use tracing::info_span;".into())
    );
    assert!(
        chained
            .required_imports
            .contains(&"use tokio::time::timeout;".into())
    );

    // Canonical fills must be merged
    assert_eq!(
        chained
            .canonical_example_fills
            .get("body")
            .map(|s| s.as_str()),
        Some("Ok(())")
    );
    assert_eq!(
        chained
            .canonical_example_fills
            .get("timeout_sec")
            .map(|s| s.as_str()),
        Some("30")
    );
}
