use business::domain::workday::repository::WorkdayRepositoryError;
use business::domain::{CreateDto, CreateUseCase, Workday, WorkdayPersistence, WorkdayRepository};
use chrono::naive::NaiveDate;
use chrono::{Date, Utc};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;

pub mod business;

struct FileWorkdayPersistence;

impl WorkdayPersistence for FileWorkdayPersistence {
    fn insert(&self, workday: &Workday) -> anyhow::Result<(), WorkdayRepositoryError> {
        let open_result = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("db.txt");

        match open_result {
            Ok(file) => {
                let write_result = writeln!(&file, "{}", workday.date().format("%Y-%m-%d"));

                if let Err(error) = write_result {
                    return Err(WorkdayRepositoryError::Persistence(error));
                }

                return Ok(());
            }
            Err(error) => Err(WorkdayRepositoryError::Persistence(error)),
        }
    }

    fn find_by_day(
        &self,
        date: chrono::Date<Utc>,
    ) -> anyhow::Result<Option<Workday>, WorkdayRepositoryError> {
        let file_result = File::open("ddb.txt");

        if let Err(error) = file_result {
            return Err(WorkdayRepositoryError::Persistence(error));
        }

        let reader = BufReader::new(file_result.unwrap());

        for line_result in reader.lines() {
            if let Err(error) = line_result {
                return Err(WorkdayRepositoryError::Persistence(error));
            }

            let date_line = NaiveDate::parse_from_str(&line_result.unwrap(), "%Y-%m-%d").unwrap();
            let date_line = Date::from_utc(date_line, Utc);

            if date_line.eq(&date) {
                return Ok(Some(Workday::new(date_line)));
            }
        }

        return Ok(None);
    }
}

fn main() {
    let persistence = FileWorkdayPersistence;
    let repository = WorkdayRepository {
        persistence: Box::from(persistence),
    };

    let use_case = CreateUseCase::new(repository);

    let result = use_case.execute(CreateDto { date: Utc::today() });

    match result {
        Ok(workday) => println!("Workday created {:?}", workday),
        Err(error) => println!("Error: {}", error),
    }
}
