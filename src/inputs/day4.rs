use crate::util::str_parsing::StringParser;
use crate::Parts;
use std::collections::HashSet;

pub fn execute(input: &str, part: &Parts) {
    let solution: u32 = match part {
        Parts::Part1 => {
            let cards: Vec<Card> = input.lines().map(|line| Card::from_str(line)).collect();
            cards.iter().map(|x| x.points()).sum()
        }
        Parts::Part2 => {
            let mut cardstack: Vec<(u32, Card)> = input
                .lines()
                .map(|line| (1, Card::from_str(line)))
                .collect();
            let mut cards_popped = 0;
            while cardstack.len() > 0 {
                let (amount, card) = cardstack.remove(0);
                cards_popped += amount;
                for id in card.get_card_ids() {
                    if id < cardstack.len() {
                        cardstack[id].0 += 1 * amount
                    }
                }
            }
            cards_popped
        }
    };

    println!("Solution is {}", solution);
}

#[derive(Debug)]
struct Card {
    _id: u32,
    winning_numbers: HashSet<u32>,
    having_numbers: Vec<u32>,
}

impl Card {
    pub fn points(&self) -> u32 {
        let winning: u32 = self
            .having_numbers
            .iter()
            .map(|x| self.winning_numbers.contains(x))
            .filter(|x| x == &true)
            .count() as u32;

        match winning {
            0 => 0,
            _ => (2 as u32).pow(winning - 1),
        }
    }

    pub fn get_card_ids(&self) -> Vec<usize> {
        let winning: usize = self
            .having_numbers
            .iter()
            .map(|x| self.winning_numbers.contains(x))
            .filter(|x| x == &true)
            .count();

        (0..winning).collect()
    }

    pub fn from_str(input: &str) -> Self {
        let mut chars = StringParser::new(input.chars());
        chars.dismiss_str("Card");
        chars.dismiss_any_whitespace();
        let id = chars.parse_num();
        chars.dismiss_char(':');
        chars.dismiss_any_whitespace();

        let mut winning_numbers = HashSet::new();

        loop {
            match chars.peek() {
                Some('|') => {
                    chars.dismiss_char('|');
                    break;
                }
                Some(' ') => chars.dismiss_any_whitespace(),
                Some('0'..='9') => {
                    winning_numbers.insert(chars.parse_num::<u32>());
                }
                Some(_) | None => {
                    panic!()
                }
            }
        }

        let mut having_numbers = Vec::new();

        loop {
            match chars.peek() {
                Some(' ') => chars.dismiss_any_whitespace(),
                Some('0'..='9') => {
                    having_numbers.push(chars.parse_num::<u32>());
                }
                Some(_) => {
                    panic!()
                }
                None => break,
            }
        }

        Card {
            _id: id,
            winning_numbers,
            having_numbers,
        }
    }
}
