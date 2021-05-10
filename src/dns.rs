use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::net::UdpSocket;
use trust_dns_client::rr::rdata::soa::SOA;
use trust_dns_client::rr::{LowerName, Name, RData, Record, RecordSet, RecordType, RrKey};
use trust_dns_server::authority::{Catalog, ZoneType};
use trust_dns_server::server::ServerFuture;
use trust_dns_server::store::in_memory::InMemoryAuthority;
use std::env;
use dotenv;

static DEFAULT_TCP_REQUEST_TIMEOUT: u64 = 5;


struct DnsManager {
    catalog: Catalog,
    dyn_root_zone: String,
}

impl DnsManager {
    pub fn new(dyn_root_zone: String) -> DnsManager {

        let catalog: Catalog = Catalog::new();

        return DnsManager {
            catalog,
            dyn_root_zone,
        };
    }

    fn get_initial_records(domain: &str) ->  BTreeMap<RrKey, RecordSet> {
        let authority_name = Name::from_str(domain).unwrap();
        let soa_serial = 1;
        let soa_name = Name::from_str(domain).unwrap();
        let soa_rdata = RData::SOA(SOA::new(
            Name::from_str(domain).unwrap(),      // mname
            Name::from_str(&format!("root.{}", domain)).unwrap(), // rname
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
        let mut authority_records = BTreeMap::new();
        authority_records.insert(soa_rr_key, soa_record_set);
        authority_records
    }

    fn upsert_domain(mut authority: InMemoryAuthority, domain: String, ip: Ipv4Addr) -> InMemoryAuthority {
        let dyn_name = Name::from_str(&domain).unwrap();
        let dyn_ttl = 60;
        let dyn_rdata = RData::A(ip);
        let dyn_record = Record::from_rdata(dyn_name, dyn_ttl, dyn_rdata);
        authority.upsert(dyn_record, authority.serial());
        authority
    }

    fn build_catalog(&mut self) {
        let authority_records = DnsManager::get_initial_records(&self.dyn_root_zone);
        let authority_name = Name::from_str(&self.dyn_root_zone).unwrap();

        let authority_zone_type = ZoneType::Master;
        let authority_allow_axfr = false;

        // first create an authority for root_dyn_zone
        let mut authority = InMemoryAuthority::new(
            authority_name.clone(),
            authority_records,
            authority_zone_type,
            authority_allow_axfr,
        )
            .unwrap();

        // then upsert records into the authority for all records in database
        let domain1 = format!("test.{}", self.dyn_root_zone);
        let ip1 = Ipv4Addr::new(1, 1, 1, 1);
        authority = DnsManager::upsert_domain(authority, domain1, ip1);

        let domain2 = format!("peach.{}", self.dyn_root_zone);
        let ip2 = Ipv4Addr::new(1, 1, 1, 3);
        authority = DnsManager::upsert_domain(authority, domain2, ip2);

        // finally put the authority into the catalog
        self.catalog.upsert(
            LowerName::new(&authority_name),
            Box::new(Arc::new(RwLock::new(authority))),
        );
    }


    fn upsert_test(&mut self) {

        // first insert the authority for the root dyn zone
        self.build_catalog();

        // second upsert, for sub-sub

        // third upsert, for sub-sub
//        let domain2 = &format!("peach.{}", self.dyn_root_zone);
//        let ip2 = Ipv4Addr::new(1, 1, 1, 2);
//        self.upsert(domain2, ip2);
//
//        // update upsert, for sub-sub
//        let domain2 = &format!("test.{}", self.dyn_root_zone);
//        let ip2 = Ipv4Addr::new(1, 1, 1, 3);
//        self.upsert(domain2, ip2);

    }
}


pub async fn server() -> ServerFuture<Catalog> {
    info!("Trust-DNS {} starting", trust_dns_server::version());

    dotenv::from_path("/etc/peach-dyndns.conf").ok();
    let dyn_root_zone = env::var("DYN_ROOT_ZONE").expect("DYN_ROOT_ZONE not set");
    let mut dns_manager = DnsManager::new(dyn_root_zone.to_string());

//    // first insert
//    dns_manager.upsert(
//        "test.dyn.peachcloud.org".to_string(),
//        Ipv4Addr::new(1, 1, 1, 1),
//    );
//
//    // second insert
//    dns_manager.upsert(
//        "test.dyn.peachcloud.org".to_string(),
//        Ipv4Addr::new(1, 1, 1, 3),
//    );
//
//    // third insert
//    dns_manager.upsert(
//        "peach.dyn.peachcloud.org".to_string(),
//        Ipv4Addr::new(1, 1 , 2, 3),
//    );

    dns_manager.upsert_test();

    let ip_addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
        let listen_port: u16 = 12323;
        let tcp_request_timeout = Duration::from_secs(DEFAULT_TCP_REQUEST_TIMEOUT);

        let sock_addr = SocketAddr::new(ip_addr, listen_port);
        let udp_socket = UdpSocket::bind(&sock_addr)
            .await
            .expect("could not bind udp socket");
        let tcp_listener = TcpListener::bind(&sock_addr)
            .await
            .expect("could not bind tcp listener");

    let mut server = ServerFuture::new(dns_manager.catalog);

    // load all the listeners
    info!("DNS server listening for UDP on {:?}", udp_socket);
    server.register_socket(udp_socket);

    info!("DNS server listening for TCP on {:?}", tcp_listener);
    server.register_listener(tcp_listener, tcp_request_timeout);
    info!("awaiting DNS connections...");

    server
}
