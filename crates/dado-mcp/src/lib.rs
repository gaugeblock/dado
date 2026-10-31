//! Model Context Protocol (MCP) stdio server for `dado`.

#![forbid(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod dispatch;
pub mod schema;

use dado_core::PatternRegistry;
use gaugeblock_mcp::{McpHandler, RpcRequest, RpcResponse, ToolOutcome};
use serde_json::Value;
use std::io::{self};
use std::path::Path;

/// MCP server version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// `dado`'s tool surface, borrowing the registry the tools read.
#[derive(Debug)]
pub struct DadoHandler<'a> {
    registry: &'a PatternRegistry,
}

impl<'a> DadoHandler<'a> {
    /// Builds a handler over a loaded registry.
    #[must_use]
    pub fn new(registry: &'a PatternRegistry) -> Self {
        Self { registry }
    }
}

impl McpHandler for DadoHandler<'_> {
    fn server_name(&self) -> &str {
        "dado-mcp"
    }

    fn list_tools(&self) -> Value {
        Value::Array(schema::list_tools())
    }

    fn call_tool(&self, name: &str, arguments: &Value) -> ToolOutcome {
        match dispatch::dispatch_tool(name, arguments, self.registry) {
            Ok(content) => ToolOutcome::Json(content),
            Err(message) => ToolOutcome::Failed(message),
        }
    }
}

/// Runs the stdio JSON-RPC 2.0 MCP server loop over stdin and stdout.
///
/// # Errors
///
/// Returns the failure to build the registry, or the I/O failure on stdin or stdout.
pub fn run_stdio_server(root_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut registry = PatternRegistry::new()?;
    let _ = registry.load_from_dir(root_dir);
    tracing::info!("Starting dado-mcp stdio server loop");

    let handler = DadoHandler::new(&registry);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    gaugeblock_mcp::serve(stdin.lock(), &mut stdout, &handler)?;
    Ok(())
}

/// Handles a single message.
///
/// Kept because the existing tests drive the server this way; the routing lives in `gaugeblock-mcp`.
#[must_use]
pub fn handle_message(msg: &Value, registry: &PatternRegistry) -> Option<Value> {
    let request: RpcRequest = serde_json::from_value(msg.clone()).ok()?;
    gaugeblock_mcp::dispatch(&DadoHandler::new(registry), &request).map(RpcResponse::into_value)
}
