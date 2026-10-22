//! Error types for `dado-core`.

use thiserror::Error;

/// Result alias for `dado-core` operations.
pub type Result<T> = std::result::Result<T, DadoError>;

/// Strongly typed errors produced by the `dado` pattern engine.
#[derive(Debug, Error)]
pub enum DadoError {
    /// Syntax error when parsing an inline `?[hole_id: Type @ kind]` marker.
    #[error("Hole syntax error at offset {offset}: {message}")]
    HoleSyntax {
        /// Descriptive message of the syntax error.
        message: String,
        /// Byte offset in the template string where error occurred.
        offset: usize,
    },

    /// A duplicate hole identifier was detected in a template.
    #[error("Duplicate hole identifier: {hole_id}")]
    DuplicateHole {
        /// Identifier of the duplicated hole.
        hole_id: String,
    },

    /// Hole found in template does not match manifest declaration.
    #[error("Hole '{hole_id}' declaration mismatch: expected '{expected}', found '{found}'")]
    HoleMismatch {
        /// Identifier of the mismatched hole.
        hole_id: String,
        /// Expected property or type.
        expected: String,
        /// Found property or type.
        found: String,
    },

    /// A required hole value was not supplied during rendering.
    #[error("Missing fill value for hole '{hole_id}' in pattern '{pattern_id}'")]
    MissingHoleFill {
        /// Identifier of the unfilled hole.
        hole_id: String,
        /// Pattern requiring the hole.
        pattern_id: String,
    },

    /// A requested pattern was not found in the registry.
    #[error("Pattern not found: {pattern_id}")]
    PatternNotFound {
        /// Pattern identifier that was missing.
        pattern_id: String,
    },

    /// A requested mixin was not found in the registry.
    #[error("Mixin not found: {mixin_id}")]
    MixinNotFound {
        /// Mixin identifier that was missing.
        mixin_id: String,
    },

    /// Target hole for mixin anchor was not found in the base pattern.
    #[error("Target anchor hole '{target_hole_id}' not found in pattern '{pattern_id}'")]
    TargetHoleNotFound {
        /// Missing anchor hole id.
        target_hole_id: String,
        /// Base pattern id.
        pattern_id: String,
    },

    /// Tree-sitter AST validation discovered syntax errors.
    #[error("AST validation failed for pattern '{pattern_id}': {diagnostics:?}")]
    AstValidationFailed {
        /// Pattern id that failed validation.
        pattern_id: String,
        /// Syntax diagnostics collected from Tree-Sitter error nodes.
        diagnostics: Vec<String>,
    },

    /// Tree-sitter internal parse error.
    #[error("Tree-sitter parse error: {0}")]
    TreeSitter(String),

    /// Polyglot codegen error.
    #[error("Polyglot codegen error: {0}")]
    PolyglotError(String),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// TOML deserialization error.
    #[error("TOML decode error: {0}")]
    TomlDe(#[from] toml::de::Error),

    /// TOML serialization error.
    #[error("TOML encode error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    /// JSON error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}
