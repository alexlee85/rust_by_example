struct MySmartPointer {
    data: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping MySmartPoiniter with data {}", self.data);
    }
}

fn main() {
    let a = MySmartPointer {
        data: String::from("hello"),
    };
    // will early drop a here
    drop(a);
    let b = MySmartPointer {
        data: String::from("world"),
    };
    println!("MySmartPointers created~~~!");

    use std::rc::Rc;
    let m = Rc::new(2);
    let n = m.clone();
    assert_eq!(m, n);
    
}
