use data::*;
use std::fs::File;
use std::io::prelude::*;

pub fn parse_file(filename: &str) -> Expr {
    let mut f = File::open(filename).expect("Failed to open file.");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Failed to read file.");
    parse(&s)
}

pub fn parse(data: &String) -> Expr {
    let parens = count_parens(data);
    if parens.0 != parens.1 {
        panic!("One or more unmatched parentheses.");
    }
    let mut tokens = tokenize(&lines_to_spaces(&data))
        .iter()
        .rev()
        .map(|t| t.clone())
        .collect::<Vec<String>>();
    tokens_to_expr(&mut tokens)
}

fn count_parens(data: &String) -> (usize, usize) {
    data.chars()
        .fold((0usize, 0usize),
        |acc, item| {
            match item {
                '(' => (acc.0 + 1, acc.1),
                ')' => (acc.0, acc.1 + 1),
                _   => acc
            }
        })
}

fn tokens_to_expr(tokens: &mut Vec<String>) -> Expr {
    if tokens.is_empty() {
        panic!("No tokens to parse.");
    }
    let token: String;
    token = tokens.pop().unwrap();
    if token == "(" {
        let mut l = Vec::new();
        while tokens.last().unwrap().as_str() != ")" {
            l.push(tokens_to_expr(tokens));
        }
        //tokens.pop().unwrap();
        Expr::Exprs(Box::new(remove_spaces(l)))
    } else if token == ")" {
        panic!("Unexpected )");
    } else if token == "\"" {
        if !tokens.contains(&"\"".to_string()) {
            panic!("No end quote.");
        }
        let mut s = Vec::new();
        while tokens.last().unwrap().as_str() != "\"" {
            s.push(tokens.pop().unwrap());
        }
        tokens.pop().unwrap();
        s.pop().unwrap();
        Expr::Expr(Object::String(s.split_first().unwrap().1.concat()))
    } else {
        Expr::Expr(atomize(token))
    }
}

fn remove_spaces(l: Vec<Expr>) -> Vec<Expr> {
    let space = " ".to_string();
    let data = l.iter()
        .filter(|&x| {
        if let &Expr::Expr(Object::Symbol(ref s)) = x {
            if s == &space {
                false
            } else {
                true
            }
        } else {
            true
        }})
        .map(|x| x.clone())
        .collect::<Vec<Expr>>();
    data
}

fn atomize(token: String) -> Object {
    if token.contains('.') {
        match token.parse::<f64>() {
            Ok(f) => Object::Number(Number::Float(f)),
            _ => Object::Symbol(token),
        }
    } else {
        match token.parse::<isize>() {
            Ok(i) => Object::Number(Number::Int(i)),
            _ => Object::Symbol(token),
        }
    }
}

fn tokenize(data: &String) -> Vec<String> {
    let newdata = data.replace("(", " ( ")
        .replace(")", " ) ")
        .replace("\"", " \" ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut spaceddata = Vec::new();
    for s in newdata.iter() {
        spaceddata.push(s.clone());
        spaceddata.push(" ".to_string());
    }
    spaceddata
}

fn lines_to_spaces(data: &String) -> String {
    data.replace("\r\n", " ").replace("\n", " ")
}
