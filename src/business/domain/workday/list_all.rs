use std::sync::{Arc, Mutex};

use super::dto::WorkdayDto;
use super::repository::WorkdayRepository;

pub struct ListAllUseCase {
    repository: Arc<Mutex<WorkdayRepository>>,
}

impl ListAllUseCase {
    pub fn new(repository: Arc<Mutex<WorkdayRepository>>) -> Self {
        Self { repository }
    }

    pub fn execute(&mut self) -> Result<Box<[WorkdayDto]>, Box<dyn std::error::Error>> {
        let guard = self.repository.lock().unwrap();

        let result = guard.find_all();

        if let Ok(all) = result {
            return Ok(all
                .into_iter()
                .map(|workday| WorkdayDto {
                    date: workday.date(),
                })
                .collect());
        }

        Err(Box::new(result.err().unwrap()))
    }
}
