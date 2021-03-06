use std::collections::HashMap;
use stdlisp::BASE_FUNCTIONS;
use std::sync::Arc;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign, RemAssign};
use std::ops::{Add, Sub, Mul, Div, Rem};

#[derive(Debug, Clone)]
pub enum Inhibit {
    Continue,
    Stop(Option<String>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
            Expr::Expr(Object::ConditionalCase(ref case, ref body)) => {
                Expr::Expr(Object::ConditionalCase(
                        Box::new((**case).replace_all(replacement_hm)),
                        body.iter().map(|e| (*e).replace_all(replacement_hm)).collect::<Vec<_>>()
                        ))
            }
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
    Boolean(Boolean),
    List(Box<Vec<Object>>),
    ConditionalCase(Box<Expr>, Vec<Expr>),
    Function(Function),
    Exit(Option<String>)
}

impl Object {
    //pub fn unwrap_symbol(&self) -> Option<&str> {
        //match *self {
            //Object::Symbol(ref name) => Some(name),
            //_ => None,
        //}
    //}
    pub fn unwrap_boolean(&self) -> Option<&Boolean> {
        match *self {
            Object::Boolean(ref boolean) => Some(boolean),
            _ => None,
        }
    }
    pub fn unwrap_number(&self) -> Option<&Number> {
        match *self {
            Object::Number(ref number) => Some(number),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Boolean {
    True,
    False
}

impl Into<bool> for Boolean {
    fn into(self) -> bool {
        match self {
            Boolean::True => true,
            Boolean::False => false,
        }
    }
}

impl Into<Boolean> for bool {
    fn into(self) -> Boolean {
        if self {
            Boolean::True
        } else {
            Boolean::False
        }
    }
}

impl Into<Object> for bool {
    fn into(self) -> Object {
        Object::Boolean(self.into())
    }
}

#[derive(Debug, Clone)]
pub enum Number {
    Int(i64),
    Float(f64, String) // store the string for hashing and partialeq
}

macro_rules! gen_assign_fn {
    ( $assign:ty, $fnname:ident, $op:ident ) => {
        impl $assign for Number {
            fn $fnname(&mut self, other: Number) {
                match *self {
                    Number::Int(i) => {
                        match other {
                            Number::Int(oi) => {
                                *self = Number::Int(i.$op(oi));
                            }
                            Number::Float(of, _) => {
                                let after_op = (i as f64).$op(of);
                                *self = Number::Float(after_op, after_op.to_string());
                            }
                        }
                    }
                    Number::Float(f, _) => {
                        match other {
                            Number::Int(oi) => {
                                let after_op = f.$op(oi as f64);
                                *self = Number::Float(after_op, after_op.to_string());
                            }
                            Number::Float(of, _) => {
                                let after_op = f.$op(of);
                                *self = Number::Float(after_op, after_op.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}

gen_assign_fn!(AddAssign, add_assign, add);
gen_assign_fn!(SubAssign, sub_assign, sub);
gen_assign_fn!(MulAssign, mul_assign, mul);
gen_assign_fn!(DivAssign, div_assign, div);
gen_assign_fn!(RemAssign, rem_assign, rem);

impl Hash for Number {
    fn hash<SipHasher>(&self, state: &mut SipHasher) where SipHasher: Hasher {
        match *self {
            Number::Int(i) => {
                i.hash(state);
            }
            Number::Float(_, ref fstr) => {
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
    pub variables: HashMap<String, Object>
}

#[allow(dead_code)]
impl Env {
    pub fn new() -> Env {
        Env {
            variables: {
                let mut hm = HashMap::new();
                for &(name, ref func) in BASE_FUNCTIONS.iter() {
                    hm.insert(name.to_string(), Object::Function((*func).clone()));
                }
                hm
            }
        }
    }
    pub fn with_functions(functions: Vec<(String, Function)>) -> Env {
        Env {
            variables: {
                let mut hm = HashMap::new();
                for &(name, ref func) in BASE_FUNCTIONS.iter() {
                    hm.insert(name.to_string(), Object::Function((*func).clone()));
                }
                for (name, func) in functions {
                    hm.insert(name, Object::Function(func));
                }
                hm
            }
        }
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
            panic!(format!("Variable {:?} cannot be set because it already exists in current env.", var));
        } else {
            self.variables.insert(var, value);
        }
    }
    pub fn set_variable(&mut self, var: String, value: Object) {
        if !self.variables().keys().any(|x| x == &var) {
            panic!("Variable {:?} cannot be changed because it does not exist.");
        } else {
            *self.variables.get_mut(&var).unwrap() = value;
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum LispFn {
    Builtin(BuiltinFn),
    UserDef(Vec<Object>, Vec<Expr>), // input vars, body
}

pub type BuiltinFnSignature = fn(Vec<Object>, &mut Env) -> Result<Option<Object>, String>;

pub struct BuiltinFn {
    name: String,
    inner: BuiltinFnSignature
}

impl BuiltinFn {
    pub fn new(name: &str, func: BuiltinFnSignature) -> BuiltinFn {
        BuiltinFn {
            name: name.to_string(),
            inner: func
        }
    }
    fn name(&self) -> &str {
        &self.name
    }
    pub fn inner(&self) -> BuiltinFnSignature {
        self.inner
    }
}

impl Hash for BuiltinFn {
    fn hash<SipHasher>(&self, state: &mut SipHasher) where SipHasher: Hasher {
        self.name().hash(state);
    }
}

impl Eq for BuiltinFn {}

impl PartialEq for BuiltinFn {
    fn eq(&self, other: &BuiltinFn) -> bool {
        self.name() == other.name()
    }
}

impl Clone for BuiltinFn {
    fn clone(&self) -> Self {
        BuiltinFn {
            name: self.name.clone(),
            inner: self.inner
        }
    }
}

impl fmt::Debug for BuiltinFn {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "BuiltinFn {{name: {}}}", self.name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Function {
    pub procedure: Arc<LispFn>,
}

//TODO
impl Function {
    pub fn from_exprs(declaration_vars: &[Expr], body: &[Expr]) -> Result<Function, String> {
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
            procedure: Arc::new(LispFn::UserDef(
                vars,
                body.iter().map(|ref e| (*e).clone()).collect::<Vec<Expr>>()
            )),
        })
    }
}
