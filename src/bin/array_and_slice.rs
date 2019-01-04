use std::mem;

fn analyze_slice(slice: &[u8]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs: [u8; 5] = [1,2,3,4,5];
    let ys: [u8; 500] = [0; 500];

    println!("array is: {:?}", xs);
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    println!("array size: {}", xs.len());
    
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    println!("borrow the whole array as slice");
    analyze_slice(&xs);

    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);
}
