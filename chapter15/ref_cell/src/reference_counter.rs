/*
    We use the Rc<T> type when we want to allocate some data on the heap for 
    multiple parts of our program to read and we can’t determine 
    at compile time which part will finish using the data last. 
    If we knew which part would finish last, we could just make that part the data’s owner, 
    and the normal ownership rules enforced at compile time would take effect.
*/

use std::rc::Rc; 

#[derive(Debug)]
pub enum List {
    Cons(i32, Rc<List>),
    Nil,
}



