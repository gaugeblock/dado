//! Rust struct generator from `SharedTypeContract`.

use super::{CanonicalType, parse_canonical_type};
use chalkline_core::SharedTypeContract;

/// Emits a Rust serde-annotated `pub struct` from a `SharedTypeContract`.
pub fn generate_rust_struct(contract: &SharedTypeContract) -> String {
    let mut out = String::new();
    out.push_str("use serde::{Deserialize, Serialize};\n\n");
    out.push_str("#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]\n");
    out.push_str("#[serde(rename_all = \"snake_case\")]\n");
    out.push_str(&format!("pub struct {} {{\n", contract.name));

    for field in &contract.fields {
        let parsed = parse_canonical_type(&field.canonical_type);
        let rust_type = to_rust_type(&parsed);

        let final_type = if !field.required && !matches!(parsed, CanonicalType::Option(_)) {
            out.push_str("    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n");
            format!("Option<{rust_type}>")
        } else {
            rust_type
        };

        out.push_str(&format!("    pub {}: {final_type},\n", field.name));
    }

    out.push_str("}\n");
    out
}

fn to_rust_type(canon: &CanonicalType) -> String {
    match canon {
        CanonicalType::Uuid => "uuid::Uuid".to_string(),
        CanonicalType::String => "String".to_string(),
        CanonicalType::Bool => "bool".to_string(),
        CanonicalType::I32 => "i32".to_string(),
        CanonicalType::I64 => "i64".to_string(),
        CanonicalType::F64 => "f64".to_string(),
        CanonicalType::Vec(inner) => format!("Vec<{}>", to_rust_type(inner)),
        CanonicalType::Option(inner) => format!("Option<{}>", to_rust_type(inner)),
        CanonicalType::Map(k, v) => {
            format!(
                "std::collections::BTreeMap<{}, {}>",
                to_rust_type(k),
                to_rust_type(v)
            )
        }
        CanonicalType::Custom(name) => name.clone(),
    }
}
