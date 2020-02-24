extern crate bytes;

use bytes::Bytes;

fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str {
    let x = str::replace(line, prefix, "");
    println!("{}", x);

    line
}

fn main() {
    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang);
        v = skip_prefix(line, p.as_str());
    }

    println!("{}", v);

    let name = "中文";
    for b in name.as_bytes() {
        println!("{}", b);
    }

    for c in name.chars() {
        println!("{}", c);
    }

    let mut a = Bytes::from(&b"nice job"[..]);
    let b = a.split_to(1);
    println!("{:?}, {:?}", a, b);

    println!("{}", "hello ".to_owned() + "world ~~~~");
    println!("{}", "hello ".to_owned() + &"world ~~~~".to_owned());
}
