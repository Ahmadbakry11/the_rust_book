pub mod shoes;
pub mod store;

use shoes::Shoe;
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    let x1 = &v1_iter.next();
    println!("{x1:?}");
    let x2 = &v1_iter.next();
    println!("{x2:?}");
    let x3 = &v1_iter.next();
    println!("{x3:?}");
    let x4 = &v1_iter.next();
    println!("{x4:?}");

    while let Some(value) = v1_iter.next() {
        println!("I got that: {value}");
    }

    /*
        .iter() produces an iterator that borrows the elements immutably
        to produce an iterator that borrows the elements mutably use iter_mut that
        yields mutable references to the elements in the item(here vectore).
    */

    let mut v2 = vec![4, 5, 6];

    let v2_iter = v2.iter_mut();

    for val in v2_iter {
        *val *= 2
    }

    println!("Now v2 values have been duplicated, check here: {v2:?}");

    /*
        into_iter() produces an iterator that consumes the collection and owns its values.
        You can iterate over the values and have ownership over its items
        and you can edit each item but this will have no effect on the original collection
        since it has been moved or consumed.
    */

    let v3 = vec![7, 8, 9];

    let v3_iter = v3.into_iter();

    // println!("{v3:?}") =====> no longer available.The value has been moved.

    /*
        We can edit the collection and have new one with the modified values
        like below:
    */

    let modified_v3: Vec<i32> = v3_iter.into_iter().map(|value| value * 2).collect();
    println!("Now we got a recent version of v3: {modified_v3:?}"); //[14, 16, 18]

    let shoes: Vec<Shoe> = vec![
        Shoe {
            size: 10,
            style: "sneaker",
        },
        Shoe {
            size: 13,
            style: "sandal",
        },
        Shoe {
            size: 10,
            style: "boot",
        },
    ];

    let target_size = 10;

    let target_shoes = store::get_shoes(shoes, target_size);

    println!("Now the suitable shoes to me are: {target_shoes:?}");
}
