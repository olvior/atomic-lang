use crate::{
    errors::Error,
    dbg_m
};


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType {
    // built in functions
    Exit,
    PutChar,

    AssignEq,

    // math
    Plus,
    Minus,
    Star,
    ForwardsSlash,

    ParenOpen,
    ParenClose,
    BraceOpen,
    BraceClose,

    // types
    IntType,
    IntegerLit,

    Function,

    Identifier,

    Semicolon,
    NoToken,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Token {
    pub token: TokenType,
    pub info: String,
    pub line: usize,
}

pub struct Tokeniser {
    source: String,
    debug: bool,
    index: usize,
    line_num: usize,
}

impl Tokeniser {
    pub fn new(source: String, debug: bool) -> Tokeniser {
        Tokeniser {
            source,
            debug,
            index: 0,
            line_num: 1,
        }
    }

    pub fn tokenise(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens: Vec<Token> = vec!();

        while self.index < self.source.len() {
            let Some(current_word) = self.get_next_word() else { self.index += 1; continue; };
            
            dbg_m(&current_word, self.debug);

            let mut token_type = match current_word.as_str() {
                ";" => TokenType::Semicolon,
                "(" => TokenType::ParenOpen,
                ")" => TokenType::ParenClose,
                "{" => TokenType::BraceOpen,
                "}" => TokenType::BraceClose,
                "=" => TokenType::AssignEq,

                "+" => TokenType::Plus,
                "-" => TokenType::Minus,
                "*" => TokenType::Star,
                "/" => TokenType::ForwardsSlash,


                "exit" => TokenType::Exit,
                "putchar" => TokenType::PutChar,

                "int" => TokenType::IntType,
                "fn" => TokenType::Function,

                _ => TokenType::NoToken,
            };

            if token_type == TokenType::NoToken {
                if current_word.chars().nth(0).expect("Word was empty").is_numeric() {
                    token_type = TokenType::IntegerLit;
                } else if current_word.chars().nth(0).expect("Word was empty").is_alphabetic() {
                    token_type = TokenType::Identifier;
                }
                else {
                    let err = self.create_err(format!("Could not tokenise {}", current_word));

                    return Err(err);
                }
            }

            tokens.push(Token { token: token_type, info: current_word, line: self.line_num });
        }

        return Ok(tokens);
    }

    fn create_err(&self, msg: String) -> Error {
        Error { line: self.line_num, msg }
    }

    fn skip_whitespace(&mut self) {
        while  self.index < self.source.len()
            && self.source.chars().nth(self.index).expect("Failed to get string").is_whitespace()
        {
            if self.source.chars().nth(self.index).unwrap() == '\n' {
                self.line_num += 1;
            }

            self.index += 1;
        }
    }

    fn get_next_word(&mut self) -> Option<String> {
        self.skip_whitespace();

        if self.index >= self.source.len() {
            return None;
        }

        let first_char = self.source.chars().nth(self.index).expect("Could not index string!");

        let second_char = self.source.chars().nth(self.index + 1);

        // `//` comment testing
        if second_char.is_some() {
            if second_char.expect("Internal error") == '/' && first_char == '/' {
                // skip the comment until new line
                while self.index < self.source.len() && self.source.chars().nth(self.index).expect("error") != '\n' {
                    self.index += 1;
                }
                
                // we didn't get any tokens, we just skipped comment
                return None;
            }
        }

        let mut word = String::from(first_char);
        self.index += 1;

        for c in self.source.get(self.index..).expect("Could not collect source into chars").chars() {
            if first_char.is_alphabetic() {
                if !(c.is_alphanumeric() || ['_', '-'].contains(&c)) {
                    break;
                }
            }

            else if first_char.is_numeric() {
                if !c.is_numeric() {
                    break;
                }
            } else {
                if c != first_char {
                    break;
                }
            }

            self.index += 1;
            word.push(c);
        }


        if word == String::new() {
            return None;
        }


        return Some(word);
    }
}

