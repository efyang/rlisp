use std::collections::HashMap;
use stdlisp::BASE_FUNCTIONS;
use std::sync::Arc;

pub type LispFn = fn(Vec<Object>, &mut Env) -> Result<Option<Object>, String>;

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
}

#[derive(Debug, Clone)]
pub enum Object {
    Symbol(String),
    String(String),
    Number(Number),
    List(Box<Vec<Object>>),
    Exit(Option<String>)
}

impl Object {
    pub fn unwrap_symbol(&self) -> Option<&str> {
        match *self {
            Object::Symbol(ref name) => Some(name),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    Int(isize),
    Float(f64)
}

#[derive(Clone)]
pub struct Env<'a> {
    pub functions: Vec<Function<'a>>,
    pub variables: HashMap<String, Object>
}

#[allow(dead_code)]
impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            functions: BASE_FUNCTIONS.to_vec(),
            variables: HashMap::new(),
        }
    }
    pub fn with_functions(functions: Vec<Function<'a>>) -> Env {
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
    pub fn add_function(&mut self, function: Function<'a>) {
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

#[derive(Clone)]
pub struct Function<'a> {
    pub name: &'a str,
    pub procedure: Arc<LispFn>,
}

//TODO
impl<'a> Function<'a> {
    fn from_exprs(name: &'a str, declaration_vars: &[Expr], body: &[Expr]) -> Result<Function<'a>, String> {
        let unknown_fn = |args: Vec<Object>, env: &mut Env| -> Result<Option<Object>, String> {
            Ok(None)
        };

        Ok(Function {
            name: name,
            procedure: Arc::new(unknown_fn as LispFn),
        })
    }
}
