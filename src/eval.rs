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

pub trait Eval {
    fn eval(&self, _: &mut Env) -> Result<Option<Object>, String>;
}

impl Eval for Vec<Expr> {
    fn eval(&self, env: &mut Env) -> Result<Option<Object>, String> {
        if self.len() != 0 {
            if self.len() != 1 {
                for i in 0..self.len() - 1 {
                    try!(self[i].eval(env));
                }
            }
            self[self.len() - 1].eval(env)
        } else {
            Err("Cannot eval empty list of exprs".to_string())
        }
    }
}

impl Eval for Expr {
    fn eval(&self, env: &mut Env) -> Result<Option<Object>, String> {
        match *self {
            Expr::Exprs(ref exprs) => {
                let (orig_head, tail): (&Expr, &[Expr]) = exprs.split_first().unwrap();
                let head;
                if let Expr::Exprs(_) = *orig_head {
                    head = Expr::Expr(try!((*orig_head).eval(env)).unwrap());
                } else {
                    head = (*orig_head).clone();
                }
                if let Expr::Expr(Object::Symbol(ref function_name)) = head {
                    let args = tail.to_vec();
                    if function_name == "define" {
                        let (first, rest) = args.split_first().unwrap();
                        match *first {
                            Expr::Expr(Object::Symbol(ref var)) => {
                                define_variable(var, rest, env)
                            },
                            Expr::Expr(ref tried_ident) => {
                                Err(format!("Invalid variable identifier \"{:?}\"", tried_ident))
                            },
                            Expr::Exprs(ref fndef) => {
                                define_function(fndef, rest, env)
                            },
                        }
                    } else if function_name == "lambda" {
                        let (first, rest) = args.split_first().unwrap();
                        if let Expr::Exprs(ref fndef) = *first {
                            Ok(Some(Object::Function(try!(Function::from_exprs(fndef, rest)))))
                        } else {
                            Err("Invalid lambda function".to_string())
                        }
                    } else {
                        eval_function_named(function_name, &args, env)
                    }
                } else if let Expr::Expr(Object::Function(ref function)) = head {
                    eval_function((*function).clone(), tail, env)
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

fn define_variable(var: &str, args: &[Expr], env: &mut Env) -> Result<Option<Object>, String> {
    match args.last().unwrap().clone().eval(env) {
        Ok(Some(value)) => {
            env.add_variable(var.to_string(), value);
            return Ok(None);
        },
        Ok(None) => return Err("Cannot set variable to nonetype".to_string()),
        Err(e) => return Err(e),
    };
}

fn define_function(declaration: &[Expr], args: &[Expr], env: &mut Env) -> Result<Option<Object>, String> {
    if args.len() > 1 {
        return Err(format!("Function body of function {:?} too short", declaration[0]));
    } else {
        if let Expr::Expr(Object::Symbol(ref fn_name)) = declaration[0] {
            let fnargs = &declaration[1..declaration.len()];
            let body = args;
            let function = try!(Function::from_exprs(fnargs, body));
            env.add_variable(fn_name.to_string(), Object::Function(function));
            return Ok(None);
        } else {
            return Err(format!("Invalid function identifier {:?}", declaration[0]));
        }
    }
}

fn eval_function_named(function_name: &str, args: &[Expr], env: &mut Env) -> Result<Option<Object>, String> {
    let function = match_first_function(function_name, env.variables());
    if function.is_ok() {
        eval_function(function.ok().unwrap(), args, env)
    } else {
        Err(function.err().unwrap())
    }
}

fn eval_function(function: Function, args: &[Expr], env: &mut Env) -> Result<Option<Object>, String> {
    let mut evaled_args: Vec<Object> = Vec::new();
    for expr in args.iter() {
        let evalresult = expr.eval(env);
        match evalresult {
            Ok(Some(Object::Exit(_))) => return evalresult,
            Ok(Some(r)) => evaled_args.push(r),
            Ok(None) => {},
            Err(_) => return evalresult,
        }
    }
    match *function.procedure {
        LispFn::Builtin(ref innerfn) => {
            let evaluated = (innerfn.inner())(evaled_args, env);
            match evaluated {
                Ok(Some(r)) => Ok(Some(r)),
                Ok(None) => Ok(None),
                Err(e) => Err(e)
            }
        }
        LispFn::UserDef(ref vars, ref body) => {
            if evaled_args.len() != vars.len() {
                Err(format!("Function {:?} run with {} args; should be run with {} args", function, evaled_args.len(), vars.len()))
            } else {
                let mut var_mappings = HashMap::new();
                for (var, var_mapping) in vars.iter().zip(evaled_args.iter()) {
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
}

fn match_first_function<'a>(function_name: &str, vars: HashMap<String, Object>) -> Result<Function, String> {
    for (var, object) in vars.iter() {
        if var == function_name {
            if let &Object::Function(ref func) = object {
                return Ok((*func).clone());
            }
        }
        
    }
    Err(format!("No such function {:?}", function_name))
}
