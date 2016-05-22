#![allow(dead_code)]
use data::*;
use std::sync::Arc;
use eval::Eval;

macro_rules! generate_base_fn {
    ($fnname:ident, $name:ident) => {
        Function {name: stringify!($fnname).to_string(), procedure: Arc::new(LispFn::Builtin($name as BuiltinFn))}
    }
}

macro_rules! generate_normal_base_fn {
    ($name:ident) => {generate_base_fn!($name, $name)}
}

lazy_static! {
    pub static ref BASE_FUNCTIONS: [Function; 6] = [
        generate_normal_base_fn!(list),
        generate_normal_base_fn!(cons),
        generate_normal_base_fn!(print),
        generate_normal_base_fn!(exit),
        generate_normal_base_fn!(cond),
        Function {name: "=".to_string(), procedure: Arc::new(LispFn::Builtin(equals as BuiltinFn))}
    ];
}

//fn add(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
//let mut numbers = Vec::with_capacity(args.len());
//for a in args.iter() {
//match get_number(a) {
//Ok(r) => numbers.push(r),
//Err(e) => return Err(e)
//}
//}

//Ok(Object::Number(numbers.first().unwrap().clone()))
//}

//fn subtract(args: Vec<Object>, env: &mut Env) -> Object {
//for object in args.iter() {

//}
//}

//fn get_number(object: &Object) -> Result<Number, String> {
//if let &Object::Number(ref n) = object {
//Ok(n.clone())
//} else {
//Err(format!("Object {:?} is not a number.", object))
//}
//}

//fn define(args: Vec<Object>, env: &mut Env) ->

//fn to_str(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
//if args.len() > 1 {
//Err("Invalid number of args for to_str. Should be 1.")
//} else {

//}
//}

fn equals(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    if args.len() == 0 {
        Err("Not enough args for =".to_string())
    } else {
        let ref first = args[0];
        if args.iter().all(|o| o == first) {
            Ok(Some(Object::Boolean(Boolean::True)))
        } else {
            Ok(Some(Object::Boolean(Boolean::False)))
        }
    }
}

fn cond(args: Vec<Object>, env: &mut Env) -> Result<Option<Object>, String> {
    if args.len() < 2 {
        Err("Not enough args for cond".to_string())
    } else if args.iter().all(|e| {
        match *e {
            Object::ConditionalCase(_, _) => true,
            _ => false
        }
    }) {
        let valid_else;
        if let Object::ConditionalCase(ref case, _) = args[args.len() - 1] {
            if let Expr::Expr(Object::Symbol(ref name)) = **case {
                if name == "else" {
                    valid_else = true;
                } else {
                    valid_else = false;
                }
            } else {
                valid_else = false;
            }
        } else {
            valid_else = false;
        }
        if valid_else {
            for i in 0..(args.len() - 1) {
                if let Object::ConditionalCase(ref case, ref body) = args[i] {
                    if let Some(Object::Boolean(ref boolean)) = try!(case.eval(env)) {
                        if let Boolean::True = *boolean {
                            return (*body).eval(env);
                        }
                    } else {
                        return Err(format!("Case {:?} does not return a boolean", case));
                    }
                }
            }
            if let Object::ConditionalCase(_, ref body) = args[args.len() - 1] {
                (*body).eval(env)
            } else {
                Err("Conditional evaluation error".to_string())
            }
        } else {
            Err("Invalid final arg for cond".to_string())
        }
    } else {
        Err("Not all arguments of cond are conditional cases".to_string())
    }
}

fn list(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    Ok(Some(Object::List(Box::new(args))))
}

fn cons(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    if !(args.len() == 2) {
        //invalid arg number
        Err("Invalid number of arguments for cons.".to_string())
    } else {
        let first = args.first().unwrap().clone();
        let last = args.last().unwrap().clone();
        if let Object::List(elems) = first {
            //list is in the head position; append the element
            if let Object::List(_) = last {
                Err(format!("Cannot cons two lists."))
            } else {
                let mut tmpvec = *elems.clone();
                tmpvec.push(last.clone());
                Ok(Some(Object::List(Box::new(tmpvec))))
            }
        } else {
            //list or elem is in the tail position; append the list or make a new list
            if let Object::List(elems) = last {
                let mut tmpvec = vec![first];
                for x in elems.iter() {
                    tmpvec.push(x.clone());
                }
                Ok(Some(Object::List(Box::new(tmpvec))))
            } else {
                Ok(Some(Object::List(Box::new(args))))
            }
        }
    }
}

fn print(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    if args.len() != 1 {
        Err("Invalid number of args for print".to_string())
    } else {
        println!("{:?}", args[0]);
        Ok(None)
    }
}

fn exit(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    if args.len() > 1 {
        Err("Invalid number of arguments for exit.".to_string())
    } else if args.len() == 1 {
        // PLACEHOLDER
        Ok(Some(Object::Exit(Some(format!("{:?}", args[0])))))
    } else {
        Ok(Some(Object::Exit(None)))
    }
}
