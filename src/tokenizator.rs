use crate::parser::Token;
use serde_json::json; // Импорт функции json для создания JSON объектов
use serde_json::Value; // Импорт типа Value из библиотеки serde_json
use std::collections::HashMap; // Импорт HashMap из стандартной библиотеки // Импорт типа Token из модуля parser.rs

// Функция для конвертации токена в JSON объект
pub fn token_to_json(token: &Token) -> Value {
    match token {
        Token::Tor(value) => json!({
            "key": {
                "token": "write", // название токена
                "value": value // значение
            }
        }), // Преобразуем токен Tor в JSON объект
        Token::Mup { name, args, block } => {
            let args_json: HashMap<_, _> = args
                .iter()
                .enumerate()
                .map(|(i, arg)| (format!("arg{}", i), json!(arg)))
                .collect(); // Преобразуем аргументы функции в HashMap с JSON объектами

            let block_json = tokens_to_json(block); // Рекурсивно конвертируем блок токенов в JSON объект

            json!({
                "key": {
                    "token": "function", // название токена
                    "name": name, // название функции
                    "args": args_json, // аргументы функции
                    "block": block_json // блок кода
                }
            }) // Преобразуем токен Mup в JSON объект
        }
    }
}

// Функция для конвертации вектора токенов в JSON массив
pub fn tokens_to_json(tokens: &[Token]) -> Value {
    let result: Vec<_> = tokens.iter().map(token_to_json).collect(); // Преобразуем каждый токен в JSON объект и собираем в вектор

    json!({ "program": result }) // Возвращаем JSON объект с ключом "program" и массивом токенов внутри
}
