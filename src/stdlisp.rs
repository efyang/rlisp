#![allow(dead_code)]
use data::*;

pub static BASE_FUNCTIONS: &'static [Function<'static>] = &[
    Function {name: "+", procedure: &(add as LispFn)},
    //Function {name: "-", procedure: &(subtract as fn(Vec<Object>, &mut Env) -> Object)},
    Function {name: "list", procedure: &(list as LispFn)},
    Function {name: "cons", procedure: &(cons as LispFn)},
    Function {name: "exit", procedure: &(exit as LispFn)},
];

fn add(args: Vec<Object>, _: &mut Env) -> Object {
    let numbers = args.iter()
        .map(|x| get_number(x))
        .collect::<Vec<Number>>();

    Object::Number(numbers.first().unwrap().clone())
}

//fn subtract(args: Vec<Object>, env: &mut Env) -> Object {
    //for object in args.iter() {
    
    //}
//}

fn get_number(object: &Object) -> Number {
    if let &Object::Number(ref n) = object {
        n.clone()
    } else {
        panic!("Object {:?} is not a number.", object);
    }
}

//fn define(args: Vec<Object>, env: &mut Env) -> 

fn list(args: Vec<Object>, _: &mut Env) -> Object {
    Object::List(Box::new(args))
}

fn cons(args: Vec<Object>, _: &mut Env) -> Object {
    if !(args.len() == 2) {
        //invalid arg number
        panic!("Invalid number of arguments for cons.")
    } else {
        let first = args.first().unwrap().clone();
        let last = args.last().unwrap().clone();
        if let Object::List(elems) = first {
            //list is in the head position; append the element
            if let Object::List(_) = last {
                panic!("Cannot cons two lists.");
            } else {
                let mut tmpvec = *elems.clone();
                tmpvec.push(last.clone());
                Object::List(Box::new(tmpvec))
            }
        } else {
            //list or elem is in the tail position; append the list or make a new list
            if let Object::List(elems) = last {
                let mut tmpvec = vec![first];
                for x in elems.iter() {
                    tmpvec.push(x.clone());
                }
                Object::List(Box::new(tmpvec))
            } else {
                Object::List(Box::new(args))
            }
        }    
    } 
}

fn exit(args: Vec<Object>, _: &mut Env) -> Object {
    panic!("rlisp exited.");
}
