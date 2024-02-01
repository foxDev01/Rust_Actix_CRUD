use actix_web::{get, post ,web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
use serde_json::json;
use tera::Tera;
use rusqlite::{Connection, Result, params};
use actix_files as fs;
use actix_web::delete;
use rusqlite::types::FromSql;
use rusqlite::types::Value;
use rusqlite::Row;
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct RequestData {
    content: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Exercise {
    id: i32,
    exercise_name: String,
    sets_reps: String, // option если придут NULL
    date: String,
}


// Обработчик для GET-запроса на "/"
#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    // Рендеринг HTML-шаблона
    let rendered = tmpl
        .render("index.html", &tera::Context::new())
        .expect("Не удалось отрендерить шаблон");
    HttpResponse::Ok().body(rendered)
}

// Измененная обработчик для сохранения в SQLite
#[post("/save_sqlite")]
async fn save_to_sqlite_handler(data: web::Json<Exercise>) -> HttpResponse {
    // Сериализация данных в JSON-строку для отладки
    let content = serde_json::to_string(&data);

    // Если есть ошибка при сериализации, вернуть ошибку сервера
    if let Err(e) = content {
        return HttpResponse::InternalServerError().body(format!("Failed to serialize data: {}", e));
    }

    // Получение данных в формате JSON из Result
    let content = content.unwrap();
    println!("content: {}", content);
    save_to_sqlite(&content);
    // Отправка успешного ответа с данными
    HttpResponse::Ok().body(format!("Data saved to SQLite successfully! Content: {}", content))
}

#[get("/get_data")]
async fn get_data() -> impl Responder {

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
#[delete("/delete_exercise/{id}")]
async fn delete_exercise(id: web::Path<i32>) -> HttpResponse {
    // Ваш код для удаления упражнения с идентификатором id
    if let Err(e) = delete_exercise_from_database(id.into_inner()).await {
        // Обработка ошибки удаления, например, отправка 500 Internal Server Error
        return HttpResponse::InternalServerError().body(format!("Failed to delete exercise: {}", e));
    }
 
    // Успешный ответ
    HttpResponse::Ok().json(json!({"message": "Exercise deleted successfully"}))
}

// Функция для удаления упражнения из базы данных
async fn delete_exercise_from_database(id: i32) -> Result<(), Box<dyn std::error::Error>> {
    // Ваш код для удаления упражнения из базы данных
    let conn = Connection::open("exercise_data.db")?;

    conn.execute(
        "DELETE FROM exercises WHERE id = ?",
        params![id],
    )?;

    Ok(())
}



// Основная функция, инициализация сервера Actix
#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Инициализация базы данных SQLite
    setup_database().expect("Failed to set up database");
    // Создание экземпляра Tera и указание пути к шаблонам
    let tera = Tera::new("templates/**/*").expect("Не удалось создать экземпляр Tera");

    // Настройка HTTP-сервера
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(save_to_sqlite_handler)
            .service(index)
            .service(get_data)
            .service(delete_exercise)
                    // Обработка статических файлов
            .configure(|app| {
            app.service(fs::Files::new("/static", "static").show_files_listing());
        })
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
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
