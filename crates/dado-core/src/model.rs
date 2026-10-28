//! Data models for AST patterns, typed holes, and structural mixins.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

// `Language`, `TypedHole` and `DadoPattern` live in the shared contracts crate (GB-143); they are
// re-exported here so every existing `dado_core::model::*` path keeps working. The wire `Language`
// already spells itself the lowercase way the pattern packs use, so dado no longer needs a custom
// serde module for it.
pub use gaugeblock_contracts::{DadoPattern, Language, TypedHole};

/// Composable structural wrapper that envelopes an inner anchor hole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DadoMixin {
    /// Unique mixin identifier (e.g., "rust.mixin.tracing_instrument").
    pub mixin_id: String,
    /// Target programming language.
    pub language: Language,
    /// Target hole identifier in the base pattern to wrap (defaults to "body").
    #[serde(default = "default_target_hole")]
    pub target_hole_id: String,
    /// Envelope template containing `?[__inner__]` where original hole will be placed.
    pub envelope_template: String,
    /// Additional imports required by this mixin.
    #[serde(default)]
    pub required_imports: Vec<String>,
    /// Additional typed holes introduced by this mixin.
    #[serde(default)]
    pub additional_holes: Vec<TypedHole>,
    /// Example valid fills for additional holes.
    #[serde(default)]
    pub canonical_example_fills: BTreeMap<String, String>,
}

fn default_target_hole() -> String {
    "body".to_string()
}

/// Result of substituting hole fills and injecting imports into a template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpliceResult {
    /// Rendered source code including top-level injected imports and spliced body.
    pub rendered_code: String,
    /// List of import statements injected during splicing.
    pub injected_imports: Vec<String>,
    /// Exact byte offsets `(start, end)` for each filled hole in `rendered_code`.
    pub hole_spans: BTreeMap<String, (usize, usize)>,
    /// Whether the output was validated with zero Tree-Sitter ERROR/MISSING nodes.
    pub ast_valid: bool,
    /// Any warnings or diagnostic messages collected during parsing.
    pub diagnostics: Vec<String>,
}

/// Verification result for a single pattern against its canonical fills.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatternVerificationReceipt {
    /// Pattern identifier verified.
    pub pattern_id: String,
    /// Whether verification passed with 0 Tree-Sitter ERROR nodes.
    pub passed: bool,
    /// Injected imports during verification.
    pub injected_imports_count: usize,
    /// Error diagnostics if any.
    pub diagnostics: Vec<String>,
}
