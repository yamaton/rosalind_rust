/*
Computer the number of times a pattern appears in a text
http://rosalind.info/problems/ba1a/

NOTE: Take care of overlapping. i.e. pattern_count("ATATA", "ATA") == 2

Input file should contain two lines; the first is a text, and the second is a pattern to look for.

*/
extern crate rosalind_rust;
use rosalind_rust::Cli;
use std::io::{prelude::*, BufReader};
use structopt::StructOpt;

pub fn pattern_count(text: &[u8], pattern: &[u8]) -> i32 {
    let k = pattern.len();
    return text.windows(k).filter(|&seg| seg == pattern).count() as i32;
}

#[test]
fn test_ba1a() {
    let seq = b"GCGCG";
    let pat = b"GCG";
    assert_eq!(pattern_count(seq, pat), 2);
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let f = std::fs::File::open(&args.path)?;
    let reader = BufReader::new(f);
    let mut iter = reader.lines();
    let content = iter.next().unwrap().unwrap();
    let pattern = iter.next().unwrap().unwrap();
    let count = pattern_count(&content.as_bytes(), &pattern.as_bytes());

    println!("{}: {}", &pattern, count);
    Ok(())
}
