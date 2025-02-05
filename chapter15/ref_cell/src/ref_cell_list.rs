use std::{rc::Rc, cell::RefCell};
/*
    since Refcells allow you to mutate data even if there are immutable refs to that data.
    Let's upgrade our List<T> to have the ability to mutate data inside.Like incrementing values
    by one.

    We will assume that multiple refs are trying to muatate that data.
    Since RefCell<T> gives us the ability to have only one mutable ref to the data
    and Rc<T> gives us the ability to have multiple immutable refs to that data of type T.

    We can combine both to acheive our target 
*/ 

#[derive(Debug)]
pub enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}


// let value = Rc::new(RefCell::new(5));

// let a = Rc::new(Cons(&value, Rc::new(Nil)));