use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use webservice::errors::AppError;
use webservice::routers::{course_routes, general_routes, teacher_routes};
use webservice::state::AppState;
use std::{env, io};
use std::sync::Mutex;
use dotenv::dotenv;
use http::header::{AUTHORIZATION, ACCEPT, CONTENT_TYPE};


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
        // 跨域配置
        let cors = Cors::default()
            // 允许某个域名
            // .allowed_origin("http://localhost:8080/")
            // 允许某些域名前缀(这里引入了一个闭包，只要闭包返回 true 的逻辑都被允许)
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "DELETE", "PUT", "POST"])
            .allowed_headers(vec![AUTHORIZATION, ACCEPT])
            .allowed_header(CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req|{
                AppError::InvalidaValue("Please provide valid Json Input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .wrap(cors)
            .configure(teacher_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
