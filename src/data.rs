use std::collections::HashMap;
use stdlisp::BASE_FUNCTIONS;
use std::sync::Arc;
use std::hash::{Hash, Hasher};

pub type BuiltinFn = fn(Vec<Object>, &mut Env) -> Result<Option<Object>, String>;

#[derive(Debug, Clone)]
pub enum Inhibit {
    Continue,
    Stop(Option<String>),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Expr(Object),
    Exprs(Box<Vec<Expr>>),
}

impl Expr {
    pub fn unwrap_expr(&self) -> Option<&Object> {
        match *self {
            Expr::Expr(ref object) => Some(object),
            _ => None,
        }
    }

    pub fn replace_all(&self, replacement_hm: &HashMap<&Object, &Object>) -> Expr {
        match *self {
            Expr::Expr(ref object) => {
                if let Some(replacement) = replacement_hm.get(object) {
                    Expr::Expr((*replacement).clone())
                } else {
                    (*self).clone()
                }
            }
            Expr::Exprs(ref exprs) => {
                Expr::Exprs(Box::new((*exprs)
                                     .iter()
                                     .map(|e| e.replace_all(replacement_hm))
                                     .collect::<Vec<_>>()))
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Object {
    Symbol(String),
    String(String),
    Number(Number),
    List(Box<Vec<Object>>),
    Exit(Option<String>)
}

//impl Object {
    //pub fn unwrap_symbol(&self) -> Option<&str> {
        //match *self {
            //Object::Symbol(ref name) => Some(name),
            //_ => None,
        //}
    //}
//}

#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64, String) // store the string for hashing and partialeq
}

impl Hash for Number {
    fn hash<SipHasher>(&self, state: &mut SipHasher) where SipHasher: Hasher {
        match *self {
            Number::Int(i) => {
                i.hash(state);
            }
            Number::Float(f, ref fstr) => {
                fstr.hash(state);
            }
        }
    }
}

impl Eq for Number {}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        match (self, other) {
            (&Number::Int(i), &Number::Int(oi)) => {
                i == oi
            }
            (&Number::Float(_, ref f), &Number::Float(_, ref of)) => {
                f == of
            }
            _ => false
        }
    }
}


#[derive(Clone)]
pub struct Env {
    pub functions: Vec<Function>,
    pub variables: HashMap<String, Object>
}

#[allow(dead_code)]
impl Env {
    pub fn new() -> Env {
        Env {
            functions: BASE_FUNCTIONS.to_vec(),
            variables: HashMap::new(),
        }
    }
    pub fn with_functions(functions: Vec<Function>) -> Env {
        let mut funcs: Vec<Function> = BASE_FUNCTIONS.to_vec().clone();
        funcs.clone_from_slice(&functions);
        Env {
            functions: funcs,
            variables: HashMap::new(),
        }
    }
    pub fn functions(&self) -> Vec<Function> {
        self.functions.clone()
    }
    pub fn variables(&self) -> HashMap<String, Object> {
        self.variables.clone()
    }
    pub fn get_variable(&self, varname: &String) -> Object {
        self.variables[varname].clone()
    }
    pub fn var_exists(&self, varname: &String) -> bool {
        self.variables.contains_key(varname)
    }
    pub fn add_variable(&mut self, var: String, value: Object) {
        if self.variables().keys().any(|x| x == &var) {
            panic!("Variable {:?} cannot be set because it already exists in current env.");
        } else {
            self.variables.insert(var, value);
        }
    }
    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }
    pub fn set_variable(&mut self, var: String, value: Object) {
        if !self.variables().keys().any(|x| x == &var) {
            panic!("Variable {:?} cannot be changed because it does not exist.");
        } else {
            *self.variables.get_mut(&var).unwrap() = value;
        }
    }
}

pub enum LispFn {
    Builtin(BuiltinFn),
    UserDef(Vec<Object>, Vec<Expr>), // input vars, body
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub procedure: Arc<LispFn>,
}

//TODO
impl Function {
    pub fn from_exprs(name: &str, declaration_vars: &[Expr], body: &[Expr]) -> Result<Function, String> {
        let mut vars = Vec::new();
        for var in declaration_vars {
            match var.unwrap_expr() {
                Some(obj) => {
                    if let Object::Symbol(_) = *obj {
                        vars.push((*obj).clone());
                    } else {
                        return Err(format!("Invalid var name {:?}", var));
                    }
                }
                None => return Err(format!("Invalid var name {:?}", var)),
            }
        }
        Ok(Function {
            name: name.to_string(),
            procedure: Arc::new(LispFn::UserDef(
                vars,
                body.iter().map(|ref e| (*e).clone()).collect::<Vec<Expr>>()
            )),
        })
    }
}
