use std::fs;
use std::time::Instant;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

pub struct Puzzle {
    pub day: Day,
    pub part: Parts,
    pub input_type: InputType,
}

impl Puzzle {
    fn as_path(&self) -> String {
        format!(
            "input/{}/{}_{}.txt",
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
            (Day::Day2, part) => day2::execute(&input, part),
            (Day::Day3, part) => day3::execute(&input, part),
            (Day::Day4, part) => day4::execute(&input, part),
            (Day::Day5, part) => day5::execute(&input, part),
            (Day::Day6, part) => day6::execute(&input, part),
        }
        if start.elapsed().as_secs() >= 5 {
            println!("Execution took {}s", start.elapsed().as_secs())
        } else if start.elapsed().as_millis() >= 5 {
            println!("Execution took {} ms", start.elapsed().as_millis())
        } else if start.elapsed().as_micros() >= 5 {
            println!("Execution took {} µs", start.elapsed().as_micros())
        }
    }
}

#[allow(dead_code)]
pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
}

impl Day {
    fn as_str(&self) -> &str {
        match self {
            Self::Day1 => "day1",
            Self::Day2 => "day2",
            Self::Day3 => "day3",
            Self::Day4 => "day4",
            Self::Day5 => "day5",
            Self::Day6 => "day6",
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
