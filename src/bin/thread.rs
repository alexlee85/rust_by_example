use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });

    // here will cause a compile error, because v is already moved
    // drop(v);

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
