use business::domain::{CreateDto, CreateUseCase, ListAllUseCase, WorkdayRepository};
use chrono::prelude::*;
use infrastructure::persistence::InMemoryWorkdayPersistence;
use std::sync::Mutex;

pub mod business;
pub mod infrastructure;

fn main() {
    let repository = Mutex::new(WorkdayRepository {
        persistence: Box::from(InMemoryWorkdayPersistence::new()),
    });

    let mut create_use_case = CreateUseCase::new(&repository);
    let mut list_all_use_case = ListAllUseCase::new(&repository);

    create_use_case
        .execute(CreateDto {
            date: Utc.ymd(2014, 7, 1),
        })
        .unwrap();

    println!("{:?}", list_all_use_case.execute());

    create_use_case
        .execute(CreateDto {
            date: Utc.ymd(2014, 7, 2),
        })
        .unwrap();

    println!("{:?}", list_all_use_case.execute());
}
