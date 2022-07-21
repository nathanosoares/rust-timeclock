use std::sync::{Mutex, Arc};

use chrono::{TimeZone, Utc};

use super::dto::{CreateDto, WorkdayDto};
use super::entity::Workday;
use super::repository::WorkdayRepository;
use super::session::Session;

pub struct CreateUseCase<'a> {
    repository: Arc<&'a Mutex<WorkdayRepository>>,
}

impl<'a> CreateUseCase<'a> {
    pub fn new(repository: &'a Mutex<WorkdayRepository>) -> Self {
        Self { repository: Arc::new(repository) }
    }

    pub fn execute(&mut self, dto: CreateDto) -> Result<WorkdayDto, Box<dyn std::error::Error>> {
        let mut workday = Workday::new(dto.date);
        workday.add_session(Session::new(
            Utc.ymd(2022, 07, 01).and_hms(13, 30, 0),
            Some(Utc.ymd(2022, 07, 01).and_hms(18, 00, 0)),
        ))?;

        let dto = WorkdayDto {
            date: workday.date()
        };

        let mut guard = self.repository.lock().unwrap();

        guard.create(workday)?;

        Ok(dto)
    }
}
