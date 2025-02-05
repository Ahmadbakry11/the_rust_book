
pub mod cell;
pub mod ref_cell;
use cell::Node;
use ref_cell::RefCellNode;

/*  Assuming we need to implement a node in a Graph:
    We have a Node struct that has a value and a series of ajacent nodes.
    The adjacent field contains refs to the adjacent Nodes

    struct Node<'a> {
        value: i32,
        adjacent: Vec<&'a Node<'a>>
    }

    impl<'a> Node<'a> {
        fn new(value: i32, adjacent: Option<Vec<&'a Node<'a>>>) -> Self {
            Self {
                value,
                adjacent: adjacent.unwrap_or(vec![]),
            }
        }
*/

    
    /*
        Here we need to increment each value in the node and its adjacents by 1
        to mutate the value of the node.we have one of 2 choices: accept a mutable ref to the node parameter
        or full ownership of the node parameter
        To mutate the ref to target node.But the compiler will complain
        when trying to iterate over the adjacents.Because each iteration has an access 
        to an immutable ref that can not be mutated and that can not be mutated
        
        The below code does not compile.

        fn add_one(mut node: Node) { 
            node.value += 1;
            for n in node.adjacent {
                n.value += 1;
            }
        }

        So, no way to have a shared ref like: fn add_one(node: &Node) {

        But the borrow checker will not allow that.Although this is a memory safe.
        Because there is no memory issue behind calling add_one(&b) where b is Node.
        Since there are no Race conditions and no potential for Dangling refs.

        This is a perfect scenario for taking advantage of interior mutability design pattern.

        Lets talk about that.
        Rust borrow checker rules has the following properties:
        1- it rejects any unsafe memory programs.
        2- it can not accept all memory safe programs.

        So to work around that by having an option to mutate values even 
        if those values are immutable refs.
        rust provides a pattern called interior mutability Design Pattern
        To implement The Interior Mutability, we have to use unsafe code blocks

        Rust has 4 smart pointers that implemembt IM under the hood, so when used externally,
        they are safe to use.
        Cell<T>
        RefCell<T>
        RwLock


        # MIT:
        Ref: https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch15-05-interior-mutability.html#:~:text=Interior%20mutability%20is%20a%20design,disallowed%20by%20the%20borrowing%20rules.
        =============
        Interior mutability is a design pattern in Rust that allows you to mutate 
        data even when there are immutable references to that data:
        normally, this action is disallowed by the borrowing rules. 
    */

    /*
    
        Rust have several,types that allow that pattern like Cell<T>, RefCell<T>
        and so on.

        In Rust, the Cell type from the std::cell module provides interior mutability, 
        allowing you to mutate a value even when the Cell itself is immutable. 
        This is useful for simple types like i32, u32, or other Copy types.

        let c = Cell::new(9);   it is an immutable data.
        even though we can mutate the data contained in c
        c.set(10);     Now c carrys 10.

        since it is immutable, we can not reassign it another cell or even another value
        like:

        c = Cell::new(17);    ===Bad

        c = 17;  ===Bad

    */ 
fn main() {

    let a = Node::new(2, None);
    let b = Node::new(7, Some(vec![&a]));
    let c = Node::new(5, Some(vec![&a]));

    // dbg!(&a);
    // dbg!(&b);
    // dbg!(&c);

    // Now lets update b & c by calling add_one
   
    Node::add_one(&b);
    println!("==========Now checking the values in b Node===================");
    dbg!(&b);
    /*
        [src/main.rs:108:5] &b = Node {
            value: Cell {
                value: 8,
            },
            adjacent: [
                Node {
                    value: Cell {
                        value: 3,
                    },
                    adjacent: [],
                },
            ],
        }
    */ 
    Node::add_one(&c);
    println!("==========Now checking the values in c Node===================");

    /*
        [src/main.rs:112:5] &c = Node {
            value: Cell {
                value: 6,
            },
            adjacent: [
                Node {
                    value: Cell {
                        value: 4,
                    },
                    adjacent: [],
                },
            ],
        }
    */ 
    dbg!(&c);


    let i = RefCellNode::new(String::from("iiiiiii"), None);
    let j = RefCellNode::new(String::from("jjjjjjj"), Some(vec![&i]));
    let k = RefCellNode::new(String::from("kkkkkkk"), Some(vec![&i]));

    dbg!(&i);
    dbg!(&j);
    dbg!(&k);

    RefCellNode::add_urgency(&j);
    RefCellNode::add_urgency(&k);

    dbg!(&j);
    dbg!(&k);

    /*
        [src/main.rs:159:5] &j = RefCellNode {
            value: RefCell {
                value: "jjjjjjj!",
            },
            adjacent: [
                RefCellNode {
                    value: RefCell {
                        value: "iiiiiii!!",
                    },
                    adjacent: [],
                },
            ],
        }
        
        [src/main.rs:160:5] &k = RefCellNode {
            value: RefCell {
                value: "kkkkkkk!",
            },
            adjacent: [
                RefCellNode {
                    value: RefCell {
                        value: "iiiiiii!!",
                    },
                    adjacent: [],
                },
            ],
        }

    
    */ 

}
