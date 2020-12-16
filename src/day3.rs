use crate::parsing;

pub fn run_day3_part1() -> i64 {
    let field = parsing::parse_input::<String>("resources/day3/input.txt");
    count_trees_during_traversal(&field, 3, 1)
}

pub fn run_day3_part2() -> i64 {
    let field = parsing::parse_input::<String>("resources/day3/input.txt");
    let mut product_of_tree_numbers = 1;

    let step_combinations = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    for (horizontal_step, vertical_step) in step_combinations.iter() {
        product_of_tree_numbers *= count_trees_during_traversal(&field, *horizontal_step, *vertical_step);
    }

    product_of_tree_numbers
}

pub fn count_trees_during_traversal(field: &Vec<String>, horizontal_step: i32, vertical_step: i32) -> i64 {
    let mut current_position = (0, 0);
    let mut tree_count = 0;

    loop {
        current_position = travel(field, &current_position, horizontal_step, vertical_step);

        if current_position.1 >= field.len() as i32 {
            break;
        }

        tree_count += is_tree_at_position(field, &current_position) as i32;
    }

    tree_count as i64
}

fn travel(field: &Vec<String>, current_position: &(i32, i32), horizontal_step: i32, vertical_step: i32) -> (i32, i32) {
    let mut x = current_position.0;
    let mut y = current_position.1;

    x += horizontal_step;
    let number_of_chars_in_row = field[0].chars().count() as i32;
    if x >= number_of_chars_in_row {
        x -= number_of_chars_in_row;
    }

    y += vertical_step;

    (x, y)
}

fn is_tree_at_position(field: &Vec<String>, current_position: &(i32, i32)) -> bool {
    let row = &field[current_position.1 as usize];
    let character = char::from(row.as_bytes()[current_position.0 as usize]);

    character == '#'
}
