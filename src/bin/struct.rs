struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let mut origin = Point { x: 0, y: 0 };
    origin.x = 10;
    println!("({}, {})", origin.x, origin.y);

    let black = Color(0, 0, 0);
    println!("({}, {}, {})", black.0, black.1, black.2);
    println!("{:?}", black);
}
