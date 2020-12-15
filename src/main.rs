mod day1;
mod day2;
mod parsing;

fn main() {
    let result = day2::count_valid_passwords(day2::Validator::Toboggan);

    println!("{0}", result);
}
