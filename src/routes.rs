// routes.rs
use actix_web::web;
use crate::controllers::exercise_controller;


// Функция для конфигурации маршрутов
pub fn configure(cfg: &mut web::ServiceConfig) {
    
    cfg
        .service(web::resource("/").route(web::get().to(exercise_controller::index)))
        .service(web::resource("/save_sqlite").route(web::post().to(exercise_controller::save_to_sqlite_handler)))
        .service(web::resource("/get_data").route(web::get().to(exercise_controller::get_data)))
        .service(web::resource("/delete_exercise/{id}").route(web::delete().to(exercise_controller::delete_exercise)))
        .service(web::resource("/update_exercise/{id}").route(web::put().to(exercise_controller::update_exercise)));
}