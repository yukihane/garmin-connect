//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

//use std::env;
use std::io::{self, Write};

use futures::Future;
use futures::stream::Stream;

use hyper::Client;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn main() {

    let url = "https://connect.garmin.com/en-US/";

    let url = url.parse::<hyper::Uri>().unwrap();
    //    if url.scheme() != Some("http") {
    //        println!("This example only works with 'http' URLs.");
    //        return;
    //    }

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    // https://hyper.rs/guides/client/configuration/
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let work = client
        .get(url)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: \n{}", res.headers());

            res.body().for_each(|chunk| {
                io::stdout().write_all(&chunk).map_err(From::from)
            })
        })
        .map(|_| {
            println!("\n\nDone.");
        });

    core.run(work).unwrap();
}
