use actix_web::{web, HttpResponse};

use crate::{
    dbaccess::teacher::{
        delete_teacher_db, get_all_teacher_db, get_teacher_detail_db, post_new_teacher_db, update_teacher_details_db
    }, 
    errors::AppError, 
    models::teacher::{CreateTeacher, UpdateTeacher}, 
    state::AppState
};


pub async fn get_all_teacher(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AppError> {
    get_all_teacher_db(&app_state.db)
        .await
        .map(|teachers|HttpResponse::Ok().json(teachers))
}


pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let teacher_id = params.into_inner();
    get_teacher_detail_db(&app_state.db, teacher_id)
        .await
        .map(|teacher|HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(
    app_state: web::Data<AppState>,
    new_teacher: web::Json<CreateTeacher>,
) -> Result<HttpResponse, AppError> {
    post_new_teacher_db(&app_state.db, new_teacher.into())
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    update_teacher: web::Json<UpdateTeacher>
) -> Result<HttpResponse, AppError> {
    let teacher_id = params.into_inner();
    update_teacher_details_db(&app_state.db, teacher_id, UpdateTeacher::from(update_teacher))
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, AppError> {
    let  teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests {
    use std::{env, sync::Mutex};

    use actix_web::{guard::Connect, http::StatusCode, web};
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;

    use crate::{models::teacher::CreateTeacher, state::AppState};

    use super::{delete_teacher, get_all_teacher, get_teacher_details, post_new_teacher};

    #[actix_rt::test]
    async fn get_all_teacher_success_test() {
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

        let resp = get_all_teacher(app_state)
            .await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_teacher_details_success() {
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

        let params = web::Path::from(1);
        let resp = get_teacher_details(app_state, params)
            .await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_new_teacher_success() {
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

        let new_teacher = CreateTeacher {
            name: "Third teacher".into(),
            picture_url: "http://unicorn.pro".into(),
            profile: "A teacher in Machine learning".into(),
        };

        let resp = post_new_teacher(app_state, web::Json(new_teacher))
            .await
            .unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
    }


    #[actix_rt::test()]
    async fn delete_teacher_success() {
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

        let params = web::Path::from(1);

        let resp = delete_teacher(app_state, params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }


}
