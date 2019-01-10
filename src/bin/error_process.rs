#[allow(dead_code)]

use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Seek, SeekFrom, Write};

fn main() {
    // this will cause a panic
    // panic!("this is a panic");
    //
    // this will also cause a panic
    // let v = vec![1,2,3,4];
    // v[99];

    let file_name = "hello.txt";
    // let f = File::open(file_name);
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => File::create(file_name)
    //             .unwrap_or_else(|e| panic!("create file {} error: {:?}", file_name, e)),
    //         _ => panic!("open file failed: {:?}", error),
    //     },
    // };

    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap_or_else(|e| panic!("open file {} error: {:?}", file_name, e));

    // write need use std::fs::OpenOptions
    f.write(b"Hello, World~~~!\n").expect("write to file error");
    f.flush().expect("flush to file error");

    let mut buff: Vec<u8> = Vec::new();
    f.seek(SeekFrom::Start(0)).expect("seek on file error");
    f.read_to_end(&mut buff).expect("read file error");
    println!("file content is: {:?}", String::from_utf8_lossy(&buff));
}
