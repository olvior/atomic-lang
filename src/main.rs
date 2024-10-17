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

mod errors;
use errors::{external_error, inline_error};

#[derive(PartialEq)]
enum Options {
    // delete the asm and object files
    Clean,
    Debug,
}

/// A struct with the io paths, and the command line options
struct Settings {
    f_in: String,
    f_out: String,
    options: Vec<Options>,
}

/// Returns a `Settings` object from the command line arguments
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
                        'd' => Options::Debug,

                        _ => external_error(&format!("Unknown option {}", c)),
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
        //external_error(&format!("Usage: {} path/to/file.at", args[0]));
        arguments.push("test.at".to_string());
    }

    // we don't have the settings yet
    if options.contains(&Options::Debug) {
        dbg!(&arguments);
    }

    let f_in = arguments[0].clone();

    let f_out: String;

    if arguments.len() > 1 {
        f_out = arguments[1].clone();
    } else {
        f_out = format!("bin/{}.out", &f_in);

        if options.contains(&Options::Debug) {
            dbg!(&f_out);
        }
    }

    let settings = Settings { f_in, f_out, options };

    return settings;
}

fn read_in(settings: &Settings) -> String {
    let path = Path::new(&settings.f_in);
    
    let Ok(mut file) = File::open(path) else { 
        external_error(&format!("Could not open {}", &settings.f_in))
    };

    let mut contents = String::new();

    if let Err(err) = file.read_to_string(&mut contents) {
        external_error(&format!("Could not read {} due to {}", &settings.f_in, err))
    }

    return contents;
}

fn write_out(settings: &Settings, asm: &str) {
    let path = &format!("{}.asm", &settings.f_out);

    let Ok(mut output_file) = File::create(path) else {
        external_error("Could not create output file");
    };

    write!(output_file, "{}", asm).expect("Could not write output asm file!");
}

/// Uses `nasm` to assemble the code into an object file
fn assemble(settings: &Settings) {
    let out_path = &settings.f_out;

    let nasm_output = Command::new("sh")
        .arg("-c")
        .arg(&format!("nasm -felf64 {}.asm -o {}.o", out_path, out_path))
        .output()
        .expect("Could not execute nasm command");

    if nasm_output.status.success() {
        dbg_p("Assembled", settings);
    } else {
        external_error(
            &format!("Did not assemble:\n{}",
                String::from_utf8(nasm_output.stderr).unwrap()
            )
        );
    }
}

/// Uses `ld` to link the object file and make an executable
fn link(settings: &Settings) {
    let out_path = &settings.f_out;

    let linker_output = Command::new("sh")
        .arg("-c")
        .arg(&format!("ld -o {} {}.o", out_path, out_path))
        .output()
        .expect("Could not execute ld command");

    if linker_output.status.success() {
        dbg_p("Linked", settings);
    } else {
        external_error(
            &format!("Could not link:\n{}",
                String::from_utf8(linker_output.stderr).unwrap()
            )
        );
    }
}

fn clean_asm(settings: &Settings) {
    let out_path = &settings.f_out;

    let _rm_asm_output = Command::new("sh")
        .arg("-c")
        .arg(&format!("rm {}.asm", out_path))
        .output()
        .expect("Could not rm asm");
    
    dbg_p("Cleaned asm file", settings);
}

fn clean_object(settings: &Settings) {
    let out_path = &settings.f_out;

    let _rm_obj_output = Command::new("sh")
        .arg("-c")
        .arg(&format!("rm {}.o", out_path))
        .output()
        .expect("Could not rm object file");

    dbg_p("Cleaned object file", settings);
}

/// Removes the intermediary files, the asm code and object file
fn clean_files(settings: &Settings) {
    clean_asm(settings);
    clean_object(settings);
}

fn main() {
    let settings = collect_settings();

    let source_code = read_in(&settings);

    // step one: tokenise the source code
    let mut tokeniser = Tokeniser::new(source_code, settings.options.contains(&Options::Debug));
    let tokenised = tokeniser.tokenise();

    if let Err(err) = tokenised {
        inline_error(err, &settings);
    }
    
    let tokenised = tokenised.unwrap();

    dbg_p(&tokenised, &settings);


    // step two: parse the tokens into an ast
    let mut parser = Parser { tokens: tokenised, index: 0 };
    let parse_tree = parser.parse();

    if let Err(err) = parse_tree {
        inline_error(err, &settings);
    }

    let parse_tree = parse_tree.unwrap();

    dbg_p(&parse_tree, &settings);


    // generate asm code from the ast
    let mut generator = CodeGen::new();
    let asm = &generator.gen_output(&parse_tree);

    
    // output and generate executable
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

/// Calls `dbg!` if the options contain `Options::Debug`
fn dbg_p<T: std::fmt::Debug>(thing: T, settings: &Settings) {
    debug_print(thing, settings);
}

fn debug_print<T: std::fmt::Debug>(thing: T, settings: &Settings) {
    if settings.options.contains(&Options::Debug) {
        dbg!(thing);
    }
}

fn dbg_m<T: std::fmt::Display>(thing: T, should_print: bool) {
    debug_print_bool(thing, should_print);
}

fn debug_print_bool<T: std::fmt::Display>(thing: T, should_print: bool) {
    if should_print {
        eprintln!("{}", thing);
    }
}

