//#![deny(warnings)]
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate url;

//use std::env;
use std::io::{self, Write};

use url::form_urlencoded;
use url::Url;

use futures::Future;
use futures::stream::Stream;

use hyper::{Method, Body, Client, Request};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

fn main() {

    let url = "https://sso.garmin.com/sso/login";

    let query = vec![
        ("service", "https://connect.garmin.com/post-auth/login"),
        ("clientId", "GarminConnect"),
        ("gauthHost", "https://sso.garmin.com/sso"),
        ("consumeServiceTicket", "false"),
    ];

    let url = Url::parse_with_params(url, query).unwrap();


    //    let url = Url::parse_with_params(url, query.into_iter());
    //
    let url = url.as_str().parse::<hyper::Uri>().unwrap();
    //    if url.scheme() != Some("http") {
    //        println!("This example only works with 'http' URLs.");
    //        return;
    //    }

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    // https://hyper.rs/guides/client/configuration/
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    println!("url: {}", url);

    let req = Request::<Body>::new(Method::Get, url);

    // http://siciarz.net/24-days-of-rust-hyper/
    let query = vec![
        ("service", "https://connect.garmin.com/post-auth/login"),
        ("clientId", "GarminConnect"),
        ("gauthHost", "https://sso.garmin.com/sso"),
        ("consumeServiceTicket", "false"),
    ];

    //    let param = form_urlencoded::Serializer::new(String::new())
    //        .extend_pairs(query.into_iter())
    //        .finish();
    //
    //        url.

    //    println!("param: {}", param);
    //
    let work = client
        .request(req)
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
