# Behavioral Specifications & Invariants

## Formal Type Contracts

- **`mod::CanonicalType`** (`enum`): Neutral canonical types supported across polyglot boundaries.
- **`args::Cli`** (`struct`): Public interface entity
- **`args::Commands`** (`enum`): Available subcommands for `dado`.
- **`args::ContractArgs`** (`struct`): Arguments for `dado contract`.
- **`error::DadoError`** (`enum`): Strongly typed errors produced by the `dado` pattern engine.
- **`model::DadoMixin`** (`struct`): Composable structural wrapper that envelopes an inner anchor hole.
- **`model::DadoPattern`** (`struct`): A parameterized AST pattern containing template code and typed holes.
- **`args::InitArgs`** (`struct`): Arguments for `dado init`.
- **`hole::InlineHoleSpan`** (`struct`): A parsed inline hole marker with its byte offsets in the template string.
- **`args::InspectArgs`** (`struct`): Arguments for `dado inspect`.
- **`args::ListArgs`** (`struct`): Arguments for `dado list`.
- **`args::McpArgs`** (`struct`): Arguments for `dado mcp`.
- **`mod::PatternRegistry`** (`struct`): Central registry holding loaded AST patterns and structural mixins.
- **`model::PatternVerificationReceipt`** (`struct`): Verification result for a single pattern against its canonical fills.
- **`args::RenderArgs`** (`struct`): Arguments for `dado render`.
- **`mod::SpliceOptions`** (`struct`): Splicing options for controlling import injection and AST validation.
- **`model::SpliceResult`** (`struct`): Result of substituting hole fills and injecting imports into a template.
- **`model::TypedHole`** (`struct`): A single Typed Hole inside a `dado` AST template: `?[hole_id: expected_type @ ast_node_kind]`.
- **`args::VerifyPackArgs`** (`struct`): Arguments for `dado verify-pack`.
