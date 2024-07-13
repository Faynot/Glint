// use serde_json::to_string_pretty; // Импорт функции для красивого вывода JSON
use std::env; // Импорт модуля для работы с аргументами командной строки
use std::process; // Импорт модуля для работы с процессами
mod errors;
mod parser; // Импорт модуля parser.rs
mod tokenizator; // Импорт модуля tokenizator.rs // Импорт модуля errors.rs

fn main() {
    let args: Vec<String> = env::args().collect(); // Получаем аргументы командной строки
    if args.len() != 2 {
        eprintln!("Usage: cargo run <filename>.glt"); // Выводим сообщение об использовании, если аргументы не верны
        process::exit(1); // Завершаем программу с кодом ошибки
    }

    let filename = &args[1]; // Получаем имя файла из аргументов командной строки
    let input = match parser::read_file(filename) {
        // Читаем содержимое файла
        Ok(content) => content, // Если чтение прошло успешно, получаем содержимое файла
        Err(err) => {
            // Если возникла ошибка
            eprintln!("Error reading file {}: {}", filename, err); // Выводим сообщение об ошибке
            process::exit(1); // Завершаем программу с кодом ошибки
        }
    };

    let tokens = match parser::parse_tokens(&input) {
        // Парсим токены из содержимого файла
        Ok(tokens) => tokens, // Если парсинг прошёл успешно, получаем токены
        Err(err) => {
            // Если возникла ошибка
            eprintln!("Error parsing tokens: {}", err); // Выводим сообщение об ошибке
            process::exit(1); // Завершаем программу с кодом ошибки
        }
    };

    let json_data = tokenizator::tokens_to_json(&tokens); // Конвертируем токены в JSON
    if let Err(err) = serde_json::to_string_pretty(&json_data) {
        // Конвертируем JSON в красиво отформатированную строку
        eprintln!("Error serializing tokens to JSON: {}", err); // Выводим сообщение об ошибке при сериализации JSON
        process::exit(1); // Завершаем программу с кодом ошибки
    }

    println!("{}", serde_json::to_string_pretty(&json_data).unwrap()); // Выводим отформатированную строку JSON
}
