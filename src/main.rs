#[macro_use]
extern crate log;
extern crate env_logger;

extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate hyper_tls;

pub mod utils;
pub mod parser;

use futures::{future, Future};
use futures::stream::Stream;

use hyper::{Client, Error};
use hyper_tls::HttpsConnector;

use utils::parse_html;


fn main() {


    info!("Starting site crawl!");

    let url = "https://www.reddit.com/".parse::<hyper::Uri>().unwrap();

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();

    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    // client.get -> hyper::client::FutureResponse
    // .and_then -> Execute another future after this one has resolved successfully
    let work = client.get(url).and_then(|res| {
        // println!("Response: {}", res.status());
        // println!("Headers: \n{}", res.headers());

        res.body()
            .fold(Vec::new(), |mut v, chunk| {
                v.extend(&chunk[..]);
                future::ok::<_, Error>(v)
            })
            .and_then(|chunks| {
                let s = String::from_utf8(chunks).unwrap();
                print!("BODY: {}\n", s);

                let links = parse_html(s);
                for (link, count) in &links {
                    println!("({}) -> {}", count, link);
                }

                future::ok::<_, Error>(0)
            })

    });

    core.run(work).unwrap();
}
