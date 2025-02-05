use std::sync::mpsc::{self, SendError};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
 
    // thread::spawn(move || {
    //     let message = String::from("Hi");
    //     let result = tx.send(message);

    //     match result{
    //         Err(SendError(x)) => println!("I could not send the message: {x}"),
    //         _ => println!("Done sending")
    //     }
    // });
    
    // let received_message = rx.recv().unwrap();

    // println!("I go that message: {received_message}");

    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    thread::spawn(move || {
        let other_messages = vec![
            String::from("More"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for message in other_messages {
            tx1.send(message).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for message in rx {
        println!("I got: {message}");
    }
}
