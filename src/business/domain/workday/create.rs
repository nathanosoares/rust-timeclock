use chrono::{Date, TimeZone, Utc};

use super::entity::Workday;
use super::repository::WorkdayRepository;
use super::session::Session;

pub struct CreateUseCase<'a> {
    repository: &'a mut WorkdayRepository,
}

pub struct CreateDto {
    pub date: Date<Utc>,
}

#[derive(Debug)]
pub struct WorkdayDto {}

impl<'a> CreateUseCase<'a> {
    pub fn new(repository: &'a mut WorkdayRepository) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self, dto: CreateDto) -> Result<WorkdayDto, Box<dyn std::error::Error>> {
        let mut workday = Workday::new(dto.date);
        workday.add_session(Session::new(
            Utc.ymd(2022, 07, 01).and_hms(13, 30, 0),
            Some(Utc.ymd(2022, 07, 01).and_hms(18, 00, 0)),
        ))?;

        let dto = WorkdayDto {};

        self.repository.create(workday)?;

        Ok(dto)
    }
}
