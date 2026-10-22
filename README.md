# dado

> **Deterministic Polyglot AST Pattern & Typed Hole Engine**  
> Part of the [`gaugeblock`](https://gaugeblock.dev) neurosymbolic code synthesis suite.

`dado` eliminates 85% of LLM token generation by providing parameterized polyglot AST templates with explicit Typed Holes, composable Structural Mixins, Tree-Sitter validated AST splicing, and zero-LLM polyglot contract codegen.

## Features

1. **Inline Typed Hole Parser & Span Extractor**: Parse `?[hole_id: ExpectedType @ ast_node_kind]` markers and extract exact byte spans.
2. **Embedded Starter Packs & Local Overrides**: 15+ verified Rust and TypeScript patterns embedded via `include_str!`, plus support for `.dado/patterns/` overrides.
3. **Composable Structural Mixins**: Apply reusable structural wrappers (`tracing`, `sqlx_tx`, `tokio_timeout`, `try_catch_log`) cleanly with anchor chaining on `?[__inner__]`.
4. **Tree-Sitter Validated AST Splicer**: Substitute holes and inject missing imports with zero `ERROR` or `MISSING` nodes.
5. **Zero-LLM Polyglot Contract Codegen**: Emit typed Rust serde structs and TypeScript interfaces / Zod schemas from `chalkline` contracts.

