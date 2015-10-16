use data::*;
use std::thread;

impl Expr {
    pub fn eval(&self, env: &mut Env) -> Result<Object, String> {
        if let &Expr::Exprs(ref exprs) = self {
            //let evaluated = exprs
                //.iter()
                //.map(|ref mut x| x.eval(env))
                //.collect::<Vec<Object>>();
            let mut evaluated: Vec<Object> = Vec::new(); for expr in exprs.iter() {
                let evalresult = expr.eval(env);
                match evalresult {
                    Ok(r) => evaluated.push(r),
                    Err(_) => return evalresult,
                }
            }
            let splitted: (&Object, &[Object]) = evaluated.split_first().unwrap();
            let function_name = splitted.0;
            let args = splitted.1.to_vec();
            if let &Object::Symbol(ref fn_name) = function_name {
                eval_function(fn_name, args, env)
            } else {
                Err(format!("Invalid function name {:?}", function_name))
            }
        } else {
            if let &Expr::Expr(ref object) = self {
                Ok(object.clone())
            } else {
                Err(format!("Failed to eval {:?}", self))
            }
        }
    }
}

fn eval_function(function_name: &String, args: Vec<Object>, env: &mut Env) -> Result<Object, String> {
    let function = match_first_function(function_name, env.functions.clone());
    if function.is_ok() {
        Ok((function.ok().unwrap().procedure)(args, env))
    } else {
        Err(function.err().unwrap())
    }
}

fn match_first_function<'a>(function_name: &String, functions: Vec<Function<'a>>) -> Result<Function<'a>, String> {
    if functions.is_empty() {
        return Err(format!("No such function {:?}", function_name));
    }
    let (head, tail): (&Function, &[Function]) = functions.split_first().unwrap();
    let current = head;
    if &current.name.to_string() == function_name {
        Ok(current.clone())
    } else {
        match_first_function(function_name, tail.to_vec())
    }
}
