#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use futures::try_join;
use std::io;
use tokio::task;


use std::net::Ipv4Addr;
use std::str::FromStr;
use trust_dns_client::client::{Client, SyncClient};
use trust_dns_client::udp::UdpClientConnection;
use trust_dns_client::tcp::TcpClientConnection;
use trust_dns_client::op::DnsResponse;
use trust_dns_client::op::update_message;
use trust_dns_client::rr::{DNSClass, Name, RData, Record, RecordType, RecordSet};
use trust_dns_server::authority::{
    AuthLookup, Authority, LookupError, MessageRequest, UpdateResult,
};


fn simple_test() {
//    let address = "127.0.0.1:12323".parse().unwrap();
    let address = "167.99.136.83:53".parse().unwrap();
    let conn = UdpClientConnection::new(address).unwrap();
    let client = SyncClient::new(conn);
//    let conn = TcpClientConnection::new(address).unwrap();
//    let client = SyncClient::new(conn);

       // Specify the name, note the final '.' which specifies it's an FQDN
    let name = Name::from_str("time.commoninternet.net.").unwrap();

    // NOTE: see 'Setup a connection' example above
    // Send the query and get a message response, see RecordType for all supported options
    println!("++ making query");
    let response: DnsResponse = client.query(&name, DNSClass::IN, RecordType::A).unwrap();
    println!("++ received response");

    // Messages are the packets sent between client and server in DNS, DnsResonse's can be
    //  dereferenced to a Message. There are many fields to a Message, It's beyond the scope
    //  of these examples to explain them. See trust_dns::op::message::Message for more details.
    //  generally we will be interested in the Message::answers
    let answers: &[Record] = response.answers();

    // Records are generic objects which can contain any data.
    //  In order to access it we need to first check what type of record it is
    //  In this case we are interested in A, IPv4 address
    println!("found: {:?}", answers[0].rdata())
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

    let response: DnsResponse = client.create(rrset, zone_origin).expect("failed to create record");
    println!("response: {:?}", response);
}


fn main() {

//    simple_test();
    update_test();
}
