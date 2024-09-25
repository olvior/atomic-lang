use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug)]
#[derive(PartialEq)]
enum TokenType {
    Return,
    IntegerLit,
    Semicolon,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Token {
    token: TokenType,
    info: String,
}

fn tokenise(source: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec!();
    let mut buffer = String::new();

    for (_i, c) in source.chars().enumerate() {
        if buffer.starts_with(|c: char| c.is_alphabetic()){
            if !c.is_alphanumeric() {
                // add the token
                if let Some(token) = finish_token(&buffer) {
                    tokens.push(token);
                }

                buffer = String::new();
            }
        }

        if buffer.starts_with(|c: char| c.is_numeric()){
            if !c.is_numeric() {
                // add the token
                if let Some(token) = finish_token(&buffer) {
                    tokens.push(token);
                }

                buffer = String::new();
            }
        }

        if c == ';' {
            tokens.push(Token{token: TokenType::Semicolon, info: String::new()});

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
            "return" => Some(Token{token: TokenType::Return, info: "".to_string()}),
            _ => None,
        };
    }

}


fn tokens_to_asm(tokens: &Vec<Token>) -> String {
    let mut asm = "global _start\n_start:\n".to_string();

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        
        if token.token == TokenType::Return {
            if i + 1 < tokens.len() {
                if tokens[i + 1].token == TokenType::IntegerLit {
                    if i + 2 < tokens.len() {
                        if tokens[i + 2].token == TokenType::Semicolon {
                            asm.push_str("    mov rax, 60\n");
                            asm.push_str(&format!("    mov rdi, {}\n", tokens[i + 1].info));
                            asm.push_str("    syscall\n")
                        }
                    }
                }
            }
        }

        i += 1;
    }


    return asm;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Useage: ./lithium path/to/file.li path/to/out");
        return;
    }

    let file_path = &args[1];
    let out_path = &args[2];
    
    let path = Path::new(file_path);

    let Ok(mut file) = File::open(path) else { println!("Could not find {}", file_path); return; };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(err) => panic!("Could not read {} due to {}", file_path, err),
        Ok(_) => (),
    }

    let tokenised = tokenise(&s);
    let asm = tokens_to_asm(&tokenised);

    let path = &format!("{}.asm", out_path);
    let Ok(mut output_file) = File::create(path) else {
        println!("Could not create output file!");
        return;
    };

    write!(output_file, "{}", asm).expect("Could not write output asm file!");

    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!("nasm -felf64 {}.asm -o {}.o", out_path, out_path))
        .output();
    
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!("ld -o {} {}.o", out_path, out_path))
        .output();
}
