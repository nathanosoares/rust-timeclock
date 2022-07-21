use crate::business::domain::{Workday, WorkdayPersistence, WorkdayRepositoryError};

pub struct InMemoryWorkdayPersistence {
    workdays: Vec<Workday>,
}

impl InMemoryWorkdayPersistence {
    pub fn new() -> Self {
        InMemoryWorkdayPersistence {
            workdays: Vec::new(),
        }
    }
}

impl WorkdayPersistence for InMemoryWorkdayPersistence {
    fn insert(&mut self, workday: Workday) -> anyhow::Result<(), WorkdayRepositoryError> {
        self.workdays.push(workday);
        Ok(())
    }

    fn find_by_day(
        &self,
        date: chrono::Date<chrono::Utc>,
    ) -> anyhow::Result<Option<&Workday>, WorkdayRepositoryError> {
        let found = self.workdays.iter().find(|item| item.date().eq(&date));

        Ok(found)
    }

    fn find_all(&self) -> anyhow::Result<Vec<Workday>, WorkdayRepositoryError> {
        Ok(self.workdays.to_owned())
    }
}
