use crate::{
    dbaccess::course::{
        delete_course_db, get_course_detail_db, get_course_for_teacher_db, post_new_course_db, update_course_db
    }, 
    errors::AppError, 
    models::course::{CreateCourse, UpdateCourse}
};

use crate::state::AppState;
use actix_web::{web, HttpResponse };

// 新增课程的 Post 
pub async fn post_new_course(
    new_course: web::Json<CreateCourse>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    println!("Received new course");
    post_new_course_db(
        &app_state.db, 
        new_course.try_into()?,
    ).await
    .map(|course| HttpResponse::Ok().json(course))
}

// 获取某位老师的所有课程 Get 请求
pub async fn get_courses_for_teacher (
    app_state: web::Data<AppState>,
    // 获取路径中的参数, 元组类型, 可以根据顺序获取多个
    // params: web::Path<(i32,)>,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> { 
    let teacher_id = params.into_inner();
    // let teacher_id: i32 = i32::try_from(params.0).unwrap();
    get_course_for_teacher_db(
        &app_state.db, 
        teacher_id,
    ).await
    .map(|courses| HttpResponse::Ok().json(courses))
}    

// 获取具体某个老师的某个课程
pub async fn get_course_detail(
    app_state: web::Data<AppState> ,
    // params: web::Path<(usize, usize)>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppError> {
    // let teacher_id = i32::try_from(params.0).unwrap();
    // let course_id = i32::try_from(params.1).unwrap();
    let (teacher_id, course_id) = params.into_inner();
    get_course_detail_db(
        &app_state.db, 
        teacher_id, 
        course_id
    ).await
    .map(|course|  HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppError> {
    let (teacher_id, course_id) = params.into_inner();
    delete_course_db(&app_state.db, teacher_id, course_id)
    .await
    .map(|resp| HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    upate_course: web::Json<UpdateCourse>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, AppError> {
    let (teacher_id, course_id) = params.into_inner();
    update_course_db(
        &app_state.db, 
        teacher_id, 
        course_id, 
        upate_course.into()
    )
        .await
        .map(|course| HttpResponse::Ok().json(course))
}


#[cfg(test)]
mod test {

    use super::*;
    use actix_web::{http::StatusCode, ResponseError};
    use dotenv::dotenv;
    use std::env;
    use sqlx::postgres::PgPoolOptions;
    use std::sync::Mutex;

    // 由于是异步函数，测试时候需要加上 #[actix_rt::test]
    #[ignore]
    #[actix_rt::test]
    async fn test_course_create() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let course = web::Json(CreateCourse {
            teacher_id: 1,
            name: "Test Course".into(),
            description: Some("This is a course".into()),
            format: None,
            structure: None,
            duration: None,
            price: None,
            language: Some("English".into()),
            level: Some("Beginner".into()),
        });
        
        let app_state: web::Data::<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let response = post_new_course(
            course, app_state
        )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_all_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let teacher_id: web::Path<i32> = web::Path::from(1);
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
        let teacher_id: web::Path<(i32,i32)> = web::Path::from((1,1));
        let response = get_course_detail(
            app_state, teacher_id
        ).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn get_one_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new().connect(&db_url)
            .await
            .unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1,100));
        let resp = get_course_detail(app_state, params).await;
        match resp {
            Ok(_) => println!("Something wrong..."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }

    #[actix_rt::test]
    async fn update_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL not set in .env file");
        let db_pool = PgPoolOptions::new().connect(&db_url)
            .await.unwrap();

        let app_state = web::Data::new(AppState{ 
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let update_course = UpdateCourse {
            name: Some("Course name changed".into()),
            description: Some("This is another test course".into()),
            format: None,
            level: Some("Intermediate".into()),
            price: None,
            duration: None,
            language: Some("Chinese".into()),
            structure: None,
        };

        let params: web::Path<(i32, i32)> = web::Path::from((1,2));
        let update_params = web::Json(update_course);
        let resp = update_course_details(
            app_state,
            update_params,
            params,
        )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }


    #[ignore]
    #[actix_rt::test]
    async fn delete_course_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new().connect(&db_url)
            .await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let resp = delete_course(app_state, params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }


    #[actix_rt::test]
    async fn test_course_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL is not set in .env file");
        let db_pool = PgPoolOptions::new().connect(&db_url)
            .await
            .unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1,1001));
        let resp = delete_course(app_state, params).await;
        match resp {
            Ok(_) => println!("Something wrong...."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
}
