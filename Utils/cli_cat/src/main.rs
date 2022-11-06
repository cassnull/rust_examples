use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Concatenate FILE(s), or standard input, to standard output
#[derive(Parser)]
#[command(name = "cat", author, version, about, long_about = None)]
struct Args {
    /// Input file(s)
    #[arg(name = "FILE", default_value = "-")]
    files: Vec<String>,
    /// Number all output lines
    #[arg(short = 'n', long = "number", default_value_t = false)]
    number_lines: bool,
    /// Number nonempty output lines, overrides -n
    #[arg(short = 'b', long = "number-nonblank", default_value_t = false)]
    number_nonblank_lines: bool,
    /// display TAB characters as ^I
    #[arg(short = 'T', long = "show-tabs", default_value_t = false)]
    show_tabs: bool,
}

#[derive(Debug)]
struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    show_tabs: bool,
}

fn get_args() -> Result<Config> {
    let matches = Args::parse();
    Ok(Config {
        files: matches.files,
        number_lines: matches.number_lines,
        number_nonblank_lines: matches.number_nonblank_lines,
        show_tabs: matches.show_tabs,
    })
}

fn run(config: Config) -> Result<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = replace_tab(&config, line_result?);
                    if config.number_lines {
                        println!("{:>6}{}{}", line_num + 1, tab(&config), line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}{}{}", last_num, tab(&config), line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn tab(config: &Config) -> &'static str {
    if config.show_tabs {
        "^I"
    } else {
        "\t"
    }
}

fn replace_tab(config: &Config, str: String) -> String {
    if config.show_tabs {
        str.replace("\t", "^I")
    } else {
        str
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use rand::{distributions::Alphanumeric, Rng};
    use std::error::Error;
    use std::fs;

    type TestResult = Result<(), Box<dyn Error>>;

    const PRG: &str = "cat";
    const EMPTY: &str = "tests/inputs/empty.txt";
    const FOX: &str = "tests/inputs/fox.txt";
    const SPIDERS: &str = "tests/inputs/spiders.txt";
    const BUSTLE: &str = "tests/inputs/the-bustle.txt";

    // --------------------------------------------------
    #[test]
    fn usage() -> TestResult {
        for flag in &["-h", "--help"] {
            Command::cargo_bin(PRG)?
                .arg(flag)
                .assert()
                .stdout(predicate::str::contains("Usage"));
        }
        Ok(())
    }

    // --------------------------------------------------
    fn gen_bad_file() -> String {
        loop {
            let filename: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect();

            if fs::metadata(&filename).is_err() {
                return filename;
            }
        }
    }

    // --------------------------------------------------
    #[test]
    fn skips_bad_file() -> TestResult {
        let bad = gen_bad_file();
        let expected = format!("{}: .* [(]os error 2[)]", bad);
        Command::cargo_bin(PRG)?
            .arg(&bad)
            .assert()
            .success()
            .stderr(predicate::str::is_match(expected)?);
        Ok(())
    }

    // --------------------------------------------------
    fn run(args: &[&str], expected_file: &str) -> TestResult {
        let expected = fs::read_to_string(expected_file)?;
        Command::cargo_bin(PRG)?
            .args(args)
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    // --------------------------------------------------
    fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
        let input = fs::read_to_string(input_file)?;
        let expected = fs::read_to_string(expected_file)?;
        Command::cargo_bin(PRG)?
            .args(args)
            .write_stdin(input)
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    // --------------------------------------------------
    #[test]
    fn bustle_stdin() -> TestResult {
        run_stdin(BUSTLE, &["-"], "tests/expected/the-bustle.txt.stdin.out")
    }

    // --------------------------------------------------
    #[test]
    fn bustle_stdin_n() -> TestResult {
        run_stdin(
            BUSTLE,
            &["-n", "-"],
            "tests/expected/the-bustle.txt.n.stdin.out",
        )
    }

    // --------------------------------------------------
    #[test]
    fn bustle_stdin_b() -> TestResult {
        run_stdin(
            BUSTLE,
            &["-b", "-"],
            "tests/expected/the-bustle.txt.b.stdin.out",
        )
    }

    // --------------------------------------------------
    #[test]
    fn empty() -> TestResult {
        run(&[EMPTY], "tests/expected/empty.txt.out")
    }

    // --------------------------------------------------
    #[test]
    fn empty_n() -> TestResult {
        run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
    }

    // --------------------------------------------------
    #[test]
    fn empty_b() -> TestResult {
        run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox() -> TestResult {
        run(&[FOX], "tests/expected/fox.txt.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox_n() -> TestResult {
        run(&["-n", FOX], "tests/expected/fox.txt.n.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox_b() -> TestResult {
        run(&["-b", FOX], "tests/expected/fox.txt.b.out")
    }

    // --------------------------------------------------
    #[test]
    fn spiders() -> TestResult {
        run(&[SPIDERS], "tests/expected/spiders.txt.out")
    }

    // --------------------------------------------------
    #[test]
    fn spiders_n() -> TestResult {
        run(&["--number", SPIDERS], "tests/expected/spiders.txt.n.out")
    }

    // --------------------------------------------------
    #[test]
    fn spiders_b() -> TestResult {
        run(
            &["--number-nonblank", SPIDERS],
            "tests/expected/spiders.txt.b.out",
        )
    }

    // --------------------------------------------------
    #[test]
    fn bustle() -> TestResult {
        run(&[BUSTLE], "tests/expected/the-bustle.txt.out")
    }

    // --------------------------------------------------
    #[test]
    fn bustle_n() -> TestResult {
        run(&["-n", BUSTLE], "tests/expected/the-bustle.txt.n.out")
    }

    // --------------------------------------------------
    #[test]
    fn bustle_b() -> TestResult {
        run(&["-b", BUSTLE], "tests/expected/the-bustle.txt.b.out")
    }

    // --------------------------------------------------
    #[test]
    fn all() -> TestResult {
        run(&[FOX, SPIDERS, BUSTLE], "tests/expected/all.out")
    }

    // --------------------------------------------------
    #[test]
    fn all_n() -> TestResult {
        run(&[FOX, SPIDERS, BUSTLE, "-n"], "tests/expected/all.n.out")
    }

    // --------------------------------------------------
    #[test]
    fn all_b() -> TestResult {
        run(&[FOX, SPIDERS, BUSTLE, "-b"], "tests/expected/all.b.out")
    }
}
