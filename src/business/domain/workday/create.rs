use chrono::{Date, Utc};

use super::entity::Workday;
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
        let workday = Workday::new(dto.date);

        self.repository.create(&workday)?;

        Ok(workday)
    }
}
