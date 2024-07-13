use crate::errors::GlintError; // Импорт типа ошибок из модуля errors
                               // use std::collections::HashMap; // Импорт HashMap из стандартной библиотеки
use std::fs; // Импорт модуля fs из стандартной библиотеки для работы с файловой системой

// Определение перечисления для токенов
#[derive(Debug)]
pub enum Token {
    Tor(String), // Токен типа Tor с содержимым типа String
    Mup {
        name: String,
        args: Vec<String>,
        block: Vec<Token>,
    }, // Токен типа Mup с полями name (String), args (вектор String) и block (вектор Token)
}

// Функция для парсинга строки и возвращения опционального токена или ошибки
pub fn parse_line(line: &str) -> Result<Option<Token>, GlintError> {
    if line.starts_with("write") {
        let value = line
            .trim_start_matches("write")
            .trim()
            .trim_matches('"')
            .to_string();
        Ok(Some(Token::Tor(value)))
    } else if line.starts_with("function") {
        let rest = line.trim_start_matches("function").trim();
        let mut parts = rest.splitn(2, '(');
        let name = parts
            .next()
            .ok_or_else(|| GlintError::UnknownToken("function name".to_string()))?
            .trim()
            .to_string();
        let args = parts
            .next()
            .ok_or_else(|| GlintError::UnknownToken("function arguments".to_string()))?
            .trim_end_matches(')')
            .split(',')
            .map(|s| {
                s.trim()
                    .trim_matches(':')
                    .trim_matches(')')
                    .trim()
                    .to_string()
            })
            .collect::<Vec<_>>();
        Ok(Some(Token::Mup {
            name,
            args,
            block: Vec::new(),
        }))
    } else if line.trim().is_empty() {
        Ok(None) // Обрабатываем пустые строки и возвращаем None
    } else {
        Err(GlintError::UnknownToken(line.to_string())) // Возвращаем ошибку для неизвестного токена
    }
}

// Функция для парсинга блока строк и возвращения вектора токенов или ошибки
pub fn parse_block(
    lines: &mut std::iter::Peekable<std::str::Lines>,
    indent: usize,
) -> Result<Vec<Token>, GlintError> {
    let mut block = Vec::new();
    while let Some(line) = lines.peek() {
        if line.chars().take(indent).all(|c| c == ' ' || c == '\t') {
            let line = lines
                .next()
                .ok_or_else(|| GlintError::UnknownToken("block line".to_string()))?
                .trim_start();
            if let Some(token) = parse_line(line)? {
                match token {
                    Token::Mup { name, args, .. } => {
                        let block_tokens = parse_block(lines, indent + 4)?;
                        block.push(Token::Mup {
                            name,
                            args,
                            block: block_tokens,
                        });
                    }
                    _ => block.push(token),
                }
            }
        } else {
            break;
        }
    }
    Ok(block) // Возвращаем успешно спарсенный блок токенов
}

// Функция для парсинга всего входного текста и возвращения вектора токенов или ошибки
pub fn parse_tokens(input: &str) -> Result<Vec<Token>, GlintError> {
    let mut tokens = Vec::new();
    let mut lines = input.lines().peekable();

    while let Some(line) = lines.next() {
        if let Some(token) = parse_line(line)? {
            match token {
                Token::Mup { name, args, .. } => {
                    let block_tokens = parse_block(&mut lines, 4)?;
                    tokens.push(Token::Mup {
                        name,
                        args,
                        block: block_tokens,
                    });
                }
                _ => tokens.push(token),
            }
        }
    }
    Ok(tokens) // Возвращаем успешно спарсенные токены
}

// Функция для чтения содержимого файла и возвращения его как строки или ошибки
pub fn read_file(filename: &str) -> Result<String, GlintError> {
    fs::read_to_string(filename).map_err(|err| GlintError::IoError(err)) // Чтение файла в строку или возвращение ошибки ввода-вывода
}
