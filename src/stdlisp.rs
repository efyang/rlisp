#![allow(dead_code)]
use data::*;
use std::sync::Arc;
use eval::Eval;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign, RemAssign};

macro_rules! generate_base_fn {
    ($fnname:expr, $name:ident) => {
        ($fnname, Function {procedure: Arc::new(LispFn::Builtin(BuiltinFn::new($fnname, $name)))})
    }
}

macro_rules! generate_normal_base_fn {
    ($name:ident) => {generate_base_fn!(stringify!($name), $name)}
}

lazy_static! {
    pub static ref BASE_FUNCTIONS: [(&'static str, Function); 14] = [
        generate_normal_base_fn!(list),
        generate_normal_base_fn!(cons),
        generate_normal_base_fn!(print),
        generate_normal_base_fn!(exit),
        generate_normal_base_fn!(cond),
        generate_base_fn!("=", equals),
        generate_normal_base_fn!(and),
        generate_normal_base_fn!(or),
        generate_normal_base_fn!(not),
        generate_base_fn!("+", add),
        generate_base_fn!("-", sub),
        generate_base_fn!("*", mul),
        generate_base_fn!("/", div),
        generate_base_fn!("%", rem),
    ];
}

macro_rules! gen_math_func {
    ( $name:ident, $op:ident ) => {
        fn $name(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
            if args.iter().all(|o| {if let &Object::Number(_) = o {true} else {false}}) && args.len() > 1 {
                let mut base = args[0].unwrap_number().unwrap().to_owned();
                for arg in args.iter().skip(1).map(|o| (*o.unwrap_number().unwrap()).clone()) {
                    base.$op(arg);
                }
                Ok(Some(Object::Number(base)))
            } else {
                Err(format!("Invalid or too little args for function {}", stringify!($name)))
            }
        }
    }
}

gen_math_func!(add, add_assign);
gen_math_func!(sub, sub_assign);
gen_math_func!(mul, mul_assign);
gen_math_func!(div, div_assign);
gen_math_func!(rem, rem_assign);

macro_rules! x_only {
    ( $item_ident:ident; $qualifier:ident; $args:expr; $operation:block ) => {
        if $qualifier(&$args) {
            $item_ident = $args.iter().map(|obj| obj.unwrap_boolean().unwrap()).collect::<Vec<_>>();
            $operation
        } else {
            Err(format!("Function only usable on {} items", stringify!($qualifier)))
        }
    }
}

fn all_boolean(args: &[Object]) -> bool {
    args.iter().all(|o| {
        if let &Object::Boolean(_) = o {
            true
        } else {
            false
        }
    })
}

fn and(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    let items;
    x_only!(items; all_boolean; args;
            {
                if items.iter().all(|&o| o == &Boolean::True) {
                    Ok(Some(true.into()))
                } else {
                    Ok(Some(false.into()))
                }
            })
}

fn or(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    let items;
    x_only!(items; all_boolean; args;
            {
                if items.iter().any(|&o| o == &Boolean::True) {
                    Ok(Some(true.into()))
                } else {
                    Ok(Some(false.into()))
                }
            })
}

fn not(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    let items;
    x_only!(items; all_boolean; args;
            {
                if items.len() != 1 {
                    Err("Invalid number of args for logical not".to_string())
                } else {
                    let notted: bool = (*items[0]).clone().into();
                    Ok(Some((!notted).into()))
                }
            })
}

//fn xor(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
//unimplemented!()
//}

//fn bit_and(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
//unimplemented!()
//}

//fn bit_or(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
//unimplemented!()
//}

//fn bit_not(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
//unimplemented!()
//}

//fn bit_xor(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
//unimplemented!()
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
