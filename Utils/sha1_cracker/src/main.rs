use sha1::Digest;
use std::{
    env, error,
    fs::File,
    io,
    io::{BufRead, BufReader},
    path::Path,
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn encode_sha1(text: &str) -> String {
    hex::encode(sha1::Sha1::digest(text.as_bytes()))
}

fn get_lines(filename: impl AsRef<Path>) -> Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

fn file_to_lines(filename: impl AsRef<Path>) -> Result<impl Iterator<Item = String>> {
    Ok(get_lines(filename)?.map(|line| line.expect("Couldn't read line").trim().into()))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage:");
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(());
    }

    let hash_to_crack = args[2].trim();
    if hash_to_crack.len() != SHA1_HEX_STRING_LENGTH {
        return Err("sha1 hash is not valid".into());
    }
    for common_password in file_to_lines(&args[1])? {
        if hash_to_crack == encode_sha1(&common_password) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }
    println!("password not found in wordlist :(");

    Ok(())
}
