use error_chain::error_chain;

use regex::Regex;
use std::process::Command;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        Regex(regex::Error);
        Utf8(std::string::FromUtf8Error);
    }
}

#[derive(PartialEq, Default, Clone, Debug)]
struct Commit {
    hash: String,
    message: String,
}

fn main() -> Result<()> {
    let number = 5;
    let output = Command::new("git")
        .arg("log")
        .arg("--oneline")
        .args(&["-n", number.to_string().as_str()])
        .output()?;

    if !output.status.success() {
        error_chain::bail!("Command executed with failing error code");
    }

    let pattern = Regex::new(
        r"(?x)
          ([0-9a-fA-F]+) # commit hash
          (.*)           # The commit message",
    )?;

    String::from_utf8(output.stdout)?
        .lines()
        .filter_map(|line| pattern.captures(line))
        .map(|cap| Commit {
            hash: cap[1].to_string(),
            message: cap[2].trim().to_string(),
        })
        .take(number)
        .for_each(|x| println!("{:?}", x));

    Ok(())
}
