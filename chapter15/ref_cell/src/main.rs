use std::cell::RefCell;
use std::rc::Rc;

pub mod reference_counter;
pub mod ref_cell_list;

// use reference_counter::List::{Cons, Nil};
use ref_cell_list::List::{Cons, Nil};

fn main() {
    let x = RefCell::new(90);
    {
    let mut y = x.borrow_mut();
    *y += 9;

    println!("{y}");
    }
    
    let z = x.borrow();

    println!("X after mutation is {}", *z);

    /*
        In the code below, when trying to create list b that has list a within,
        Box takes ownership of a and we are not able to create c.Because a has been moved.

        There is another solution to work around by using references and hence depending
        on the implememntation of lifetimes.But this is a cumbersome.

    */ 

    /*
        Code below won't compile.

            let a = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

            let b = Cons(5, Box::new(a));

            let c = Cons(10, Box::new(a));
    
    */

    /*
        instead, we will use another type called Rc<T>
        it provides multiple immutable references of a specific resource
        let's see hpw to implement that.
    */

    // let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    // println!("Now the ref count is {}", Rc::strong_count(&a));   // 1

    // let b = Cons(5, Rc::clone(&a));
    // println!("Now the ref count is {}", Rc::strong_count(&a)); // 2

    // let c = Cons(10, Rc::clone(&a));
    // println!("Now the ref count is {}", Rc::strong_count(&a)); // 3

    /*  
        instead of taking ownership of a, we will clone the Rc<List> that a is holding.
        Here we are cloning a ref to a.
        and to clone a ref to a , a itself has to be of type Rc,
        that is why we are adding Rc::new(a)
        The implementation of Rc::clone doesn’t make a deep copy of all the data like 
        most types’ implementations of clone do. The call to Rc::clone only 
        increments the reference count, which doesn’t take much time. 
        Deep copies of data can take a lot of time. 
    */ 

    // dbg!(&a);
    // dbg!(&b);
    // dbg!(&c);

    /*
        We can detect the effect of Rc::clone() by using Rc::strong_count(&a).
        Note that in order to get benefits of the ref counting, all the operations
        need to be performed on a type Rc<T>
        So, we select the data we need it to be shared and add it to Rc<T>
    */

    
let value = Rc::new(RefCell::new(5));

let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

let b = Cons(Rc::new(RefCell::new(5)), Rc::clone(&a));
let c = Cons(Rc::new(RefCell::new(9)), Rc::clone(&a));
println!("===============================Before the mutation=======================");

dbg!(&a);
dbg!(&b);
dbg!(&c);

// let x = Rc::new(RefCell::new(10));
// println!("{x:?}");
// *x.borrow_mut() += 10;

*value.borrow_mut() += 99;

println!("{x:?}");

println!("===============================After the mutation=======================");

dbg!(&a);
dbg!(&b);
dbg!(&c);


}

