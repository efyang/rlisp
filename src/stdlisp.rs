#![allow(dead_code)]
use data::*;

pub static BASE_FUNCTIONS: &'static [Function<'static>] = &[
    //Function {name: "+", procedure: &(add as LispFn)},
    //Function {name: "-", procedure: &(subtract as fn(Vec<Object>, &mut Env) -> Object)},
    Function {name: "list", procedure: &(list as LispFn)},
    Function {name: "cons", procedure: &(cons as LispFn)},
    Function {name: "exit", procedure: &(exit as LispFn)},
];

//use Result for all of these functions to catch runtime errors

//fn add(args: Vec<Object>, _: &mut Env) -> Result<OptionObject, String> {
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

fn list(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    Ok(Some(Object::List(Box::new(args))))
}

fn cons(args: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    if !(args.len() == 2) {
        //invalid arg number
        Err(format!("Invalid number of arguments for cons."))
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

fn exit(_: Vec<Object>, _: &mut Env) -> Result<Option<Object>, String> {
    Err("rlisp exited successfully.".to_string())
}
