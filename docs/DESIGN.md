# Architectural Design & Decision Records — `dado`

> **Crates:** `dado`, `dado-core`, `dado-cli`, `dado-mcp`  
> **Status:** Active Architectural Decision Records (ADRs)

---

## Architectural Decision Records (ADRs)

### ADR-001: Inline Typed Hole Syntax vs Handlebars/Jinja Text Templating
- **Status:** Accepted
- **Context:** General-purpose text template engines (Handlebars, Jinja, Mustache) treat source code as arbitrary text strings. They cannot enforce target semantic types, cannot restrict syntax node kinds (e.g. statement vs expression), and frequently produce syntax errors when brackets collide.
- **Decision:** Introduce a dedicated typed hole syntax: `?[hole_id: ExpectedType @ ast_node_kind]`. Parse hole spans into explicit byte ranges, extract semantic constraints, and validate that values match the expected AST node kind.
- **Consequences:** Templates remain valid syntax structures that can be audited by language parsers while providing explicit type constraints directly to downstream grammar compilers (`batten`).

### ADR-002: Tree-Sitter AST Validation Over Regular Expression Matching
- **Status:** Accepted
- **Context:** Template rendering can fail silently if hole substitutions introduce unclosed parentheses, invalid identifiers, or malformed syntax that only gets caught later during compilation.
- **Decision:** Parse every spliced output with the respective language's Tree-sitter grammar (using `tree-sitter-rust` and `tree-sitter-typescript`). Walk the resulting tree and reject any rendering containing `ERROR` or `MISSING` nodes.
- **Consequences:** Splicing failures are caught instantly in `<2ms` at the template boundary rather than triggering expensive compiler subprocesses.

### ADR-003: Anchor-Based Composable Mixin Chaining (`?[__inner__]`)
- **Status:** Accepted
- **Context:** Real-world code patterns frequently require cross-cutting wrappers (logging, Prometheus metrics, timeouts, database transactions) that should be modular and reusable without duplicating base templates.
- **Decision:** Define structural mixins as templates containing a special `?[__inner__]` anchor hole. When multiple mixins are specified (`--mixin tracing --mixin tokio_timeout`), compose them outwards from the base pattern by recursive substitution, automatically adjusting indentation.
- **Consequences:** Highly modular templates with zero combinatorial explosion of pattern variants.

### ADR-004: Embedded Starter Packs (`include_str!`) with Local Project Overrides
- **Status:** Accepted
- **Context:** `dado` must work immediately out of the box with zero network access and zero configuration, while allowing teams to define custom project-specific patterns.
- **Decision:** Embed 15+ verified starter packs directly into the binary using `include_str!`, while checking for local overrides under `.dado/patterns/` and `.dado/mixins/` if present.
- **Consequences:** Zero cold-start delay, 100% offline capability, and seamless customization.

### ADR-005: Zero-LLM Deterministic Contract Codegen for Multi-Language Schemas
- **Status:** Accepted
- **Context:** Generating data transfer objects across Rust backends and TypeScript frontends frequently causes subtle field type mismatches when generated via stochastic LLM sampling.
- **Decision:** Implement pure deterministic codegen in `dado contract`: compile `SharedTypeContract` models into canonical Rust serde structs and TypeScript Zod schemas using strict AST emitters with zero neural generation.
- **Consequences:** 100% contract fidelity with zero token spend and zero hallucination risk.
