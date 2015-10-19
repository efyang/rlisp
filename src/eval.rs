use data::*;

impl Expr {
    pub fn eval(&self, env: &mut Env) -> Result<Option<Object>, String> {
        if let &Expr::Exprs(ref exprs) = self {
            let mut evaluated: Vec<Object> = Vec::new();
            for expr in exprs.iter() {
                let evalresult = expr.eval(env);
                match evalresult {
                    Ok(Some(r)) => evaluated.push(r),
                    Ok(None) => {},
                    Err(_) => return evalresult,
                }
            }
            let (head, tail): (&Object, &[Object]) = evaluated.split_first().unwrap();
            let function_name = head;
            let args = tail.to_vec();
            if let &Object::Symbol(ref fn_name) = function_name {
                eval_function(fn_name, args, env)
            } else {
                Err(format!("Invalid function name {:?}", function_name))
            }
        } else {
            if let &Expr::Expr(ref object) = self {
                Ok(Some(object.clone()))
            } else {
                Err(format!("Failed to eval {:?}", self))
            }
        }
    }
}

fn eval_function(function_name: &String, args: Vec<Object>, env: &mut Env) -> Result<Option<Object>, String> {
    let function = match_first_function(function_name, env.functions.clone());
    if function.is_ok() {
        let evaluated = (function.ok().unwrap().procedure)(args, env);
        match evaluated {
            Ok(Some(r)) => Ok(Some(r)),
            Ok(None) => Ok(None),
            Err(e) => Err(e)
        }
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
