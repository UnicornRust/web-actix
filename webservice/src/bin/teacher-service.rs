use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use std::{env, io};
use std::sync::Mutex;
use dotenv::dotenv;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;
#[path = "../db_access.rs"]
mod db_access;
#[path = "../errors.rs"]
mod errors;


use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set");

    let db_pool = PgPoolOptions::new().connect(&database_url)
        .await
        .unwrap();

    // 挂载一个共享数据
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm OK.".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool, 
        // courses: Mutex::new(vec![]),
    });
    let app = move || { 
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
