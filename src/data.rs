use std::collections::HashMap;
use stdlisp::BASE_FUNCTIONS;

pub type LispFn = fn(Vec<Object>, &mut Env) -> Result<Option<Object>, String>;

#[derive(Debug, Clone)]
pub enum Expr {
    Expr(Object),
    Exprs(Box<Vec<Expr>>),
}

#[derive(Debug, Clone)]
pub enum Object {
    Symbol(String),
    String(String),
    Number(Number),
    List(Box<Vec<Object>>),
}

#[derive(Debug, Clone)]
pub enum Number {
    Int(isize),
    Float(f64),
}

#[derive(Clone)]
pub struct Env<'a> {
    pub functions: Vec<Function<'a>>,
    pub variables: HashMap<String, Object>,
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
    //pub fn change_variable(&mut self, var: String, value: Object) {
        //if !self.variables().keys().any(|x| x == &var) {
            //panic!("Variable {:?} cannot be changed because it does not exist.");
        //} else {
            //self.variables[&var] = value;
        //}
    //}
}

#[derive(Clone)]
pub struct Function<'a> {
    pub name: &'static str,
    pub procedure: &'a LispFn,
}
