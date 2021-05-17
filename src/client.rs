use futures::try_join;
use std::io;
use tokio::task;

use std::net::Ipv4Addr;
use std::str::FromStr;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::op::update_message;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordSet, RecordType};
use trust_dns_client::tcp::TcpClientConnection;
use trust_dns_client::udp::UdpClientConnection;
use trust_dns_server::authority::{
    AuthLookup, Authority, LookupError, MessageRequest, UpdateResult,
};

pub fn check_domain_available_using_nslookup(domain: &str) -> bool {
    let address = "167.99.136.83:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
    let name = Name::from_str(domain).unwrap();

    println!("++ making query {:?}", domain);
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
    println!("++ received response");
    let answers: &[Record] = response.answers();

    if answers.len() > 0 {
        println!("found: {:?}", answers[0].rdata());
        true
    } else {
        false
    }
}

fn update_test() {
    let address = "167.99.136.83:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);

    // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str("test.time.commoninternet.net").unwrap();

    let record = Record::from_rdata(name.clone(), 8, RData::A(Ipv4Addr::new(127, 0, 0, 10)));
    let rrset: RecordSet = record.clone().into();
    let zone_origin = Name::from_str("time.commoninternet.net").unwrap();

    let response: DnsResponse = client
        .create(rrset, zone_origin)
        .expect("failed to create record");
    println!("response: {:?}", response);
}

fn main() {
    check_domain_available_using_nslookup("test");
    //    update_test();
}
