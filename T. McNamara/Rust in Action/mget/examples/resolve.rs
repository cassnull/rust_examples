use mget::dns::resolve;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let domain_name = "google.com";
    let dns_server_address = "8.8.8.8";

    let response = resolve(dns_server_address, domain_name)?;

    assert!(response.is_some());

    let server_ip = response.expect("invalid IP address received");

    println!("{}", server_ip);

    Ok(())
}
