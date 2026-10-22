//! Tool schemas for the `dado` Model Context Protocol (MCP) server.

#![forbid(unsafe_code)]

use serde_json::{Value, json};

/// Returns the list of MCP tools exposed by `dado`.
#[must_use]
pub fn list_tools() -> Vec<Value> {
    vec![
        json!({
            "name": "dado_list_patterns",
            "description": "List registered AST patterns filtered by optional language or query string.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "language": {
                        "type": "string",
                        "description": "Optional language filter (e.g. 'rust', 'typescript', 'tsx')."
                    },
                    "query": {
                        "type": "string",
                        "description": "Optional search filter for pattern ID or description."
                    }
                }
            }
        }),
        json!({
            "name": "dado_inspect_pattern",
            "description": "Inspect a specific AST pattern manifest, listing typed holes and compatible mixins.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "pattern_id": {
                        "type": "string",
                        "description": "Pattern identifier (e.g. 'rust.axum.json_handler.v1')."
                    }
                },
                "required": ["pattern_id"]
            }
        }),
        json!({
            "name": "dado_render",
            "description": "Render a pattern with optional mixins, hole fills, and Tree-sitter AST validation.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "pattern_id": {
                        "type": "string",
                        "description": "Pattern identifier to render."
                    },
                    "mixins": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Optional list of mixin IDs to apply in sequence before hole substitution."
                    },
                    "holes": {
                        "type": "object",
                        "description": "Key-value mapping from hole_id to filled expression string."
                    },
                    "include_imports_header": {
                        "type": "boolean",
                        "description": "Whether to prepend missing required imports to rendered output (defaults to true)."
                    }
                },
                "required": ["pattern_id", "holes"]
            }
        }),
        json!({
            "name": "dado_emit_contract",
            "description": "Deterministically generate Rust serde struct, TypeScript interface, or Zod schema from a SharedTypeContract.",
            "inputSchema": {
                "type": "object",
                "properties": {
                    "contract": {
                        "type": "object",
                        "description": "SharedTypeContract schema object containing 'name' and 'fields'."
                    },
                    "target": {
                        "type": "string",
                        "enum": ["rust", "typescript", "zod"],
                        "description": "Target language or schema format."
                    }
                },
                "required": ["contract", "target"]
            }
        }),
        json!({
            "name": "dado_verify_pack",
            "description": "Run pack verification across all registered patterns using canonical_example_fills.",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
    ]
}
