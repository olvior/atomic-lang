#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType {
    Exit,

    ParenOpen,
    ParenClose,

    IntegerLit,

    Variable,

    Semicolon,
    NoToken,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Token {
    pub token: TokenType,
    pub info: String,
}

pub fn tokenise(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec!();
    let mut buffer = String::new();

    for (_i, c) in source.chars().enumerate() {
        if buff_is_finished(&buffer, c) {
            if let Some(token) = finish_token(&buffer) {
                tokens.push(token);
            }
            buffer = String::new();
        }

        let simple_token: TokenType = match c {
            ';' => TokenType::Semicolon,
            '(' => TokenType::ParenOpen,
            ')' => TokenType::ParenClose,
            _   => TokenType::NoToken,
        };

        if simple_token != TokenType::NoToken {
            tokens.push(Token{token: simple_token, info: String::new()});
            continue;
        }



        if !(c.is_whitespace()) {
            buffer.push(c);
        }
    }
    return tokens;
}

fn finish_token(buffer: &str) -> Option<Token> {
    if buffer.starts_with(|c: char| c.is_numeric()) {
        let token = Token{token: TokenType::IntegerLit, info: buffer.to_string()};
        return Some(token);
    } else {
        return match buffer {
            "exit" => Some(Token{token: TokenType::Exit, info: "".to_string()}),
            _ => Some(Token{ token: TokenType::Variable, info: buffer.to_string()}),
        };
    }
}

fn buff_is_finished(buffer: &str, c: char) -> bool {
    if buffer.starts_with(|c: char| c.is_numeric()) {
        if c.is_numeric() { return false; } else { return true; }
    } else if buffer.starts_with(|c: char| c.is_alphabetic()) {
        if c.is_alphanumeric() { return false; } else { return true; }
    }

    return false;
}

