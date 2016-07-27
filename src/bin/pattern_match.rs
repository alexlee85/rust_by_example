fn main() {
    let pair = (0, -2);

    println!("Tell me about {:?}", pair);

    match pair {
        (0, y) => println!("first is 0 and `y` is {:?}", y),
        _ => println!("I don't care"),
    }
}
