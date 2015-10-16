#![feature(custom_derive)]
#![feature(convert)]
#![feature(clone_from_slice)]
#![feature(slice_splits)]
extern crate clap;
mod parser;
mod data;
mod eval;
mod stdlisp;
mod repl;

use clap::App;
use data::*;
use parser::*;
use repl::repl;

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
            let parsed = parse_file(input);
            match parsed {
                Ok(rp) => {let evaluated = rp.eval(&mut stdenv);
                    match evaluated {
                        Ok(r) => println!("{:?}", r),
                        Err(e) => println!("Eval of input: \r\n\r\n{input}\r\n failed with error: \r\n\r\n {e}", input = input, e = e)
                    }
                },
                Err(e) => {println!("Parsing of input: \r\n\r\n{input}\r\n failed with error: \r\n\r\n {e}", input = input, e = e)}
            }
        }
    } else {
        repl(None);
    }
}

