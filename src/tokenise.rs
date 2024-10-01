#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum TokenType {
    Exit,

    AssignEq,

    ParenOpen,
    ParenClose,

    IntType,
    IntegerLit,

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
}

pub struct Tokeniser {
    pub source: String,
    pub index: usize,
}

impl Tokeniser {
    pub fn tokenise(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec!();

        while self.index < self.source.len() {
            let Some(current_word) = self.get_next_word() else { self.index += 1; continue; };
            println!("{}", current_word);

            let mut token_type = match current_word.as_str() {
                ";" => TokenType::Semicolon,
                "(" => TokenType::ParenOpen,
                ")" => TokenType::ParenClose,
                "=" => TokenType::AssignEq,

                "exit" => TokenType::Exit,

                "int" => TokenType::IntType,

                _ => TokenType::NoToken,
            };

            if token_type == TokenType::NoToken {
                if current_word.chars().nth(0).expect("Word was empty").is_numeric() {
                    token_type = TokenType::IntegerLit;
                } else {
                    token_type = TokenType::Identifier;
                }
            }

            tokens.push(Token { token: token_type, info: current_word });
        }

        return tokens;
    }

    fn skip_whitespace(&mut self) {
        while  self.index < self.source.len()
            && self.source.chars().nth(self.index).expect("Failed to get string").is_whitespace()
        {
            self.index += 1;
        }
    }

    fn get_next_word(&mut self) -> Option<String> {
        self.skip_whitespace();

        let first_char = self.source.chars().nth(self.index).expect("Could not index string!");
        let mut word = String::new();


        for c in self.source.get(self.index..).expect("Could not collect source into chars").chars() {
            if first_char.is_alphabetic() {
                if !c.is_alphanumeric() {
                    break;
                }
            }
            else if first_char.is_numeric() {
                if !c.is_numeric() {
                    break;
                }
            } else {
                self.index += 1;
                word.push(c);
                break;
            }
            
            self.index += 1;
            word.push(c);
        }


        if word == String::new() {
            return None;
        }
        

        self.skip_whitespace();
        return Some(word);
    }
}

