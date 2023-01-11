use clap::{Arg, Command};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Command::new("fview")
        .bin_name("fview")
        .arg(
            Arg::new("filename")
                .value_parser(clap::builder::NonEmptyStringValueParser::new())
                .required(true),
        )
        .arg(
            clap::Arg::new("bytes-per-line")
                .long("bytes-per-line")
                .value_parser(clap::value_parser!(usize))
                .default_value("16"),
        );
    let matches = cmd.get_matches();

    let fname: &String = matches.get_one("filename").expect("required");

    let bytes_per_line: usize = *matches.get_one("bytes-per-line").expect("defaulted");

    let mut file = File::open(fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = vec![0; bytes_per_line];

    loop {
        let read_count = file.read(&mut buffer)?;
        print!("[0x{:08x}] ", pos);

        let bytes = &buffer[..read_count];
        for byte in bytes {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }

        let unchecked_str = String::from_utf8_lossy(&bytes)
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\u{0B}", "\\v")
            .replace("\u{0C}", "\\f");

        println!(
            "{:>1$}",
            unchecked_str,
            (bytes_per_line - read_count) * 3 + unchecked_str.chars().count()
        );

        if read_count != bytes_per_line {
            break;
        }

        pos += read_count;
    }

    Ok(())
}
