use super::{NAME, VERSION, AUTHOR, INFO};
use data::*;
use parser::*;
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
    let mut history: Vec<String> = Vec::new();
    if let Some(filename) = file {
        let parsed = parse_file(filename);
        match parsed {
            Ok(rp) => {
                let evaluated = rp.eval(&mut stdenv);
                match evaluated {
                    Ok(r) => println!("{:?}", r),
                    Err(e) => println!("Eval of input file: \r\n\r\n{input}\r\n failed with error: \r\n\r\n {e}", input = filename, e = e)
                }
            },
            Err(e) => println!("Parsing of input file: \r\n\r\n{input}\r\n failed with error: \r\n\r\n {e}", input = filename, e = e)
        }
    }
    loop {
        //use ncurses to get chars?
        writer.write(b">>> ").expect("Failed to write line.");
        writer.flush().expect("Failed to flush stdout.");
        let mut input = String::new();
        reader.read_line(&mut input).expect("Failed to read line.");
        if input != "\n".to_string() {
            history.push(input.clone());
            let parsed = parse(&input);
            match parsed {
                Ok(rp) => {
                    let evaluated = rp.eval(&mut stdenv);
                    match evaluated {
                        Ok(r) => println!("{:?}", r),
                        Err(e) => println!("Eval of input: \r\n\r\n{input}\r\n failed with error: \r\n\r\n {e}", input = input, e = e)
                    }
                },
                Err(e) => println!("Parsing of input: \r\n\r\n{input}\r\n failed with error: \r\n\r\n {e}", input = input, e = e)
            }
            println!("{:?}", history);
        }
    }
}
