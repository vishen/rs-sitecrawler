extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;

use std::io::{self, Write};

use futures::Future;
use futures::future;
use futures::stream::Stream;

use hyper::{Client, Error};
use hyper_tls::HttpsConnector;

/*
    - unwrap():
        - https://github.com/rust-lang/rust/blob/09e2ad13d0aa01143bcb20dece3ff6c5a7e34ea3/src/libcore/option.rs#L329-L359
        - Semantically, it means "While this may or may not have a value, I'm saying it does. If it doesn't, just crash, I don't want to handle the error."
*/

fn main() {

    let url = "https://google.com".parse::<hyper::Uri>().unwrap();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let client = Client::configure()
                        .connector(HttpsConnector::new(4, &handle).unwrap())
                        .build(&handle);

    // client.get -> hyper::client::FutureResponse
    // .and_then -> Execute another future after this one has resolved successfully
    let work = client.get(url).and_then(|res| {
        println!("Response: {}", res.status());
        println!("Headers: \n{}", res.headers());

        /*
        // .for_each() -> Runs this stream to completion, executing the provided closure for each element on the stream.
        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)

        })
        */

        res.body().fold(Vec::new(), |mut v, chunk| {
            v.extend(&chunk[..]);
            future::ok::<_, Error>(v)
        }).and_then(|chunks| {
            let s = String::from_utf8(chunks).unwrap();
            print!("BODY: {}\n", s);
            future::ok::<_, Error>(0)
        })

    });

    core.run(work).unwrap();
}
