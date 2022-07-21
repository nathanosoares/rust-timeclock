use chrono::{Date, Utc};

#[derive(Debug)]
pub struct WorkdayDto {}

pub struct CreateDto {
    pub date: Date<Utc>,
}
