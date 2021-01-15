/*

http://rosalind.info/problems/ba1b/

Find all the most frequent k-mers

Sample:
ACGTTGCATGTCGCATGATGCATGAGAGCT
4

CATG GCAT

*/


extern crate rosalind_rust;

use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use structopt::StructOpt;


// https://github.com/dib-lab/bioinf_algorithms/

pub fn most_frequent(text: &[u8], k: usize) -> Vec<String> {
    let mut kmers = HashMap::new();
    let mut maxval = 0;
    for kmer in text.windows(k as usize) {
        let counter = kmers.entry(kmer).or_insert(0);
        *counter += 1;
        maxval = std::cmp::max(maxval, *counter);
    }

    kmers
        .iter()
        .filter_map(|(&kmer, &count)| {
            if count == maxval {
                Some(String::from_utf8(kmer.to_vec()).unwrap())
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn test_ba1b() {
    let seq = b"ACGTTGCATGTCGCATGATGCATGAGAGCT";
    let k = 4;
    let mut res = most_frequent(seq, k);
    res.sort();
    assert_eq!(res, vec!["CATG", "GCAT"]);
}

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    let f = std::fs::File::open(&args.path)?;
    let reader = BufReader::new(f);
    let mut iter = reader.lines();
    let text = iter.next().unwrap().unwrap();
    let k = iter.next().unwrap().unwrap();
    let res = most_frequent(&text.as_bytes(), k.parse::<usize>().unwrap());
    println!("{}", res.join(" "));
    Ok(())
}
