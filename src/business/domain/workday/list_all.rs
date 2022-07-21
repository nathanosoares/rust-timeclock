use std::sync::{Mutex, Arc};

use super::dto::WorkdayDto;
use super::repository::WorkdayRepository;

pub struct ListAllUseCase<'a> {
    repository: Arc<&'a Mutex<WorkdayRepository>>,
}

impl<'a> ListAllUseCase<'a> {
    pub fn new(repository: &'a Mutex<WorkdayRepository>) -> Self {
        Self { repository: Arc::new(repository) }
    }

    pub fn execute(&mut self) -> Result<Box<[WorkdayDto]>, Box<dyn std::error::Error>> {
        let guard = self.repository.lock().unwrap();

        let result = guard.find_all();

        if let Ok(all) = result {
            return Ok(all.into_iter().map(|_workday| WorkdayDto {}).collect());
        }

        Err(Box::new(result.err().unwrap()))
    }
}
