use serde::Serialize;
use std::fmt;

pub fn format_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string_pretty(value).unwrap_or_else(|_| "{}".to_string())
}

pub struct JsonOutput<T>(pub T);

impl<T: Serialize> fmt::Display for JsonOutput<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format_json(&self.0))
    }
}
