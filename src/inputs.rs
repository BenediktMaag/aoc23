use std::fs;
use std::time::Instant;
mod day1;

pub struct Puzzle {
    pub day: Day,
    pub part: Parts,
    pub input_type: InputType,
}

impl Puzzle {
    fn as_path(&self) -> String {
        format!(
            "input/{}/{}_{}_{}.txt",
            self.day.as_str(),
            self.day.as_str(),
            self.part.as_str(),
            self.input_type.as_str()
        )
    }

    pub fn execute(&self) {
        println!(
            "Executing {} {} with {}",
            self.day.as_str(),
            self.part.as_str(),
            self.input_type.as_str()
        );
        let start = Instant::now();
        let input = fs::read_to_string(self.as_path()).expect("Couldnt open input file");

        match (&self.day, &self.part) {
            (Day::Day1, part) => day1::execute(&input, part),
        }

        println!("Execution took {} ms", start.elapsed().as_millis())
    }
}

pub enum Day {
    Day1,
}

impl Day {
    fn as_str(&self) -> &str {
        match self {
            Self::Day1 => "day1",
        }
    }
}

#[allow(dead_code)]
pub enum Parts {
    Part1,
    Part2,
}

impl Parts {
    fn as_str(&self) -> &str {
        match self {
            Self::Part1 => "part1",
            Self::Part2 => "part2",
        }
    }
}

#[allow(dead_code)]
pub enum InputType {
    Example,
    Input,
}

impl InputType {
    fn as_str(&self) -> &str {
        match self {
            Self::Example => "example",
            Self::Input => "input",
        }
    }
}
