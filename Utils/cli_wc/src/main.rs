use clap::{ArgGroup, Parser};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The wc utility displays the number of lines, words,
/// and bytes contained in each input file, or standard input
/// (if no file is specified) to the standard output.
#[derive(Parser)]
#[command(name = "wc", author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("opts")
        .required(false)
        .multiple(false)
        .args(&["bytes", "chars"]),
))]
struct Args {
    /// Input file(s)
    #[arg(name = "FILE", default_value = "-")]
    files: Vec<String>,
    /// print the byte counts
    #[arg(short = 'c', long = "bytes", default_value_t = false, group = "opts")]
    bytes: bool,
    /// print the character counts
    #[arg(short = 'm', long = "chars", default_value_t = false, group = "opts")]
    chars: bool,
    /// print the word counts
    #[arg(short = 'w', long = "words", default_value_t = false)]
    words: bool,
    /// print the newline counts
    #[arg(short = 'l', long = "lines", default_value_t = false)]
    lines: bool,
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    bytes: bool,
    chars: bool,
    words: bool,
    lines: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_bytes: usize,
    num_chars: usize,
    num_words: usize,
    num_lines: usize,
}

fn get_args() -> Result<Config> {
    let matches = Args::parse();

    let mut bytes = matches.bytes;
    let chars = matches.chars;
    let mut words = matches.words;
    let mut lines = matches.lines;

    if [words, bytes, chars, lines].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }
    Ok(Config {
        files: matches.files,
        bytes,
        chars,
        words,
        lines,
    })
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(config: Config) -> Result<()> {
    let mut total_bytes = 0;
    let mut total_chars = 0;
    let mut total_words = 0;
    let mut total_lines = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", &filename)
                        },
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        );
    }

    Ok(())
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

fn count(mut file: impl BufRead) -> Result<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use assert_cmd::Command;
    use predicates::prelude::*;
    use rand::{distributions::Alphanumeric, Rng};
    use std::fs;
    use std::io::Cursor;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    const PRG: &str = "wc";
    const EMPTY: &str = "tests/inputs/empty.txt";
    const FOX: &str = "tests/inputs/fox.txt";
    const ATLAMAL: &str = "tests/inputs/atlamal.txt";

    // --------------------------------------------------
    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    // --------------------------------------------------
    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }

    // --------------------------------------------------
    fn gen_bad_file() -> String {
        loop {
            let filename = rand::thread_rng()
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
    fn dies_chars_and_bytes() -> TestResult {
        Command::cargo_bin(PRG)?
            .args(&["-m", "-c"])
            .assert()
            .failure()
            .stderr(predicate::str::contains(
                "The argument '--chars' cannot be used with '--bytes'",
            ));
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
    #[test]
    fn skips_bad_file() -> TestResult {
        let bad = gen_bad_file();
        let expected = format!("{}: .* [(]os error 2[)]", bad);
        Command::cargo_bin(PRG)?
            .arg(bad)
            .assert()
            .success()
            .stderr(predicate::str::is_match(expected)?);
        Ok(())
    }

    // --------------------------------------------------
    #[test]
    fn empty() -> TestResult {
        run(&[EMPTY], "tests/expected/empty.txt.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox() -> TestResult {
        run(&[FOX], "tests/expected/fox.txt.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox_bytes() -> TestResult {
        run(&["--bytes", FOX], "tests/expected/fox.txt.c.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox_chars() -> TestResult {
        run(&["--chars", FOX], "tests/expected/fox.txt.m.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox_words() -> TestResult {
        run(&["--words", FOX], "tests/expected/fox.txt.w.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox_lines() -> TestResult {
        run(&["--lines", FOX], "tests/expected/fox.txt.l.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox_words_bytes() -> TestResult {
        run(&["-w", "-c", FOX], "tests/expected/fox.txt.wc.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox_words_lines() -> TestResult {
        run(&["-w", "-l", FOX], "tests/expected/fox.txt.wl.out")
    }

    // --------------------------------------------------
    #[test]
    fn fox_bytes_lines() -> TestResult {
        run(&["-l", "-c", FOX], "tests/expected/fox.txt.cl.out")
    }

    // --------------------------------------------------
    #[test]
    fn atlamal() -> TestResult {
        run(&[ATLAMAL], "tests/expected/atlamal.txt.out")
    }

    // --------------------------------------------------
    #[test]
    fn atlamal_bytes() -> TestResult {
        run(&["-c", ATLAMAL], "tests/expected/atlamal.txt.c.out")
    }

    // --------------------------------------------------
    #[test]
    fn atlamal_words() -> TestResult {
        run(&["-w", ATLAMAL], "tests/expected/atlamal.txt.w.out")
    }

    // --------------------------------------------------
    #[test]
    fn atlamal_lines() -> TestResult {
        run(&["-l", ATLAMAL], "tests/expected/atlamal.txt.l.out")
    }

    // --------------------------------------------------
    #[test]
    fn atlamal_words_bytes() -> TestResult {
        run(&["-w", "-c", ATLAMAL], "tests/expected/atlamal.txt.wc.out")
    }

    // --------------------------------------------------
    #[test]
    fn atlamal_words_lines() -> TestResult {
        run(&["-w", "-l", ATLAMAL], "tests/expected/atlamal.txt.wl.out")
    }

    // --------------------------------------------------
    #[test]
    fn atlamal_bytes_lines() -> TestResult {
        run(&["-l", "-c", ATLAMAL], "tests/expected/atlamal.txt.cl.out")
    }

    // --------------------------------------------------
    #[test]
    fn atlamal_stdin() -> TestResult {
        let input = fs::read_to_string(ATLAMAL)?;
        let expected = fs::read_to_string("tests/expected/atlamal.txt.stdin.out")?;
        Command::cargo_bin(PRG)?
            .write_stdin(input)
            .assert()
            .stdout(expected);
        Ok(())
    }

    // --------------------------------------------------
    #[test]
    fn test_all() -> TestResult {
        run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")
    }

    // --------------------------------------------------
    #[test]
    fn test_all_lines() -> TestResult {
        run(&["-l", EMPTY, FOX, ATLAMAL], "tests/expected/all.l.out")
    }

    // --------------------------------------------------
    #[test]
    fn test_all_words() -> TestResult {
        run(&["-w", EMPTY, FOX, ATLAMAL], "tests/expected/all.w.out")
    }

    // --------------------------------------------------
    #[test]
    fn test_all_bytes() -> TestResult {
        run(&["-c", EMPTY, FOX, ATLAMAL], "tests/expected/all.c.out")
    }

    // --------------------------------------------------
    #[test]
    fn test_all_words_bytes() -> TestResult {
        run(&["-cw", EMPTY, FOX, ATLAMAL], "tests/expected/all.wc.out")
    }

    // --------------------------------------------------
    #[test]
    fn test_all_words_lines() -> TestResult {
        run(&["-wl", EMPTY, FOX, ATLAMAL], "tests/expected/all.wl.out")
    }

    // --------------------------------------------------
    #[test]
    fn test_all_bytes_lines() -> TestResult {
        run(&["-cl", EMPTY, FOX, ATLAMAL], "tests/expected/all.cl.out")
    }
}
