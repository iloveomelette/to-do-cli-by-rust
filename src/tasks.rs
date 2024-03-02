use chrono::{serde::ts_seconds, DateTime, Local};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,

    /*
     * Instructs the `ts_seconds` module to serialize (or deserialize vice versa) a value of type DateTime<Local>
     * as a UNIX epoch (seconds since January 1, 1970).
     */
    #[serde(with = "ts_seconds")]
    pub created_at: DateTime<Local>,
}

impl Task {
    pub fn new(text: String) -> Task {
        let created_at: DateTime<Local> = Local::now();
        Task { text, created_at }
    }
}
