#[macro_use]
extern crate log;

use std::io;
use futures::{try_join, future, Future};
use nest::{Error, Store, Value};
use tokio::task;

mod cli;
mod dns;
mod http;

#[tokio::main]
async fn main() {
    let args = cli::args().expect("error parsing args");

    let dns_future = task::spawn(dns::server());
    let http_future = task::spawn(http::server());

    let result = try_join!(dns_future, http_future);

    if let Err(e) = result {
            io::Error::new(
                io::ErrorKind::Interrupted,
                "Server stopping due to interruption",
            );
            error!("server failure: {}", e);
        }

    println!("were stopping for some reason");
}
