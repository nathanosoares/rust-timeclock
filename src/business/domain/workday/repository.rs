use chrono::{Date, Utc};

use crate::business::domain::Workday;


#[derive(thiserror::Error, Debug, PartialEq)]
pub enum WorkdayRepositoryError {
    #[error("An open session already exists.")]
    Persistence,

    #[error("An open session already exists.")]
    EntityAlreadyExists,

    #[error("An open session already exists.")]
    EntityNotFound,
}

pub trait WorkdayPersistence {
    fn insert(&self, workday: &Workday) -> anyhow::Result<(), WorkdayRepositoryError>;

    fn find_by_day(&self, date: Date<Utc>) -> anyhow::Result<Option<Workday>, WorkdayRepositoryError>;
}

pub struct WorkdayRepository {
    pub persistence: Box<dyn WorkdayPersistence>,
}

impl WorkdayRepository {
    pub fn create(&self, workday: &Workday) -> anyhow::Result<(), WorkdayRepositoryError> {
        if let Some(_) = self.persistence.find_by_day(workday.date())? {
            return Err(WorkdayRepositoryError::EntityAlreadyExists);
        }

        self.persistence.insert(workday)?;

        Ok(())
    }
}
