use chrono::{Date, Utc};

#[derive(Debug)]
pub struct WorkdayDto {
    pub date: Date<Utc>,
}

pub struct CreateDto {
    pub date: Date<Utc>,
}
