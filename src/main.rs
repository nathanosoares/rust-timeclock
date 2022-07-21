use business::domain::{CreateDto, CreateUseCase, WorkdayRepository};
use chrono::Utc;
use infrastructure::persistence::InMemoryWorkdayPersistence;

pub mod business;
pub mod infrastructure;

fn main() {
    let repository = &mut WorkdayRepository {
        persistence: Box::from(InMemoryWorkdayPersistence::new()),
    };

    let use_case = &mut CreateUseCase::new(repository);

    let result = use_case.execute(CreateDto { date: Utc::today() });

    match result {
        Ok(workday) => println!("Workday created {:?}", workday),
        Err(error) => println!("Error: {}", error),
    }
}
