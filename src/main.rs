// main.rs
mod controllers;
mod models;
mod routes;
mod views;
use tera::Tera;
use actix_web::{web, App, HttpServer};
use env_logger;
use actix_files as fs; 


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let tera = Tera::new("templates/**/*").expect("Не удалось создать экземпляр Tera");
    HttpServer::new(move|| {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .configure(routes::configure )
            .configure(|app| {
                app.service(fs::Files::new("/static", "static").show_files_listing());
            })                
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
