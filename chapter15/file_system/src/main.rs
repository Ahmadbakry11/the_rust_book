use std::rc::Rc;

#[derive(Debug)]
struct File {
    name: String,
    path: String,
    size: usize
}

impl File {
    fn new(name: String, path: String, size: usize) -> Self {
        Self {
            name,
            path,
            size,
        }
    }
}

#[derive(Debug)]
struct Folder {
    name: String,
    children: Vec<Rc<Node>>,
}

impl Folder {
    fn new(name: String) -> Self {
        Self {
            name,
            children: vec![],
        }
    }

    fn add_child(&mut self, child: Rc<Node>) {
        self.children.push(child);
    }
}

#[derive(Debug)]
enum Node {
    File(File),
    Folder(Folder),
}

fn main() {
    let a = Rc::new(Node::File(File::new(
        String::from("file1"), 
        String::from("path1"), 
        99))
    );

    let b = Rc::new(Node::File(File::new(
        String::from("file2"), 
        String::from("path2"), 
        110))
    );

    let c = Rc::new(Node::File(File::new(
        String::from("file3"), 
        String::from("path3"), 
        119))
    );

    let mut folder1 = Folder::new(String::from("folder1"));
    let mut folder2 = Folder::new(String::from("folder2"));

    folder1.add_child(Rc::clone(&a));
    folder2.add_child(Rc::clone(&a));
    
    let children1 = folder1.children;
    let children2 = folder2.children;
    println!("Now foler 1 has these children: {children1:?}");
    println!("Now foler 2 has these children: {children2:?}");

}
