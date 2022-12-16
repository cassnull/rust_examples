use semver::{Error, Version};

fn main() -> Result<(), Error> {
    let mut parsed_version = Version::parse("0.2.6")?;

    assert_eq!(parsed_version, Version::new(0, 2, 6));

    parsed_version.patch += 1;
    assert_eq!(parsed_version.to_string(), "0.2.7");
    println!("New patch release: v{}", parsed_version);

    parsed_version.minor += 1;
    parsed_version.patch = 0;
    assert_eq!(parsed_version.to_string(), "0.3.0");
    println!("New minor release: v{}", parsed_version);

    parsed_version.major += 1;
    parsed_version.minor = 0;
    assert_eq!(parsed_version.to_string(), "1.0.0");
    println!("New major release: v{}", parsed_version);

    let v2 = Version::parse("2.0.0")?;
    assert_eq!(parsed_version.cmp(&v2).is_lt(), true);

    Ok(())
}
