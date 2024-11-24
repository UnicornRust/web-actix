use crate::models::Course;

use actix_web::App;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;
use super::errors::AppError;

pub async fn get_course_for_teacher_db(
    pool: &PgPool, 
    teacher_id: i32
) -> Result<Vec<Course>, AppError> {
    let rows = sqlx::query!(
        r#"Select id, teacher_id, name, time from course where teacher_id = $1"#,
        teacher_id
    ).fetch_all(pool)
        .await?;

    let courses = rows.iter().map(|x| Course {
        id: Some(x.id),
        teacher_id: x.teacher_id,
        name: x.name.clone(),
        time: Some(NaiveDateTime::from(x.time.unwrap())),
    }).collect(); 
    match courses.len() {
        0 => Err(AppError::NotFound("Course not found for teacher".into())),
        _ => Ok(courses),
    }
}
 

pub async fn get_course_detail_db(
    pool: &PgPool, 
    teacher_id: i32, 
    course_id: i32
) -> Result<Course, AppError> {
    let row = sqlx::query!(
        r#"Select id, teacher_id, name. time from course where teacher_id = $1 and course_id = $2"#,
        teacher_id,
        course_id,
    )
        .fetch_one(pool)
        .await;

    if let Ok(row) = row {
        Ok(Course {
            id: Some(row.id),
            teacher_id: row.teacher_id,
            name: row.name.clone(),
            time: Some(NaiveDateTime::from(row.time.unwrap())),
        })
    }else {
        Err(AppError::NotFound("Course Id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool, 
    new_course: Course,
) -> Result<Course, AppError> {
    let row = sqlx::query!(
        r#"Insert into course(id, teacher_id, name, time) values ($1, $2, $3)
           Returning id, teacher_id, name, time
        "#,
        new_course.id, 
        new_course.teacher_id, 
        new_course.name,
    )
        .fetch_one(pool)
        .await?;

    Ok(Course {
        time: Some(NaiveDateTime::from(row.time.unwrap())),
        id: Some(row.id),
        ..new_course
    })
}
