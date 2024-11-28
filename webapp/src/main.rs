use std::{env, io::Result};
use dotenv::dotenv;
use actix_web::{web, App, HttpServer};
use tera::Tera;
use webapp::routes::app_config;


#[actix_web::main]
async fn main()  -> Result<()>{

    dotenv().ok();
    let host_port = env::var("HOST_PORT")
        .expect("HOST_PORT address is not set in .env file");

    println!("Listening on : {}", &host_port);


    HttpServer::new(move || {
        let tera = Tera::new(
            concat!(env!("CARGO_MANIFEST_DIR"), "/static/**")
        ).unwrap();
        App::new()
            .app_data(web::Data::new(tera))
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
