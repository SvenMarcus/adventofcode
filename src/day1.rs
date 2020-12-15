use crate::parsing;

pub fn run_day_one() {
    let input = parsing::parse_input::<i32>("resources/day1/input.txt");
    let product: i32;
    match find_three_integers_for_sum(&input, &2020) {
        Some(result) => product = result.0 * result.1 * result.2,
        None => {
            println!("No result");
            return;
        }
    };

    println!("The product is {0}", product);
}

pub fn find_three_integers_for_sum(input: &Vec<i32>, wanted_sum: &i32) -> Option<(i32, i32, i32)> {
    let mut copied_input = input.clone();
    copied_input.sort();
    for element in copied_input.iter() {
        let difference = *wanted_sum - element;
        match find_two_integers_for_sum_with_sorted_input(&copied_input, &difference) {
            Some(result) => return Some((result.0, result.1, *element)),
            None => continue,
        };
    }

    None
}

pub fn find_two_integers_for_sum(input: &Vec<i32>, wanted_sum: &i32) -> Option<(i32, i32)> {
    let mut copied_input = input.clone();
    copied_input.sort();
    find_two_integers_for_sum_with_sorted_input(&copied_input, wanted_sum)
}

fn find_two_integers_for_sum_with_sorted_input(
    sorted_input: &Vec<i32>,
    wanted_sum: &i32,
) -> Option<(i32, i32)> {
    let mut left_ptr = 0;
    let mut right_ptr = sorted_input.len() - 1;

    while left_ptr < right_ptr {
        let left_value = sorted_input[left_ptr];
        let right_value = sorted_input[right_ptr];
        let sum = left_value + right_value;

        if sum == *wanted_sum {
            return Some((left_value, right_value));
        }

        if sum > *wanted_sum {
            right_ptr -= 1;
            continue;
        }

        if sum < *wanted_sum {
            left_ptr += 1;
            continue;
        }
    }

    None
}
