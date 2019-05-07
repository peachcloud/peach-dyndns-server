#[macro_use]
extern crate log;

use std::io;

use futures::{future, Future};
use nest::{Error, Store, Value};
use tokio::runtime::Runtime;
use tokio_executor;
use trust_dns_server::logger;

mod dns;
mod http;

fn main() {
    logger::debug();

    let mut runtime = Runtime::new().expect("error when creating tokio Runtime");

    let main_future: Box<Future<Item = (), Error = ()> + Send> =
        Box::new(future::lazy(move || {
            tokio_executor::spawn(dns::server());
            tokio_executor::spawn(http::server());

            future::empty()
        }));

    if let Err(e) = runtime.block_on(main_future.map_err(|_| {
        io::Error::new(
            io::ErrorKind::Interrupted,
            "Server stopping due to interruption",
        )
    })) {
        error!("server failure: {}", e);
    }

    // we're exiting for some reason...
    info!("stopping!?");
}
