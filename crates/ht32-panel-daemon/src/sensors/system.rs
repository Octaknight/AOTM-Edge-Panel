//! System information sensor for hostname, uptime, and time.

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

/// System information provider.
pub struct SystemInfo;

impl SystemInfo {
    /// Creates a new system info provider.
    pub fn new() -> Self {
        Self
    }

    /// Returns the hostname of the system.
    pub fn hostname(&self) -> String {
        fs::read_to_string("/etc/hostname")
            .map(|s| s.trim().to_string())
            .or_else(|_| {
                fs::read_to_string("/proc/sys/kernel/hostname").map(|s| s.trim().to_string())
            })
            .unwrap_or_else(|_| "unknown".to_string())
    }

    /// Returns the current time formatted as "HH:MM".
    pub fn time(&self) -> String {
        let (hours, minutes, _, _, _, _, _) = self.time_components();
        format!("{:02}:{:02}", hours, minutes)
    }

    /// Returns individual time/date components: (hour, minute, day, month, year, day_of_week).
    /// Day of week: 0=Sunday, 1=Monday, ..., 6=Saturday.
    pub fn time_components(&self) -> (u8, u8, u8, u8, u16, u8, u64) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        let secs = now.as_secs();

        // Get local time offset
        let offset = self.timezone_offset();
        let local_secs = if offset >= 0 {
            secs.wrapping_add(offset as u64)
        } else {
            secs.wrapping_sub((-offset) as u64)
        };

        // Time of day
        let hours = ((local_secs % 86400) / 3600) as u8;
        let minutes = ((local_secs % 3600) / 60) as u8;

        // Calculate date using days since Unix epoch
        let days_since_epoch = (local_secs / 86400) as i64;

        // Day of week (Jan 1, 1970 was Thursday = 4)
        let day_of_week = ((days_since_epoch + 4) % 7) as u8;

        // Calculate year, month, day
        let (year, month, day) = Self::days_to_ymd(days_since_epoch);

        (hours, minutes, day, month, year, day_of_week, local_secs)
    }

    /// Converts days since Unix epoch to (year, month, day).
    fn days_to_ymd(days: i64) -> (u16, u8, u8) {
        // Algorithm from https://howardhinnant.github.io/date_algorithms.html
        let z = days + 719468;
        let era = if z >= 0 { z } else { z - 146096 } / 146097;
        let doe = (z - era * 146097) as u32; // day of era
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe as i64 + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u8;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u8;
        let y = if m <= 2 { y + 1 } else { y };
        (y as u16, m, d)
    }

    /// Returns the timezone offset in seconds from UTC (positive = east of UTC).
    fn timezone_offset(&self) -> i64 {
        // Try to read from /etc/localtime or use environment
        // For simplicity, check TZ environment or default to 0 (UTC)
        // A full implementation would parse the timezone database
        if let Ok(tz) = std::env::var("TZ") {
            // Simple parsing for offset-based timezones like "UTC-5" or "UTC+10"
            if tz.starts_with("UTC") || tz.starts_with("GMT") {
                let offset_str = &tz[3..];
                if let Ok(hours) = offset_str.parse::<i64>() {
                    // Note: TZ convention is inverted (UTC-5 means +5 hours from UTC)
                    return hours * -3600;
                }
            }
        }

        // Try to get from libc (more reliable)
        #[cfg(target_os = "linux")]
        {
            use std::io::Read;
            if let Ok(mut file) = fs::File::open("/etc/timezone") {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    // Common timezones - simplified lookup
                    let tz = contents.trim();
                    return match tz {
                        "America/New_York" | "US/Eastern" => -5 * 3600,
                        "America/Los_Angeles" | "US/Pacific" => -8 * 3600,
                        "Europe/London" | "GB" => 0,
                        "Europe/Paris" | "Europe/Berlin" => 3600,
                        "Asia/Tokyo" | "Japan" => 9 * 3600,
                        "Asia/Kolkata" | "Asia/Calcutta" => (5 * 3600) + 1800, // UTC+5:30
                        _ => 0,
                    };
                }
            }
        }

        0 // Default to UTC
    }

    /// Returns the system uptime formatted as "Xd Yh Zm".
    pub fn uptime(&self) -> String {
        let uptime_secs = self.uptime_seconds();

        let days = uptime_secs / 86400;
        let hours = (uptime_secs % 86400) / 3600;
        let minutes = (uptime_secs % 3600) / 60;

        if days > 0 {
            format!("{}d {}h {}m", days, hours, minutes)
        } else if hours > 0 {
            format!("{}h {}m", hours, minutes)
        } else {
            format!("{}m", minutes)
        }
    }

    /// Returns the uptime in seconds.
    pub fn uptime_seconds(&self) -> u64 {
        fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|content| {
                content
                    .split_whitespace()
                    .next()
                    .and_then(|s| s.parse::<f64>().ok())
                    .map(|f| f as u64)
            })
            .unwrap_or(0)
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}
