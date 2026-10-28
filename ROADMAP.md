# Product Roadmap & Strategic Horizons — `dado`

> **Active Suite Lifecycle:** `0.1.0-beta.X`  
> **Target:** Deterministic Polyglot AST Pattern & Typed Hole Engine

---

## 1. Active Milestone: `v0.1.0-beta.3` (Current Focus)

- [x] **Inline Typed Hole Syntax:** Extraction and validation of `?[id: Type @ kind]` markers.
- [x] **Tree-sitter Splicing:** Zero-error AST verification for Rust, TypeScript, and JavaScript.
- [x] **Composable Structural Mixins:** Anchor-based chaining (`?[__inner__]`) with whitespace re-indentation.
- [x] **Contract Codegen:** Deterministic Rust serde and TypeScript Zod emission from SharedTypeContract schemas.
- [x] **Model Context Protocol (MCP):** JSON-RPC 2.0 stdio server exposing 5 core tools.
- [x] **9-Document Architecture Compliance:** Certified clean under `chalkline docs check`.

---

## 2. Horizon 1: Polyglot Expansion & Symbol Validation (`v0.2.0`)

- [ ] **Python & Go Language Grammars:** Add Tree-sitter parsers and starter packs for Python (FastAPI/Pydantic) and Go (Gin/Stdlib).
- [ ] **Miter Symbol Type Validation:** Validate hole fill candidates against `miter` symbol index to catch type mismatches before splicing.
- [ ] **Pattern Verification Harness:** Automated integration tests verifying that rendered patterns compile with `cargo check` and `tsc`.
- [ ] **Interactive TUI Pattern Browser:** Terminal visualizer for browsing pattern templates and previewing mixin compositions.

---

## 3. Horizon 2: Reverse Pattern Mining & Live Watcher (`v0.3.0`)

- [ ] **Reverse AST Pattern Miner:** Mine recurring code structures from historical git commits using `miter` clone detection and automatically emit candidate `dado` patterns.
- [ ] **Hot-Reload Pattern Watcher:** File-system notification daemon (`notify`) reloading local `.dado/patterns/` without restarting MCP or CLI daemons.
- [ ] **Dynamic Hole Expression Evaluator:** Support computed default values and identifier derivations in hole expressions.

---

## 4. Horizon 3: Federation & Enterprise Scale (`v1.0.0`)

- [ ] **Federated Pattern Registries:** Secure distributed sharing of proprietary enterprise pattern packs.
- [ ] **Zero-Copy In-Memory Splicing:** Shared-memory ring buffer connecting `dado` directly to `joiner` during synthesis loops.
- [ ] **Long-Term Pattern Stability:** Freeze pattern schema format for multi-year backward compatibility.
