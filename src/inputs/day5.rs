use crate::util::str_parsing::StringParser;
use crate::Parts;
use itertools::Itertools;
use rayon::prelude::*;
use std::ops::Range;

pub fn execute(input: &str, part: &Parts) {
    let seeding = Seeding::from_str(input, part);
    let solution: u64 = match part {
        Parts::Part1 => seeding.converted_seeds().into_iter().min().unwrap(),
        Parts::Part2 => {
            let mut location: u64 = 0;
            let seed_iter = seeding.seeds_ranges.iter();
            loop {
                // print!("{location}: ");
                let seed = seeding.reverse_convert(location);
                // print!("{seed}\n");
                if seed_iter
                    .clone()
                    .map(|x| x.contains(&seed))
                    .any(|x| x == true)
                {
                    break location;
                }
                location += 1;
            }
        }
    };

    println!("Solution is {}", solution);
}

#[derive(Debug)]
struct Seeding {
    seeds: Vec<u64>,
    seeds_ranges: Vec<std::ops::Range<u64>>,
    mappings: Vec<Mapping>,
}

impl Seeding {
    pub fn converted_seeds(&self) -> Vec<u64> {
        self.seeds.iter().map(|x| self.convert_seed(*x)).collect()
    }

    fn convert_seed(&self, seed: u64) -> u64 {
        let mut var = seed;
        // print!("{} ", &var);
        for map in &self.mappings {
            var = map.convert(var);
            // print!("{} ", &var);
        }
        // println!();
        var
    }

    fn reverse_convert(&self, seed: u64) -> u64 {
        let mut var = seed;
        // print!("{} ", &var);
        for map in self.mappings.iter().rev() {
            var = map.reverse_convert(var);
            // print!("{} ", &var);
        }
        // println!();
        var
    }

    pub fn from_str(input: &str, part: &Parts) -> Self {
        let mut lines = input.lines();
        let mut first_line = StringParser::new(lines.next().unwrap().chars());
        first_line.dismiss_str("seeds: ");
        let seeds: Vec<u64> = first_line.parse_num_vec_until_newline();
        lines.next();
        let mut mappings = vec![];
        loop {
            let mut mapping = Mapping::new();
            match lines.next() {
                Some(_) => loop {
                    match lines.next() {
                        Some("") | None => {
                            mappings.push(mapping);
                            break;
                        }
                        Some(line) => mapping.feed_line(line),
                    }
                },
                None => break,
            }
        }
        match part {
            Parts::Part1 => Seeding {
                seeds,
                seeds_ranges: vec![],
                mappings,
            },
            Parts::Part2 => {
                let seeds_vector: Vec<Range<u64>> = seeds
                    .into_iter()
                    .tuples::<(u64, u64)>()
                    .map(|(start, size)| (start..start + size))
                    .collect();
                Seeding {
                    seeds: vec![],
                    seeds_ranges: seeds_vector,
                    mappings,
                }
            }
        }
    }
}

#[derive(Debug)]
struct Mapping {
    maps: Vec<ConversionRange<u64, i64>>,
}

impl Mapping {
    pub fn new() -> Self {
        Mapping { maps: vec![] }
    }

    pub fn feed_line(&mut self, line: &str) {
        let mut parser = StringParser::new(line.chars());
        let destination_start: u64 = parser.parse_num();
        parser.dismiss_any_whitespace();
        let source_start: u64 = parser.parse_num();
        parser.dismiss_any_whitespace();
        let size: u64 = parser.parse_num();
        self.maps.push(ConversionRange {
            range: source_start..source_start + size,
            reverse_range: destination_start..destination_start + size,
            offset: (destination_start as i64 - source_start as i64),
        })
    }

    pub fn convert(&self, input: u64) -> u64 {
        for map in &self.maps {
            if map.range.contains(&input) {
                return (input as i64 + map.offset) as u64;
            }
        }
        input
    }

    pub fn reverse_convert(&self, input: u64) -> u64 {
        for map in &self.maps {
            if map.reverse_range.contains(&input) {
                return (input as i64 - map.offset) as u64;
            }
        }
        input
    }
}

#[derive(Debug)]
struct ConversionRange<R, O> {
    pub range: Range<R>,
    pub reverse_range: Range<R>,
    pub offset: O,
}
