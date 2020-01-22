use chrono::{DateTime, Utc, Duration};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    const GIGA_SECOND: i64 = 1_000_000_000;
    start + Duration::seconds(GIGA_SECOND)    
}
