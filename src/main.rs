mod inputs;

use inputs::{Day, InputType, Parts, Puzzle};

fn main() {
    let day = Day::Day1;
    let part = Parts::Part2;
    let input_type = InputType::Example;
    Puzzle {
        day,
        part,
        input_type,
    }
    .execute()
}
