//! Data models for AST patterns, typed holes, and structural mixins.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;

/// Serde helper for `miter_engine::Language`.
pub mod language_serde {
    use super::*;
    use miter_engine::Language;

    /// Serializes a `Language` to its string name.
    pub fn serialize<S>(lang: &Language, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(lang.name())
    }

    /// Deserializes a `Language` from a string.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Language, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Language::Rust),
            "typescript" | "ts" => Ok(Language::TypeScript),
            "tsx" => Ok(Language::Tsx),
            "javascript" | "js" => Ok(Language::JavaScript),
            "python" | "py" => Ok(Language::Python),
            other => Err(serde::de::Error::custom(format!(
                "Unknown language: {other}"
            ))),
        }
    }
}

/// A single Typed Hole inside a `dado` AST template: `?[hole_id: expected_type @ ast_node_kind]`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedHole {
    /// Unique identifier for this hole within the template (e.g., "query_sql").
    pub hole_id: String,
    /// Expected semantic/static type of the hole expression (e.g., "&str", "AppError").
    pub expected_type: String,
    /// AST target node kind (e.g., "expression", "statement", "match_arm").
    pub ast_node_kind: String,
    /// Maximum permitted token budget when generating this hole.
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u16,
    /// Whether literal constant expressions are permitted in this hole.
    #[serde(default = "default_allow_literals")]
    pub allow_literals: bool,
}

fn default_max_tokens() -> u16 {
    64
}

fn default_allow_literals() -> bool {
    true
}

impl TypedHole {
    /// Creates a new `TypedHole` with sensible defaults.
    pub fn new(
        hole_id: impl Into<String>,
        expected_type: impl Into<String>,
        ast_node_kind: impl Into<String>,
    ) -> Self {
        Self {
            hole_id: hole_id.into(),
            expected_type: expected_type.into(),
            ast_node_kind: ast_node_kind.into(),
            max_tokens: 64,
            allow_literals: true,
        }
    }
}

/// A parameterized AST pattern containing template code and typed holes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DadoPattern {
    /// Canonical pattern identifier (e.g., "rust.axum.json_handler.v1").
    pub pattern_id: String,
    /// Target programming language.
    #[serde(with = "language_serde")]
    pub language: miter_engine::Language,
    /// Human-readable description of what this pattern scaffolds.
    pub description: String,
    /// Source code template containing `?[hole_id: Type @ kind]` markers.
    pub template_source: String,
    /// Required import statements (e.g. `use axum::Json;`).
    #[serde(default)]
    pub required_imports: Vec<String>,
    /// Declared holes in this template.
    pub holes: Vec<TypedHole>,
    /// Example valid fills for deterministic self-testing.
    #[serde(default)]
    pub canonical_example_fills: BTreeMap<String, String>,
}

/// Composable structural wrapper that envelopes an inner anchor hole.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DadoMixin {
    /// Unique mixin identifier (e.g., "rust.mixin.tracing_instrument").
    pub mixin_id: String,
    /// Target programming language.
    #[serde(with = "language_serde")]
    pub language: miter_engine::Language,
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
