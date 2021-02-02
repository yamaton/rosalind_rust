/*
http://rosalind.info/problems/ba1c/

Find the reverse complement of a string

Sample:
AAAACCCGGT

ACCGGGTTTT

*/

extern crate rosalind_rust;
use rosalind_rust::Cli;
use std::io::{prelude::*, BufReader};
use structopt::StructOpt;

fn revcomp(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A' => 'T',
            'T' => 'A',
            'G' => 'C',
            'C' => 'G',
            _ => 'N',
        })
        .rev()
        .collect::<String>()
}

#[test]
fn test_ba1c() {
    let seq = String::from("AAAACCCGGT");
    assert_eq!(revcomp(&seq), String::from("ACCGGGTTTT"));
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let f = std::fs::File::open(&args.path)?;
    let reader = BufReader::new(f);
    let mut iter = reader.lines();
    let text = iter.next().unwrap().unwrap();
    let res = revcomp(&text);
    println!("{}", res);
    Ok(())
}
