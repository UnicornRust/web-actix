
use crate::models::Course;

use super::AppState;
use actix_web::{web, HttpResponse };
use chrono::Utc;


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
) -> HttpResponse {
    println!("Received new course");
    let course_count = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == new_course.teacher_id)
        .collect::<Vec<Course>>()
        .len();
    let new_course = Course {
        teacher_id: new_course.teacher_id,
        id: Some(course_count + 1),
        name: new_course.name.clone(),
        time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Course added")
}

// 获取某位老师的所有课程 Get 请求
pub async fn get_courses_for_teacher (
    app_state: web::Data<AppState>,
    // 获取路径中的参数, 元组类型, 可以根据顺序获取多个
    params: web::Path<(usize,)>,
) -> HttpResponse {
    let teacher_id: usize = params.0;
    let filtered_courses = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.teacher_id == teacher_id)
        .collect::<Vec<Course>>();

    if filtered_courses.len() > 0 {
        HttpResponse::Ok().json(filtered_courses)
    }else {
        HttpResponse::Ok().json("No Courses found for teacher".to_string())
    }

}    

// 获取具体某个老师的某个课程
pub async fn get_course_detail(
    app_state: web::Data<AppState> ,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let (teacher_id, course_id) = params.into_inner();
    let selected_course = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|x| x.teacher_id == teacher_id && x.id == Some(course_id))
        .ok_or("Course Not Found");

    if let Ok(course) = selected_course {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("Course Not Found".to_string())
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    // 由于是异步函数，测试时候需要加上 #[actix_rt::test]
    #[actix_rt::test]
    async fn test_course_create() {
        let course = web::Json(Course {
            teacher_id: 1,
            id: None,
            name: "Test Course".into(),
            time: None,
        });
        
        let app_state: web::Data::<AppState> = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });

        let response = new_course(course, app_state).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_all_course_success() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let teacher_id: web::Path<(usize,)> = web::Path::from((1,));
        let response = get_courses_for_teacher(
            app_state,
            teacher_id,
        ).await;
        assert_eq!(response.status(), StatusCode::OK)
    }

    #[actix_rt::test]
    async fn test_get_course_detail() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let teacher_id: web::Path<(usize,usize)> = web::Path::from((1,1));
        let response = get_course_detail(app_state, teacher_id).await;
        assert_eq!(response.status(), StatusCode::OK)
    }
}