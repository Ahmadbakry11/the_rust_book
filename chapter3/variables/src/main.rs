fn main() {
   let tup = (1.9, 2, 'z');
   let a = [0; 5];
   let c: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
   let (x, _y, _z) = tup;
   println!("The floating element is {x}");
   println!("The first element is {}", tup.0);
   println!("The second element is {}", tup.1);
   println!("The array first element is {}", a[0]);
   println!("The fifth char element is {}", c[4]);   
}
