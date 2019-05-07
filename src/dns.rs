use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

use futures::{future, Future};
use tokio_tcp::TcpListener;
use tokio_udp::UdpSocket;
use trust_dns::rr::rdata::soa::SOA;
use trust_dns::rr::{LowerName, Name, RData, Record, RecordSet, RecordType, RrKey};
use trust_dns_server;
use trust_dns_server::authority::{Catalog, ZoneType};
use trust_dns_server::server::ServerFuture;
use trust_dns_server::store::in_memory::InMemoryAuthority;

static DEFAULT_TCP_REQUEST_TIMEOUT: u64 = 5;

pub fn server() -> Box<Future<Item = (), Error = ()> + Send> {
    info!("Trust-DNS {} starting", trust_dns_server::version());

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

    let soa_serial = 1;
    let soa_name = Name::from_str("dyn.peach.cloud.").unwrap();
    let soa_rdata = RData::SOA(SOA::new(
        Name::from_str("dyn.peach.cloud.").unwrap(),      // mname
        Name::from_str("root.dyn.peach.cloud.").unwrap(), // rname
        soa_serial,                                       // serial
        604800,                                           // refresh
        86400,                                            // retry
        2419200,                                          // expire
        86400,                                            // negtive cache ttl
    ));
    let mut soa_record_set = RecordSet::new(&soa_name, RecordType::SOA, soa_serial);
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

    let server = ServerFuture::new(catalog);

    Box::new(future::lazy(move || {
        // load all the listeners
        info!("DNS server listening for UDP on {:?}", udp_socket);
        server.register_socket(udp_socket);

        info!("DNS server listening for TCP on {:?}", tcp_listener);
        server
            .register_listener(tcp_listener, tcp_request_timeout)
            .expect("DNS server could not register TCP listener");

        info!("awaiting DNS connections...");

        future::empty()
    }))
}
