use chrono::{Date, Utc};

use crate::business::domain::Workday;

#[derive(thiserror::Error, Debug)]
pub enum WorkdayRepositoryError {
    #[error(transparent)]
    Persistence(#[from] std::io::Error),

    #[error("Entity already exists.")]
    EntityAlreadyExists,

    #[error("Entity not found.")]
    EntityNotFound,
}

pub trait WorkdayPersistence {
    fn insert(&mut self, workday: Workday) -> anyhow::Result<(), WorkdayRepositoryError>;

    fn find_by_day(
        &self,
        date: Date<Utc>,
    ) -> anyhow::Result<Option<&Workday>, WorkdayRepositoryError>;

    fn find_all(&self) -> anyhow::Result<Vec<Workday>, WorkdayRepositoryError>;
}

pub struct WorkdayRepository {
    pub persistence: Box<dyn WorkdayPersistence>,
}

impl WorkdayRepository {
    pub fn create(&mut self, workday: Workday) -> anyhow::Result<(), WorkdayRepositoryError> {
        if let Some(_) = self.persistence.find_by_day(workday.date())? {
            return Err(WorkdayRepositoryError::EntityAlreadyExists);
        }

        self.persistence.insert(workday)?;

        Ok(())
    }

    pub fn find_all(&self) -> anyhow::Result<Vec<Workday>, WorkdayRepositoryError> {
        self.persistence.find_all()
    }
}
