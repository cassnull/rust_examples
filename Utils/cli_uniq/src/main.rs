use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// The uniq utility reads the specified input_file comparing
/// adjacent lines, and writes a copy of each unique input line
/// to the output_file.
#[derive(Parser)]
#[command(name = "uniq", author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(name = "IN_FILE", default_value = "-")]
    in_file: String,
    /// Output file
    #[arg(name = "OUT_FILE")]
    out_file: Option<String>,
    /// Precede each output line with the count of the number of times
    /// the line occurred in the input, followed by a single space.
    #[arg(short = 'c', long = "count")]
    count: bool,
}

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

fn get_args() -> Result<Config> {
    let matches = Args::parse();

    Ok(Config {
        in_file: matches.in_file,
        out_file: matches.out_file,
        count: matches.count,
    })
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> Result<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> Result<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;

    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }
        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }
        count += 1;
        line.clear();
    }

    print(count, &previous)?;

    Ok(())
}

fn main() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use rand::{distributions::Alphanumeric, Rng};
    use std::fs;
    use tempfile::NamedTempFile;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    struct Test {
        input: &'static str,
        out: &'static str,
        out_count: &'static str,
    }

    const PRG: &str = "uniq";

    const EMPTY: Test = Test {
        input: "tests/inputs/empty.txt",
        out: "tests/expected/empty.txt.out",
        out_count: "tests/expected/empty.txt.c.out",
    };

    const ONE: Test = Test {
        input: "tests/inputs/one.txt",
        out: "tests/expected/one.txt.out",
        out_count: "tests/expected/one.txt.c.out",
    };

    const TWO: Test = Test {
        input: "tests/inputs/two.txt",
        out: "tests/expected/two.txt.out",
        out_count: "tests/expected/two.txt.c.out",
    };

    const THREE: Test = Test {
        input: "tests/inputs/three.txt",
        out: "tests/expected/three.txt.out",
        out_count: "tests/expected/three.txt.c.out",
    };

    const SKIP: Test = Test {
        input: "tests/inputs/skip.txt",
        out: "tests/expected/skip.txt.out",
        out_count: "tests/expected/skip.txt.c.out",
    };

    const T1: Test = Test {
        input: "tests/inputs/t1.txt",
        out: "tests/expected/t1.txt.out",
        out_count: "tests/expected/t1.txt.c.out",
    };

    const T2: Test = Test {
        input: "tests/inputs/t2.txt",
        out: "tests/expected/t2.txt.out",
        out_count: "tests/expected/t2.txt.c.out",
    };

    const T3: Test = Test {
        input: "tests/inputs/t3.txt",
        out: "tests/expected/t3.txt.out",
        out_count: "tests/expected/t3.txt.c.out",
    };

    const T4: Test = Test {
        input: "tests/inputs/t4.txt",
        out: "tests/expected/t4.txt.out",
        out_count: "tests/expected/t4.txt.c.out",
    };

    const T5: Test = Test {
        input: "tests/inputs/t5.txt",
        out: "tests/expected/t5.txt.out",
        out_count: "tests/expected/t5.txt.c.out",
    };

    const T6: Test = Test {
        input: "tests/inputs/t6.txt",
        out: "tests/expected/t6.txt.out",
        out_count: "tests/expected/t6.txt.c.out",
    };

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
    fn dies_bad_file() -> TestResult {
        let bad = gen_bad_file();
        let expected = format!("{}: .* [(]os error 2[)]", bad);
        Command::cargo_bin(PRG)?
            .arg(bad)
            .assert()
            .failure()
            .stderr(predicate::str::is_match(expected)?);
        Ok(())
    }

    // --------------------------------------------------
    // HELPER FUNCTIONS
    fn run(test: &Test) -> TestResult {
        let expected = fs::read_to_string(test.out)?;
        Command::cargo_bin(PRG)?
            .arg(test.input)
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    // --------------------------------------------------
    fn run_count(test: &Test) -> TestResult {
        let expected = fs::read_to_string(test.out_count)?;
        Command::cargo_bin(PRG)?
            .args(&[test.input, "-c"])
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    // --------------------------------------------------
    fn run_stdin(test: &Test) -> TestResult {
        let input = fs::read_to_string(test.input)?;
        let expected = fs::read_to_string(test.out)?;
        Command::cargo_bin(PRG)?
            .write_stdin(input)
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    // --------------------------------------------------
    fn run_stdin_count(test: &Test) -> TestResult {
        let input = fs::read_to_string(test.input)?;
        let expected = fs::read_to_string(test.out_count)?;
        Command::cargo_bin(PRG)?
            .arg("--count")
            .write_stdin(input)
            .assert()
            .success()
            .stdout(expected);
        Ok(())
    }

    // --------------------------------------------------
    fn run_outfile(test: &Test) -> TestResult {
        let expected = fs::read_to_string(test.out)?;
        let outfile = NamedTempFile::new()?;
        let outpath = &outfile.path().to_str().unwrap();
        Command::cargo_bin(PRG)?
            .args(&[test.input, outpath])
            .assert()
            .success()
            .stdout("");

        let contents = fs::read_to_string(&outpath)?;
        assert_eq!(&expected, &contents);

        Ok(())
    }

    // --------------------------------------------------
    fn run_outfile_count(test: &Test) -> TestResult {
        let outfile = NamedTempFile::new()?;
        let outpath = &outfile.path().to_str().unwrap();

        Command::cargo_bin(PRG)?
            .args(&[test.input, outpath, "--count"])
            .assert()
            .success()
            .stdout("");

        let expected = fs::read_to_string(test.out_count)?;
        let contents = fs::read_to_string(&outpath)?;
        assert_eq!(&expected, &contents);

        Ok(())
    }

    // --------------------------------------------------
    fn run_stdin_outfile_count(test: &Test) -> TestResult {
        let input = fs::read_to_string(test.input)?;
        let outfile = NamedTempFile::new()?;
        let outpath = &outfile.path().to_str().unwrap();

        Command::cargo_bin(PRG)?
            .args(&["-", outpath, "-c"])
            .write_stdin(input)
            .assert()
            .stdout("");

        let expected = fs::read_to_string(test.out_count)?;
        let contents = fs::read_to_string(&outpath)?;
        assert_eq!(&expected, &contents);

        Ok(())
    }

    // --------------------------------------------------
    #[test]
    fn empty() -> TestResult {
        run(&EMPTY)
    }

    #[test]
    fn empty_count() -> TestResult {
        run_count(&EMPTY)
    }

    #[test]
    fn empty_stdin() -> TestResult {
        run_stdin(&EMPTY)
    }

    #[test]
    fn empty_stdin_count() -> TestResult {
        run_stdin_count(&EMPTY)
    }

    #[test]
    fn empty_outfile() -> TestResult {
        run_outfile(&EMPTY)
    }

    #[test]
    fn empty_outfile_count() -> TestResult {
        run_outfile_count(&EMPTY)
    }

    #[test]
    fn empty_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&EMPTY)
    }

    // --------------------------------------------------
    #[test]
    fn one() -> TestResult {
        run(&ONE)
    }

    #[test]
    fn one_count() -> TestResult {
        run_count(&ONE)
    }

    #[test]
    fn one_stdin() -> TestResult {
        run_stdin(&ONE)
    }

    #[test]
    fn one_stdin_count() -> TestResult {
        run_stdin_count(&ONE)
    }

    #[test]
    fn one_outfile() -> TestResult {
        run_outfile(&ONE)
    }

    #[test]
    fn one_outfile_count() -> TestResult {
        run_outfile_count(&ONE)
    }

    #[test]
    fn one_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&ONE)
    }

    // --------------------------------------------------
    #[test]
    fn two() -> TestResult {
        run(&TWO)
    }

    #[test]
    fn two_count() -> TestResult {
        run_count(&TWO)
    }

    #[test]
    fn two_stdin() -> TestResult {
        run_stdin(&TWO)
    }

    #[test]
    fn two_stdin_count() -> TestResult {
        run_stdin_count(&TWO)
    }

    #[test]
    fn two_outfile() -> TestResult {
        run_outfile(&TWO)
    }

    #[test]
    fn two_outfile_count() -> TestResult {
        run_outfile_count(&TWO)
    }

    #[test]
    fn two_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&TWO)
    }

    // --------------------------------------------------
    #[test]
    fn three() -> TestResult {
        run(&THREE)
    }

    #[test]
    fn three_count() -> TestResult {
        run_count(&THREE)
    }

    #[test]
    fn three_stdin() -> TestResult {
        run_stdin(&THREE)
    }

    #[test]
    fn three_stdin_count() -> TestResult {
        run_stdin_count(&THREE)
    }

    #[test]
    fn three_outfile() -> TestResult {
        run_outfile(&THREE)
    }

    #[test]
    fn three_outfile_count() -> TestResult {
        run_outfile_count(&THREE)
    }

    #[test]
    fn three_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&THREE)
    }

    // --------------------------------------------------
    #[test]
    fn skip() -> TestResult {
        run(&SKIP)
    }

    #[test]
    fn skip_count() -> TestResult {
        run_count(&SKIP)
    }

    #[test]
    fn skip_stdin() -> TestResult {
        run_stdin(&SKIP)
    }

    #[test]
    fn skip_stdin_count() -> TestResult {
        run_stdin_count(&SKIP)
    }

    #[test]
    fn skip_outfile() -> TestResult {
        run_outfile(&SKIP)
    }

    #[test]
    fn skip_outfile_count() -> TestResult {
        run_outfile_count(&SKIP)
    }

    #[test]
    fn skip_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&SKIP)
    }

    // --------------------------------------------------
    #[test]
    fn t1() -> TestResult {
        run(&T1)
    }

    #[test]
    fn t1_count() -> TestResult {
        run_count(&T1)
    }

    #[test]
    fn t1_stdin() -> TestResult {
        run_stdin(&T1)
    }

    #[test]
    fn t1_stdin_count() -> TestResult {
        run_stdin_count(&T1)
    }

    #[test]
    fn t1_outfile() -> TestResult {
        run_outfile(&T1)
    }

    #[test]
    fn t1_outfile_count() -> TestResult {
        run_outfile_count(&T1)
    }

    #[test]
    fn t1_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&T1)
    }

    // --------------------------------------------------
    #[test]
    fn t2() -> TestResult {
        run(&T2)
    }

    #[test]
    fn t2_count() -> TestResult {
        run_count(&T2)
    }

    #[test]
    fn t2_stdin() -> TestResult {
        run_stdin(&T2)
    }

    #[test]
    fn t2_stdin_count() -> TestResult {
        run_stdin_count(&T2)
    }

    #[test]
    fn t2_outfile() -> TestResult {
        run_outfile(&T2)
    }

    #[test]
    fn t2_outfile_count() -> TestResult {
        run_outfile_count(&T2)
    }

    #[test]
    fn t2_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&T2)
    }

    // --------------------------------------------------
    #[test]
    fn t3() -> TestResult {
        run(&T3)
    }

    #[test]
    fn t3_count() -> TestResult {
        run_count(&T3)
    }

    #[test]
    fn t3_stdin() -> TestResult {
        run_stdin(&T3)
    }

    #[test]
    fn t3_stdin_count() -> TestResult {
        run_stdin_count(&T3)
    }

    #[test]
    fn t3_outfile() -> TestResult {
        run_outfile(&T3)
    }

    #[test]
    fn t3_outfile_count() -> TestResult {
        run_outfile_count(&T3)
    }

    #[test]
    fn t3_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&T3)
    }

    // --------------------------------------------------
    #[test]
    fn t4() -> TestResult {
        run(&T4)
    }

    #[test]
    fn t4_count() -> TestResult {
        run_count(&T4)
    }

    #[test]
    fn t4_stdin() -> TestResult {
        run_stdin(&T4)
    }

    #[test]
    fn t4_stdin_count() -> TestResult {
        run_stdin_count(&T4)
    }

    #[test]
    fn t4_outfile() -> TestResult {
        run_outfile(&T4)
    }

    #[test]
    fn t4_outfile_count() -> TestResult {
        run_outfile_count(&T4)
    }

    #[test]
    fn t4_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&T4)
    }

    // --------------------------------------------------
    #[test]
    fn t5() -> TestResult {
        run(&T5)
    }

    #[test]
    fn t5_count() -> TestResult {
        run_count(&T5)
    }

    #[test]
    fn t5_stdin() -> TestResult {
        run_stdin(&T5)
    }

    #[test]
    fn t5_stdin_count() -> TestResult {
        run_stdin_count(&T5)
    }

    #[test]
    fn t5_outfile() -> TestResult {
        run_outfile(&T5)
    }

    #[test]
    fn t5_outfile_count() -> TestResult {
        run_outfile_count(&T5)
    }

    #[test]
    fn t5_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&T5)
    }

    // --------------------------------------------------
    #[test]
    fn t6() -> TestResult {
        run(&T6)
    }

    #[test]
    fn t6_count() -> TestResult {
        run_count(&T6)
    }

    #[test]
    fn t6_stdin() -> TestResult {
        run_stdin(&T6)
    }

    #[test]
    fn t6_stdin_count() -> TestResult {
        run_stdin_count(&T6)
    }

    #[test]
    fn t6_outfile() -> TestResult {
        run_outfile(&T6)
    }

    #[test]
    fn t6_outfile_count() -> TestResult {
        run_outfile_count(&T6)
    }

    #[test]
    fn t6_stdin_outfile_count() -> TestResult {
        run_stdin_outfile_count(&T6)
    }
}
