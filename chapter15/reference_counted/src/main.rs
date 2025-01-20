use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
   let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
   println!("The ref count after creating a: {}", Rc::strong_count(&a));

   let b = Cons(4, Rc::clone(&a));
   println!("The ref count after creating b: {}", Rc::strong_count(&a));

   {
        let c = Cons(10, Rc::clone(&a));
        println!("The ref count after creating c: {}", Rc::strong_count(&a));
   }

   println!("The ref count after c gets out of scope: {}", Rc::strong_count(&a));

}
