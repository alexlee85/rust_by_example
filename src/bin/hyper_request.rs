extern crate futures;
extern crate hyper;
extern crate tokio_core;

use hyper::Client;
use hyper::{Request, Method};
use futures::{Future, Stream};
use std::io::{self, Write};
use std::error::Error;
use tokio_core::reactor::Core;

fn main() {
    start().unwrap();
}

fn start() -> Result<(), Box<Error>> {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());

    let uri = "http://update.mxj360.com/rest/devices/mtool/config?uuid=1234567890".parse()?;
    let req = Request::new(Method::Post, uri);

    let work = client.request(req).and_then(|res| {
        println!("content: {}", res.status());
        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(From::from)
        })
    });

    Ok(core.run(work)?)
}