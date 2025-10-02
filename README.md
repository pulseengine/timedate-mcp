# TimeDate MCP Server

A Model Context Protocol (MCP) server providing comprehensive time and date operations with timezone support,
built with Rust and the PulseEngine MCP framework.

## Features

- **Current Time**: Get current time in any timezone
- **Time Calculations**: Add/subtract time from dates
- **Timezone Conversion**: Convert time between different timezones
- **Timezone Information**: Get detailed timezone data including DST status
- **Time Format Detection**: Detect and work with 12-hour/24-hour formats
- **Timezone Listing**: Browse available timezones with filtering

## Installation

Install via npm:

```bash
npx @pulseengine/timedate-mcp-server
```

Or install globally:

```bash
npm install -g @pulseengine/timedate-mcp-server
timedate-mcp-server
```

## Available Tools

### `get_current_time`

Get the current time in the specified timezone (defaults to system timezone).

**Parameters:**

- `timezone` (optional): Target timezone (e.g., "America/New_York", "Europe/London")

### `get_time_at`

Get time at a specific date and timezone.

**Parameters:**

- `date_time`: Date/time string (RFC3339, "YYYY-MM-DD HH:MM:SS", or "YYYY-MM-DD")
- `timezone` (optional): Target timezone

### `calculate_time_offset`

Add or subtract time from a given date.

**Parameters:**

- `base_time`: Base time ("now" or date string)
- `offset_hours`: Hours to add (positive) or subtract (negative)
- `timezone` (optional): Target timezone

### `get_timezone_info`

Get information about the current system timezone.

### `convert_timezone`

Convert time between different timezones.

**Parameters:**

- `time`: Time to convert ("now" or date string)
- `from_timezone`: Source timezone
- `to_timezone`: Target timezone

### `get_time_format`

Detect time format preference (12-hour vs 24-hour).

### `list_timezones`

List available timezones with optional filtering.

**Parameters:**

- `filter` (optional): Filter string to match timezone names

## Example Usage

```javascript
// Get current time in Tokyo
await server.get_current_time({ timezone: "Asia/Tokyo" });

// Convert New York time to London time
await server.convert_timezone({
  time: "2024-01-15 14:30:00",
  from_timezone: "America/New_York",
  to_timezone: "Europe/London"
});

// Add 5 hours to current time
await server.calculate_time_offset({
  base_time: "now",
  offset_hours: 5,
  timezone: "UTC"
});
```

## Response Format

All time responses include:

- `timestamp`: ISO 8601/RFC3339 formatted time
- `timezone`: Timezone identifier
- `utc_offset`: UTC offset (e.g., "+0900", "-0500")
- `is_dst`: Daylight saving time status
- `format_12h`: 12-hour format display
- `format_24h`: 24-hour format display

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/pulseengine/timedate-mcp.git
cd timedate-mcp

# Build the project
cargo build --release

# Run tests
cargo test

# Run the server
cargo run
```

### Cross-Platform Builds

```bash
# Build for all platforms
npm run build:all

# Or build for specific platforms
npm run build:linux-x64
npm run build:darwin-x64
npm run build:darwin-arm64
npm run build:win32-x64
```

### Pre-commit Hooks

This project uses pre-commit hooks to ensure code quality:

```bash
# Install pre-commit (if not already installed)
pip install pre-commit

# Install git hooks
pre-commit install

# Run manually on all files
pre-commit run --all-files
```

**Hooks included:**

- Trailing whitespace removal
- End-of-file fixing
- YAML/JSON/TOML validation
- Markdown linting
- Rust formatting (cargo fmt)
- Rust linting (cargo clippy)
- Rust compilation check

## Architecture

Built using:

- **Rust**: Core implementation for performance and safety
- **PulseEngine MCP Framework**: MCP protocol implementation with macros
- **Chrono & Chrono-TZ**: Comprehensive timezone and datetime handling
- **Tokio**: Async runtime for scalable operations

## License

MIT License - see [LICENSE](LICENSE) for details.

## Contributing

Contributions welcome! Please read our contributing guidelines and submit pull requests to our [GitHub repository](https://github.com/pulseengine/timedate-mcp).
