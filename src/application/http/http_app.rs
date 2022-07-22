use crate::application::App;
use async_trait::async_trait;
use axum::{routing::get, Router};

pub struct HttpApp;

#[async_trait]
impl App for HttpApp {
    async fn start(&self) {
        // build our application with a single route
        let app = Router::new().route("/", get(|| async { "Hello, World!" }));

        // run it with hyper on localhost:3000
        axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

impl HttpApp {
    pub fn new() -> Self {
        Self {}
    }
}
/*
use business::domain::{CreateDto, CreateUseCase, ListAllUseCase, WorkdayRepository};
use chrono::prelude::*;
use infrastructure::persistence::InMemoryWorkdayPersistence;
use std::sync::Arc;
use std::sync::Mutex;
pub mod business;
pub mod infrastructure;

fn main() {
    let repository = Mutex::new(WorkdayRepository {
        persistence: Box::from(InMemoryWorkdayPersistence::new()),
    });

    let arc = Arc::new(repository);
    let mut create_use_case = CreateUseCase::new(arc.clone());
    let mut list_all_use_case = ListAllUseCase::new(arc.clone());

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

*/
