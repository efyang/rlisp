#![feature(custom_derive)]
#![feature(convert)]
#![feature(clone_from_slice)]
#![feature(slice_splits)]
#![feature(str_char)]
extern crate clap;
mod parser;
mod data;
mod eval;
mod stdlisp;

use clap::App;
use data::*;
use parser::*;
use std::io::{self, Write};

const NAME: &'static str = "rlisp";
const VERSION: &'static str = "1.0";
const AUTHOR: &'static str = "Edward Yang <edward.yang6771@gmail.com>";
const INFO: &'static str = "A basic lisp interpreter in rust.";

fn main() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(INFO)
        .args_from_usage(
            "-i --interactive 'optional - Enables interactive repl - enabled if no file specified'
            -f --file=[FILE] 'optional - specifies a file to load'")
        .get_matches();
    let mut stdenv = Env::new();
    if let Some(input) = matches.value_of("FILE") {
        if matches.is_present("interactive") {
            repl(Some(input));
        } else {
            println!("{:?}", parse_file(input).eval(&mut Env::new()));
        }
    } else {
        repl(None);
    }
    //let parsed = parse(&"(cons 1 (list 1 2))".to_string());
    //let mut stdenv = Env::new();
    //println!("{:?}", parsed.eval(&mut stdenv));
}
//find some way to detect arrow key presses?
//have a history?
fn repl(file: Option<&str>) {
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
    if let Some(filename) = file {
        println!("{:?}", parse_file(filename).eval(&mut stdenv));
    }
    loop {
        writer.write(b">>> ").expect("Failed to write line.");
        writer.flush().expect("Failed to flush stdout.");
        let mut input = String::new();
        reader.read_line(&mut input).expect("Failed to read line.");
        if input != "\n".to_string() {
            println!("{:?}", parse(&input).eval(&mut stdenv));
        }
    }
}
