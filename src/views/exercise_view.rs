use tera::Tera;

// Структура для управления представлением
pub struct ViewManager {
    tera: Tera,
}

impl ViewManager {
    // Конструктор для создания нового экземпляра ViewManager
    pub fn new() -> Self {
        // Инициализация Tera и другие необходимые действия
        let tera = Tera::new("templates/**/*").expect("Не удалось создать экземпляр Tera");

        Self { tera }
    }

    // Метод для рендеринга HTML-шаблона
    pub fn render_template(&self, template_name: &str) -> Result<String, tera::Error> {
        // Логика рендеринга шаблона
        let context = tera::Context::new(); // Можете дополнить контекст данных, если это нужно
        self.tera.render(template_name, &context)
    }
}
