use crate::errors::{FsError, Result};
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use humansize::{format_size, BINARY};

/// Parse human-readable size string (e.g., "10KB", "2 MiB", "500B")
pub fn parse_size(input: &str) -> Result<u64> {
    let input = input.trim().to_uppercase();
    let input = input.replace(" ", "");

    // Try to split number from unit
    let (num_str, unit) = input
        .char_indices()
        .find(|(_, c)| c.is_alphabetic())
        .map(|(idx, _)| input.split_at(idx))
        .unwrap_or((&input, ""));

    let number: f64 = num_str.parse().map_err(|_| FsError::InvalidSize {
        input: input.clone(),
    })?;

    let multiplier: u64 = match unit {
        "" | "B" => 1,
        "KB" => 1_000,
        "KIB" => 1_024,
        "MB" => 1_000_000,
        "MIB" => 1_048_576,
        "GB" => 1_000_000_000,
        "GIB" => 1_073_741_824,
        "TB" => 1_000_000_000_000,
        "TIB" => 1_099_511_627_776,
        _ => {
            return Err(FsError::InvalidSize {
                input: input.clone(),
            })
        }
    };

    Ok((number * multiplier as f64) as u64)
}

/// Format size in human-readable format using binary units
pub fn format_size_human(size: u64) -> String {
    format_size(size, BINARY)
}

/// Parse date string (ISO8601, YYYY-MM-DD, or relative like "7 days ago")
pub fn parse_date(input: &str) -> Result<DateTime<Utc>> {
    // Try parsing as RFC3339/ISO8601 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(input) {
        return Ok(dt.with_timezone(&Utc));
    }

    // Try YYYY-MM-DD format
    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
        return Utc
            .from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap())
            .single()
            .ok_or_else(|| FsError::InvalidDate {
                input: input.to_string(),
            });
    }

    // Try relative date parsing (e.g., "7 days ago", "2 weeks ago", "1 month ago")
    if let Some(relative_date) = parse_relative_date(input) {
        return Ok(relative_date);
    }

    Err(FsError::InvalidDate {
        input: input.to_string(),
    })
}

/// Parse relative date strings like "7 days ago", "2 weeks ago", "1 month ago"
fn parse_relative_date(input: &str) -> Option<DateTime<Utc>> {
    use chrono::Duration;

    let input = input.trim().to_lowercase();
    let parts: Vec<&str> = input.split_whitespace().collect();

    // Expected format: "<number> <unit> ago"
    if parts.len() != 3 || parts[2] != "ago" {
        return None;
    }

    let number: i64 = parts[0].parse().ok()?;
    let unit = parts[1];

    let now = Utc::now();

    match unit {
        "second" | "seconds" | "sec" | "secs" => Some(now - Duration::seconds(number)),
        "minute" | "minutes" | "min" | "mins" => Some(now - Duration::minutes(number)),
        "hour" | "hours" | "hr" | "hrs" => Some(now - Duration::hours(number)),
        "day" | "days" => Some(now - Duration::days(number)),
        "week" | "weeks" => Some(now - Duration::weeks(number)),
        "month" | "months" => Some(now - Duration::days(number * 30)),
        "year" | "years" => Some(now - Duration::days(number * 365)),
        _ => None,
    }
}

/// Check if output is to a TTY (terminal)
pub fn is_tty() -> bool {
    crossterm::tty::IsTty::is_tty(&std::io::stdout())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_size() {
        assert_eq!(parse_size("100").unwrap(), 100);
        assert_eq!(parse_size("10KB").unwrap(), 10_000);
        assert_eq!(parse_size("10KiB").unwrap(), 10_240);
        assert_eq!(parse_size("2MB").unwrap(), 2_000_000);
        assert_eq!(parse_size("2 MiB").unwrap(), 2_097_152);
        assert_eq!(parse_size("1GB").unwrap(), 1_000_000_000);
        assert_eq!(parse_size("1 GiB").unwrap(), 1_073_741_824);
        assert!(parse_size("invalid").is_err());
        assert!(parse_size("10XB").is_err());
    }

    #[test]
    fn test_format_size_human() {
        assert_eq!(format_size_human(0), "0 B");
        assert_eq!(format_size_human(1023), "1023 B");
        assert_eq!(format_size_human(1024), "1 KiB");
        assert_eq!(format_size_human(1_048_576), "1 MiB");
    }

    #[test]
    fn test_parse_date() {
        // YYYY-MM-DD format
        let result = parse_date("2024-01-01");
        assert!(result.is_ok());

        // ISO8601 format
        let result = parse_date("2024-01-01T12:00:00Z");
        assert!(result.is_ok());

        // Invalid format
        assert!(parse_date("invalid").is_err());
    }
}
