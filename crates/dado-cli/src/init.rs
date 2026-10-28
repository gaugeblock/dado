//! Repository initialization handler for `dado init`.

use std::path::Path;

/// Executes `dado init`.
pub fn handle_init(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let dado_dir = if dir.join(".gaugeblock").is_dir() {
        dir.join(".gaugeblock").join("dado")
    } else {
        dir.join(".dado")
    };
    let patterns_dir = dado_dir.join("patterns");
    let mixins_dir = dado_dir.join("mixins");

    std::fs::create_dir_all(&patterns_dir)?;
    std::fs::create_dir_all(&mixins_dir)?;

    let example_pattern = r#"[[patterns]]
pattern_id = "custom.example.v1"
language = "rust"
description = "Custom example pattern"
template_source = """
pub fn example_fn() -> ?[ret_type: Type @ type] {
    ?[body: Statement @ statement]
}
"""
required_imports = []

[[patterns.holes]]
hole_id = "ret_type"
expected_type = "Type"
ast_node_kind = "type"

[[patterns.holes]]
hole_id = "body"
expected_type = "Statement"
ast_node_kind = "statement"

[patterns.canonical_example_fills]
ret_type = "i32"
body = "42"
"#;
    std::fs::write(patterns_dir.join("example.toml"), example_pattern)?;

    println!("Initialized dado configuration at {}", dado_dir.display());
    Ok(())
}
