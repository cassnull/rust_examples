#![feature(test)]

extern crate test;

use std::error::Error;
use std::fs;
use test::Bencher;

#[allow(dead_code)]
fn search_iter<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[allow(dead_code)]
fn search_for<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[bench]
fn bench_search_for(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("The Adventures of Sherlock Holmes.txt")?;

    b.iter(|| search_for("the", &contents));

    Ok(())
}

#[bench]
fn bench_search_iter(b: &mut Bencher) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string("The Adventures of Sherlock Holmes.txt")?;

    b.iter(|| search_iter("the", &contents));

    Ok(())
}
