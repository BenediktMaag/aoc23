use crate::Parts;

pub fn execute(input: &str, part: &Parts) {
    let lines = input.lines();
    let games: Vec<Game> = lines.map(|line| Game::from_str(line)).collect();

    let solution: u32 = match part {
        Parts::Part1 => games.iter().filter(|x| x.is_valid()).map(|x| x.id).sum(),
        Parts::Part2 => games.iter().map(|x| x.power()).sum(),
    };

    println!("Solution is {}", solution);
}

#[derive(Debug)]
struct Game {
    id: u32,
    pullings: Vec<Pulling>,
}

impl Game {
    pub fn from_str(line: &str) -> Self {
        let mut input = line.strip_prefix("Game ").unwrap();
        let end_num = input.find(":").unwrap();
        let id = u32::from_str_radix(&input[0..end_num], 10).unwrap();
        input = &input[end_num..input.len()];
        input = input.strip_prefix(":").unwrap();

        let pullings = input
            .split(';')
            .into_iter()
            .map(|x| Pulling::from_str(x))
            .collect();

        Game { id, pullings }
    }

    pub fn is_valid(&self) -> bool {
        self.pullings.iter().map(|x| x.is_valid()).min().unwrap()
    }

    pub fn power(&self) -> u32 {
        let red_min = self.pullings.iter().max_by_key(|x| x.red).unwrap().red;
        let green_min = self.pullings.iter().max_by_key(|x| x.green).unwrap().green;
        let blue_min = self.pullings.iter().max_by_key(|x| x.blue).unwrap().blue;
        red_min * green_min * blue_min
    }
}

#[derive(Debug)]
struct Pulling {
    red: u32,
    green: u32,
    blue: u32,
}

impl Pulling {
    pub fn from_str(pulling: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for item in pulling.split(",") {
            match Pulling::parse_single_item(item) {
                ("red", value) => red = value,
                ("green", value) => green = value,
                ("blue", value) => blue = value,
                (kw, _) => panic!("Unexpected keyword {kw}"),
            }
        }
        Pulling { red, green, blue }
    }

    pub fn is_valid(&self) -> bool {
        (self.red <= 12) & (self.green <= 13) & (self.blue <= 14)
    }

    fn parse_single_item(input: &str) -> (&str, u32) {
        let item = input.strip_prefix(" ").unwrap();

        let end_num = item.find(" ").unwrap();
        let value = u32::from_str_radix(&item[0..end_num], 10).unwrap();

        let keyword = &item[end_num + 1..item.len()];

        (keyword, value)
    }
}
