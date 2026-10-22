//! Tool call dispatcher for `dado` MCP server.

#![forbid(unsafe_code)]

use chalkline_core::SharedTypeContract;
use dado_core::PatternRegistry;
use dado_core::mixin::apply_mixins;
use dado_core::polyglot::{generate_rust_struct, generate_ts_interface, generate_zod_schema};
use dado_core::registry::verifier::verify_pack;
use dado_core::splice::{SpliceOptions, splice_pattern};
use miter_engine::Language;
use serde_json::{Value, json};
use std::collections::BTreeMap;

/// Dispatches an MCP tool call to the appropriate `dado-core` engine method.
pub fn dispatch_tool(
    name: &str,
    args: &Value,
    registry: &PatternRegistry,
) -> Result<Value, String> {
    match name {
        "dado_list_patterns" => {
            let lang = args
                .get("language")
                .and_then(|v| v.as_str())
                .and_then(parse_lang_str);
            let query = args.get("query").and_then(|v| v.as_str());
            let patterns = registry.list_patterns(lang, query);
            let summaries: Vec<Value> = patterns
                .into_iter()
                .map(|p| {
                    json!({
                        "pattern_id": p.pattern_id,
                        "language": p.language.name(),
                        "description": p.description,
                        "holes_count": p.holes.len(),
                        "required_imports": p.required_imports,
                    })
                })
                .collect();
            Ok(json!({ "patterns": summaries }))
        }
        "dado_inspect_pattern" => {
            let pattern_id = args
                .get("pattern_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing required 'pattern_id'".to_string())?;

            let pattern = registry
                .get_pattern(pattern_id)
                .ok_or_else(|| format!("Pattern not found: {pattern_id}"))?;

            let compatible_mixins = registry.list_mixins(Some(pattern.language));
            let mixin_ids: Vec<&str> = compatible_mixins
                .iter()
                .map(|m| m.mixin_id.as_str())
                .collect();

            Ok(json!({
                "pattern": pattern,
                "compatible_mixins": mixin_ids,
            }))
        }
        "dado_render" => {
            let pattern_id = args
                .get("pattern_id")
                .and_then(|v| v.as_str())
                .ok_or_else(|| "Missing required 'pattern_id'".to_string())?;

            let base_pattern = registry
                .get_pattern(pattern_id)
                .ok_or_else(|| format!("Pattern not found: {pattern_id}"))?;

            let mut pattern_to_render = base_pattern.clone();
            if let Some(mixins_arr) = args.get("mixins").and_then(|v| v.as_array()) {
                let mut mixin_refs = Vec::new();
                for m_val in mixins_arr {
                    let m_id = m_val
                        .as_str()
                        .ok_or_else(|| "Mixin ID must be a string".to_string())?;
                    let m = registry
                        .get_mixin(m_id)
                        .ok_or_else(|| format!("Mixin not found: {m_id}"))?;
                    mixin_refs.push(m);
                }
                pattern_to_render = apply_mixins(&pattern_to_render, &mixin_refs)
                    .map_err(|e| format!("Mixin composition failed: {e}"))?;
            }

            let holes_obj = args
                .get("holes")
                .and_then(|v| v.as_object())
                .ok_or_else(|| "Missing or invalid 'holes' object".to_string())?;

            let mut fills = BTreeMap::new();
            for (k, v) in holes_obj {
                let fill_str = v.as_str().unwrap_or("").to_string();
                fills.insert(k.clone(), fill_str);
            }

            let include_header = args
                .get("include_imports_header")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);

            let options = SpliceOptions {
                available_imports: None,
                include_imports_header: include_header,
            };

            let result = splice_pattern(&pattern_to_render, &fills, &options)
                .map_err(|e| format!("Splicing failed: {e}"))?;

            Ok(json!({ "result": result }))
        }
        "dado_emit_contract" => {
            let contract_val = args
                .get("contract")
                .ok_or_else(|| "Missing required 'contract' object".to_string())?;
            let contract: SharedTypeContract = serde_json::from_value(contract_val.clone())
                .map_err(|e| format!("Failed to parse SharedTypeContract: {e}"))?;

            let target = args
                .get("target")
                .and_then(|v| v.as_str())
                .unwrap_or("rust");

            let code = match target {
                "rust" => generate_rust_struct(&contract),
                "typescript" | "ts" => generate_ts_interface(&contract),
                "zod" => generate_zod_schema(&contract),
                other => return Err(format!("Unknown contract target: {other}")),
            };

            Ok(json!({ "target": target, "code": code }))
        }
        "dado_verify_pack" => {
            let receipts = verify_pack(registry);
            let total = receipts.len();
            let passed = receipts.iter().filter(|r| r.passed).count();
            Ok(json!({
                "total": total,
                "passed": passed,
                "receipts": receipts,
            }))
        }
        unknown => Err(format!("Unknown tool: {unknown}")),
    }
}

fn parse_lang_str(s: &str) -> Option<Language> {
    match s.to_lowercase().as_str() {
        "rust" | "rs" => Some(Language::Rust),
        "typescript" | "ts" => Some(Language::TypeScript),
        "tsx" => Some(Language::Tsx),
        "javascript" | "js" => Some(Language::JavaScript),
        _ => None,
    }
}
