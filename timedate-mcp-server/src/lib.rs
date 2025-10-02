//! TimeDate MCP Server - Time and Date Operations with Timezone Support

use chrono::{DateTime, Local, TimeZone, Utc};
use chrono_tz::{Tz, TZ_VARIANTS};
use pulseengine_mcp_macros::{mcp_server, mcp_tools, mcp_resource};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeInfo {
    pub timestamp: String,
    pub timezone: String,
    pub utc_offset: String,
    pub is_dst: bool,
    pub format_12h: String,
    pub format_24h: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimezoneInfo {
    pub name: String,
    pub current_time: String,
    pub utc_offset: String,
    pub is_dst: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TimeFormatInfo {
    pub detected_format: String,
    pub is_12_hour: bool,
    pub current_time_12h: String,
    pub current_time_24h: String,
}

/// TimeDate MCP Server - Time and Date Operations with Timezone Support
#[mcp_server(
    name = "TimeDate MCP Server",
    version = "0.4.0",
    description = "A Model Context Protocol server for time and date operations with timezone support with parameterized resources",
    auth = "disabled"
)]
#[derive(Default, Clone)]
pub struct TimeDateServer;

#[mcp_tools]
impl TimeDateServer {

    /// Get time at a specific date and timezone
    pub async fn get_time_at(
        &self,
        date_time: String,
        timezone: Option<String>,
    ) -> anyhow::Result<TimeInfo> {
        let tz = match timezone {
            Some(tz_str) => Tz::from_str(&tz_str)
                .map_err(|_| anyhow::anyhow!("Invalid timezone: {}", tz_str))?,
            None => chrono_tz::UTC,
        };

        let dt = chrono::DateTime::parse_from_rfc3339(&date_time)
            .or_else(|_| chrono::DateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M:%S"))
            .or_else(|_| chrono::DateTime::parse_from_str(&date_time, "%Y-%m-%d"))
            .map_err(|_| anyhow::anyhow!("Invalid date format: {}", date_time))?
            .with_timezone(&tz);

        Ok(self.format_time_info(dt))
    }

    /// Calculate time with offset (add/subtract time)
    pub async fn calculate_time_offset(
        &self,
        base_time: String,
        offset_hours: i32,
        timezone: Option<String>,
    ) -> anyhow::Result<TimeInfo> {
        let tz = match timezone {
            Some(tz_str) => Tz::from_str(&tz_str)
                .map_err(|_| anyhow::anyhow!("Invalid timezone: {}", tz_str))?,
            None => chrono_tz::UTC,
        };

        let base_dt = if base_time.to_lowercase() == "now" {
            Utc::now().with_timezone(&tz)
        } else {
            chrono::DateTime::parse_from_rfc3339(&base_time)
                .or_else(|_| chrono::DateTime::parse_from_str(&base_time, "%Y-%m-%d %H:%M:%S"))
                .map_err(|_| anyhow::anyhow!("Invalid date format: {}", base_time))?
                .with_timezone(&tz)
        };

        let result_dt = base_dt + chrono::Duration::hours(offset_hours as i64);
        Ok(self.format_time_info(result_dt))
    }


    /// Convert time between timezones
    pub async fn convert_timezone(
        &self,
        time: String,
        from_timezone: String,
        to_timezone: String,
    ) -> anyhow::Result<TimeInfo> {
        let from_tz = Tz::from_str(&from_timezone)
            .map_err(|_| anyhow::anyhow!("Invalid source timezone: {}", from_timezone))?;
        
        let to_tz = Tz::from_str(&to_timezone)
            .map_err(|_| anyhow::anyhow!("Invalid target timezone: {}", to_timezone))?;

        let dt = if time.to_lowercase() == "now" {
            Utc::now().with_timezone(&from_tz)
        } else {
            chrono::DateTime::parse_from_rfc3339(&time)
                .or_else(|_| chrono::DateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S"))
                .map_err(|_| anyhow::anyhow!("Invalid time format: {}", time))?
                .with_timezone(&from_tz)
        };

        let converted = dt.with_timezone(&to_tz);
        Ok(self.format_time_info(converted))
    }

    
    /// Get current time in local timezone (exposed as a tool for now)
    pub async fn get_current_time(&self, timezone: Option<String>) -> anyhow::Result<TimeInfo> {
        self.get_current_time_internal(timezone).await
    }

    /// Get current timezone information  
    pub async fn get_timezone_info(&self) -> anyhow::Result<TimezoneInfo> {
        let now = Local::now();
        let _offset = now.offset();
        
        Ok(TimezoneInfo {
            name: "Local".to_string(), // Local timezone doesn't expose name directly
            current_time: now.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            utc_offset: now.format("%z").to_string(),
            is_dst: false, // Can't easily determine DST for local timezone
        })
    }

    /// Get time format preference information
    pub async fn get_time_format(&self) -> anyhow::Result<TimeFormatInfo> {
        self.get_time_format_internal().await
    }

    /// List available timezones
    pub async fn list_timezones(&self, filter: Option<String>) -> anyhow::Result<Vec<String>> {
        self.list_timezones_internal(filter).await
    }

    // Resources - Read-only data accessible via MCP resource URIs
    
    /// Get current time as a resource
    #[mcp_resource(
        uri_template = "timedate://current-time/{timezone}",
        name = "current_time",
        description = "Current time in the specified timezone",
        mime_type = "application/json"
    )]
    pub async fn current_time_resource(&self, timezone: String) -> anyhow::Result<TimeInfo> {
        let tz_option = if timezone == "local" { None } else { Some(timezone) };
        self.get_current_time_internal(tz_option).await
    }

