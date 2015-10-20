use super::{NAME, VERSION, AUTHOR, INFO};
use eval::{run_file, run_input};
use data::*;
use std::io::{self, Write};

//find some way to detect arrow key presses?
//have a history?
pub fn repl(file: Option<&str>) {
    println!("\r\nStarting REPL for {name} {version}
{author}
{info}\r\n", 
name = NAME,
version = VERSION,
author = AUTHOR,
info = INFO);
    let reader = io::stdin();
    let stdout = io::stdout();
    let mut writer = stdout.lock();
    let mut stdenv = Env::new();
    //let mut history: Vec<String> = Vec::new();
    if let Some(filename) = file {
        run_file(filename, &mut stdenv);
    }
    loop {
        //use ncurses to get chars?
        writer.write(b">>> ").expect("Failed to write line.");
        writer.flush().expect("Failed to flush stdout.");
        let mut input = String::new();
        reader.read_line(&mut input).expect("Failed to read line.");
        if input != "\n".to_string() {
            //history.push(input.clone());
            run_input(input, &mut stdenv);
            //println!("{:?}", history);
        }
    }
}
