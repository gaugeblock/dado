//! Serde adapter for the AST language on the wire.
//!
//! `batten-core` (`LexicalScope`) serialises a language as its canonical lowercase name. This module
//! was missing from the crate while that reference remained, which left `batten` unbuildable.
//!
//! The mapping is written out here rather than delegated: `miter_engine::Language` is the
//! five-variant *AST* language, while the `From` conversions in `miter-engine` bridge the thirteen
//! format wire taxonomy, so neither direction applies. Names match the wire spellings used
//! everywhere else, and the short aliases keep hand-written JSON lenient.

use miter_engine::Language;
use serde::{Deserialize, Deserializer, Serializer};

/// Serialises a language as its canonical lowercase name.
///
/// # Errors
///
/// Propagates any error from the underlying serialiser.
pub fn serialize<S>(language: &Language, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(name_of(*language))
}

/// Reads a language from a name, accepting the short aliases.
///
/// # Errors
///
/// Returns a serde error when the name is not one of the five AST languages.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Language, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = String::deserialize(deserializer)?;
    language_named(&raw)
        .ok_or_else(|| serde::de::Error::custom(format!("'{raw}' is not an AST language")))
}

/// Canonical lowercase spelling of a language.
fn name_of(language: Language) -> &'static str {
    match language {
        Language::Rust => "rust",
        Language::TypeScript => "typescript",
        Language::Tsx => "tsx",
        Language::JavaScript => "javascript",
        Language::Python => "python",
    }
}

/// Parses a language name, tolerating case and the short aliases.
fn language_named(raw: &str) -> Option<Language> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "rust" | "rs" => Some(Language::Rust),
        "typescript" | "ts" => Some(Language::TypeScript),
        "tsx" => Some(Language::Tsx),
        "javascript" | "js" => Some(Language::JavaScript),
        "python" | "py" => Some(Language::Python),
        _ => None,
    }
}
