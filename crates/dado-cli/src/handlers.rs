//! CLI subcommand execution handlers.

use crate::args::*;
use chalkline_core::SharedTypeContract;
use dado_core::PatternRegistry;
use dado_core::mixin::apply_mixins;
use dado_core::polyglot::{generate_rust_struct, generate_ts_interface, generate_zod_schema};
use dado_core::registry::verifier::verify_pack;
use dado_core::splice::{SpliceOptions, splice_pattern};
use miter_engine::Language;
use serde_json::json;
use std::collections::BTreeMap;

/// Executes `dado list`.
pub fn handle_list(
    args: &ListArgs,
    registry: &PatternRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let lang = args.lang.as_deref().and_then(parse_lang);
    let patterns = registry.list_patterns(lang, args.query.as_deref());

    if args.json {
        let list: Vec<_> = patterns
            .iter()
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
        println!("{}", serde_json::to_string_pretty(&list)?);
    } else {
        println!("Registered Dado Patterns ({} total):", patterns.len());
        println!(
            "{:<36} {:<12} {:<6} DESCRIPTION",
            "PATTERN ID", "LANGUAGE", "HOLES"
        );
        println!("{}", "-".repeat(80));
        for p in patterns {
            println!(
                "{:<36} {:<12} {:<6} {}",
                p.pattern_id,
                p.language.name(),
                p.holes.len(),
                p.description
            );
        }
    }
    Ok(())
}

/// Executes `dado inspect`.
pub fn handle_inspect(
    args: &InspectArgs,
    registry: &PatternRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let pattern = registry
        .get_pattern(&args.pattern_id)
        .ok_or_else(|| format!("Pattern not found: {}", args.pattern_id))?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(pattern)?);
    } else {
        println!("Pattern:     {}", pattern.pattern_id);
        println!("Language:    {}", pattern.language.name());
        println!("Description: {}", pattern.description);
        println!("\nRequired Imports ({}):", pattern.required_imports.len());
        for imp in &pattern.required_imports {
            println!("  {imp}");
        }
        println!("\nTyped Holes ({}):", pattern.holes.len());
        for h in &pattern.holes {
            println!(
                "  ?[{}: {} @ {}]",
                h.hole_id, h.expected_type, h.ast_node_kind
            );
        }
        println!("\nTemplate Source:\n{}", pattern.template_source);
    }
    Ok(())
}

/// Executes `dado render`.
pub fn handle_render(
    args: &RenderArgs,
    registry: &PatternRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let base = registry
        .get_pattern(&args.pattern_id)
        .ok_or_else(|| format!("Pattern not found: {}", args.pattern_id))?;

    let mut pattern = base.clone();
    if !args.mixin.is_empty() {
        let mut mixin_refs = Vec::new();
        for m_id in &args.mixin {
            let m = registry
                .get_mixin(m_id)
                .ok_or_else(|| format!("Mixin not found: {m_id}"))?;
            mixin_refs.push(m);
        }
        pattern = apply_mixins(&pattern, &mixin_refs)?;
    }

    let fills: BTreeMap<String, String> = if args.holes.starts_with('{') {
        serde_json::from_str(&args.holes)?
    } else {
        let content = std::fs::read_to_string(&args.holes)?;
        serde_json::from_str(&content)?
    };

    let options = SpliceOptions {
        available_imports: None,
        include_imports_header: true,
    };

    let res = splice_pattern(&pattern, &fills, &options)?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&res)?);
    } else {
        if !res.ast_valid {
            eprintln!(
                "Warning: Output contains AST syntax errors: {:?}",
                res.diagnostics
            );
        }
        println!("{}", res.rendered_code);
    }
    Ok(())
}

/// Executes `dado contract`.
pub fn handle_contract(args: &ContractArgs) -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(&args.input)?;
    let contract: SharedTypeContract = serde_json::from_str(&content)?;

    let code = match args.target.to_lowercase().as_str() {
        "rust" | "rs" => generate_rust_struct(&contract),
        "typescript" | "ts" => generate_ts_interface(&contract),
        "zod" => generate_zod_schema(&contract),
        other => return Err(format!("Unsupported contract target: {other}").into()),
    };

    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({ "target": args.target, "code": code }))?
        );
    } else {
        println!("{code}");
    }
    Ok(())
}

/// Executes `dado verify-pack`.
pub fn handle_verify_pack(
    args: &VerifyPackArgs,
    registry: &PatternRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let receipts = verify_pack(registry);
    let total = receipts.len();
    let passed = receipts.iter().filter(|r| r.passed).count();

    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "total": total,
                "passed": passed,
                "receipts": receipts,
            }))?
        );
    } else {
        println!("Pack Verification Results: {passed}/{total} passed\n");
        for r in &receipts {
            let status = if r.passed { "PASS" } else { "FAIL" };
            println!("[{status}] {}", r.pattern_id);
            for d in &r.diagnostics {
                println!("       Diagnostic: {d}");
            }
        }
    }

    if passed < total {
        return Err("One or more patterns failed verification".into());
    }
    Ok(())
}

fn parse_lang(s: &str) -> Option<Language> {
    match s.to_lowercase().as_str() {
        "rust" | "rs" => Some(Language::Rust),
        "typescript" | "ts" => Some(Language::TypeScript),
        "tsx" => Some(Language::Tsx),
        "javascript" | "js" => Some(Language::JavaScript),
        _ => None,
    }
}
