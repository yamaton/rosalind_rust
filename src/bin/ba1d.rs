/*

Find location of all occurences with 0-based indexing
http://rosalind.info/problems/ba1d/

Sample:
ATAT
GATATATGCATATACTT


*/
extern crate rosalind_rust;
use rosalind_rust::Cli;
use structopt::StructOpt;
use std::io::{prelude::*, BufReader};


fn find_occurences(patt: &[u8], text: &[u8]) -> Vec<usize> {
    let n = patt.len();
    text.windows(n)
        .enumerate()
        .filter(|(_, x)| x == &patt)
        .map(|(i, _)| i)
        .collect()
}

#[test]
fn test_ba1d() {
    let indices = find_occurences(b"ATAT",  b"GATATATGCATATACTT");
    assert_eq!(indices, vec![1, 3, 9]);
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let f = std::fs::File::open(&args.path)?;
    let reader = BufReader::new(f);
    let mut iter = reader.lines();
    let pattern = iter.next().unwrap().unwrap();
    let text = iter.next().unwrap().unwrap();
    let res = find_occurences(&pattern.as_bytes(), &text.as_bytes());
    println!("{}", res.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    Ok(())
}
