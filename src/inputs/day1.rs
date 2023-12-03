use std::collections::HashMap;

use crate::Parts;

pub fn execute(input: &str, part: &Parts) {
    let lines = input.lines();

    let solution: u32 = match part {
        Parts::Part1 => lines.map(|x| calc_number_in_str(x)).sum(),
        Parts::Part2 => lines
            .map(|x| add_numbers_to_string_numbers(x))
            .map(|x| calc_number_in_str(&x))
            .sum(),
    };

    println!("Solution is {}", solution);
}

fn calc_number_in_str(input: &str) -> u32 {
    let first_digit: u32 = u32::from_str_radix(&find_number_in_str(input.chars()), 10).unwrap();
    let reversed = input.chars().rev().collect::<String>();
    let second_digit: u32 = u32::from_str_radix(&find_number_in_str(reversed.chars()), 10).unwrap();

    first_digit * 10 + second_digit
}

static NUMS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn find_number_in_str(input: std::str::Chars) -> String {
    for c in input {
        if NUMS.contains(&c) {
            return c.to_string();
        }
    }
    panic!("Couldnt find number in str")
}

fn add_numbers_to_string_numbers(input: &str) -> String {
    let string_mapping: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut ret = String::with_capacity(16);

    for c in input.chars() {
        ret.push(c);
        if string_mapping
            .keys()
            .map(|x| ret.contains(x))
            .max()
            .is_some()
        {}
    }
    ret
}
