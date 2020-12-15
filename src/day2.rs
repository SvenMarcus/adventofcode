pub fn is_valid_password(
    lower_bound: &i32,
    upper_bound: &i32,
    wanted_char: &char,
    password: &str,
) -> bool {
    let count = find_number_of_occurences(wanted_char, password);
    return lower_bound <= &count && upper_bound >= &count;
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
