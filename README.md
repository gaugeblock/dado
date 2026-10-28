# `dado` — Deterministic Polyglot AST Pattern & Typed Hole Engine

[![Version](https://img.shields.io/badge/version-0.1.0--beta.2-blue.svg)](https://gaugeblock.dev)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![gaugeblock](https://img.shields.io/badge/suite-gaugeblock-orange.svg)](https://github.com/gaugeblock)
[![Documentation](https://img.shields.io/badge/docs-9--doc%20topology-green.svg)](ARCHITECTURE.md)

Part of the **[`gaugeblock`](https://gaugeblock.dev)** neurosymbolic code synthesis ecosystem.

> In fine woodworking, a **dado** is a precision rectangular groove cut across the grain of a board into which another workpiece fits snugly, forming an interlocking joint without nails. In the `gaugeblock` ecosystem, `dado` eliminates 85% of LLM token generation: rather than asking a model to hallucinate boilerplate syntax from scratch, `dado` provides parameterized AST patterns with explicit **Typed Holes** (`?[hole_id: ExpectedType @ ast_node_kind]`), composable structural mixins, and Tree-Sitter validated splicing with zero `ERROR` nodes.

---

## Why `dado`?

Modern LLM-based coding tools treat source code as flat stochastic text, generating boilerplate imports, framework wrappers, and bracket structures token-by-token.

| Without `dado` | With `dado` |
|---|---|
| **Stochastic Boilerplate:** The LLM predicts every token of HTTP handlers, SQL transactions, and CLI commands from scratch, burning 85%+ of context on predictable structure. | **Parameterized AST Patterns:** `dado` provides pre-verified structural templates for Rust and TypeScript; models only generate the 5–25 tokens needed for specific business logic. |
| **Boilerplate Pollution:** Adding tracing, timeout guards, or error handling requires models to rewrite entire functions, introducing indentation errors and typos. | **Composable Structural Mixins:** Apply mixins (`tracing`, `sqlx_tx`, `tokio_timeout`) via clean AST wrapping on `?[__inner__]` anchors without neural sampling. |
| **Malformed Splicing:** String-based template engines (Jinja/Handlebars) produce syntax errors, unclosed braces, or missing imports. | **Tree-sitter AST Splicer:** Verifies that every rendered AST contains zero `ERROR` or `MISSING` nodes and automatically injects required imports. |
| **Polyglot Type Divergence:** Writing matching backend models and frontend schemas leads to silent drift between Rust structs and TypeScript interfaces. | **Deterministic Contract Codegen:** `dado contract` compiles shared data schemas into matching Rust serde types and TypeScript Zod schemas deterministically. |

---

## 1. System Architecture

```text
┌────────────────────────────────────────────────────────┐
│ Pattern Source (Starter Packs + .gaugeblock/dado/)     │
│ Axum Handler  •  CLI Subcommand  •  SQLx Transaction   │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│ dado Core Engine (Tree-sitter Syntax Splicer)          │
│ - Typed Hole Extractor: `?[id: Type @ kind]` spans     │
│ - Structural Mixin Composer: `tracing`, `timeout`, etc.│
│ - Tree-Sitter AST Splicer: Zero-ERROR byte substitution│
│ - Polyglot Contract Codegen: Rust serde + TS Zod       │
└───────────────┬────────────────────────┬───────────────┘
                │                        │
                ▼                        ▼
┌───────────────────────────┐ ┌──────────────────────────┐
│ CLI (`dado`)              │ │ MCP Stdio Server         │
│ - dado list / inspect     │ │ - dado_list_patterns     │
│ - dado render / init      │ │ - dado_render            │
│ - dado contract           │ │ - dado_emit_contract     │
│ - dado verify-pack        │ │ - dado_verify_pack       │
└───────────────────────────┘ └──────────────────────────┘
```

---

## 2. CLI Commands Reference

`dado` provides a unified CLI for developer exploration and autonomous agent automation. Pass `--json` to any command for structured machine-readable output.

| Command | Description | MCP Tool |
|---|---|---|
| `list` | List registered AST patterns across embedded packs and local directories. | `dado_list_patterns` |
| `inspect <ID>` | Inspect a specific pattern, its typed holes, expected types, and mixins. | `dado_inspect_pattern` |
| `render` | Render an AST pattern with substituted typed hole values and mixins. | `dado_render` |
| `contract` | Generate aligned Rust serde structs or TypeScript Zod schemas from a contract. | `dado_emit_contract` |
| `verify-pack` | Verify that registered patterns parse cleanly with zero Tree-sitter error nodes. | `dado_verify_pack` |
| `init` | Initialize local `.dado/patterns/` and `.dado/mixins/` project directories. | — |
| `mcp` | Launch the Model Context Protocol (JSON-RPC 2.0 stdio) server. | — |

---

## 3. Model Context Protocol (MCP) Reference

When operating inside AI agents (Antigravity, Claude Code, Cursor, Cline), `dado` exposes 5 core tools:

| Tool | Parameters | Description |
|---|---|---|
| `dado_list_patterns` | `lang?, query?` | List all available AST templates across embedded and workspace packs. |
| `dado_inspect_pattern` | `pattern_id` | Retrieve pattern template, typed holes, required imports, and available mixins. |
| `dado_render` | `pattern_id, holes, mixin?` | Render an AST template with substituted hole values and structural mixins. |
| `dado_emit_contract` | `input, target` | Generate Rust serde structs or TypeScript Zod interfaces from contract schemas. |
| `dado_verify_pack` | — | Validate that all patterns compile to valid Tree-sitter ASTs with zero errors. |

---

## 4. Installation & Quickstart

```bash
# Install globally via cargo
cargo install --path crates/dado-cli

# Initialize local project patterns directory
dado init

# List available Rust and TypeScript patterns
dado list --lang rust

# Inspect an Axum handler pattern and its typed holes
dado inspect rust/axum_handler

# Render an AST pattern with hole fills and mixins
dado render --pattern-id rust/axum_handler \
  --holes '{"handler_name": "get_user", "response_type": "Json<User>"}' \
  --mixin tracing \
  --mixin tokio_timeout

# Emit TypeScript types from a shared type contract
dado contract --input contracts/user.json --target typescript
```

---

## 5. Canonical 9-Document Architecture

`dado` adheres strictly to the canonical 9-Document Architecture:
* **[`README.md`](./README.md):** Public elevator pitch, CLI reference, and quickstart.
* **[`CONTRIBUTING.md`](./CONTRIBUTING.md):** Developer environment setup, Rust 2024 toolchain, and test execution.
* **[`PRODUCT.md`](./PRODUCT.md):** User personas, JTBD, token reduction metrics, and problem statements.
* **[`ARCHITECTURE.md`](./ARCHITECTURE.md):** Subsystem topologies, Tree-sitter splicing pipeline, and crate boundaries.
* **[`SPEC.md`](./SPEC.md):** Formal typed hole grammar, AST validation invariants, and type contracts.
* **[`DESIGN.md`](./DESIGN.md):** Architectural Decision Records (ADRs) and trade-off rationales.
* **[`ROADMAP.md`](./ROADMAP.md):** Strategic milestones, active beta capabilities, and horizons.
* **[`CHANGELOG.md`](./CHANGELOG.md):** Chronological historical release logs and unreleased entries.
* **[`AGENTS.md`](./AGENTS.md):** Non-negotiable engineering directives, 250-line file ceiling, and verification matrix.

---

## 6. License

Licensed under the [Apache License, Version 2.0](LICENSE).
