use chrono::NaiveDateTime;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::errors::AppError;


#[derive(Debug, Deserialize, Serialize)]
pub struct Course {
    pub teacher_id: i32,
    pub id: i32,
    pub name: String,
    pub time: NaiveDateTime,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<String>,
    pub language: Option<String>,
    pub level: Option<String>,
}


pub async fn get_course_by_teacher(
    teacher_id: i32
) -> Result<Vec<Course>, AppError> {

    // 用于在 wasm 中完成 HTTP 请求
    let mut opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);

    let window = web_sys::window().ok_or("no window exists".to_string())?;

    // 构建请求对象
    let url = format!("http://localhost:3000/courses/{}", teacher_id);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json;")?;

    // 发送请求
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // 获取到请求的结果
    let json = JsFuture::from(resp.json()?).await?;
    let course: Vec<Course> = json.into_serde().unwrap();

    Ok(course)
}



pub async fn delete_course(teacher_id: i32, course_id: i32) -> () {

    let mut opts = RequestInit::new();
    opts.set_method("DELETE");
    opts.set_mode(RequestMode::Cors);

    let url = format!("http://localhost:3000/courses/{}/{}", teacher_id, course_id);

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Accept", "application/json;").unwrap();

    let window = web_sys::window().unwrap();

    let resp = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp.is_instance_of::<Response>());
    let resp: Response = resp.dyn_into().unwrap();

    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

    let _course: Course = json.into_serde().unwrap();

}


#[wasm_bindgen] 
pub async fn add_course(name: String, description: String) -> Result<Promise, JsValue> {
    let mut opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(RequestMode::Cors);
    let str_json = format!(
        r#"
         {{
            "teacher_id": 1,
            "name": "{}",
            "description": "{}"
         }}
        "#,
        name, 
        description,
    );

    opts.set_body(&JsValue::from_str(str_json.as_str()));
    let url = "http://localhost:3000/courses/";

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json;")?;
    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().ok_or("no window exists".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    Ok(resp.json()?)
}
