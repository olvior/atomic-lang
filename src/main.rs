use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

mod tokenise;
use tokenise::{Token, TokenType, Tokeniser};

pub mod parser;
use parser::Parser;

mod code_gen;
use code_gen::CodeGen;

fn main() {
    let args: Vec<String> = env::args().collect();
    // if args.len() < 3 {
    //     println!("Useage: ./lithium path/to/file.li path/to/out");
    //     return;
    // }

    // let file_path = &args[1];
    // let out_path = &args[2];
    let file_path = "test.li";
    let out_path = "bin/test";
    
    let path = Path::new(file_path);

    let Ok(mut file) = File::open(path) else { println!("Could not find {}", file_path); return; };

    let mut s = String::new();
    if let Err(err) = file.read_to_string(&mut s) { println!("Could not read {} due to {}", file_path, err); return; }

    let mut tokeniser = Tokeniser{ source: s, index: 0};
    let tokenised = tokeniser.tokenise();
    dbg!(&tokenised);
    let mut parser = Parser { tokens: tokenised, index: 0};
    let parse_tree = parser.parse();
    dbg!(&parse_tree);

    let mut generator = CodeGen::new();
    generator.generate(&parse_tree);
    let asm = generator.asm;
    print!("{}", asm);

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

fn exit_message(message: &str) {
    println!("{}", message);
    std::process::exit(127)
}
