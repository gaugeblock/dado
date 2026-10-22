//! Neutral type parser and polyglot code generation from `SharedTypeContract`.

pub mod rust_gen;
pub mod ts_gen;

pub use rust_gen::generate_rust_struct;
pub use ts_gen::{generate_ts_interface, generate_zod_schema};

/// Neutral canonical types supported across polyglot boundaries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalType {
    /// Universally unique identifier.
    Uuid,
    /// UTF-8 string.
    String,
    /// Boolean flag.
    Bool,
    /// 32-bit signed integer.
    I32,
    /// 64-bit signed integer.
    I64,
    /// 64-bit floating point.
    F64,
    /// Dynamic array / list.
    Vec(Box<CanonicalType>),
    /// Optional / nullable value.
    Option(Box<CanonicalType>),
    /// Key-value mapping.
    Map(Box<CanonicalType>, Box<CanonicalType>),
    /// Custom named type reference.
    Custom(String),
}

/// Parses a canonical type string into a structured `CanonicalType` AST.
pub fn parse_canonical_type(raw: &str) -> CanonicalType {
    let trimmed = raw.trim();
    let lower = trimmed.to_lowercase();

    if lower == "uuid" {
        CanonicalType::Uuid
    } else if lower == "string" || lower == "str" {
        CanonicalType::String
    } else if lower == "bool" || lower == "boolean" {
        CanonicalType::Bool
    } else if lower == "i32" || lower == "int" {
        CanonicalType::I32
    } else if lower == "i64" || lower == "long" {
        CanonicalType::I64
    } else if lower == "f64" || lower == "float" || lower == "double" {
        CanonicalType::F64
    } else if lower.starts_with("vec<") && lower.ends_with('>') {
        let inner = &trimmed[4..trimmed.len() - 1];
        CanonicalType::Vec(Box::new(parse_canonical_type(inner)))
    } else if lower.starts_with("option<") && lower.ends_with('>') {
        let inner = &trimmed[7..trimmed.len() - 1];
        CanonicalType::Option(Box::new(parse_canonical_type(inner)))
    } else if lower.starts_with("map<") && lower.ends_with('>') {
        let inner = &trimmed[4..trimmed.len() - 1];
        if let Some(comma_pos) = inner.find(',') {
            let key_str = &inner[..comma_pos];
            let val_str = &inner[comma_pos + 1..];
            CanonicalType::Map(
                Box::new(parse_canonical_type(key_str)),
                Box::new(parse_canonical_type(val_str)),
            )
        } else {
            CanonicalType::Custom(trimmed.to_string())
        }
    } else {
        CanonicalType::Custom(trimmed.to_string())
    }
}
