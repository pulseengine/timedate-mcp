//! TimeDate MCP Server - Time and Date Operations with Timezone Support

use timedate_mcp_server::TimeDateServer;
use tracing::info;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Initialize comprehensive logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    info!("🚀 Starting TimeDate MCP Server");
    info!("📦 Version: {}", env!("CARGO_PKG_VERSION"));
    info!("🔧 Features: Time and date operations with timezone support");

    // Create and configure the server with defaults 
    let server = TimeDateServer::with_defaults()
        .serve_stdio()
        .await?;

    info!("✅ TimeDate MCP Server started successfully");
    info!("🛠️  Available Tools:");
    info!("   • get_current_time - Get current time in specified timezone");
    info!("   • get_time_at - Get time at specific date/timezone");
    info!("   • calculate_time_offset - Add/subtract time from date");
    info!("   • get_timezone_info - Get current timezone information");
    info!("   • convert_timezone - Convert time between timezones");
    info!("   • get_time_format - Detect time format preference");
    info!("   • list_timezones - List available timezones");
    info!("🔗 Connect using any MCP client via stdio transport");

    // Run the server with automatic capability detection
    server
        .run()
        .await
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    info!("👋 TimeDate MCP Server stopped gracefully");
    Ok(())
}