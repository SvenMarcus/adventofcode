use std::str::FromStr;

use crate::parsing;

pub enum Validator {
    SledRental,
    Toboggan,
}

struct PasswordRule {
    min_occurences: i32,
    max_occurences: i32,
    required_character: char,
}

pub fn count_valid_passwords(validator: Validator) -> i32 {
    let validation_fn = match validator {
        Validator::SledRental => is_valid_password_for_sled_rental,
        Validator::Toboggan => is_valid_password_for_toboggan,
    };

    let lines = parsing::parse_input::<String>("resources/day2/input.txt");
    let mut count = 0;
    for line in lines.iter() {
        if is_password_on_line_valid(line, validation_fn) {
            count += 1;
        }
    }

    count
}

fn is_password_on_line_valid(line: &str, validator: fn(&PasswordRule, &str) -> bool) -> bool {
    let rule = match parse_password_rule_from_line(line) {
        Some(_rule) => _rule,
        _ => return false,
    };

    let password = match parse_password_from_line(line) {
        Some(_password) => _password,
        _ => return false,
    };

    validator(&rule, password)
}

fn parse_password_rule_from_line(line: &str) -> Option<PasswordRule> {
    let first_index = line.find('-')?;
    let second_index = line.find(' ')?;
    let third_index = line.find(":")?;

    let lower_bound = parse_from_str::<i32>(&line[..first_index])?;
    let upper_bound = parse_from_str::<i32>(&line[first_index + 1..second_index])?;
    let required_char = parse_from_str::<char>(&line[third_index - 1..third_index])?;

    Some(PasswordRule {
        min_occurences: lower_bound,
        max_occurences: upper_bound,
        required_character: required_char,
    })
}

fn parse_from_str<T: FromStr>(string: &str) -> Option<T> {
    match string.parse::<T>() {
        Ok(value) => Some(value),
        _ => None
    }
}

fn parse_password_from_line(line: &str) -> Option<&str> {
    let index = match line.find(": ") {
        Some(number) => number,
        _ => return None,
    };

    Some(&line[index + 1..])
}

fn is_valid_password_for_sled_rental(password_rule: &PasswordRule, password: &str) -> bool {
    let count = find_number_of_occurences(&password_rule.required_character, password);

    password_rule.min_occurences <= count && password_rule.max_occurences >= count
}

fn is_valid_password_for_toboggan(password_rule: &PasswordRule, password: &str) -> bool {
    let pw_bytes = password.as_bytes();
    let first_char = pw_bytes[password_rule.min_occurences as usize] as char;
    let second_char = pw_bytes[password_rule.max_occurences as usize] as char;

    let required_char = password_rule.required_character;
    first_char != second_char && (first_char == required_char || second_char == required_char)
}

fn find_number_of_occurences(wanted_char: &char, password: &str) -> i32 {
    let mut count = 0;
    let mut index = 0;
    let mut slice = &password[index..];
    loop {
        index = match slice.find(*wanted_char) {
            Some(next_index) => next_index + 1,
            None => break,
        };

        count += 1;
        slice = &slice[index..];
    }

    count
}
