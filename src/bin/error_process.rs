use std::error::Error;
#[allow(unused_imports)]
use std::fs::{File, OpenOptions};
#[allow(unused_imports)]
use std::io::{self, Read, Seek, SeekFrom, Write};

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

    let mut f = try_to_open_file(file_name.to_owned())
        .unwrap_or_else(|e| panic!("open file {} error: {:?}", file_name, e));

    write_to_file(&mut f).expect("write to file error");

    let mut str_buff = String::new();
    read_file_to_string(&mut f, &mut str_buff).expect("read file content error");
    println!("file content is: {}", str_buff);
}

fn try_to_open_file(file_name: String) -> Result<File, Box<dyn Error>> {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open(file_name)?;

    Ok(f)
}

fn write_to_file(f: &mut File) -> Result<(), io::Error> {
    f.write(b"Hello, World~~~!\n")?;
    f.flush()?;
    Ok(())
}

fn read_file_to_string(f: &mut File, str_buff: &mut String) -> Result<(), io::Error> {
    f.seek(SeekFrom::Start(0))?;
    f.read_to_string(str_buff)?;
    Ok(())
}
