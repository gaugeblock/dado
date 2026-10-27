# `dado` — Deterministic Polyglot AST Pattern & Typed Hole Engine

[![Version](https://img.shields.io/badge/version-0.1.0--beta.2-blue.svg)](https://gaugeblock.dev)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![gaugeblock](https://img.shields.io/badge/suite-gaugeblock-orange.svg)](https://github.com/gaugeblock)
[![Documentation](https://img.shields.io/badge/docs-9--doc%20topology-green.svg)](ARCHITECTURE.md)

Part of the **[`gaugeblock`](https://gaugeblock.dev)** neurosymbolic code synthesis ecosystem.

> In fine woodworking, a **dado** is a precision rectangular groove cut across the grain of a board into which another piece fits snugly, forming an interlocking joint without nails. In the `gaugeblock` ecosystem, `dado` eliminates 85% of LLM token generation: rather than asking a model to hallucinate boilerplate from scratch, `dado` provides parameterized AST patterns with explicit **Typed Holes** (`?[hole_id: ExpectedType @ ast_node_kind]`), composable structural mixins, and Tree-Sitter validated splicing with zero `ERROR` nodes.

---

## 1. Architecture Overview

```text
┌────────────────────────────────────────────────────────┐
│ Pattern Source (Starter Packs + .gaugeblock/dado/)     │
│ Axum Handler  •  CLI Subcommand  •  SQLx Transaction   │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│ dado Core Engine                                       │
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
│ - dado render             │ │ - dado_render            │
│ - dado contract           │ │ - dado_emit_contract     │
│ - dado verify-pack        │ │ - dado_verify_pack       │
└───────────────────────────┘ └──────────────────────────┘
```

---

## 2. Core Capabilities

* **Inline Typed Hole Syntax:** Parse `?[hole_id: ExpectedType @ ast_node_kind]` markers and extract exact byte spans and node boundaries.
* **Embedded Starter Packs & Local Overrides:** 15+ verified Rust and TypeScript patterns embedded via `include_str!`, plus support for `.gaugeblock/dado/patterns/` project overrides.
* **Composable Structural Mixins:** Apply reusable structural wrappers (`tracing`, `sqlx_tx`, `tokio_timeout`, `try_catch_log`) cleanly with anchor chaining on `?[__inner__]`.
* **Tree-Sitter Validated AST Splicer:** Substitutes filled expressions and injects missing `use` / `import` statements, validating that syntax trees contain 0 `ERROR` or `MISSING` nodes.
* **Zero-LLM Polyglot Contract Codegen:** Emits typed Rust serde structs and TypeScript interfaces / Zod schemas from `chalkline` contracts without neural sampling.

---

## 3. Workspace Crate Layout

* **`dado`**: Root facade crate re-exporting `dado-core`.
* **`crates/dado-core`**: Core domain types, pattern parsers, Tree-sitter splicer, mixin composition engine, and contract codegen.
* **`crates/dado-cli`**: Clap binary entrypoint (`dado`).
* **`crates/dado-mcp`**: JSON-RPC 2.0 stdio Model Context Protocol (MCP) server.

---

## 4. Installation & CLI Quickstart

```bash
# Install globally via cargo
cargo install --path crates/dado-cli

# 1. List available embedded and local AST patterns
dado list

# 2. Inspect an AST pattern and its typed hole signatures
dado inspect rust/axum_handler

# 3. Render a pattern with hole fills and mixins
dado render --pattern rust/axum_handler \
  --hole handler_name=get_user \
  --hole response_type=Json<User> \
  --mixin tracing \
  --mixin tokio_timeout

# 4. Generate aligned Rust and TypeScript types from a spec contract
dado contract --spec SPEC.md --lang rust
dado contract --spec SPEC.md --lang ts

# 5. Verify local pattern packs for AST validity
dado verify-pack .gaugeblock/dado/patterns
```

---

## 5. Model Context Protocol (MCP) Reference

| Tool | Parameters | Description |
|---|---|---|
| `dado_list_patterns` | `language?` | List all available AST templates across embedded and workspace packs. |
| `dado_inspect_pattern` | `pattern_id` | Retrieve pattern template, typed holes, required imports, and available mixins. |
| `dado_render` | `pattern_id, holes, mixins?` | Render an AST template with substituted hole values and structural mixins. |
| `dado_emit_contract` | `contract_json, target_lang` | Generate Rust serde structs or TypeScript Zod interfaces from contract schemas. |
| `dado_verify_pack` | `pack_dir` | Validate that all patterns in a directory compile to valid Tree-sitter ASTs. |

---

## 6. Repository Documentation

`dado` adheres strictly to the canonical **9-Document Architecture**:
* **[`CONTRIBUTING.md`](./CONTRIBUTING.md):** Local toolchain prerequisites, pattern authoring guidelines, and test execution.
* **[`ARCHITECTURE.md`](./ARCHITECTURE.md):** Subsystem decomposition, Tree-sitter grammar loading, and mixin chain topology.
* **[`SPEC.md`](./SPEC.md):** Formal TOML pattern schemas, TypedHole grammar, and error invariants.
* **[`AGENTS.md`](./AGENTS.md):** Autonomous systems directives, <=250 LOC limits, and zero-unwrap rules.
* **[`CHANGELOG.md`](./CHANGELOG.md):** Chronological historical release entries and unreleased changes.

---

## 7. Verification & Development Directives

* **File Size Ceiling:** All `.rs` source files must strictly remain under 250 LOC.
* **Tiered Verification Matrix:**
  * Tier 1: `cargo check -p dado-core` (<2s).
  * Tier 2: `cargo fmt --check -p dado-core && cargo clippy -p dado-core && cargo test -p dado-core`.
  * Tier 3: `chalkline docs check -d .` before task verification.

---

## 8. License

Licensed under the [Apache License, Version 2.0](LICENSE).
Part of the [`gaugeblock`](https://gaugeblock.dev) suite.
