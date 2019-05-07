extern crate futures;
#[macro_use]
extern crate log;
extern crate tokio;
extern crate tokio_tcp;
extern crate trust_dns_server;

use std::collections::BTreeMap;
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use futures::{future, Future};
use nest::{Error, Store, Value};
use tokio::runtime::Runtime;
use tokio::runtime::TaskExecutor;
use tokio_tcp::TcpListener;
use tokio_udp::UdpSocket;
use trust_dns::rr::rdata::soa::SOA;
use trust_dns::rr::{LowerName, Name, RData, Record, RecordSet, RecordType, RrKey};
use trust_dns_server::authority::{AuthorityObject, Catalog, ZoneType};
use trust_dns_server::config::{Config, ZoneConfig};
use trust_dns_server::logger;
use trust_dns_server::server::ServerFuture;
use trust_dns_server::store::in_memory::InMemoryAuthority;
use trust_dns_server::store::StoreConfig;

static DEFAULT_TCP_REQUEST_TIMEOUT: u64 = 5;

fn main() {
    logger::debug();

    info!("Trust-DNS {} starting", trust_dns_server::version());

    let mut io_loop = Runtime::new().expect("error when creating tokio Runtime");
    let executor = io_loop.executor();

    let ip_addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let listen_port: u16 = 12323;
    let tcp_request_timeout = Duration::from_secs(DEFAULT_TCP_REQUEST_TIMEOUT);

    let sock_addr = SocketAddr::new(ip_addr, listen_port);
    let udp_socket = UdpSocket::bind(&sock_addr)
        .unwrap_or_else(|_| panic!("could not bind to udp: {}", sock_addr));
    let tcp_listener = TcpListener::bind(&sock_addr)
        .unwrap_or_else(|_| panic!("could not bind to tcp: {}", sock_addr));

    let mut catalog: Catalog = Catalog::new();

    let authority_name = Name::from_str("dyn.peach.cloud.").unwrap();
    let mut authority_records = BTreeMap::new();
    let authority_zone_type = ZoneType::Master;
    let authority_allow_axfr = false;

    let serial = 1;
    let soa_name = Name::from_str("dyn.peach.cloud.").unwrap();
    let soa_ttl = 60;
    let soa_rdata = RData::SOA(SOA::new(
        Name::from_str("dyn.peach.cloud.").unwrap(),      // mname
        Name::from_str("root.dyn.peach.cloud.").unwrap(), // rname
        serial,                                           // serial
        604800,                                           // refresh
        86400,                                            // retry
        2419200,                                          // expire
        86400,                                            // negtive cache ttl
    ));
    let mut soa_record_set = RecordSet::new(&soa_name, RecordType::SOA, serial);
    soa_record_set.add_rdata(soa_rdata);
    let soa_rr_key = RrKey::new(
        LowerName::new(&authority_name),
        soa_record_set.record_type(),
    );
    authority_records.insert(soa_rr_key, soa_record_set);

    let mut authority = InMemoryAuthority::new(
        authority_name.clone(),
        authority_records,
        authority_zone_type,
        authority_allow_axfr,
    )
    .unwrap();

    /*
    let ns_name = Name::from_str("dyn.peach.cloud.").unwrap();
    let ns_ttl = 60;
    let ns_rdata = RData::NS(Name::from_str("localhost.").unwrap());
    let ns_record = Record::from_rdata(ns_name, ns_ttl, ns_rdata);
    authority.upsert(ns_record, authority.serial());
    */

    let dyn_name = Name::from_str("test.dyn.peach.cloud.").unwrap();
    let dyn_ttl = 60;
    let dyn_rdata = RData::A(Ipv4Addr::new(1, 1, 1, 1));
    let dyn_record = Record::from_rdata(dyn_name, dyn_ttl, dyn_rdata);
    authority.upsert(dyn_record, authority.serial());

    catalog.upsert(LowerName::new(&authority_name), Box::new(authority));

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
