//! CLI argument definitions for `dado`.

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Command-line interface for the `dado` deterministic polyglot AST pattern engine.
#[derive(Debug, Parser)]
#[command(
    name = "dado",
    about = "Deterministic AST pattern and typed hole engine for neurosymbolic synthesis",
    version
)]
pub struct Cli {
    /// Target repository directory containing optional `.dado/` (defaults to current dir).
    #[arg(long, global = true, default_value = ".")]
    pub dir: PathBuf,

    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available subcommands for `dado`.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// List registered AST patterns.
    List(ListArgs),

    /// Inspect a specific pattern and its typed holes.
    Inspect(InspectArgs),

    /// Render an AST pattern with optional mixins and hole values.
    Render(RenderArgs),

    /// Generate Rust struct or TypeScript/Zod definitions from a SharedTypeContract.
    Contract(ContractArgs),

    /// Verify registered patterns against their canonical example fills.
    VerifyPack(VerifyPackArgs),

    /// Initialize local `.dado/patterns/` and `.dado/mixins/` directories.
    Init(InitArgs),

    /// Start the stdio JSON-RPC 2.0 Model Context Protocol (MCP) server.
    Mcp(McpArgs),
}

/// Arguments for `dado list`.
#[derive(Debug, Args)]
pub struct ListArgs {
    /// Filter patterns by language (rust, typescript, tsx, javascript).
    #[arg(short, long)]
    pub lang: Option<String>,

    /// Search query matching pattern ID or description.
    #[arg(short, long)]
    pub query: Option<String>,

    /// Format output as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `dado inspect`.
#[derive(Debug, Args)]
pub struct InspectArgs {
    /// Identifier of the pattern to inspect.
    pub pattern_id: String,

    /// Format output as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `dado render`.
#[derive(Debug, Args)]
pub struct RenderArgs {
    /// Identifier of the base pattern to render.
    pub pattern_id: String,

    /// Composable structural mixin IDs to apply in sequence.
    #[arg(short, long)]
    pub mixin: Vec<String>,

    /// JSON string or file path containing `{"hole_id": "value"}` mappings.
    #[arg(long)]
    pub holes: String,

    /// Optional file containing available imports or LexicalScope JSON.
    #[arg(long)]
    pub scope: Option<PathBuf>,

    /// Format output as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `dado contract`.
#[derive(Debug, Args)]
pub struct ContractArgs {
    /// Path to `SharedTypeContract` JSON file.
    #[arg(short, long)]
    pub input: PathBuf,

    /// Target language or schema (rust, typescript, zod).
    #[arg(short, long, default_value = "rust")]
    pub target: String,

    /// Format output as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `dado verify-pack`.
#[derive(Debug, Args)]
pub struct VerifyPackArgs {
    /// Format output as JSON.
    #[arg(long)]
    pub json: bool,
}

/// Arguments for `dado init`.
#[derive(Debug, Args)]
pub struct InitArgs {}

/// Arguments for `dado mcp`.
#[derive(Debug, Args)]
pub struct McpArgs {}
