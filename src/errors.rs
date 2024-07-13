use std::fmt;

// Определение перечисления для типов ошибок
#[derive(Debug)]
pub enum GlintError {
    UnknownToken(String),    // Ошибка: неизвестный токен
    IoError(std::io::Error), // Ошибка ввода-вывода
}

// Реализация вывода для типа ошибок
impl fmt::Display for GlintError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlintError::UnknownToken(token) => write!(f, "Unknown token: {}", token), // Форматированный вывод для неизвестного токена
            GlintError::IoError(err) => write!(f, "IO error: {}", err), // Форматированный вывод для ошибки ввода-вывода
        }
    }
}

// Реализация преобразования ошибки ввода-вывода в тип GlintError
impl From<std::io::Error> for GlintError {
    fn from(err: std::io::Error) -> Self {
        GlintError::IoError(err) // Преобразование ошибки ввода-вывода в тип GlintError
    }
}
