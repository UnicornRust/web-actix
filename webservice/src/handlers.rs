use crate::{
    db_access::{
        get_course_detail_db, 
        get_course_for_teacher_db, 
        post_new_course_db
    }, errors::AppError, models::Course
};

use super::AppState;
use actix_web::{web, HttpResponse };


// 检查应用健康度
pub async fn health_check_handler(
    app_state: web::Data<AppState>
) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{}{} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

// 新增课程的 Post 
pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    println!("Received new course");
    let course = post_new_course_db(
        &app_state.db, 
        new_course.into()
    ).await
    .map(|course| HttpResponse::Ok().json(course));
    
}

// 获取某位老师的所有课程 Get 请求
pub async fn get_courses_for_teacher (
    app_state: web::Data<AppState>,
    // 获取路径中的参数, 元组类型, 可以根据顺序获取多个
    params: web::Path<(usize,)>,
) -> Result<HttpResponse, AppError> { 
    let teacher_id: i32 = i32::try_from(params.0).unwrap();
     get_course_for_teacher_db(
        &app_state.db, 
        teacher_id,
    ).await
    .map(|courses| HttpResponse::Ok().json(courses));

    
}    

// 获取具体某个老师的某个课程
pub async fn get_course_detail(
    app_state: web::Data<AppState> ,
    params: web::Path<(usize, usize)>,
) -> Result<HttpResponse, AppError> {
    let teacher_id = i32::try_from(params.0).unwrap();
    let course_id = i32::try_from(params.1).unwrap();
    get_course_detail_db(
        &app_state.db, 
        teacher_id, 
        course_id
    ).await
    .map(|course|  HttpResponse::Ok().json(course))
}


#[cfg(test)]
mod test {

    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use std::env;
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Mutex;

    // 由于是异步函数，测试时候需要加上 #[actix_rt::test]
    #[actix_rt::test]
    async fn test_course_create() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let course = web::Json(Course {
            teacher_id: 1,
            id: Some(3),
            name: "Test Course".into(),
            time: None,
        });
        
        let app_state: web::Data::<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let response = new_course(
            course, app_state
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_all_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id: web::Path<(usize,)> = web::Path::from((1,));
        let response = get_courses_for_teacher(
            app_state,
            teacher_id,
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn test_get_course_detail() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_RUL is not set in .env file");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id: web::Path<(usize,usize)> = web::Path::from((1,1));
        let response = get_course_detail(
            app_state, teacher_id
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK)
    }
}
