
use chrono::{NaiveDateTime};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, io};

#[derive(Debug)]
pub struct Course {
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    // dotenv 返回了一个 Result 对象, 如果不使用 ok 处理，意味着得到结果没有处理, rust 编译器会警告
    // 使用 ok 处理之后得到的是一个 Option 对象, 即使获取不到也不会出错，
    // 代码部署到生产环境之后也不会再使用这种 .env 文件的方式设置环境变量,
    // 而是设置在目标机器的环境中
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not set in .env file");

    println!("Database url = {}", database_url);

    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
    
    let course_rows = sqlx::query!(
        r#"select id, teacher_id, name, time from course where id = $1"#,
        1
    )
        .fetch_all(&db_pool)
        .await
        .unwrap();
    let mut course_list = vec![];
    for row in course_rows {
        course_list.push(Course {
            id: row.id,
            teacher_id: row.teacher_id,
            name: row.name,
            time: Some(NaiveDateTime::from(row.time.unwrap())),
        })
    }
    println!("Course = {:?}", course_list);
    Ok(())
}
