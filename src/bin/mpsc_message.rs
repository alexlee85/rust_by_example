use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("message"),
            String::from("from"),
            String::from("another"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let val = String::from("Hello");
        tx.send(val).unwrap();

        // this will cause a compile error, because val already move
        // println!("val is {}", val);

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("spawned"),
            String::from("thread"),
        ];

        for val2 in vals {
            tx.send(val2).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    for received2 in rx {
        println!("Got2: {}", received2);
    }
}
