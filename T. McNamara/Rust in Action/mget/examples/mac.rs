use mget::ethernet::MacAddress;

fn main() {
    let mac = MacAddress::new();
    println!("mac: {}", mac);
}
