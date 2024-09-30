use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

mod tokenise;
use code_gen::code_gen;
use tokenise::{Token, TokenType, tokenise};

mod parser;
use parser::parse;

mod code_gen;

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
        Err(err) => { println!("Could not read {} due to {}", file_path, err); return; },
        Ok(_) => (),
    }

    let tokenised = tokenise(&s);
    let Some(parse_tree) = parse(&tokenised) else { println!("Could not parse the code!"); return; };
    let asm = code_gen(&parse_tree);
    
    dbg!(&tokenised);
    dbg!(&parse_tree);
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
