use std::{fmt::Debug, fs, str::FromStr};

pub fn parse_input<T: FromStr>(file_path: &str) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    let contents = fs::read_to_string(file_path).expect("Could not read file.");

    contents
        .lines()
        .map(|value| value.parse::<T>().unwrap())
        .collect::<Vec<T>>()
}
