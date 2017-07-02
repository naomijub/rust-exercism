extern crate chrono;
use chrono::*;

// Returns a UTC DateTime one billion seconds after start.
pub fn after(start: DateTime<UTC>) -> DateTime<UTC> {
    start + Duration::seconds(1_000_000_000)
}
