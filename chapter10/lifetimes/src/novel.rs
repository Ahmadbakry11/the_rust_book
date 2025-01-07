pub struct Novel<'a> {
    title: &'a str
}

impl<'a> Novel<'a> {
    pub fn title(&self) -> &str {
        self.title
    }

    pub fn new() -> Self {
        Self {
            title: "Default title"
        }
    }

    pub fn main_title(&self, first_page: &'a str) -> Self {
        let novel_title = first_page.split('.').last().unwrap();

        Self {
            title: novel_title,
        }
    }

    pub fn announce_part(&self, part: &str) -> &str {
        println!("The new novel is announced, take a look here: {}", part);
        // let new_text = text.as_str();
        // new_text
        self.title
    }

}


/*
    elision rules are 3 rules
    First one is related to the input lifetime and the remaining two are related to
    the output lifetimes
    input lifetimes are lifetimes of functions parameters
    output lifetimes are lifetimes related to return values.
*/

/*
    First Rule:
    A compiler assigns a lifetime parameter to each parameter that is a refernce.
    so, a function with one parameter will be assigned one lifetime parameter
    and a function with two parameters will be assigned two lifetime parameters

    fn func_one(x: &str)  =====> fn func_one<'a>(x: &'a str) 

    fn func_two(x: &str, y: &str)  =====> fn func_two<'a, 'b>(x: &'a str, y: &'b str) 
    That is explaining why the below function is not compiling
        The compiler needs to know which lifetime will be assigned to the return value
        The compiler has gone through all the 3 lifetime elision and could not figure out the 
        lifetime of the rturn type
        That has to be annotated

        fn longest(x: &str, y: &str) -> &str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

*/ 

/*
    The Second Rule:
    states that if there is exactly one input lifetime parameter, this lifetime will be assigned to all output
    lifetime parameters

    fn foo<'a>(x: i32, y: &'a str) -> &'a i32
*/ 


/*
    The Third Rule:
    The third rule is that, if there are multiple input lifetime parameters, 
    but one of them is &self or &mut self because this is a method, 
    the lifetime of self is assigned to all output lifetime parameters. 
    This third rule makes methods much nicer to read and write because fewer symbols are necessary.
*/ 