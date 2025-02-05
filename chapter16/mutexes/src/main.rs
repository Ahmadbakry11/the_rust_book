use std::sync::Arc;
use std::time::Duration;
use std::{sync::Mutex, thread};
use std::rc::Rc;


fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
       let counter = Arc::clone(&counter);
       let handle = thread::spawn(move || {
           let mut num = counter.lock().unwrap();
           *num += 1;
           thread::sleep(Duration::from_millis(9));
       });

       handles.push(handle);
    }

    for i in 0..2 {
        println!("{i}");
    }

    for handle in handles {
        handle.join().unwrap()
    }

    println!("Now the counter value is: {}", *counter.lock().unwrap());
}
