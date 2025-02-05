use std::cell::Cell;
// Cell allows you to mutate the value but never have a shared ref to that cell
// you can either mutate the value or get a copy of the value in that cell.
// So, data contained should have to implemnt The copy trait like i32 and &str
// and unlike String

#[derive(Debug)]
pub struct Node<'a> {
    pub value: Cell<i32>,
    pub adjacent: Vec<&'a Node<'a>>
}

impl<'a> Node<'a> {
    pub fn new(value: i32, adjacent: Option<Vec<&'a Node<'a>>>) -> Self {
        Self {
            value: Cell::new(value),
            adjacent: adjacent.unwrap_or(vec![]),
        }
    }

    pub fn add_one(node: &Node) {
        let cur_val = node.value.get();
        node.value.set(cur_val + 1);
        /*
          Calling : for adj in node.adjacent {
          will take ownership of node.adjacent because internally it calls into_iter()
          so we call .iter();
        */ 
        for adj in node.adjacent.iter() {
            Self::add_one(adj);
        }
    }
}
