extern crate hyper;

use hyper::rt::{self, Future, Stream};
use hyper::Client;
use std::io::{self, Write};

fn main() {
    start();
}

fn start() {
    let client = Client::new();

    let uri = "http://update.mxj360.com/rest/devices/mtool/config?uuid=1234567890"
        .parse::<hyper::Uri>()
        .unwrap();

    let run = client
        .get(uri)
        .and_then(|res| {
            println!("content: {}", res.status());
            res.into_body().for_each(|chunk| {
                io::stdout()
                    .write_all(&chunk)
                    .map_err(|err| panic!("Error: {}", err))
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        })
        .map_err(|err| {
            eprintln!("Error: {}", err);
        });

    rt::run(run)
}
