use sqlx::PgPool;

use crate::{errors::AppError, models::teacher::{CreateTeacher, Teacher, UpdateTeacher}};


pub async fn get_all_teacher_db(
    pool: &PgPool
) -> Result<Vec<Teacher>, AppError> {
    let rows = sqlx::query!(
        r#"select id, name, picture_url, profile from teacher "#
    )
        .fetch_all(pool)
        .await?;
    let teachers: Vec<Teacher> = rows
        .iter()
        .map(|r| Teacher{
            id: r.id,     
            name: r.name.clone().unwrap(),
            picture_url: r.picture_url.clone().unwrap(),
            profile: r.profile.clone().unwrap(),
        }).collect();

    match teachers.len() {
        0 => Err(AppError::NotFound("No teachers found".into())),
        _ => Ok(teachers),
    }
}

pub async fn get_teacher_detail_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<Teacher, AppError> {
    let row = sqlx::query!(
        r#"select id, name, picture_url, profile from teacher where id = $1"#,
        teacher_id,
    )
        .fetch_one(pool)
        .await
        .map(|r| Teacher {
            id: r.id, 
            name: r.name.clone().unwrap(),
            picture_url: r.picture_url.clone().unwrap(),
            profile: r.profile.clone().unwrap(),
        })
        .map_err(|_err| AppError::NotFound("Teacher Id not found".into()))?;
    Ok(row)
}

pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, AppError> {
    
    let row = sqlx::query!(
        r#"
        insert into teacher (name, picture_url, profile) values ($1, $2, $3)
        returning id, name, picture_url, profile
        "#,
        new_teacher.name, 
        new_teacher.picture_url, 
        new_teacher.profile,
    )
        .fetch_one(pool)
        .await?;
    Ok(Teacher {
        id: row.id, 
        name: row.name.clone().unwrap(),
        picture_url: row.picture_url.clone().unwrap(),
        profile: row.profile.clone().unwrap(),
    })
}


pub async fn update_teacher_details_db(
    pool: &PgPool, 
    teacher_id: i32,
    new_teacher: UpdateTeacher,
) -> Result<Teacher, AppError> {
    let row = sqlx::query!(
        r#"select * from teacher where id = $1"#,
        teacher_id,
    )
        .fetch_one(pool)
        .await
        .map_err(|_e| AppError::NotFound("Teacher Id not found".into()))?;

    let temp = Teacher {
        id: row.id, 
        name: if let Some(name) = new_teacher.name {
            name
        } else {
            row.name.clone().unwrap()
        },
        picture_url: if let Some(picture_url) = new_teacher.picture_url {
            picture_url
        } else {
            row.picture_url.clone().unwrap()
        },
        profile: if let Some(profile) = new_teacher.profile {
            profile
        } else {
            row.profile.clone().unwrap()
        }
    };

    let update_row = sqlx::query!(
        r#"
        update teacher set 
            name = $1,
            picture_url = $2,
            profile = $3
        where id = $4
        returning id, name, picture_url, profile
        "#,
        temp.name, 
        temp.picture_url,
        temp.profile,
        temp.id
    )
        .fetch_one(pool)
        .await
    .map(|row| Teacher {
        id: row.id,
        name: row.name.clone().unwrap(),
        picture_url: row.picture_url.clone().unwrap(),
        profile: row.profile.clone().unwrap(),
    })
    .map_err(|_e| AppError::NotFound("Teacher Id not found".into()))?;
            
    Ok(update_row)
}


pub async fn delete_teacher_db(
    pool: &PgPool,
    teacher_id: i32,
) -> Result<String, AppError> {
    let row = sqlx::query!(
        r#"delete from teacher where id = $1"#, 
        teacher_id
    )
        .execute(pool)
        .await
    .map_err(|_e| AppError::DBError("Unable to delete teacher".into()))?;
    
    Ok(format!("Deleted teacher {:?} records", row))
}
