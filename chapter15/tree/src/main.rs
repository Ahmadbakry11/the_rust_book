use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>
}


impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            children: RefCell::new(vec![])
        }
    }

    fn push_node(&self, node: Rc<Node>) {
        self.children.borrow_mut().push(Rc::clone(&node));
    }
}

fn main() {
    let leaf = Rc::new(Node::new(9));
    let branch = Rc::new(Node::new(10));
    branch.push_node(leaf);

    println!("Now the branch children are: {:?}", branch.children);

}
