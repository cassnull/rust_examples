#[cfg(target_os = "linux")]
#[cfg(target_os = "linux")]
use syslog::{Error, Facility};

#[cfg(target_os = "linux")]
fn main() -> Result<(), Error> {
    syslog::init(
        Facility::LOG_USER,
        log::LevelFilter::Debug,
        Some("My app name"),
    )?;
    log::debug!("this is a debug {}", "message");
    log::error!("this is an error!");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("So far, only Linux systems are supported.");
}
