extern crate futures;
#[macro_use]
extern crate log;
extern crate tokio;
extern crate tokio_tcp;
extern crate trust_dns_server;

use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::time::{Duration};

use futures::{future, Future};
use tokio::runtime::Runtime;
use tokio::runtime::TaskExecutor;
use tokio_tcp::TcpListener;
use tokio_udp::UdpSocket;

use trust_dns_server::proto::rr::Name;
use trust_dns_server::authority::{AuthorityObject, Catalog, ZoneType};
use trust_dns_server::config::{Config, ZoneConfig};
use trust_dns_server::logger;
use trust_dns_server::server::ServerFuture;
use trust_dns_server::store::in_memory::{InMemoryAuthority};
use trust_dns_server::store::StoreConfig;

static DEFAULT_TCP_REQUEST_TIMEOUT: u64 = 5;

fn main() {
    logger::debug();

    info!("Trust-DNS {} starting", trust_dns_server::version());

    let mut io_loop = Runtime::new().expect("error when creating tokio Runtime");
    let executor = io_loop.executor();

    let mut catalog: Catalog = Catalog::new();

    let ip_addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let listen_port: u16 = 12323;
    let tcp_request_timeout = Duration::from_secs(DEFAULT_TCP_REQUEST_TIMEOUT);

    let sock_addr = SocketAddr::new(ip_addr, listen_port);
    let udp_socket = UdpSocket::bind(&sock_addr).unwrap_or_else(|_| panic!("could not bind to udp: {}", sock_addr));
    let tcp_listener = TcpListener::bind(&sock_addr).unwrap_or_else(|_| panic!("could not bind to tcp: {}", sock_addr));

    let mut server = ServerFuture::new(catalog);

    let server_future: Box<Future<Item = (), Error = ()> + Send> =
        Box::new(future::lazy(move || {
            // load all the listeners
            info!("listening for UDP on {:?}", udp_socket);
            server.register_socket(udp_socket);

            info!("listening for TCP on {:?}", tcp_listener);
            server
                .register_listener(tcp_listener, tcp_request_timeout)
                .expect("could not register TCP listener");

            info!("awaiting connections...");

            info!("Server starting up");
            future::empty()
        }));

    if let Err(e) = io_loop.block_on(server_future.map_err(|_| {
        io::Error::new(
            io::ErrorKind::Interrupted,
            "Server stopping due to interruption",
        )
    })) {
        error!("failed to listen: {}", e);
    }

    // we're exiting for some reason...
    info!("Trust-DNS {} stopping", trust_dns_server::version());
}


