//! TimeDate MCP Server - Time and Date Operations with Timezone Support

use chrono::{DateTime, Local, TimeZone, Utc};
use chrono_tz::{Tz, TZ_VARIANTS};
use pulseengine_mcp_macros::{mcp_server, mcp_tools};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeInfo {
    pub timestamp: String,
    pub timezone: String,
    pub utc_offset: String,
    pub is_dst: bool,
    pub format_12h: String,
    pub format_24h: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimezoneInfo {
    pub name: String,
    pub current_time: String,
    pub utc_offset: String,
    pub is_dst: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeFormatInfo {
    pub detected_format: String,
    pub is_12_hour: bool,
    pub current_time_12h: String,
    pub current_time_24h: String,
}

/// TimeDate MCP Server - Time and Date Operations with Timezone Support
#[mcp_server(
    name = "TimeDate MCP Server",
    version = "0.1.0",
    description = "A Model Context Protocol server for time and date operations with timezone support",
    auth = "disabled"
)]
#[derive(Default, Clone)]
pub struct TimeDateServer;

#[mcp_tools]
impl TimeDateServer {
    /// Get the current time in the specified timezone (defaults to UTC)
    pub async fn get_current_time(&self, timezone: Option<String>) -> anyhow::Result<TimeInfo> {
        let tz = match timezone {
            Some(tz_str) => Tz::from_str(&tz_str)
                .map_err(|_| anyhow::anyhow!("Invalid timezone: {}", tz_str))?,
            None => chrono_tz::UTC, // Default to UTC if no timezone specified
        };

        let now = Utc::now().with_timezone(&tz);
        Ok(self.format_time_info(now))
    }

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

    /// Get current timezone information
    pub async fn get_timezone_info(&self) -> anyhow::Result<TimezoneInfo> {
        let now = Local::now();
        let offset = now.offset();
        
        // Try to determine if we're in DST by comparing with standard offset
        // This is a simple heuristic since FixedOffset doesn't have dst_offset
        let _is_dst = offset.local_minus_utc() != offset.local_minus_utc();
        
        Ok(TimezoneInfo {
            name: "Local".to_string(), // Local timezone doesn't expose name directly
            current_time: now.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
            utc_offset: now.format("%z").to_string(),
            is_dst: false, // Can't easily determine DST for local timezone
        })
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

    /// Get time format preference information
    pub async fn get_time_format(&self) -> anyhow::Result<TimeFormatInfo> {
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

    /// List available timezones
    pub async fn list_timezones(&self, filter: Option<String>) -> anyhow::Result<Vec<String>> {
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