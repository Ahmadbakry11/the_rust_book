use std::{thread::{self, spawn}, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
       for i in 1..=10 {
            println!("Hi I am number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
       } 
    });

    for i in 1..=5 {
        println!("Hi, I am number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("we are accessing {v:?}");
    });

}
