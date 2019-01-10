use regex::Regex;
use std::collections::BTreeMap;
use std::io::{self, Write};

fn main() {
    let mut company: BTreeMap<String, Vec<String>> = BTreeMap::new();
    io::stdout()
        .write(b"This is a simple command line department manager system.\n")
        .expect("error when write message to stdout");
    io::stdout().flush().expect("flush stdout error");
    loop {
        io::stdout()
            .write(b">>> ")
            .expect("error when wirte to stdout");
        io::stdout().flush().expect("flush stdout error");
        if let Some((name, dep_name)) = input() {
            let v = company.entry(dep_name).or_insert(vec![]);
            v.push(name);
            v.sort();
            println!("{:#?}", company);
        } else {
            println!("Bye ~~~~");
            break;
        }
    }
}

fn input() -> Option<(String, String)> {
    let mut buffer: String = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("can't read from stdin");
    // println!("input string is {}", buffer);

    let re = Regex::new(r"add (\w+) to (\w+)").expect("compile regex error");

    re.captures(&buffer).and_then(|cap| {
        // println!("{:?}", cap);
        if cap.len() == 3 {
            println!("{}, {}", &cap[1], &cap[2]);
            Some((cap[1].to_owned(), cap[2].to_owned()))
        } else {
            None
        }
    })
}
