mod utils;
mod models;
mod errors;

use models::course:: {delete_course, get_course_by_teacher };
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlButtonElement};


// when the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
extern "C" {

    // alert 用于弹出一个对话框，方便调试
    fn alert(s: &str);

    // confirm 用于弹出一个确认对话框，方便调试
    fn confirm(s: &str) -> bool;

    // console.log 用于在 wasm 打印一些日志，方便调试
    #[wasm_bindgen(js_namespace=console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(format!("Hello, {name}!").as_str());
}


// main 函数 就相当于浏览器执行的入口
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    let window = window().expect("no global window exists");
    let document = window.document().expect("no global document exists");

    // 获取到要操作的元素
    let left_tbody = document.get_element_by_id("left-tbody")
        .expect("div id left-body not found");

    // 加载后台数据
    let courses = get_course_by_teacher(1).await.unwrap();

    for c in courses.iter() {

        let tr = document.create_element("tr")?;
        tr.set_attribute("id", format!("tr-{}", c.id).as_str())?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(format!("{}", c.id).as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        td.set_text_content(Some(c.time.format("%Y-%m-%d").to_string().as_str()));
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        if let Some(desc) = c.description.clone() {
            td.set_text_content(Some(desc.as_str()));
        }
        tr.append_child(&td)?;

        let td = document.create_element("td")?;
        // 获取按钮绑定事件
        let btn: HtmlButtonElement = document.create_element("button")
                .unwrap()
                .dyn_into::<HtmlButtonElement>()
                .unwrap();

        let cid = c.id;

        // 目前异步闭包函数不稳定, 只能通过其他方式来支持闭包异步函数
        //
        let click_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
            let r = confirm(format!("确认删除课程{} 吗?", cid).as_str());
            match r {
                true => {
                    // 异步函数, 当异步闭包函数不稳定时使用 spawn_local 以执行异步操作
                    spawn_local(delete_course(1, cid));
                    // delete_course(1, cid);
                    alert("删除成功!");
                    web_sys::window().unwrap().location().reload().unwrap();
                }
                _ => {}
            }
        }) as Box<dyn Fn(_)>); 

        btn.add_event_listener_with_callback(
            "click", 
            click_closure.as_ref().unchecked_ref()
        )?;

        // 闭包走出作用域会自动释放, 导致回调函数无法执行
        // forget 之后不会丢弃，会造成内存泄露
        click_closure.forget();
                
        btn.set_attribute("class", "btn btn-danger btn-sm")?;
        btn.set_text_content(Some("Delete"));
        td.append_child(&btn)?;
        tr.append_child(&td)?;

        left_tbody.append_child(&tr)?;
    }
    Ok(())
}
