use data::*;
use std::fs::File;
use std::io::prelude::*;

pub fn parse_file(filename: &str) -> Result<Vec<Expr>, String> {
    let mut f: File;
    match File::open(filename) {
        Ok(r) => f = r,
        Err(_) => return Err(format!("Failed to open file {:?}", filename))
    }
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => {},
        Err(_) => return Err(format!("Failed to read file {:?}", filename))
    }
    return parse(&s)
}

pub fn parse(data: &String) -> Result<Vec<Expr>, String> {
    let parens = count_parens(data);
    if parens.0 != parens.1 {
        return Err("One or more unmatched parentheses.".to_string())
    }
    let mut tokens = tokenize(&lines_to_spaces(&data))
        .iter()
        .rev()
        .map(|t| t.clone())
        .collect::<Vec<String>>();
    let mut exprs = Vec::new();
    while tokens.len() > 0 {
        exprs.push(try!(tokens_to_expr(&mut tokens)));
        for i in (0..tokens.len()).rev() {
            if tokens[i] == " " {
                tokens.remove(i);
            }
        }
    }
    Ok(exprs)
}

fn count_parens(data: &String) -> (usize, usize) {
    data.chars()
        .fold((0, 0),
        |acc, item| {
            match item {
                '(' => (acc.0 + 1, acc.1),
                ')' => (acc.0, acc.1 + 1),
                _   => acc
            }
        })
}

fn tokens_to_expr(tokens: &mut Vec<String>) -> Result<Expr, String> {
    if tokens.is_empty() {
        return Err("No tokens to parse.".to_string())
    }
    let token: String;
    token = tokens.pop().unwrap();
    if token == "(" {
        let mut l = Vec::new();
        while tokens.last().unwrap().as_str() != ")" {
            let expr = tokens_to_expr(tokens);
            match expr {
                Ok(r) => l.push(r),
                Err(_) => return expr
            }
        }
        tokens.pop().unwrap();
        Ok(Expr::Exprs(Box::new(remove_spaces(l))))
    } else if token == ")" {
        Err("Unexpected )".to_string())
    } else if token == "[" {
        let mut l = Vec::new();
        while tokens.last().unwrap().as_str() != "]" {
            let expr = tokens_to_expr(tokens);
            match expr {
                Ok(r) => l.push(r),
                Err(_) => return expr
            }
        }
        tokens.pop().unwrap();
        l = remove_spaces(l);
        if l.len() < 2 {
            Err("Conditional missing body or case declarations".to_string())
        } else {
            let ref case = l[0];
            let ref body = l[1..];
            Ok(Expr::Expr(Object::ConditionalCase(
                        Box::new((*case).clone()),
                        body.iter().map(|e| (*e).clone()).collect::<Vec<_>>()
            )))
        }
    } else if token == "]" {
        Err("Unexpected ]".to_string())
    } else if token == "\"" {
        if !tokens.contains(&"\"".to_string()) {
            return Err("No end quote.".to_string());
        }
        let mut s = Vec::new();
        while tokens.last().unwrap().as_str() != "\"" {
            s.push(tokens.pop().unwrap());
        }
        tokens.pop().unwrap();
        s.pop().unwrap();
        Ok(Expr::Expr(Object::String(s.split_first().unwrap().1.concat())))
    } else {
        Ok(Expr::Expr(atomize(token)))
    }
}

fn remove_spaces(l: Vec<Expr>) -> Vec<Expr> {
    let data = l.iter()
        .filter(|&x| {
            if let &Expr::Expr(Object::Symbol(ref s)) = x {
                if s == " " {
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
            Ok(f) => Object::Number(Number::Float(f, token.to_string())),
            _ => Object::Symbol(token),
        }
    } else {
        match token.parse::<i64>() {
            Ok(i) => Object::Number(Number::Int(i)),
            _ => {
                if &token == "true" {
                    Object::Boolean(Boolean::True)
                } else if &token == "false" {
                    Object::Boolean(Boolean::False)
                } else {
                    Object::Symbol(token)  
                }
            },
        }
    }
}

//const RESERVED_KEYWORDS: [&'static str; 8] = [
    //"list",
    //"cons",
    //"print",
    //"exit",
    //"define",
    //"true",
    //"false",
    //"cond"
//];

fn tokenize(data: &String) -> Vec<String> {
    // needs to be fixed so that parens inside quotes are not replaced
    let newdata = data
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("\"", " \" ")
        .replace("[", " [ ")
        .replace("]", " ] ")
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
