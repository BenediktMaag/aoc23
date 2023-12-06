use crate::util::str_parsing::StringParser;
use crate::Parts;
use rayon::prelude::*;

pub fn execute(input: &str, part: &Parts) {
    let races = Races::from_str(input);
    let solution: u64 = match part {
        Parts::Part1 => races.multiplied_winning(),
        Parts::Part2 => races.multiplied_winning(),
    };

    println!("Solution is {}", solution);
}

#[derive(Debug)]
struct Races {
    content: Vec<(u64, u64)>,
}

impl Races {
    pub fn from_str(input: &str) -> Self {
        let mut parser = StringParser::new(input.chars());
        parser.dismiss_str("Time:");
        let time: Vec<u64> = parser.parse_num_vec_until_newline();
        parser.dismiss_str("Distance:");
        let distance: Vec<u64> = parser.parse_num_vec_until_newline();

        Races {
            content: time.into_iter().zip(distance.into_iter()).collect(),
        }
    }

    pub fn multiplied_winning(&self) -> u64 {
        self.content
            .iter()
            .map(|(t, d)| Races::calculate_winning_posibilities(&t, &d))
            .product()
    }

    fn calculate_winning_posibilities(time: &u64, distance: &u64) -> u64 {
        (0..=*time)
            .into_par_iter()
            .map(|speed| speed * (time - speed))
            .filter(|res| res > &distance)
            .count() as u64
    }
}
