use std::process::exit;

use colored::Colorize;

use crate::{Settings, read_in};

#[derive(Debug)]
pub struct Error {
    pub line: usize,
    pub msg: String,
}

pub fn external_error(error_msg: &str) -> ! {
    eprintln!("{} {}",
            "Error:".red().bold(),
            error_msg,
    );
    exit(2);
}

pub fn inline_error(err: Error, settings: &Settings) -> ! {
    let text = read_in(settings);
    let text: Vec<&str> = text.split('\n').collect();
    let line = text[err.line - 1];

    eprintln!("{} in line {}:",
            "Error".red().bold(),
            err.line,
    );
    eprintln!("{}", line);
    eprintln!("\n{}", err.msg);
    exit(1);
}

