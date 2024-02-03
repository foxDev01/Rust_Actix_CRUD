// models.rs
use serde::{Serialize, Deserialize};

// Структуры данных
#[derive(Serialize, Deserialize, Debug)]
pub struct Exercise {
    pub id: i32,
    pub exercise_name: String,
    pub sets_reps: String,
    pub date: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatedExercise {
    pub exercise_name: String,
    pub sets_reps: String,
    pub date: String,
}