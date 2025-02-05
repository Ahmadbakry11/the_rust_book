use std::cell::{Ref, RefCell};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, x) => Some(x),
            Nil => None,
        }
    }
}

fn main() {
    // let x = Rc::new(RefCell::new(9));

    // let y = Rc::clone(&x);
    // let z = Rc::clone(&x);

    // *x.borrow_mut() += 90;

    // println!("Now the values became: {}, {}, {}", *x.borrow(), *y.borrow(), *z.borrow());


    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("an initial count of a is: {}", Rc::strong_count(&a));   //Rc: a count is 1
    println!("The next iem in a is {:?}", a.tail());
    
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));  //Rc: b count is 1 & a count is 2
    println!("Now the count of a is: {}", Rc::strong_count(&a)); // 2
    println!("Now the count of b is: {}", Rc::strong_count(&b)); // 1
    println!("The next iem in b is {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);  //Now b count is 2
    }

    println!("Now the count of a is: {}", Rc::strong_count(&a));  // 2
    println!("Now the count of b is: {}", Rc::strong_count(&b));  // 2
    println!("The next iem in a is {:?}", a.tail());
}
