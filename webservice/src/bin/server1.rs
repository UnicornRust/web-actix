use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;


// 路由配置
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}


// 处理器
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running!")
}

// 实例化 Http Server 并运行
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // 构建 app, 配置 route
    let app = move || App::new().configure(general_routes);
    // 运行 Http server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
