use semver::{Error, Prerelease, Version};

fn main() -> Result<(), Error> {
    let version_1 = Version::parse("1.0.0-alpha")?;
    let version_2 = Version::parse("1.0.0")?;

    assert_ne!(version_1.pre, Prerelease::EMPTY);
    assert_eq!(version_2.pre, Prerelease::EMPTY);

    Ok(())
}
