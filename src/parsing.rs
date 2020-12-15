use std::{fmt::Debug, fs, str::FromStr};

pub fn parse_input<T: FromStr>(file_path: &str) -> Vec<T> where <T as FromStr>::Err: Debug {
    let contents = fs::read_to_string(file_path).expect("Could not read file.");
    let mut split_contents = Vec::<T>::new();
    for line in contents.lines() {
        split_contents.push(line.parse().unwrap());
    }

    split_contents
}