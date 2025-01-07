pub mod novel;

use std::fmt::Display;

use crate::novel::Novel;
fn main() {
   let r;
   let x = 9;

   {

      r = &x;
   } 

   println!("The ref is {}", *r);


   let string1 = String::from("Hello");
   let string2 = "Hel";
   let longest_str = longest(string1.as_str(), string2);
   let smallest_str = smallest(string1.as_str(), string2);

   println!("The longest string is {longest_str}");
   println!("The smallest string is {smallest_str}");

   let basic_novel = Novel::new();
   let new_novel = basic_novel.main_title("This is now.Gone with the wind");

   println!("The new novel title is {}", new_novel.title());

   new_novel.announce_part("part");

   let ann: &str = longest_with_announcement(string1.as_str(), string2, "That is a new novel");

   println!("{ann}");

}

/*
   the below function will not compile because the return value lifetime is not 
   specified.Is it x lifetime or y lifetime
*/
// fn longest(x: &str, y: &str) -> &str {
//    if x.len() > y.len() {
//       x
//    } else {
//        y
//    }
// }

/* To fix that we need a lifetime specifier like below:*/ 

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
   if x.len() > y.len() {
      x
   } else {
      y
   }
}

// fn smallest<'a>(x: &str, y: &str) -> &'a str {
//    if x.len() > y.len() {
//       y
//    } else {
//       x
//    }
// }

/*
   The above code for smallest() function will not compile.
   The fix to the above code is to return an owned value and not a ref like below.
*/ 
fn smallest<'a>(x: &str, y: &str) -> String {
   if x.len() > y.len() {
      format!("{x}")
   } else {
      format!("{y}")
   }
}

fn longest_with_announcement<'a, T:Display>(x: &'a str, y: &'a str, ann: T) -> &'a str {
   println!("{ann}");
   if x.len() > y.len() {
      x
   } else {
       y
   }
}

//The above works pretty well 