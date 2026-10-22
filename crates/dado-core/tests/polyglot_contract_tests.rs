#![allow(clippy::unwrap_used, clippy::expect_used)]

//! Unit and integration tests for polyglot contract codegen.

use chalkline_core::{ContractField, SharedTypeContract};
use dado_core::polyglot::{generate_rust_struct, generate_ts_interface, generate_zod_schema};
use dado_core::splice::ast_check::validate_syntax;
use miter_engine::Language;

#[test]
fn test_polyglot_codegen_syntax_validity() {
    let contract = SharedTypeContract {
        name: "UserProfile".into(),
        fields: vec![
            ContractField {
                name: "id".into(),
                canonical_type: "uuid".into(),
                required: true,
            },
            ContractField {
                name: "username".into(),
                canonical_type: "string".into(),
                required: true,
            },
            ContractField {
                name: "is_active".into(),
                canonical_type: "bool".into(),
                required: true,
            },
            ContractField {
                name: "tags".into(),
                canonical_type: "vec<string>".into(),
                required: false,
            },
            ContractField {
                name: "score".into(),
                canonical_type: "option<i64>".into(),
                required: false,
            },
            ContractField {
                name: "attributes".into(),
                canonical_type: "map<string, i32>".into(),
                required: true,
            },
        ],
    };

    // 1. Rust codegen
    let rust_code = generate_rust_struct(&contract);
    assert!(rust_code.contains("pub struct UserProfile"));
    assert!(rust_code.contains("pub id: uuid::Uuid,"));
    assert!(rust_code.contains("Option<Vec<String>>"));

    let (rust_valid, rust_diag) =
        validate_syntax(&rust_code, Language::Rust).expect("Rust validation");
    assert!(
        rust_valid,
        "Generated Rust code must have 0 syntax errors: {rust_diag:?}"
    );

    // 2. TypeScript interface codegen
    let ts_interface = generate_ts_interface(&contract);
    assert!(ts_interface.contains("export interface UserProfile"));
    assert!(ts_interface.contains("tags?: Array<string>;"));

    let (ts_valid, ts_diag) =
        validate_syntax(&ts_interface, Language::TypeScript).expect("TS validation");
    assert!(
        ts_valid,
        "Generated TS interface must have 0 syntax errors: {ts_diag:?}"
    );

    // 3. Zod schema codegen
    let zod_code = generate_zod_schema(&contract);
    assert!(zod_code.contains("export const UserProfileSchema = z.object({"));
    assert!(zod_code.contains("id: z.string().uuid(),"));
    assert!(zod_code.contains("export type UserProfile = z.infer<typeof UserProfileSchema>;"));

    let (zod_valid, zod_diag) =
        validate_syntax(&zod_code, Language::TypeScript).expect("Zod validation");
    assert!(
        zod_valid,
        "Generated Zod code must have 0 syntax errors: {zod_diag:?}"
    );
}
