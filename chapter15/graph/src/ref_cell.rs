use std::cell::RefCell;
/*
    Dealing with cell is not an issue when the inner data does implement the Copy Trait
    specially if coying that data will not have a hit on the performance.

    But if we are dealing with data the does not implement the Copy Trait like String
    the compiler will complain
    ==================================================
    | the trait bound `String: Copy` is not satisfied |
    | the trait `Debug` is implemented for `Cell<T>`  |
    | required for `Cell<String>` to implement `Debug`|
    ==================================================

    RefCell is a smart pointer that allows IM design pattern
    but does not prevent the data that are not Copyable.

    RefCell has two methds borrow and borrow_mut

    borrow(): will give you a shared ref.
    borrow_mut() will give you an exclusive or mutable_ref
*/ 

#[derive(Debug)]
pub struct RefCellNode<'a> {
    pub value: RefCell<String>,
    pub adjacent: Vec<&'a RefCellNode<'a>>
}

impl<'a> RefCellNode<'a> {
    pub fn new(value: String, adjacent: Option<Vec<&'a RefCellNode<'a>>>) -> Self {
        Self {
            value: RefCell::new(value),
            adjacent: adjacent.unwrap_or(vec![]),
        }
    }

    pub fn add_urgency(node: &RefCellNode) {
        let mut cur_val = node.value.borrow_mut();
        cur_val.push('!');
        /*
          Calling : for adj in node.adjacent {
          will take ownership of node.adjacent because internally it calls into_iter()
          so we call .iter();
        */ 
        for adj in node.adjacent.iter() {
            Self::add_urgency(adj);
        }
    }
}
