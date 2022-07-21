use chrono::{DateTime, Utc};
use std::cmp::Ordering;

#[derive(Copy, Clone, PartialEq, Debug, Eq, PartialOrd)]
pub struct Session {
    started_at: DateTime<Utc>,
    ended_at: Option<DateTime<Utc>>,
}

impl Session {
    pub fn new(started_at: DateTime<Utc>, ended_at: Option<DateTime<Utc>>) -> Self {
        Self {
            started_at,
            ended_at,
        }
    }

    pub fn started_at(&self) -> DateTime<Utc> {
        self.started_at
    }

    pub fn ended_at(&self) -> Option<DateTime<Utc>> {
        self.ended_at
    }
}

impl Ord for Session {
    fn cmp(&self, other: &Self) -> Ordering {
        self.started_at.cmp(&other.started_at)
    }
}
