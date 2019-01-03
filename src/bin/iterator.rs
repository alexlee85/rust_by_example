struct Counter {
    count: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter{count: 0}
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let counter2 = Counter::new();
    for x in counter2 {
        println!("{}", x)
    }

    let mut counter = Counter::new();

    let x = counter.next().unwrap();
    println!("{}", x);

    let x = counter.next().unwrap();
    println!("{}", x);

    let x = counter.next().unwrap();
    println!("{}", x);

    let x = counter.next().unwrap();
    println!("{}", x);

    let x = counter.next().unwrap();
    println!("{}", x);

    let x = counter.next().expect("error on none");
    println!("{}", x);
}