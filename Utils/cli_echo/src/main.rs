use clap::Parser;

/// Print a line of text
#[derive(Parser)]
#[command(name = "echo", author, version, about, long_about = None)]
struct Args {
    /// Input text
    #[arg(required = true)]
    text: Vec<String>,
    /// Do not print newline
    #[arg(short = 'n', default_value_t = false)]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();
    let text = args.text;
    let omit_newline = args.omit_newline;

    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn dies_no_args() -> TestResult {
        let mut cmd = Command::cargo_bin("echo")?;
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Usage"));
        Ok(())
    }

    #[test]
    fn runs() -> TestResult {
        let mut cmd = Command::cargo_bin("echo")?;
        cmd.arg("hello").assert().success();
        Ok(())
    }

    fn run(args: &[&str], expected_file: &str) -> TestResult {
        let expected = fs::read_to_string(expected_file)?;
        Command::cargo_bin("echo")?
            .args(args)
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    #[test]
    fn hello1() -> TestResult {
        run(&["Hello there"], "tests/expected/hello1.txt")
    }

    #[test]
    fn hello2() -> TestResult {
        run(&["Hello", "there"], "tests/expected/hello2.txt")
    }

    #[test]
    fn hello1_no_newline() -> TestResult {
        run(&["Hello there", "-n"], "tests/expected/hello1.n.txt")
    }

    #[test]
    fn hello2_no_newline() -> TestResult {
        run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
    }
}
