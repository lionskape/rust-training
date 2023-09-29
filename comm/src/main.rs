#![forbid(unsafe_code)]
use std::{collections::HashSet, io::BufRead};

fn read_file_to_set(set: &mut HashSet<String>, path: String) {
    let file = std::fs::File::open(path).unwrap();
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            set.insert(line);
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut set1: HashSet<String> = HashSet::new();
    let mut set2: HashSet<String> = HashSet::new();

    if let Some(path_file_1) = args.get(1) {
        read_file_to_set(&mut set1, path_file_1.to_owned());
    }

    if let Some(path_file_2) = args.get(2) {
        read_file_to_set(&mut set2, path_file_2.to_owned());
    }

    for line in (&set1 & &set2).iter() {
        println!("{}", line);
    }
}
