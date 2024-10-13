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

#[derive(PartialEq)]
enum Options {
    // delete the asm and object files
    Clean,
}

struct Settings {
    f_in: String,
    f_out: String,
    options: Vec<Options>,
}

fn collect_settings() -> Settings {
    let args: Vec<String> = env::args().collect();

    let mut options: Vec<Options> = vec!();
    let mut arguments: Vec<String> = vec!();

    // loop but ignore first arg
    for arg in args[1..].into_iter() {
        // if its an option
        if arg.chars().nth(0).expect("Arg was empty") == '-' {
            for c in arg[1..].chars().into_iter() {

                options.push(
                    match c {
                        // clean, delete other files
                        'c' => Options::Clean,

                        _ => panic!("Unknown option {}", c),
                    }
                );

            }
        }

        // if its just an argument
        else {
            arguments.push(arg.to_string());
        }
    }

    if arguments.len() < 1 {
        exit_message("Useage: ./lithium path/to/file.li");
    }

    dbg!(&arguments);
    let f_in = arguments[0].clone();

    let f_out: String;

    if arguments.len() > 1 {
        f_out = arguments[1].clone();
    } else {
        f_out = format!("bin/{}", &f_in[0..f_in.len() - 3]);
        dbg!(&f_out);
    }

    let settings = Settings { f_in, f_out, options };

    return settings;
}

fn read_in(settings: &Settings) -> String {
    let path = Path::new(&settings.f_in);
    
    let Ok(mut file) = File::open(path) else { panic!("Could not find {}", &settings.f_in); };

    let mut contents = String::new();

    if let Err(err) = file.read_to_string(&mut contents) { panic!("Could not read {} due to {}", &settings.f_in, err); }

    return contents;
}

fn write_out(settings: &Settings, asm: &str) {
    let path = &format!("{}.asm", &settings.f_out);

    let Ok(mut output_file) = File::create(path) else {
        println!("Could not create output file!");
        return;
    };

    write!(output_file, "{}", asm).expect("Could not write output asm file!");
}

fn assemble(settings: &Settings) {
    let out_path = &settings.f_out;

    let nasm_output = Command::new("sh")
        .arg("-c")
        .arg(&format!("nasm -felf64 {}.asm -o {}.o", out_path, out_path))
        .output();

    if nasm_output.is_ok() {
        println!("Assembled");
    } else {
        println!("Did not assemble:\n{}", nasm_output.unwrap_err());
    }
}

fn link(settings: &Settings) {
    let out_path = &settings.f_out;

    let linker_output = Command::new("sh")
        .arg("-c")
        .arg(&format!("ld -o {} {}.o", out_path, out_path))
        .output();

    if linker_output.is_ok() {
        println!("Linked");
    } else {
        println!("Did not link:\n{}", linker_output.unwrap_err());
    }
}

fn clean_asm(settings: &Settings) {
    let out_path = &settings.f_out;

    let rm_asm_output = Command::new("sh")
        .arg("-c")
        .arg(&format!("rm {}.asm", out_path))
        .output();
    
    if rm_asm_output.is_ok() {
        println!("Cleaned asm file");
    } else {
        println!("Did not clean asm file:\n{}", rm_asm_output.unwrap_err());
    }
}

fn clean_object(settings: &Settings) {
    let out_path = &settings.f_out;

    let rm_obj_output = Command::new("sh")
        .arg("-c")
        .arg(&format!("rm {}.o", out_path))
        .output();

    if rm_obj_output.is_ok() {
        println!("Cleaned object file");
    } else {
        println!("Did not clean obj:\n{}", rm_obj_output.unwrap_err());
    }
}

fn clean_files(settings: &Settings) {
    clean_asm(settings);
    clean_object(settings);
}

fn main() {
    let settings = collect_settings();

    let source_code = read_in(&settings);

    let mut tokeniser = Tokeniser{ source: source_code, index: 0};
    let tokenised = tokeniser.tokenise();
    dbg!(&tokenised);

    let mut parser = Parser { tokens: tokenised, index: 0};
    let parse_tree = parser.parse();
    dbg!(&parse_tree);

    let mut generator = CodeGen::new();
    generator.generate(&parse_tree, true);
    let asm = &generator.asm;
    print!("{}", asm);

    write_out(&settings, asm);


    assemble(&settings);

    link(&settings);

    if settings.options.contains(&Options::Clean) {
        clean_files(&settings);
    }
}

fn exit_message(message: &str) {
    println!("{}", message);
    std::process::exit(127)
}

