use std::{clone, rc::Rc};

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons,Nil};

fn main(){
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));

    println!("Count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("counter after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}