    /// Get timezone information as a resource
    #[mcp_resource(
        uri_template = "timedate://timezone-info",
        name = "timezone_info", 
        description = "Information about the local timezone",
        mime_type = "application/json"
    )]
    pub async fn timezone_info_resource(&self) -> anyhow::Result<TimezoneInfo> {
        let now = Local::now();
        
        Ok(TimezoneInfo {
            name: "Local".to_string(),
            current_time: now.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            utc_offset: now.format("%z").to_string(),
            is_dst: false,
        })
    }

    /// Get list of available timezones as a resource
    #[mcp_resource(
        uri_template = "timedate://timezones/{filter}",
        name = "timezone_list",
        description = "List of available timezones, optionally filtered",
        mime_type = "application/json"
    )]
    pub async fn timezone_list_resource(&self, filter: String) -> anyhow::Result<Vec<String>> {
        let filter_option = if filter == "all" { None } else { Some(filter) };
        self.list_timezones_internal(filter_option).await
    }

    /// Get time format preferences as a resource
    #[mcp_resource(
        uri_template = "timedate://time-format",
        name = "time_format",
        description = "Time format preferences and current time in both formats",
        mime_type = "application/json"
    )]
    pub async fn time_format_resource(&self) -> anyhow::Result<TimeFormatInfo> {
        self.get_time_format_internal().await
    }
}

// Resources now working with framework v0.9.1!

// Internal implementation (for now we'll keep the improved architecture concept but use tools)
impl TimeDateServer {


    // Internal helper methods (moved from tools)
    async fn get_current_time_internal(&self, timezone: Option<String>) -> anyhow::Result<TimeInfo> {
        let tz = match timezone {
            Some(tz_str) => Tz::from_str(&tz_str)
                .map_err(|_| anyhow::anyhow!("Invalid timezone: {}", tz_str))?,
            None => chrono_tz::UTC, // Default to UTC if no timezone specified
        };

        let now = Utc::now().with_timezone(&tz);
        Ok(self.format_time_info(now))
    }

    async fn list_timezones_internal(&self, filter: Option<String>) -> anyhow::Result<Vec<String>> {
        let timezones: Vec<String> = TZ_VARIANTS
            .iter()
            .map(|tz| tz.name().to_string())
            .filter(|name| {
                match &filter {
                    Some(f) => name.to_lowercase().contains(&f.to_lowercase()),
                    None => true,
                }
            })
            .take(50) // Limit results
            .collect();

        Ok(timezones)
    }

    async fn get_time_format_internal(&self) -> anyhow::Result<TimeFormatInfo> {
        let now = Local::now();
        
        // Simple heuristic: check system locale or default to 24h
        let is_12_hour = std::env::var("LC_TIME")
            .unwrap_or_default()
            .contains("US") || std::env::var("LANG")
            .unwrap_or_default()
            .starts_with("en_US");

        Ok(TimeFormatInfo {
            detected_format: if is_12_hour { "12-hour".to_string() } else { "24-hour".to_string() },
            is_12_hour,
            current_time_12h: now.format("%I:%M:%S %p").to_string(),
            current_time_24h: now.format("%H:%M:%S").to_string(),
        })
    }
}

impl TimeDateServer {
    fn format_time_info<Tz: TimeZone>(&self, dt: DateTime<Tz>) -> TimeInfo
    where
        Tz::Offset: std::fmt::Display,
    {
        // Extract timezone name from the formatted string
        let tz_name = dt.format("%Z").to_string();
        
        TimeInfo {
            timestamp: dt.to_rfc3339(),
            timezone: tz_name,
            utc_offset: dt.format("%z").to_string(),
            is_dst: false, // Simplified - would need more complex logic to detect DST
            format_12h: dt.format("%I:%M:%S %p").to_string(),
            format_24h: dt.format("%H:%M:%S").to_string(),
        }
    }
}

