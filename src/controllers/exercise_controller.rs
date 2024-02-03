// controllers.rs
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::models::models::{Exercise, UpdatedExercise};
use tera::Tera;
use rusqlite::{Connection, Result, params};

use crate::views::exercise_view::ViewManager;


//#[put("/update_exercise/{id}")]
pub async fn update_exercise( data: web::Json<UpdatedExercise>, path: web::Path<i32>,) -> HttpResponse {
    let exercise_id = path.into_inner();
    let updated_data = data.into_inner();

    // Вызов функции для обновления данных в базе данных
    if let Err(e) = update_exercise_in_database(exercise_id, &updated_data) {
        // Обработка ошибки, например, отправка 500 Internal Server Error
        return HttpResponse::InternalServerError().body(format!("ОШибка при внесении изменений: {}", e));
    }

    // Успешный ответ
    HttpResponse::Ok().json(json!({"message": "Изменения прошли успешно"}))
}

// Обработчик для GET-запроса на "/"
//#[get("/")]
// pub async fn index(tmpl: web::Data<Tera>) -> impl Responder {
//     // Рендеринг HTML-шаблона
//     let rendered = tmpl
//         .render("index.html", &tera::Context::new())
//         .expect("Не удалось отрендерить шаблон");
//     HttpResponse::Ok().body(rendered)
// }
pub async fn index(tmpl: web::Data<Tera>) -> impl Responder {

    println!("Index");
    // Рендеринг HTML-шаблона
    let rendered = tmpl
        .render("index.html", &tera::Context::new())
        .expect("Не удалось отрендерить шаблон");
    HttpResponse::Ok().body(rendered)
    
}
// Измененная обработчик для сохранения в SQLite
//#[post("/save_sqlite")]
pub async fn save_to_sqlite_handler(data: web::Json<Exercise>) -> HttpResponse {
    // Сериализация данных в JSON-строку для отладки
    let content = serde_json::to_string(&data);

    // Если есть ошибка при сериализации, вернуть ошибку сервера
    if let Err(e) = content {
        return HttpResponse::InternalServerError().body(format!("Failed to serialize data: {}", e));
    }

    // Получение данных в формате JSON из Result
    let content = content.unwrap();
    println!("content: {}", content);
    //save_to_sqlite(&content);

    // Отправка успешного ответа с данными
    // HttpResponse::Ok().body(format!("Data saved to SQLite successfully! Content: {}", content))
    if let Err(e) = save_to_sqlite(&content) {
        // Обработка ошибки удаления, например, отправка 500 Internal Server Error
        return HttpResponse::InternalServerError().body(format!("Failed to delete exercise: {}", e));
    }
 
// Отправка успешного ответа с данными
    HttpResponse::Ok().body(format!("Данные успешно сохранены в БД: {}", content))



}

//#[get("/get_data")]
pub async fn get_data() -> impl Responder {

    println!("get_data");
    let conn = Connection::open("exercise_data.db").expect("Failed to open database");

    let mut stmt = conn
        .prepare("SELECT id, exercise_name, sets_reps, date FROM exercises")
        .expect("Failed to prepare SQL statement");

    let exercises: Vec<Exercise> = stmt
        .query_map([], |row| {
            Ok(Exercise {
                id: row.get(0)?,
                exercise_name: row.get(1)?,
                sets_reps: row.get(2)?,
                date: row.get(3)?,
            })
        })
        .expect("Failed to execute SQL query")
        .map(|res| res.unwrap())
        .collect();
        
    HttpResponse::Ok().json(exercises)
}

// В вашем коде добавьте новый маршрут для обработки DELETE-запроса
//#[delete("/delete_exercise/{id}")]
pub async fn delete_exercise(id: web::Path<i32>) -> HttpResponse {
    // Ваш код для удаления упражнения с идентификатором id
    if let Err(e) = delete_exercise_from_database(id.into_inner()).await {
        // Обработка ошибки удаления, например, отправка 500 Internal Server Error
        return HttpResponse::InternalServerError().body(format!("Failed to delete exercise: {}", e));
    }
 
    // Успешный ответ
    HttpResponse::Ok().json(json!({"message": "Удаление прошло успешно"}))
}

// Функция для удаления упражнения из базы данных
pub async fn delete_exercise_from_database(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    // Ваш код для удаления упражнения из базы данных
    let conn = Connection::open("exercise_data.db")?;

    conn.execute(
        "DELETE FROM exercises WHERE id = ?",
        params![id],
    )?;

    Ok(())
}




// Функция для создания таблицы, вызывается один раз при старте приложения
fn setup_database() -> Result<()> {
    let conn = Connection::open("exercise_data.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS exercises (
            id INTEGER PRIMARY KEY,
            exercise_name TEXT NOT NULL,
            sets_reps JSON NOT NULL,
            date TEXT NOT NULL
        )",
        (),
    )?;
    Ok(())
}

// Функция для добавления данных в SQLite базу данных
fn save_to_sqlite(data: &str) -> Result<(), Box<dyn std::error::Error>> {
   // Десериализация JSON-строки в структуру Exercise
    let exercise_data: Exercise = serde_json::from_str(data)?;
    let sets_reps_json = serde_json::to_string(&exercise_data.sets_reps)?;

    let conn = rusqlite::Connection::open("exercise_data.db")?;
    
    conn.execute(
        "INSERT INTO exercises (exercise_name, sets_reps, date) VALUES (?, ?, ?)",
        &[&exercise_data.exercise_name, &sets_reps_json, &exercise_data.date],
    )?;
    println!("save_to_sqlite: {}", data);

     Ok(())



}


// Функция для обновления упражнения в базе данных
fn update_exercise_in_database(
    exercise_id: i32,
    updated_data: &UpdatedExercise,
) -> Result<(), Box<dyn std::error::Error>> {
    // Ваш код для обновления упражнения в базе данных
    // Например, используйте Rusqlite для выполнения SQL-запроса
    // Ниже приведен пример кода для иллюстрации
    // Замените его на свою реализацию

    let conn = rusqlite::Connection::open("exercise_data.db")?;
    let sets_reps_json = serde_json::to_string(&updated_data.sets_reps)?;

    conn.execute(
        "UPDATE exercises SET exercise_name = ?, sets_reps = ?, date = ? WHERE id = ?",
        &[
            &updated_data.exercise_name,
            &sets_reps_json,
            &updated_data.date,
            &exercise_id.to_string(),
        ],
    )?;

    Ok(())
}

// сделать view шаблоны не отображаются не грузит главную . данные приходят 