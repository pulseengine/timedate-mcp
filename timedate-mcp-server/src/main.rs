//! TimeDate MCP Server - Time and Date Operations with Timezone Support

use pulseengine_mcp_server::McpServerBuilder;
use timedate_mcp_server::TimeDateServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure logging for STDIO transport
    TimeDateServer::configure_stdio_logging();

    // Start the server using the macro-generated infrastructure
    let mut server = TimeDateServer::with_defaults().serve_stdio().await?;
    server.run().await?;

    Ok(())
}
