use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{Arg, Command};
use trust_dns_client::op::{Message, MessageType, OpCode, Query};
use trust_dns_client::rr::domain::Name;
use trust_dns_client::rr::{RData, RecordType};
use trust_dns_client::serialize::binary::{BinEncodable, BinEncoder};

fn message_id() -> u16 {
    let candidate = rand::random();
    if candidate == 0 {
        return message_id();
    }
    candidate
}

fn main() {
    let app = Command::new("resolve")
        .about("A simple to use DNS resolver")
        .arg(
            Arg::new("dns-server")
                .short('s')
                .default_value("1.1.1.1"),
        )
        .arg(Arg::new("domain-name").required(true))
        .get_matches();

    let domain_name_raw: &String = app.get_one("domain-name").unwrap();
    let domain_name = Name::from_ascii(domain_name_raw).unwrap();

    let dns_server_raw: &String = app.get_one("dns-server").unwrap();
    let dns_server: SocketAddr = format!("{}:53", dns_server_raw)
        .parse()
        .expect("invalid address");

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: Vec<u8> = vec![0; 512];

    let mut msg = Message::new();
    msg.set_id(message_id())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0").expect("cannot bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let _amt = localhost
        .send_to(&request_as_bytes, dns_server)
        .expect("socket misconfigured");

    let (_amt, _remote) = localhost
        .recv_from(&mut response_as_bytes)
        .expect("timeout reached");

    let dns_message = Message::from_vec(&response_as_bytes).expect("unable to parse response");

    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            if let Some(ip) = answer.data().and_then(RData::to_ip_addr) {
                println!("{}", ip);
            } else {
                eprintln!("invalid IP address received");
            }
        }
    }
}
