use chrono::{Date, Utc, TimeZone};

use super::entity::Workday;
use super::session::Session;
use super::repository::WorkdayRepository;

pub struct CreateUseCase {
    repository: WorkdayRepository,
}

pub struct CreateDto {
    pub date: Date<Utc>,
}

impl CreateUseCase {
    pub fn new(repository: WorkdayRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&self, dto: CreateDto) -> Result<Workday, Box<dyn std::error::Error>> {
        let mut workday = Workday::new(dto.date);
        workday.add_session(Session::new(
            Utc.ymd(2022, 07, 01).and_hms(13, 30, 0),
            Some(Utc.ymd(2022, 07, 01).and_hms(18, 00, 0)),
        ))?;

        self.repository.create(&workday)?;

        Ok(workday)
    }
}
