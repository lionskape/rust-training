#![forbid(unsafe_code)]

use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

fn file_lines(file: std::fs::File) -> Vec<String> {
    BufReader::new(file)
        .lines()
        .filter_map(|s| s.ok())
        .collect()
}

fn find_common_lines(a: Vec<String>, b: Vec<String>) -> HashSet<String> {
    a.into_iter().filter(|e| b.contains(e)).collect()
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    assert_eq!(args.len(), 3);
    let file1 = std::fs::File::open(&args[1]).unwrap();
    let file2 = std::fs::File::open(&args[2]).unwrap();
    let uniq_lines = find_common_lines(file_lines(file1), file_lines(file2));
    for line in uniq_lines {
        println!("{}", line);
    }
}
