use chrono::{DateTime, Utc};

/// Get current UTC timestamp as ISO string.
pub fn now_iso() -> String {
    Utc::now().to_rfc3339()
}

/// Get current UTC timestamp.
pub fn now() -> DateTime<Utc> {
    Utc::now()
}
