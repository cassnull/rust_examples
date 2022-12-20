use error_chain::error_chain;

use semver::{Version, VersionReq};
use std::process::Command;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Utf8(std::string::FromUtf8Error);
        SemVer(semver::Error);
    }
}

fn main() -> Result<()> {
    let version_constraint = "> 1.64.0";
    let version_test = VersionReq::parse(version_constraint)?;
    let output = Command::new("cargo").arg("--version").output()?;

    if !output.status.success() {
        error_chain::bail!("Command executed with failing error code");
    }

    let stdout = String::from_utf8(output.stdout)?;
    let version = stdout
        .split(" ")
        .nth(1)
        .ok_or_else(|| "Invalid command output")?;
    let parsed_version = Version::parse(version)?;

    if !version_test.matches(&parsed_version) {
        error_chain::bail!(
            "Command version lower than minimum supported version (found {}, need {})",
            parsed_version,
            version_constraint
        );
    }

    Ok(())
}
