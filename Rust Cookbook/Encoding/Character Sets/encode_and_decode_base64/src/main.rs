use base64::{engine::general_purpose, Engine as _};
use error_chain::error_chain;
use std::str;

error_chain! {
    foreign_links {
        Base64(base64::DecodeError);
        Utf8Error(str::Utf8Error);
    }
}

fn main() -> Result<()> {
    let hello = b"hello rustaceans";
    let encoded = general_purpose::STANDARD.encode(hello);
    let decoded = general_purpose::STANDARD.decode(&encoded)?;

    println!("origin: {}", str::from_utf8(hello)?);
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded)?);

    Ok(())
}
