use std::io::*;
use std::fs::*;

fn main() {
    let file_path = "/Users/Alex/test.txt";
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .expect("open file error: ");
        file.write_all("hello, world.".as_bytes()).expect("write to file error: ");
    }

    {
        let mut file = OpenOptions::new()
            .read(true)
            .open(file_path)
            .expect("open file error: ");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("read file error: ");
        println!("{}", content);

        // println!("{:?}", file.metadata().unwrap());

        remove_file(file_path).expect("delete file error: ");
    }
}
