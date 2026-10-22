//! TypeScript interface and Zod runtime schema generators from `SharedTypeContract`.

use super::{CanonicalType, parse_canonical_type};
use chalkline_core::SharedTypeContract;

/// Emits a TypeScript `export interface` declaration from a `SharedTypeContract`.
pub fn generate_ts_interface(contract: &SharedTypeContract) -> String {
    let mut out = String::new();
    out.push_str(&format!("export interface {} {{\n", contract.name));

    for field in &contract.fields {
        let parsed = parse_canonical_type(&field.canonical_type);
        let ts_type = to_ts_type(&parsed);

        let opt_suffix = if !field.required && !matches!(parsed, CanonicalType::Option(_)) {
            "?"
        } else {
            ""
        };

        out.push_str(&format!("  {}{}: {};\n", field.name, opt_suffix, ts_type));
    }

    out.push_str("}\n");
    out
}

/// Emits a Zod schema object and inferred type from a `SharedTypeContract`.
pub fn generate_zod_schema(contract: &SharedTypeContract) -> String {
    let mut out = String::new();
    out.push_str("import { z } from \"zod\";\n\n");
    out.push_str(&format!(
        "export const {}Schema = z.object({{\n",
        contract.name
    ));

    for field in &contract.fields {
        let parsed = parse_canonical_type(&field.canonical_type);
        let mut zod_expr = to_zod_type(&parsed);

        if !field.required && !matches!(parsed, CanonicalType::Option(_)) {
            zod_expr.push_str(".optional()");
        }

        out.push_str(&format!("  {}: {},\n", field.name, zod_expr));
    }

    out.push_str("});\n\n");
    out.push_str(&format!(
        "export type {} = z.infer<typeof {}Schema>;\n",
        contract.name, contract.name
    ));
    out
}

fn to_ts_type(canon: &CanonicalType) -> String {
    match canon {
        CanonicalType::Uuid | CanonicalType::String => "string".to_string(),
        CanonicalType::Bool => "boolean".to_string(),
        CanonicalType::I32 | CanonicalType::I64 | CanonicalType::F64 => "number".to_string(),
        CanonicalType::Vec(inner) => format!("Array<{}>", to_ts_type(inner)),
        CanonicalType::Option(inner) => format!("{} | null", to_ts_type(inner)),
        CanonicalType::Map(k, v) => format!("Record<{}, {}>", to_ts_type(k), to_ts_type(v)),
        CanonicalType::Custom(name) => name.clone(),
    }
}

fn to_zod_type(canon: &CanonicalType) -> String {
    match canon {
        CanonicalType::Uuid => "z.string().uuid()".to_string(),
        CanonicalType::String => "z.string()".to_string(),
        CanonicalType::Bool => "z.boolean()".to_string(),
        CanonicalType::I32 | CanonicalType::I64 | CanonicalType::F64 => "z.number()".to_string(),
        CanonicalType::Vec(inner) => format!("z.array({})", to_zod_type(inner)),
        CanonicalType::Option(inner) => format!("{}.nullable()", to_zod_type(inner)),
        CanonicalType::Map(_, v) => format!("z.record({})", to_zod_type(v)),
        CanonicalType::Custom(name) => format!("z.lazy(() => {}Schema)", name),
    }
}
