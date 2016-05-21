use data::*;
use parser::{parse_file, parse};
use std::collections::HashMap;

#[cfg(windows)]
const NEWL: &'static str = "{nl}";
#[cfg(not(windows))]
const NEWL: &'static str = "\n";

pub fn run_file(file: &str, env: &mut Env) {
    let parsed = parse_file(file);
    if let Inhibit::Stop(exit_msg) = run_parsed(file.to_string(), parsed, env) {
        if let Some(msg) = exit_msg {
            println!("{}", msg);
        }
    }
}

pub fn run_input(input: String, env: &mut Env) -> Inhibit {
    let parsed = parse(&input);
    run_parsed(input, parsed, env)
}

fn run_parsed(original: String, parsed: Result<Vec<Expr>, String>, env: &mut Env) -> Inhibit {
    match parsed {
        Ok(exprs) => {
            for expr_idx in 0..exprs.len() {
                let evaluated = exprs[expr_idx].eval(env);
                match evaluated {
                    Ok(Some(Object::Exit(s))) => {
                        return Inhibit::Stop(s);
                    },
                    Ok(Some(r)) => {
                        if expr_idx == exprs.len() - 1 {
                            println!("{:?}", r);
                        }
                    },
                    Ok(None) => {},
                    Err(e) => println!("Eval of input: {nl}{nl}{input}{nl} failed with error: {nl}{nl} {e} {nl}", nl = NEWL, input = original, e = e)
                }
            }
        },
        Err(e) => println!("Parsing of input: {nl}{nl}{input}{nl} failed with error: {nl}{nl} {e} {nl}", nl = NEWL, input = original, e = e)
    }
    return Inhibit::Continue;
}

impl Expr {
    pub fn eval(&self, env: &mut Env) -> Result<Option<Object>, String> {
        match *self {
            Expr::Exprs(ref exprs) => {
                let (head, tail): (&Expr, &[Expr]) = exprs.split_first().unwrap();
                if let Expr::Expr(Object::Symbol(ref function_name)) = *head {
                    let args = tail.to_vec();
                    if function_name == "define" {
                        let (first, rest) = args.split_first().unwrap();
                        match *first {
                            Expr::Expr(Object::Symbol(ref var)) => {
                                //define a variable
                                match args.last().unwrap().clone().eval(env) {
                                    Ok(Some(value)) => {
                                        env.add_variable(var.clone(), value);
                                        return Ok(None);
                                    },
                                    Ok(None) => return Err("Cannot set variable to nonetype".to_string()),
                                    Err(e) => return Err(e),
                                };
                            },
                            Expr::Expr(ref tried_ident) => {
                                return Err(format!("Invalid variable identifier \"{:?}\"", tried_ident));
                            },
                            Expr::Exprs(ref fndef) => {
                                //define a function
                                //TODO
                                if args.len() < 2 {
                                    return Err(format!("Function body of function {:?} too short", fndef[0]));
                                } else {
                                    if let Expr::Expr(Object::Symbol(ref fn_name)) = fndef[0] {
                                        let fnargs = &fndef[1..fndef.len()];
                                        let body = rest;
                                        let function = try!(Function::from_exprs(fn_name, fnargs, body));
                                        env.add_function(function);
                                        return Ok(None);
                                    } else {
                                        return Err(format!("Invalid function identifier {:?}", fndef[0]));
                                    }
                                }
                            },
                        }
                        Ok(None)
                    }
                    /*else if function_name == &"quote".to_string() {
                      Ok(Some())
                      }*/
                    else {
                        let mut evaluated: Vec<Object> = Vec::new();
                        for expr in args.iter() {
                            let evalresult = expr.eval(env);
                            match evalresult {
                                Ok(Some(Object::Exit(_))) => return evalresult,
                                Ok(Some(r)) => evaluated.push(r),
                                Ok(None) => {},
                                Err(_) => return evalresult,
                            }
                        }
                        eval_function(function_name, evaluated, env)
                    }
                } else {
                    Err(format!("Invalid function name {:?}", head))
                }
            }
            Expr::Expr(ref object) => {
                match *object {
                    Object::Symbol(ref varname) => {
                        if env.var_exists(varname) {
                            Ok(Some(env.get_variable(varname)))
                        } else {
                            Err(format!("No such variable {}", varname))
                        }
                    },
                    _ => Ok(Some(object.clone()))
                }
            }
        }
    }
}

fn eval_function(function_name: &String, args: Vec<Object>, env: &mut Env) -> Result<Option<Object>, String> {
    let function = match_first_function(function_name, env.functions.clone());
    if function.is_ok() {
        match *function.ok().unwrap().procedure {
            LispFn::Builtin(ref innerfn) => {
                let evaluated = (innerfn)(args, env);
                match evaluated {
                    Ok(Some(r)) => Ok(Some(r)),
                    Ok(None) => Ok(None),
                    Err(e) => Err(e)
                }
            }
            LispFn::UserDef(ref vars, ref body) => {
                if args.len() != vars.len() {
                    Err(format!("Function {:?} run with {} args; should be run with {} args", function_name, args.len(), vars.len()))
                } else {
                    let mut var_mappings = HashMap::new();
                    // arg -> arg_mapping
                    for (var, var_mapping) in vars.iter().zip(args.iter()) {
                        var_mappings.insert(var, var_mapping);
                    }
                    let newbody = body
                        .iter()
                        .map(|ref expr| expr.replace_all(&var_mappings))
                        .collect::<Vec<_>>();
                    if newbody.len() > 1 {
                        let (final_expr, leading_exprs) = body.split_last().unwrap();
                        for expr in leading_exprs {
                            try!(expr.replace_all(&var_mappings).eval(env));
                        }
                        final_expr.replace_all(&var_mappings).eval(env)
                    } else {
                        newbody.first()
                            .unwrap()
                            .replace_all(&var_mappings)
                            .eval(env)
                    }
                }
            }
        }
    } else {
        Err(function.err().unwrap())
    }
}

fn match_first_function<'a>(function_name: &str, functions: Vec<Function>) -> Result<Function, String> {
    for function in functions {
        if function.name == function_name {
            return Ok(function.clone())
        }
    }
    Err(format!("No such function {:?}", function_name))
}
