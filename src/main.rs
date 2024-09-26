use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

mod tokenise;
use tokenise::{Token, TokenType, tokenise};

fn tokens_to_asm(tokens: &Vec<Token>) -> String {
    let mut asm = "global _start\n_start:\n".to_string();

    let mut i = 0;
    while i < tokens.len() {
        let token = &tokens[i];
        
        if token.token == TokenType::Exit {
            if i + 4 < tokens.len() {
                if tokens[i + 1].token == TokenType::ParenOpen && tokens[i+2].token == TokenType::IntegerLit && tokens[i+3].token == TokenType::ParenClose 
                && tokens[i+4].token == TokenType::Semicolon {
                    asm.push_str("    mov rax, 60\n");
                    asm.push_str(&format!("    mov rdi, {}\n", tokens[i + 2].info));
                    asm.push_str("    syscall\n")
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
    dbg!(&tokenised);
    let asm = tokens_to_asm(&tokenised);
    dbg!(&asm);

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

    //dbg!(a);
    
    
    let _ = Command::new("sh")
        .arg("-c")
        .arg(&format!("ld -o {} {}.o", out_path, out_path))
        .output();
}
