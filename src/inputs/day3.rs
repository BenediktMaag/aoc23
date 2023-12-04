use crate::Parts;
use std::collections::HashSet;

pub fn execute(input: &str, part: &Parts) {
    let solution: u32 = match part {
        Parts::Part1 => {
            let field = Field::from_str(input);
            field.get_part_numbers().iter().sum()
        }
        Parts::Part2 => {
            let field = GearField::from_str(input);
            field.get_gear_numbers().iter().sum()
        }
    };

    println!("Solution is {}", solution);
}

#[derive(Debug)]
struct Field {
    symbols: HashSet<(u32, u32)>,
    numbers: Vec<Number>,
}

impl Field {
    pub fn from_str(input: &str) -> Self {
        let mut symbols = HashSet::new();
        let mut numbers = vec![];

        let mut chars = input.chars().peekable();
        let mut x = 1;
        let mut row = 1;
        while let Some(c) = chars.next() {
            match c {
                '1'..='9' => {
                    let mut number = c.to_string();
                    let start = x;
                    while let Some('0'..='9') = chars.peek() {
                        number.push(chars.next().unwrap());
                        x += 1;
                    }
                    let end = x;
                    x += 1;
                    let id = u32::from_str_radix(&number, 10).unwrap();
                    numbers.push(Number {
                        id,
                        row,
                        range: (start, end),
                    })
                }
                '.' => x += 1,
                '\r' | '\n' => {
                    if let Some('\r' | '\n') = chars.peek() {
                        chars.next().unwrap();
                    }
                    row += 1;
                    x = 1;
                }
                _ => {
                    symbols.insert((x, row));
                    x += 1;
                }
            }
        }

        Field { symbols, numbers }
    }

    pub fn get_part_numbers(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|x| x.is_part(&self.symbols))
            .map(|x| x.id)
            .collect()
    }
}

#[derive(Debug)]
struct Gear {
    row: u32,
    x: u32,
}

impl Gear {
    pub fn get_gear_number_if_valid(&self, numbers: &Vec<GearNumber>) -> u32 {
        let valid_numbers: Vec<u32> = numbers
            .iter()
            .filter(|x| x.coordinates.contains(&(self.row, self.x)))
            .map(|x| x.id)
            .collect();
        if valid_numbers.len() == 2 {
            valid_numbers[0] * valid_numbers[1]
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct GearField {
    possible_gears: Vec<Gear>,
    numbers: Vec<GearNumber>,
}

impl GearField {
    pub fn from_str(input: &str) -> Self {
        let mut possible_gears = Vec::new();
        let mut numbers = vec![];

        let mut chars = input.chars().peekable();
        let mut x = 1;
        let mut row = 1;
        while let Some(c) = chars.next() {
            match c {
                '1'..='9' => {
                    let mut number = c.to_string();
                    let start = x;
                    while let Some('0'..='9') = chars.peek() {
                        number.push(chars.next().unwrap());
                        x += 1;
                    }
                    let end = x;
                    x += 1;
                    let id = u32::from_str_radix(&number, 10).unwrap();
                    let mut coordinates = HashSet::new();
                    for column in start - 1..=end + 1 {
                        for local_row in row - 1..=row + 1 {
                            coordinates.insert((local_row, column));
                        }
                    }
                    numbers.push(GearNumber { id, coordinates })
                }
                '*' => {
                    possible_gears.push(Gear { row, x });
                    x += 1
                }
                '\r' | '\n' => {
                    if let Some('\r' | '\n') = chars.peek() {
                        chars.next().unwrap();
                    }
                    row += 1;
                    x = 1;
                }
                _ => {
                    x += 1;
                }
            }
        }

        GearField {
            possible_gears,
            numbers,
        }
    }

    pub fn get_gear_numbers(&self) -> Vec<u32> {
        self.possible_gears
            .iter()
            .map(|x| x.get_gear_number_if_valid(&self.numbers))
            .collect()
    }
}

#[derive(Debug)]
struct Number {
    pub id: u32,
    pub row: u32,
    pub range: (u32, u32),
}

impl Number {
    pub fn is_part(&self, symbols: &HashSet<(u32, u32)>) -> bool {
        let (from_x, to_x) = self.range;
        for row in self.row - 1..=self.row + 1 {
            for x in from_x - 1..=to_x + 1 {
                if symbols.contains(&(x, row)) {
                    return true;
                }
            }
        }
        return false;
    }
}
#[derive(Debug)]
struct GearNumber {
    pub id: u32,
    pub coordinates: HashSet<(u32, u32)>,
}
