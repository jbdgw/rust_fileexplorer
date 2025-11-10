//! Frecency calculation for project ranking
//!
//! Implements a Firefox-style frecency algorithm that combines
//! frequency (access count) and recency (time since last access)
//! to intelligently rank projects.

use chrono::{DateTime, Duration, Utc};

/// Calculate frecency score for a project
///
/// Combines frequency (how often accessed) and recency (how recently accessed)
/// into a single score for ranking projects.
///
/// Formula:
/// - Frequency component: ln(access_count + 1) * 10.0
/// - Recency component: time-decay buckets (100 pts for recent, 10 pts for old)
/// - Final score: frequency + recency
///
/// # Arguments
/// * `access_count` - Number of times project has been accessed
/// * `last_accessed` - When the project was last accessed (None if never)
///
/// # Returns
/// A score where higher values indicate more relevant projects
pub fn calculate_frecency(access_count: u32, last_accessed: Option<DateTime<Utc>>) -> f64 {
    // Frequency component: logarithmic scaling prevents very high counts from dominating
    // Adding 1 before ln ensures ln(0+1) = 0 for never-accessed projects
    let frequency_score = ((access_count + 1) as f64).ln() * 10.0;

    // Recency component: time-decay based on age
    let recency_score = if let Some(last_access) = last_accessed {
        let now = Utc::now();
        let age = now.signed_duration_since(last_access);
        recency_weight(age)
    } else {
        0.0 // Never accessed
    };

    frequency_score + recency_score
}

/// Calculate recency weight based on time since last access
///
/// Uses time buckets similar to Firefox's frecency algorithm:
/// - 0-4 days: 100 points (very recent)
/// - 5-14 days: 70 points (recent)
/// - 15-31 days: 50 points (this month)
/// - 32-90 days: 30 points (this quarter)
/// - 90+ days: 10 points (old)
///
/// This creates a gentle decay curve that keeps recently-used projects
/// highly ranked while not completely forgetting older projects.
fn recency_weight(age: Duration) -> f64 {
    let days = age.num_days();

    match days {
        0..=4 => 100.0,   // Within 4 days - highly relevant
        5..=14 => 70.0,   // Within 2 weeks - still recent
        15..=31 => 50.0,  // Within month - relevant
        32..=90 => 30.0,  // Within 3 months - somewhat relevant
        _ => 10.0,        // Older - less relevant but not forgotten
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_calculate_frecency_never_accessed() {
        let score = calculate_frecency(0, None);
        // ln(1) * 10 + 0 = 0
        assert_eq!(score, 0.0);
    }

    #[test]
    fn test_calculate_frecency_accessed_today() {
        let now = Utc::now();
        let score = calculate_frecency(5, Some(now));

        // ln(6) * 10 + 100
        let expected = (6.0_f64).ln() * 10.0 + 100.0;
        assert!((score - expected).abs() < 0.001);
    }

    #[test]
    fn test_calculate_frecency_accessed_week_ago() {
        let week_ago = Utc::now() - Duration::days(7);
        let score = calculate_frecency(3, Some(week_ago));

        // ln(4) * 10 + 70
        let expected = (4.0_f64).ln() * 10.0 + 70.0;
        assert!((score - expected).abs() < 0.001);
    }

    #[test]
    fn test_calculate_frecency_accessed_month_ago() {
        let month_ago = Utc::now() - Duration::days(20);
        let score = calculate_frecency(10, Some(month_ago));

        // ln(11) * 10 + 50
        let expected = (11.0_f64).ln() * 10.0 + 50.0;
        assert!((score - expected).abs() < 0.001);
    }

    #[test]
    fn test_calculate_frecency_accessed_long_ago() {
        let long_ago = Utc::now() - Duration::days(100);
        let score = calculate_frecency(2, Some(long_ago));

        // ln(3) * 10 + 10
        let expected = (3.0_f64).ln() * 10.0 + 10.0;
        assert!((score - expected).abs() < 0.001);
    }

    #[test]
    fn test_recency_weight() {
        assert_eq!(recency_weight(Duration::days(0)), 100.0);
        assert_eq!(recency_weight(Duration::days(2)), 100.0);
        assert_eq!(recency_weight(Duration::days(4)), 100.0);
        assert_eq!(recency_weight(Duration::days(5)), 70.0);
        assert_eq!(recency_weight(Duration::days(10)), 70.0);
        assert_eq!(recency_weight(Duration::days(20)), 50.0);
        assert_eq!(recency_weight(Duration::days(60)), 30.0);
        assert_eq!(recency_weight(Duration::days(100)), 10.0);
    }

    #[test]
    fn test_frecency_favors_recent_over_frequent() {
        let recent_low_count = calculate_frecency(2, Some(Utc::now()));
        let old_high_count = calculate_frecency(20, Some(Utc::now() - Duration::days(100)));

        // Recent project with low count should score higher than
        // old project with high count (demonstrates recency bias)
        assert!(recent_low_count > old_high_count);
    }

    #[test]
    fn test_frecency_frequency_still_matters() {
        let recent_high = calculate_frecency(20, Some(Utc::now()));
        let recent_low = calculate_frecency(2, Some(Utc::now()));

        // With same recency, higher frequency should win
        assert!(recent_high > recent_low);
    }
}
